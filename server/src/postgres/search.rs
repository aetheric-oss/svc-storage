use super::{get_psql_pool, ArrErr, PsqlField, PsqlFieldType};
use crate::common::PSQL_LOG_TARGET;
use crate::postgres::PsqlFieldSend;
use crate::resources::base::Resource;
use crate::resources::{
    AdvancedSearchFilter, ComparisonOperator, PredicateOperator, SortOption, SortOrder,
};

use chrono::{DateTime, Utc};
use postgres_types::ToSql;
use std::collections::VecDeque;
use tokio_postgres::Row;
use uuid::Uuid;

/// struct to save search col values while processing the [AdvancedSearchFilter](crate::resources::AdvancedSearchFilter)
/// needed to save column information for a search value so it can be converted later
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SearchCol {
    /// the [postgres_types::Type] of the column
    pub col_type: PsqlFieldType,
    /// the search column name as known in the database
    pub col_name: String,
    /// the search field as [String]
    pub value: Option<String>,
}
impl SearchCol {
    fn set_value(&mut self, val: String) {
        self.value = Some(val);
    }
}

/// Trait implementing advanced search function for resources
#[tonic::async_trait]
pub trait PsqlSearch
where
    Self: Resource + Sized,
{
    /// Generic search function based on advanced filters
    async fn advanced_search(filter: AdvancedSearchFilter) -> Result<Vec<Row>, ArrErr> {
        let definition = Self::get_definition();
        let client = get_psql_pool().get().await?;

        let mut filter_params: Vec<SearchCol> = vec![];
        let mut sort_expressions: Vec<String> = vec![];
        let mut search_query = format!(r#"SELECT * FROM "{}""#, definition.psql_table);
        let mut next_param_index: i32 = 1;

        // Go over all the filters and compose the search query string.
        for filter in filter.filters.iter() {
            let col = filter.search_field.clone();
            let field_type = definition.try_get_field(&col)?.field_type.clone();
            let operator: PredicateOperator =
                match PredicateOperator::from_i32(filter.predicate_operator) {
                    Some(val) => val,
                    None => {
                        return Err(ArrErr::Error(format!(
                            "Can't convert i32 [{}] into PredicateOperator Enum value",
                            filter.predicate_operator
                        )));
                    }
                };
            let comparison_operator = match filter.comparison_operator {
                Some(operator) => match ComparisonOperator::from_i32(operator) {
                    Some(operator) => operator.as_str_name(),
                    None => {
                        return Err(ArrErr::Error(format!(
                            "Can't convert i32 [{}] into ComparisonOperator Enum value",
                            operator
                        )));
                    }
                },
                None => "WHERE",
            };

            let (filter_str, cur_param_index) = get_filter_str(
                SearchCol {
                    col_name: col,
                    col_type: field_type,
                    value: None,
                },
                filter.search_value.clone(),
                &mut filter_params,
                next_param_index,
                operator,
            )?;

            search_query.push_str(&format!(" {} {} ", comparison_operator, filter_str));
            next_param_index = cur_param_index;
        }

        // Validate filter params making sure they are conform the column field type.
        // Adding the value to the list of query parameters if valid.
        let mut params: Vec<Box<PsqlFieldSend>> = vec![];
        for param in filter_params.iter() {
            params.push(Self::_param_from_search_col(param)?);
        }

        // Check if we need to order the results on given parameters
        if !filter.order_by.is_empty() {
            for sort_option in filter.order_by.iter() {
                if definition.has_field(&sort_option.sort_field) {
                    sort_expressions.push(try_get_sort_str(sort_option)?);
                } else {
                    psql_error!(
                        "Invalid field provided [{}] for sort order in advanced_search",
                        sort_option.sort_field
                    );
                }
            }
            search_query.push_str(&format!(" ORDER BY {}", sort_expressions.join(",")));
        }
        if filter.results_per_page >= 0 && filter.page_number > 0 {
            let offset: i64 = (filter.results_per_page * (filter.page_number - 1)).into();
            search_query.push_str(&format!(" LIMIT ${}", next_param_index));
            params.push(Box::new(filter.results_per_page as i64));
            next_param_index += 1;
            search_query.push_str(&format!(" OFFSET ${}", next_param_index));
            params.push(Box::new(offset));
        }
        let search_sql = &client.prepare_cached(&search_query).await?;

        psql_info!(
            "Searching table [{}] with query [{}]",
            definition.psql_table,
            search_query
        );

        let mut ref_params: Vec<&PsqlField> = vec![];
        for field in params.iter() {
            ref_params.push(field.as_ref());
        }
        let rows = client
            .query(search_sql, &ref_params[..])
            .await
            .map_err(ArrErr::from)?;

        Ok(rows)
    }

    /// Converts the passed string value for the search field into the right Sql type.
    /// for internal use
    fn _param_from_search_col(col: &SearchCol) -> Result<Box<dyn ToSql + Sync + Send>, ArrErr> {
        let col_val = col.value.as_ref().ok_or({
            let err = format!(
                "(_param_from_search_col) called while search col [{}] has no value",
                col.col_name,
            );
            psql_error!("{}", err);
            ArrErr::Error(err)
        })?;
        match col.col_type {
            PsqlFieldType::ANYENUM => {
                let int_val: i32 = match col_val.parse() {
                    Ok(val) => val,
                    Err(e) => {
                        let err = format!(
                            "Can't convert search col [{}] with value [{}] to i32: {}",
                            col.col_name, col_val, e
                        );
                        psql_error!("{}", err);
                        return Err(ArrErr::Error(err));
                    }
                };
                match Self::get_enum_string_val(&col.col_name.clone(), int_val) {
                    Some(string_val) => Ok(Box::new(string_val)),
                    None => {
                        let err = format!(
                            "Can't convert search col [{}] with value [{}] to enum string for value [{}]",
                            col.col_name, col_val, int_val
                        );
                        psql_error!("{}", err);
                        Err(ArrErr::Error(err))
                    }
                }
            }
            _ => param_from_search_col(col),
        }
    }
}

pub(crate) fn get_filter_str(
    mut search_col: SearchCol,
    values: Vec<String>,
    params: &mut Vec<SearchCol>,
    cur_param_index: i32,
    operator: PredicateOperator,
) -> Result<(String, i32), ArrErr> {
    let mut filter_str;
    let mut next_param_index = cur_param_index;
    psql_debug!("Found [{}] filter", operator.as_str_name());
    match operator {
        PredicateOperator::Equals => {
            filter_str = format!(r#" "{}" = ${}"#, search_col.col_name, next_param_index);
            let val: String = get_single_search_value(values)?;
            search_col.set_value(val);
            params.push(search_col);
            next_param_index += 1;
        }
        PredicateOperator::NotEquals => {
            filter_str = format!(r#" "{}" <> ${}"#, search_col.col_name, next_param_index);
            let val: String = get_single_search_value(values)?;
            search_col.set_value(val);
            params.push(search_col);
            next_param_index += 1;
        }
        PredicateOperator::In => {
            let mut search_values = vec![];
            for value in values {
                let mut col = search_col.clone();
                search_values.push(format!("${}", next_param_index));
                col.set_value(value.to_string());
                params.push(col);
                next_param_index += 1;
            }
            filter_str = format!(
                r#" "{}" IN ({})"#,
                search_col.col_name,
                search_values.join(",")
            );
        }
        PredicateOperator::Between => {
            let mut values: VecDeque<String> = (values).into();

            let min = match values.pop_front() {
                Some(val) => val,
                None => {
                    return Err(ArrErr::Error(
                            "Error in advanced search parameters. Between operator is expecting 2 values but got zero.".to_string()
                        ));
                }
            };
            filter_str = format!(
                r#" "{}" BETWEEN ${}"#,
                search_col.col_name, next_param_index
            );

            let mut col = search_col.clone();
            col.set_value(min);
            params.push(col);
            next_param_index += 1;

            let max = match values.pop_front() {
                Some(val) => val,
                None => {
                    return Err(ArrErr::Error(
                            "Error in advanced search parameters. Between operator is expecting 2 values but got only one.".to_string()
                        ));
                }
            };
            filter_str.push_str(&format!(r#" AND ${}"#, next_param_index));
            let mut col = search_col.clone();
            col.set_value(max);
            params.push(col);
            next_param_index += 1;
        }
        PredicateOperator::IsNull => {
            filter_str = format!(r#" "{}" IS NULL"#, search_col.col_name);
        }
        PredicateOperator::IsNotNull => {
            filter_str = format!(r#" "{}" IS NOT NULL"#, search_col.col_name);
        }
        PredicateOperator::Ilike => {
            filter_str = format!(
                r#" "{}"::text ILIKE ${}"#,
                search_col.col_name, next_param_index
            );
            search_col.set_value(get_single_search_value(values)?);
            params.push(search_col);
            next_param_index += 1;
        }
        PredicateOperator::Like => {
            filter_str = format!(
                r#" "{}"::text LIKE ${}"#,
                search_col.col_name, next_param_index
            );
            search_col.set_value(get_single_search_value(values)?);
            params.push(search_col);
            next_param_index += 1;
        }
        PredicateOperator::Greater => {
            filter_str = format!(r#" "{}" > ${}"#, search_col.col_name, next_param_index);
            let val: String = get_single_search_value(values)?;
            search_col.set_value(val);
            params.push(search_col);
            next_param_index += 1;
        }
        PredicateOperator::GreaterOrEqual => {
            filter_str = format!(r#" "{}" >= ${}"#, search_col.col_name, next_param_index);
            let val: String = get_single_search_value(values)?;
            search_col.set_value(val);
            params.push(search_col);
            next_param_index += 1;
        }
        PredicateOperator::Less => {
            filter_str = format!(r#" "{}" < ${}"#, search_col.col_name, next_param_index);
            let val: String = get_single_search_value(values)?;
            search_col.set_value(val);
            params.push(search_col);
            next_param_index += 1;
        }
        PredicateOperator::LessOrEqual => {
            filter_str = format!(r#" "{}" <= ${}"#, search_col.col_name, next_param_index);
            let val: String = get_single_search_value(values)?;
            search_col.set_value(val);
            params.push(search_col);
            next_param_index += 1;
        }
    }

    Ok((filter_str, next_param_index))
}

pub(crate) fn try_get_sort_str(sort_option: &SortOption) -> Result<String, ArrErr> {
    let sort_order: SortOrder = match SortOrder::from_i32(sort_option.sort_order) {
        Some(val) => val,
        None => {
            return Err(ArrErr::Error(format!(
                "Can't convert i32 [{}] into SortOperator Enum value",
                sort_option.sort_order
            )));
        }
    };

    Ok(format!(
        r#""{}" {}"#,
        sort_option.sort_field,
        sort_order.as_str_name()
    ))
}

fn get_single_search_value(search_value: Vec<String>) -> Result<String, ArrErr> {
    if search_value.len() == 1 {
        Ok(search_value[0].clone())
    } else {
        Err(ArrErr::Error(format!(
            "Error in advanced search parameters. Expecting a single value, but got [{}] values",
            search_value.len()
        )))
    }
}

/// Converts the passed string value for a field into the right Sql type.
/// for internal use
pub(super) fn param_from_search_col(
    col: &SearchCol,
) -> Result<Box<dyn ToSql + Sync + Send>, ArrErr> {
    let col_val = col.value.as_ref().ok_or({
        let err = format!(
            "(param_from_search_col) called while search col [{}] has no value",
            col.col_name,
        );
        psql_error!("{}", err);
        ArrErr::Error(err)
    })?;
    match col.col_type {
        PsqlFieldType::BOOL => match col_val.parse::<bool>() {
            Ok(val) => Ok(Box::new(val)),
            Err(e) => {
                let err = format!(
                    "Can't convert search col [{}] with value [{}] to boolean: {}",
                    col.col_name, col_val, e
                );
                psql_error!("{}", err);
                Err(ArrErr::Error(err))
            }
        },
        PsqlFieldType::NUMERIC => match col_val.parse::<f64>() {
            Ok(val) => Ok(Box::new(val)),
            Err(e) => {
                let err = format!(
                    "Can't convert search col [{}] with value [{}] to f64: {}",
                    col.col_name, col_val, e
                );
                psql_error!("{}", err);
                Err(ArrErr::Error(err))
            }
        },
        PsqlFieldType::INT2 => match col_val.parse::<i16>() {
            Ok(val) => Ok(Box::new(val)),
            Err(e) => {
                let err = format!(
                    "Can't convert search col [{}] with value [{}] to i16: {}",
                    col.col_name, col_val, e
                );
                psql_error!("{}", err);
                Err(ArrErr::Error(err))
            }
        },
        PsqlFieldType::INT4 => match col_val.parse::<i32>() {
            Ok(val) => Ok(Box::new(val)),
            Err(e) => {
                let err = format!(
                    "Can't convert search col [{}] with value [{}] to i32: {}",
                    col.col_name, col_val, e
                );
                psql_error!("{}", err);
                Err(ArrErr::Error(err))
            }
        },
        PsqlFieldType::INT8 => match col_val.parse::<i64>() {
            Ok(val) => Ok(Box::new(val)),
            Err(e) => {
                let err = format!(
                    "Can't convert search col [{}] with value [{}] to i64: {}",
                    col.col_name, col_val, e
                );
                psql_error!("{}", err);
                Err(ArrErr::Error(err))
            }
        },
        PsqlFieldType::UUID => match Uuid::parse_str(col_val) {
            Ok(val) => Ok(Box::new(val)),
            Err(e) => {
                let err = format!(
                    "Can't convert search col [{}] with value [{}] to Uuid: {}",
                    col.col_name, col_val, e
                );
                psql_error!("{}", err);
                Err(ArrErr::Error(err))
            }
        },
        PsqlFieldType::TIMESTAMPTZ => match col_val.parse::<DateTime<Utc>>() {
            Ok(val) => Ok(Box::new(val)),
            Err(e) => {
                let err = format!(
                    "Can't convert search col [{}] with value [{}] to DateTime<Utc>: {}",
                    col.col_name, col_val, e
                );
                psql_error!("{}", err);
                Err(ArrErr::Error(err))
            }
        },
        PsqlFieldType::BYTEA => {
            let val = col_val.clone().into_bytes();
            Ok(Box::new(val))
        }
        _ => Ok(Box::new(col_val.clone())),
    }
}
