use super::{ArrErr, PsqlFieldType};
use crate::common::PSQL_LOG_TARGET;
use crate::resources::{PredicateOperator, SortOption, SortOrder};
use std::collections::VecDeque;

#[derive(Eq, PartialEq, Clone, Debug)]
/// struct to save search col values while processing the [AdvancedSearchFilter](crate::resources::AdvancedSearchFilter)
/// needed to save column information for a search value so it can be converted later
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
