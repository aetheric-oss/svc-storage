//! Table search implementations

use super::{get_psql_client, ArrErr, PsqlField, PsqlFieldType};
use crate::grpc::server::{
    search::get_single_search_value, AdvancedSearchFilter, ComparisonOperator, PredicateOperator,
    SortOption, SortOrder,
};
use crate::postgres::PsqlFieldSend;
use crate::resources::base::Resource;

use lib_common::time::{DateTime, Utc};
use lib_common::uuid::Uuid;
use postgres_types::ToSql;
use std::collections::VecDeque;
use tokio_postgres::Row;

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
        let client = get_psql_client().await?;

        let mut filter_params: Vec<SearchCol> = vec![];
        let mut sort_expressions: Vec<String> = vec![];
        let mut search_query = format!(r#"SELECT * FROM "{}""#, definition.psql_table);
        let mut next_param_index: i32 = 1;

        // Go over all the filters and compose the search query string.
        for filter in filter.filters.iter() {
            let col = filter.search_field.clone();

            // Check if provided search col is part of the primary key
            let field_type = if definition.get_psql_id_cols().contains(&col) {
                PsqlFieldType::UUID
            } else {
                definition.try_get_field(&col)?.field_type.clone()
            };

            let operator: PredicateOperator =
                match PredicateOperator::try_from(filter.predicate_operator) {
                    Ok(val) => val,
                    Err(e) => {
                        return Err(ArrErr::Error(format!(
                            "Can't convert i32 [{}] into PredicateOperator Enum value: {}",
                            filter.predicate_operator, e
                        )));
                    }
                };
            let comparison_operator = match filter.comparison_operator {
                Some(operator) => match ComparisonOperator::try_from(operator) {
                    Ok(operator) => operator.as_str_name(),
                    Err(e) => {
                        return Err(ArrErr::Error(format!(
                            "Can't convert i32 [{}] into ComparisonOperator Enum value: {}",
                            operator, e
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
        for search_col in filter_params.iter() {
            params.push(Self::_param_from_search_col(search_col)?);
        }

        // Check if we need to order the results on given parameters
        if !filter.order_by.is_empty() {
            for sort_option in filter.order_by.iter() {
                if definition.has_field(&sort_option.sort_field) {
                    sort_expressions.push(try_get_sort_str(sort_option)?);
                } else {
                    psql_error!(
                        "Invalid field provided [{}] for sort order in advanced_search.",
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
            "Searching table [{}] with query [{}].",
            definition.psql_table,
            search_query
        );
        psql_debug!("Params: {:?}", params);

        let mut ref_params: Vec<&PsqlField> = vec![];
        for field in params.iter() {
            ref_params.push(field.as_ref());
        }
        let rows = client
            .query(search_sql, &ref_params[..])
            .await
            .map_err(|e| {
                let error = format!(
                    "Failed to search table [{}], an error returned from the database.",
                    definition.psql_table
                );
                psql_error!("{}: [{}]", error, e);
                ArrErr::Error(error)
            })?;

        Ok(rows)
    }

    /// Converts the passed string value for the search field into the right Sql type.
    /// for internal use
    fn _param_from_search_col(col: &SearchCol) -> Result<Box<dyn ToSql + Sync + Send>, ArrErr> {
        let col_val = match &col.value {
            Some(val) => val,
            None => {
                let err = format!("Search col [{}] has no value: {:?}", col.col_name, col);
                psql_error!("{}", err);
                return Err(ArrErr::Error(err));
            }
        };

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
                            "Can't convert search col [{}] with value [{}] to enum string for value [{}].",
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
    psql_debug!("Found [{}] filter.", operator.as_str_name());
    match operator {
        PredicateOperator::Equals => {
            filter_str = format!(r#" "{}" = ${}"#, search_col.col_name, next_param_index);
            let val: String = get_single_search_value(&values).map_err(ArrErr::Error)?;
            search_col.set_value(val);
            params.push(search_col.clone());
            next_param_index += 1;
        }
        PredicateOperator::NotEquals => {
            filter_str = format!(r#" "{}" <> ${}"#, search_col.col_name, next_param_index);
            let val: String = get_single_search_value(&values).map_err(ArrErr::Error)?;
            search_col.set_value(val);
            params.push(search_col.clone());
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
        PredicateOperator::NotIn => {
            let mut search_values = vec![];
            for value in values {
                let mut col = search_col.clone();
                search_values.push(format!("${}", next_param_index));
                col.set_value(value.to_string());
                params.push(col);
                next_param_index += 1;
            }
            filter_str = format!(
                r#" "{}" NOT IN ({})"#,
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
            search_col.set_value(format!(
                "%{}%",
                get_single_search_value(&values).map_err(ArrErr::Error)?
            ));
            params.push(search_col.clone());
            next_param_index += 1;
        }
        PredicateOperator::Like => {
            filter_str = format!(
                r#" "{}"::text LIKE ${}"#,
                search_col.col_name, next_param_index
            );
            search_col.set_value(format!(
                "%{}%",
                get_single_search_value(&values).map_err(ArrErr::Error)?
            ));
            params.push(search_col.clone());
            next_param_index += 1;
        }
        PredicateOperator::Greater => {
            filter_str = format!(r#" "{}" > ${}"#, search_col.col_name, next_param_index);
            let val: String = get_single_search_value(&values).map_err(ArrErr::Error)?;
            search_col.set_value(val);
            params.push(search_col.clone());
            next_param_index += 1;
        }
        PredicateOperator::GreaterOrEqual => {
            filter_str = format!(r#" "{}" >= ${}"#, search_col.col_name, next_param_index);
            let val: String = get_single_search_value(&values).map_err(ArrErr::Error)?;
            search_col.set_value(val);
            params.push(search_col.clone());
            next_param_index += 1;
        }
        PredicateOperator::Less => {
            filter_str = format!(r#" "{}" < ${}"#, search_col.col_name, next_param_index);
            let val: String = get_single_search_value(&values).map_err(ArrErr::Error)?;
            search_col.set_value(val);
            params.push(search_col.clone());
            next_param_index += 1;
        }
        PredicateOperator::LessOrEqual => {
            filter_str = format!(r#" "{}" <= ${}"#, search_col.col_name, next_param_index);
            let val: String = get_single_search_value(&values).map_err(ArrErr::Error)?;
            search_col.set_value(val);
            params.push(search_col.clone());
            next_param_index += 1;
        }
        PredicateOperator::GeoIntersect => {
            filter_str = format!(
                r#" st_intersects("{}", st_force2d(st_geomfromtext(${})))"#,
                search_col.col_name, next_param_index
            );
            search_col.set_value(get_single_search_value(&values).map_err(ArrErr::Error)?);
            params.push(search_col.clone());
            next_param_index += 1;
        }
        PredicateOperator::GeoWithin => {
            filter_str = format!(
                r#" st_within("{}", st_force2d(st_geomfromtext(${})))"#,
                search_col.col_name, next_param_index
            );
            search_col.set_value(get_single_search_value(&values).map_err(ArrErr::Error)?);
            params.push(search_col.clone());
            next_param_index += 1;
        }
        PredicateOperator::GeoDisjoint => {
            filter_str = format!(
                r#" st_disjoint("{}", st_force2d(st_geomfromtext(${})))"#,
                search_col.col_name, next_param_index
            );
            search_col.set_value(get_single_search_value(&values).map_err(ArrErr::Error)?);
            params.push(search_col.clone());
            next_param_index += 1;
        }
    }

    Ok((filter_str, next_param_index))
}

pub(crate) fn try_get_sort_str(sort_option: &SortOption) -> Result<String, ArrErr> {
    let sort_order: SortOrder = match SortOrder::try_from(sort_option.sort_order) {
        Ok(val) => val,
        Err(e) => {
            return Err(ArrErr::Error(format!(
                "Can't convert i32 [{}] into SortOperator Enum value: {}",
                sort_option.sort_order, e
            )));
        }
    };

    Ok(format!(
        r#""{}" {}"#,
        sort_option.sort_field,
        sort_order.as_str_name()
    ))
}

/// Converts the passed string value for a field into the right Sql type.
/// for internal use
pub(super) fn param_from_search_col(
    col: &SearchCol,
) -> Result<Box<dyn ToSql + Sync + Send>, ArrErr> {
    psql_debug!("Called for col: {:?}", col);
    let col_val = match &col.value {
        Some(val) => val,
        None => {
            let err = format!("Search col [{}] has no value: {:?}", col.col_name, col);
            psql_error!("{}", err);
            return Err(ArrErr::Error(err));
        }
    };
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
        PsqlFieldType::FLOAT4 => match col_val.parse::<f32>() {
            Ok(val) => Ok(Box::new(val)),
            Err(e) => {
                let err = format!(
                    "Can't convert search col [{}] with value [{}] to f32: {}",
                    col.col_name, col_val, e
                );
                psql_error!("{}", err);
                Err(ArrErr::Error(err))
            }
        },
        PsqlFieldType::FLOAT8 => match col_val.parse::<f64>() {
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
        PsqlFieldType::INT8_ARRAY => match col_val.parse::<i64>() {
            Ok(val) => Ok(Box::new(vec![val])),
            Err(e) => {
                let err = format!(
                    "Can't convert search col [{}] with value [{}] to Vec<i64>: {}",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::base::ResourceObject;
    use crate::{test_util::simple_resource::*, test_util::*, DEFAULT_SRID};

    #[tokio::test]
    async fn test_get_filter_str_for_equals() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_text_col"),
                col_type: PsqlFieldType::TEXT,
                value: None,
            },
            vec![String::from("test_text_value")],
            &mut filter_params,
            next_param_index,
            PredicateOperator::Equals,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 2);
        assert_eq!(filter_str, String::from(r#" "test_text_col" = $1"#));
    }

    #[tokio::test]
    async fn test_get_filter_str_for_not_equals() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_text_col"),
                col_type: PsqlFieldType::TEXT,
                value: None,
            },
            vec![String::from("test_text_value")],
            &mut filter_params,
            next_param_index,
            PredicateOperator::NotEquals,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 2);
        assert_eq!(filter_str, String::from(r#" "test_text_col" <> $1"#));
    }

    #[tokio::test]
    async fn test_get_filter_str_for_in() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_text_col"),
                col_type: PsqlFieldType::TEXT,
                value: None,
            },
            vec![String::from("test_text_value")],
            &mut filter_params,
            next_param_index,
            PredicateOperator::In,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 2);
        assert_eq!(filter_str, String::from(r#" "test_text_col" IN ($1)"#));
    }

    #[tokio::test]
    async fn test_get_filter_str_for_not_in() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_text_col"),
                col_type: PsqlFieldType::TEXT,
                value: None,
            },
            vec![String::from("test_text_value")],
            &mut filter_params,
            next_param_index,
            PredicateOperator::NotIn,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 2);
        assert_eq!(filter_str, String::from(r#" "test_text_col" NOT IN ($1)"#));
    }

    #[tokio::test]
    async fn test_get_filter_int_for_between() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_int_col"),
                col_type: PsqlFieldType::INT8,
                value: None,
            },
            vec![format!("{}", 2), format!("{}", 5)],
            &mut filter_params,
            next_param_index,
            PredicateOperator::Between,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 3);
        assert_eq!(
            filter_str,
            String::from(r#" "test_int_col" BETWEEN $1 AND $2"#)
        );
    }

    #[tokio::test]
    async fn test_get_filter_for_between_error() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        // Get error from providing zero values
        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_int_col"),
                col_type: PsqlFieldType::INT8,
                value: None,
            },
            vec![],
            &mut filter_params,
            next_param_index,
            PredicateOperator::Between,
        );
        assert!(result.is_err());
        assert_eq!(next_param_index, 1);

        // Get error from providing 1 value (should be 2)
        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_int_col"),
                col_type: PsqlFieldType::INT8,
                value: None,
            },
            vec![format!("{}", 1)],
            &mut filter_params,
            next_param_index,
            PredicateOperator::Between,
        );
        assert!(result.is_err());
        assert_eq!(next_param_index, 1);
    }

    #[tokio::test]
    async fn test_get_filter_str_for_is_null() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_text_col"),
                col_type: PsqlFieldType::TEXT,
                value: None,
            },
            vec![],
            &mut filter_params,
            next_param_index,
            PredicateOperator::IsNull,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 1);
        assert_eq!(filter_str, String::from(r#" "test_text_col" IS NULL"#));
    }

    #[tokio::test]
    async fn test_get_filter_str_for_is_not_null() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_text_col"),
                col_type: PsqlFieldType::TEXT,
                value: None,
            },
            vec![],
            &mut filter_params,
            next_param_index,
            PredicateOperator::IsNotNull,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 1);
        assert_eq!(filter_str, String::from(r#" "test_text_col" IS NOT NULL"#));
    }

    #[tokio::test]
    async fn test_get_filter_str_for_ilike() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_text_col"),
                col_type: PsqlFieldType::TEXT,
                value: None,
            },
            vec![String::from("test_text_value")],
            &mut filter_params,
            next_param_index,
            PredicateOperator::Ilike,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 2);
        assert_eq!(
            filter_str,
            String::from(r#" "test_text_col"::text ILIKE $1"#)
        );
    }

    #[tokio::test]
    async fn test_get_filter_str_for_like() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_text_col"),
                col_type: PsqlFieldType::INT8,
                value: None,
            },
            vec![String::from("test_text_value")],
            &mut filter_params,
            next_param_index,
            PredicateOperator::Like,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 2);
        assert_eq!(
            filter_str,
            String::from(r#" "test_text_col"::text LIKE $1"#)
        );
    }

    #[tokio::test]
    async fn test_get_filter_int_for_greater() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_int_col"),
                col_type: PsqlFieldType::INT8,
                value: None,
            },
            vec![format!("{}", 1)],
            &mut filter_params,
            next_param_index,
            PredicateOperator::Greater,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 2);
        assert_eq!(filter_str, String::from(r#" "test_int_col" > $1"#));
    }

    #[tokio::test]
    async fn test_get_filter_int_for_greater_or_equal() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_int_col"),
                col_type: PsqlFieldType::INT8,
                value: None,
            },
            vec![format!("{}", 1)],
            &mut filter_params,
            next_param_index,
            PredicateOperator::GreaterOrEqual,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 2);
        assert_eq!(filter_str, String::from(r#" "test_int_col" >= $1"#));
    }

    #[tokio::test]
    async fn test_get_filter_int_for_less() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_int_col"),
                col_type: PsqlFieldType::INT8,
                value: None,
            },
            vec![format!("{}", 1)],
            &mut filter_params,
            next_param_index,
            PredicateOperator::Less,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 2);
        assert_eq!(filter_str, String::from(r#" "test_int_col" < $1"#));
    }

    #[tokio::test]
    async fn test_get_filter_int_for_less_or_equal() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_int_col"),
                col_type: PsqlFieldType::INT8,
                value: None,
            },
            vec![format!("{}", 1)],
            &mut filter_params,
            next_param_index,
            PredicateOperator::LessOrEqual,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 2);
        assert_eq!(filter_str, String::from(r#" "test_int_col" <= $1"#));
    }

    #[tokio::test]
    async fn test_get_filter_for_geo_intersect() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_polygon_col"),
                col_type: PsqlFieldType::POLYGON,
                value: None,
            },
            vec![format!("{}", 1)],
            &mut filter_params,
            next_param_index,
            PredicateOperator::GeoIntersect,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 2);
        assert_eq!(
            filter_str,
            String::from(r#" st_intersects("test_polygon_col", st_force2d(st_geomfromtext($1)))"#)
        );
    }

    #[tokio::test]
    async fn test_get_filter_for_geo_within() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_polygon_col"),
                col_type: PsqlFieldType::POLYGON,
                value: None,
            },
            vec![format!("{}", 1)],
            &mut filter_params,
            next_param_index,
            PredicateOperator::GeoWithin,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 2);
        assert_eq!(
            filter_str,
            String::from(r#" st_within("test_polygon_col", st_force2d(st_geomfromtext($1)))"#)
        );
    }

    #[tokio::test]
    async fn test_get_filter_for_geo_disjoint() {
        let mut filter_params: Vec<SearchCol> = vec![];
        let next_param_index: i32 = 1;

        let result = get_filter_str(
            SearchCol {
                col_name: String::from("test_polygon_col"),
                col_type: PsqlFieldType::POLYGON,
                value: None,
            },
            vec![format!("{}", 1)],
            &mut filter_params,
            next_param_index,
            PredicateOperator::GeoDisjoint,
        );
        assert!(result.is_ok());
        let (filter_str, next_param_index) = result.unwrap();
        assert_eq!(next_param_index, 2);
        assert_eq!(
            filter_str,
            String::from(r#" st_disjoint("test_polygon_col", st_force2d(st_geomfromtext($1)))"#)
        );
    }

    #[tokio::test]
    async fn test_get_param_from_search_col() {
        assert_init_done().await;
        ut_info!("start");

        // Our Data object should have fields for each possible field_type.
        // We'll use it to loop over all the fields and test the expected return
        // value for that type.
        let definition = ResourceObject::<Data>::get_definition();
        for field in definition.get_psql_id_cols() {
            let val = Uuid::new_v4();
            let search_col = SearchCol {
                col_name: field.clone(),
                col_type: PsqlFieldType::UUID,
                value: Some(val.to_string()),
            };
            let result = param_from_search_col(&search_col);
            assert!(result.is_ok());
            let value = result.unwrap();
            assert_eq!(format!("{:?}", val), format!("{:?}", value))
        }

        for (field, field_definition) in definition.fields {
            let (string_val, display_val) = match field_definition.field_type {
                PsqlFieldType::UUID => {
                    let val = Uuid::new_v4();
                    (val.to_string(), format!("{:?}", val))
                }
                PsqlFieldType::TIMESTAMPTZ => {
                    let val = &Utc::now();
                    (val.to_string(), format!("{:?}", val))
                }
                PsqlFieldType::ANYENUM => {
                    let val = "TEST";
                    (val.to_string(), format!("{:?}", val))
                }
                PsqlFieldType::POINT => {
                    let val = format!("SRID={};POINT Z(1.0 2.0 10.0)", DEFAULT_SRID);
                    (val.to_string(), format!("{:?}", val))
                }
                PsqlFieldType::POLYGON => {
                    let val = format!("SRID={};POLYGON Z((1.1 1.1 10.0, 2.1 2.2 10.0), (3.1 3.2 10.0, 4.1 4.2 10.0))", DEFAULT_SRID);
                    (val.to_string(), format!("{:?}", val))
                }
                PsqlFieldType::PATH => {
                    let val = format!(
                        "SRID={};LineString Z(1.1 1.1 10.0, 2.1 2.2 10.0)",
                        DEFAULT_SRID
                    );
                    (val.to_string(), format!("{:?}", val))
                }
                PsqlFieldType::TEXT => {
                    let val: String = String::from("search text");
                    (val.to_string(), format!("{:?}", val))
                }
                PsqlFieldType::INT8 => {
                    let val: i64 = 64;
                    (val.to_string(), format!("{:?}", val))
                }
                PsqlFieldType::INT8_ARRAY => {
                    let val: i64 = 64;
                    (val.to_string(), format!("[{:?}]", val))
                }
                PsqlFieldType::FLOAT4 => {
                    let val: f32 = 32.0;
                    (val.to_string(), format!("{:?}", val))
                }
                PsqlFieldType::FLOAT8 => {
                    let val: f64 = 64.0;
                    (val.to_string(), format!("{:?}", val))
                }
                PsqlFieldType::JSON => {
                    let val = "[1,2,3]";
                    (val.to_string(), format!("{:?}", val))
                }
                PsqlFieldType::BOOL => {
                    let val: bool = true;
                    (val.to_string(), format!("{:?}", val))
                }
                PsqlFieldType::BYTEA => {
                    let val = b"Test".to_vec();
                    (
                        std::str::from_utf8(&val).unwrap().to_string(),
                        format!("{:?}", val),
                    )
                }
                _ => {
                    panic!(
                        "Conversion errors found in fields for table [{}], unknown field type [{}].",
                        definition.psql_table, field_definition.field_type.name()
                    )
                }
            };
            let search_col = SearchCol {
                col_name: field.clone(),
                col_type: field_definition.field_type,
                value: Some(string_val),
            };
            let result = param_from_search_col(&search_col);
            assert!(result.is_ok());
            let value = result.unwrap();
            assert_eq!(display_val, format!("{:?}", value))
        }
        ut_info!("success");
    }

    #[tokio::test]
    async fn test_get_param_from_search_col_invalid_bool() {
        assert_init_done().await;
        ut_info!("start");

        let search_col = SearchCol {
            col_name: String::from("invalid_bool"),
            col_type: PsqlFieldType::BOOL,
            value: Some(String::from("invalid")),
        };
        let result = param_from_search_col(&search_col);
        assert!(result.is_err());

        ut_info!("success");
    }
    #[tokio::test]
    async fn test_get_param_from_search_col_invalid_float4() {
        assert_init_done().await;
        ut_info!("start");

        let search_col = SearchCol {
            col_name: String::from("invalid_float4"),
            col_type: PsqlFieldType::FLOAT4,
            value: Some(String::from("invalid")),
        };
        let result = param_from_search_col(&search_col);
        assert!(result.is_err());

        ut_info!("success");
    }
    #[tokio::test]
    async fn test_get_param_from_search_col_invalid_float8() {
        assert_init_done().await;
        ut_info!("start");

        let search_col = SearchCol {
            col_name: String::from("invalid_float8"),
            col_type: PsqlFieldType::FLOAT8,
            value: Some(String::from("invalid")),
        };
        let result = param_from_search_col(&search_col);
        assert!(result.is_err());

        ut_info!("success");
    }
    #[tokio::test]
    async fn test_get_param_from_search_col_invalid_int8() {
        assert_init_done().await;
        ut_info!("start");

        let search_col = SearchCol {
            col_name: String::from("invalid_int8"),
            col_type: PsqlFieldType::INT8,
            value: Some(String::from("invalid")),
        };
        let result = param_from_search_col(&search_col);
        assert!(result.is_err());

        ut_info!("success");
    }
    #[tokio::test]
    async fn test_get_param_from_search_col_invalid_uuid() {
        assert_init_done().await;
        ut_info!("start");

        let search_col = SearchCol {
            col_name: String::from("invalid_uuid"),
            col_type: PsqlFieldType::UUID,
            value: Some(String::from("invalid")),
        };
        let result = param_from_search_col(&search_col);
        assert!(result.is_err());

        ut_info!("success");
    }
    #[tokio::test]
    async fn test_get_param_from_search_col_invalid_timestamptz() {
        assert_init_done().await;
        ut_info!("start");

        let search_col = SearchCol {
            col_name: String::from("invalid_timestamptz"),
            col_type: PsqlFieldType::TIMESTAMPTZ,
            value: Some(String::from("invalid")),
        };
        let result = param_from_search_col(&search_col);
        assert!(result.is_err());

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_try_get_sort_str() {
        assert_init_done().await;
        ut_info!("start");

        // check returned string for ASC
        let mut sort_option = SortOption {
            sort_field: String::from("test"),
            sort_order: 0, // ASC
        };
        let result = try_get_sort_str(&sort_option);
        assert!(result.is_ok());

        let search_str = result.unwrap();
        assert_eq!(r#""test" ASC"#, search_str);

        // check returning string for DESC
        sort_option.sort_order = 1;
        let result = try_get_sort_str(&sort_option);
        assert!(result.is_ok());

        let search_str = result.unwrap();
        assert_eq!(r#""test" DESC"#, search_str);

        // invalid sort_order
        sort_option.sort_order = 10;
        let result = try_get_sort_str(&sort_option);
        assert!(result.is_err());
    }
}
