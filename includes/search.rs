use super::{AdvancedSearchFilter, ComparisonOperator, FilterOption, PredicateOperator};

/// Implement helper functions for [AdvancedSearchFilter] to provide a more readable way to
/// build up the struct's values.
///
/// Examples:
/// ```
/// use svc_storage_client_grpc::AdvancedSearchFilter;
/// let filter = AdvancedSearchFilter::search_equals(String::from("status"), String::from("enabled"))
///     .and_equals(String::from("resource_id"), String::from("53acfe06-dd9b-42e8-8cb4-12a2fb2fa693"))
///     .and_between(String::from("created_at"), String::from("2022-04-10 22:10:57+02:00"), String::from("2022-04-12 22:10:57+02:00"))
///     .results_per_page(10)
///     .page_number(1);
/// ```
impl AdvancedSearchFilter {
    fn search(column: String, values: Vec<String>, predicate_operator: PredicateOperator) -> Self {
        Self {
            filters: vec![FilterOption {
                search_field: column,
                search_value: values,
                predicate_operator: predicate_operator.into(),
                comparison_operator: None,
            }],
            page_number: 0,
            results_per_page: -1,
            order_by: vec![],
        }
    }

    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Equals]
    /// * comparison operator: [None]
    pub fn search_equals(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::Equals)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::NotEquals]
    /// * comparison operator: [None]
    pub fn search_not_equals(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::NotEquals)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `values` [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::In]
    /// * comparison operator: [None]
    pub fn search_in(column: String, values: Vec<String>) -> Self {
        Self::search(column, values, PredicateOperator::In)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `min` and `max` values as entries in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Between]
    /// * comparison operator: [None]
    pub fn search_between(column: String, min: String, max: String) -> Self {
        Self::search(column, vec![min, max], PredicateOperator::Between)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: an empty [Vec]
    /// * predicate operator: [PredicateOperator::IsNull]
    /// * comparison operator: [None]
    pub fn search_is_null(column: String) -> Self {
        Self::search(column, vec![], PredicateOperator::IsNull)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: an empty [Vec]
    /// * predicate operator: [PredicateOperator::IsNotNull]
    /// * comparison operator: [None]
    pub fn search_is_not_null(column: String) -> Self {
        Self::search(column, vec![], PredicateOperator::IsNotNull)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Ilike]
    /// * comparison operator: [None]
    pub fn search_ilike(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::Ilike)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Like]
    /// * comparison operator: [None]
    pub fn search_like(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::Like)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Greater]
    /// * comparison operator: [None]
    pub fn greater(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::Greater)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::GreaterOrEqual]
    /// * comparison operator: [None]
    pub fn greater_or_equal(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::GreaterOrEqual)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Less]
    /// * comparison operator: [None]
    pub fn less(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::Less)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::LessOrEqual]
    /// * comparison operator: [None]
    pub fn less_or_equal(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::LessOrEqual)
    }

    fn add_filter(
        mut self,
        column: String,
        values: Vec<String>,
        predicate_operator: PredicateOperator,
        comparison_operator: ComparisonOperator,
    ) -> Self {
        self.filters.push(FilterOption {
            search_field: column,
            search_value: values,
            predicate_operator: predicate_operator.into(),
            comparison_operator: Some(comparison_operator.into()),
        });
        self
    }

    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Equals]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_equals(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::Equals,
            ComparisonOperator::And,
        )
    }
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::NotEquals]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_not_equals(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::NotEquals,
            ComparisonOperator::And,
        )
    }
    /// wrapper function for internal `add_filter` function, adding a new [FilterOption] to itself while returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `values` [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::In]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_in(self, column: String, values: Vec<String>) -> Self {
        self.add_filter(
            column,
            values,
            PredicateOperator::In,
            ComparisonOperator::And,
        )
    }
    /// wrapper function for internal `add_filter` function, adding a new [FilterOption] to itself while returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `min` and `max` values as entries in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Between]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_between(self, column: String, min: String, max: String) -> Self {
        self.add_filter(
            column,
            vec![min, max],
            PredicateOperator::Between,
            ComparisonOperator::And,
        )
    }
    /// wrapper function for internal `add_filter` function, adding a new [FilterOption] to itself while returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: an empty [Vec]
    /// * predicate operator: [PredicateOperator::IsNull]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_is_null(self, column: String) -> Self {
        self.add_filter(
            column,
            vec![],
            PredicateOperator::IsNull,
            ComparisonOperator::And,
        )
    }
    /// wrapper function for internal `add_filter` function, adding a new [FilterOption] to itself while returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: an empty [Vec]
    /// * predicate operator: [PredicateOperator::IsNotNull]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_is_not_null(self, column: String) -> Self {
        self.add_filter(
            column,
            vec![],
            PredicateOperator::IsNotNull,
            ComparisonOperator::And,
        )
    }
    /// wrapper function for internal `add_filter` function, adding a new [FilterOption] to itself while returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Ilike]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_ilike(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::Ilike,
            ComparisonOperator::And,
        )
    }
    /// wrapper function for internal `add_filter` function, adding a new [FilterOption] to itself while returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Like]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_like(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::Like,
            ComparisonOperator::And,
        )
    }
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Greater]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_greater(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::Greater,
            ComparisonOperator::And,
        )
    }
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::GreaterOrEqual]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_greater_or_equal(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::GreaterOrEqual,
            ComparisonOperator::And,
        )
    }
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Less]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_less(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::Less,
            ComparisonOperator::And,
        )
    }
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::LessOrEqual]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_less_or_equal(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::LessOrEqual,
            ComparisonOperator::And,
        )
    }

    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Equals]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_equals(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::Equals,
            ComparisonOperator::Or,
        )
    }
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::NotEquals]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_not_equals(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::NotEquals,
            ComparisonOperator::Or,
        )
    }
    /// wrapper function for internal `add_filter` function, adding a new [FilterOption] to itself while returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `values` [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::In]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_in(self, column: String, values: Vec<String>) -> Self {
        self.add_filter(
            column,
            values,
            PredicateOperator::In,
            ComparisonOperator::Or,
        )
    }
    /// wrapper function for internal `add_filter` function, adding a new [FilterOption] to itself while returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `min` or `max` values as entries in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Between]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_between(self, column: String, min: String, max: String) -> Self {
        self.add_filter(
            column,
            vec![min, max],
            PredicateOperator::Between,
            ComparisonOperator::Or,
        )
    }
    /// wrapper function for internal `add_filter` function, adding a new [FilterOption] to itself while returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: an empty [Vec]
    /// * predicate operator: [PredicateOperator::IsNull]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_is_null(self, column: String) -> Self {
        self.add_filter(
            column,
            vec![],
            PredicateOperator::IsNull,
            ComparisonOperator::Or,
        )
    }
    /// wrapper function for internal `add_filter` function, adding a new [FilterOption] to itself while returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: an empty [Vec]
    /// * predicate operator: [PredicateOperator::IsNotNull]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_is_not_null(self, column: String) -> Self {
        self.add_filter(
            column,
            vec![],
            PredicateOperator::IsNotNull,
            ComparisonOperator::Or,
        )
    }
    /// wrapper function for internal `add_filter` function, adding a new [FilterOption] to itself while returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Ilike]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_ilike(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::Ilike,
            ComparisonOperator::Or,
        )
    }
    /// wrapper function for internal `add_filter` function, adding a new [FilterOption] to itself while returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Like]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_like(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::Like,
            ComparisonOperator::Or,
        )
    }
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Greater]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_greater(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::Greater,
            ComparisonOperator::Or,
        )
    }
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::GreaterOrEqual]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_greater_or_equal(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::GreaterOrEqual,
            ComparisonOperator::Or,
        )
    }
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Less]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_less(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::Less,
            ComparisonOperator::Or,
        )
    }
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::LessOrEqual]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_less_or_equal(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::LessOrEqual,
            ComparisonOperator::Or,
        )
    }

    /// sets `results_per_page` field with given `amount`
    pub fn results_per_page(mut self, amount: i32) -> Self {
        self.results_per_page = amount;
        self
    }

    /// sets `page_number` field with given `page`
    pub fn page_number(mut self, page: i32) -> Self {
        self.page_number = page;
        self
    }
}
