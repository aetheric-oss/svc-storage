use super::{AdvancedSearchFilter, ComparisonOperator, FilterOption, PredicateOperator};

/// Implement helper functions for [AdvancedSearchFilter] to provide a more readable way to
/// build up the struct's values.
///
/// Examples:
/// ```
/// use svc_storage::resources::AdvancedSearchFilter;
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
    /// * search_value: the provided `values` [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::NotIn]
    /// * comparison operator: [None]
    pub fn search_not_in(column: String, values: Vec<String>) -> Self {
        Self::search(column, values, PredicateOperator::NotIn)
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
    pub fn search_greater(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::Greater)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::GreaterOrEqual]
    /// * comparison operator: [None]
    pub fn search_greater_or_equal(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::GreaterOrEqual)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::Less]
    /// * comparison operator: [None]
    pub fn search_less(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::Less)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::LessOrEqual]
    /// * comparison operator: [None]
    pub fn search_less_or_equal(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::LessOrEqual)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a
    ///   [Vec\<String\>] in a [Well Know Text](https://www.cockroachlabs.com/docs/v23.1/well-known-text) format
    /// * predicate operator: [PredicateOperator::GeoIntersect]
    /// * comparison operator: [None]
    pub fn search_geo_intersect(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::GeoIntersect)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a
    ///   [Vec\<String\>] in a [Well Know Text](https://www.cockroachlabs.com/docs/v23.1/well-known-text) format
    /// * predicate operator: [PredicateOperator::GeoWithin]
    /// * comparison operator: [None]
    pub fn search_geo_within(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::GeoWithin)
    }
    /// wrapper function for internal `search` function returning a new [AdvancedSearchFilter] object
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a
    ///   [Vec\<String\>] in a [Well Know Text](https://www.cockroachlabs.com/docs/v23.1/well-known-text) format
    /// * predicate operator: [PredicateOperator::GeoDisjoint]
    /// * comparison operator: [None]
    pub fn search_geo_disjoint(column: String, value: String) -> Self {
        Self::search(column, vec![value], PredicateOperator::GeoDisjoint)
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
    /// * search_value: the provided `values` [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::NotIn]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_not_in(self, column: String, values: Vec<String>) -> Self {
        self.add_filter(
            column,
            values,
            PredicateOperator::NotIn,
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
    /// * search_value: the provided `value` as single entry in a
    ///   [Vec\<String\>] in a [Well Know Text](https://www.cockroachlabs.com/docs/v23.1/well-known-text) format
    /// * predicate operator: [PredicateOperator::GeoIntersect]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_geo_intersect(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::GeoIntersect,
            ComparisonOperator::And,
        )
    }
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a
    ///   [Vec\<String\>] in a [Well Know Text](https://www.cockroachlabs.com/docs/v23.1/well-known-text) format
    /// * predicate operator: [PredicateOperator::GeoWithin]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_geo_within(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::GeoWithin,
            ComparisonOperator::And,
        )
    }
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a
    ///   [Vec\<String\>] in a [Well Know Text](https://www.cockroachlabs.com/docs/v23.1/well-known-text) format
    /// * predicate operator: [PredicateOperator::GeoDisjoint]
    /// * comparison operator: [ComparisonOperator::And]
    pub fn and_geo_disjoint(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::GeoDisjoint,
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
    /// * search_value: the provided `values` [Vec\<String\>]
    /// * predicate operator: [PredicateOperator::NotIn]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_not_in(self, column: String, values: Vec<String>) -> Self {
        self.add_filter(
            column,
            values,
            PredicateOperator::NotIn,
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
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a
    ///   [Vec\<String\>] in a [Well Know Text](https://www.cockroachlabs.com/docs/v23.1/well-known-text) format
    /// * predicate operator: [PredicateOperator::GeoIntersect]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_geo_intersect(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::GeoIntersect,
            ComparisonOperator::Or,
        )
    }
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a
    ///   [Vec\<String\>] in a [Well Know Text](https://www.cockroachlabs.com/docs/v23.1/well-known-text) format
    /// * predicate operator: [PredicateOperator::GeoWithin]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_geo_within(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::GeoWithin,
            ComparisonOperator::Or,
        )
    }
    /// wrapper function for internal `add_filter` function returning [Self]
    ///
    /// Adds a [FilterOption] to `filters` using:
    /// * search_field: the provided `column` [String]
    /// * search_value: the provided `value` as single entry in a
    ///   [Vec\<String\>] in a [Well Know Text](https://www.cockroachlabs.com/docs/v23.1/well-known-text) format
    /// * predicate operator: [PredicateOperator::GeoDisjoint]
    /// * comparison operator: [ComparisonOperator::Or]
    pub fn or_geo_disjoint(self, column: String, value: String) -> Self {
        self.add_filter(
            column,
            vec![value],
            PredicateOperator::GeoDisjoint,
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

/// Helper function for search library to get a single value from the provided
/// values field.
// allow dead_code is added for the client library since it only needs this
// function while using the stub_client feature. The server needs it at all
// times though.
#[allow(dead_code)]
pub(crate) fn get_single_search_value(search_value: &Vec<String>) -> Result<String, String> {
    grpc_debug!(
        "(get_single_search_value) get value from: {:?}.",
        search_value
    );
    if search_value.len() == 1 {
        Ok(search_value[0].clone())
    } else {
        Err(format!(
            "Error in advanced search parameters. Expecting a single value, but got [{}] values",
            search_value.len()
        ))
    }
}

#[cfg(any(feature = "stub_client", feature = "stub_server"))]
pub(crate) fn filter_for_operator(
    search_field: &str,
    search_values: &Vec<String>,
    unfiltered: &Vec<serde_json::Value>,
    filtered: &mut Vec<serde_json::Value>,
    operator: PredicateOperator,
) -> Result<(), String> {
    for object in unfiltered {
        let mut val = match search_field {
            "id" => object[search_field].clone(),
            _ => {
                let data = &object["data"];
                data[search_field].clone()
            }
        };

        let ids = &object["ids"];
        println!("(filter_for_operator) (MOCK) test ids field [{}].", ids,);
        if val == serde_json::Value::Null && *ids != serde_json::Value::Null {
            println!("(filter_for_operator) (MOCK) found ids [{}].", ids,);
            serde_json::from_value::<Vec<super::FieldValue>>(ids.clone())
                .map_err(|e| {
                    format!(
                        "Could not convert [{:?}] to Ids from json value: {}",
                        ids, e
                    )
                })?
                .iter()
                .find(|id| id.field == search_field)
                .map(|id| val = serde_json::Value::String(id.value.clone()));
        }

        println!(
            "(filter_for_operator) (MOCK) got value [{}] for object [{}].",
            val, object
        );
        match operator {
            PredicateOperator::Equals => {
                let search_val: String = get_single_search_value(search_values)?;
                let cmp_val: String;
                if let Some(string) = val.as_str() {
                    cmp_val = string.to_string();
                } else {
                    cmp_val = format!("{}", val);
                }
                println!(
                    "(filter_for_operator) (MOCK) Equals filter with value [{}] for val [{}].",
                    search_val, cmp_val
                );
                if cmp_val == *search_val {
                    println!("(filter_for_operator) (MOCK) Equals found!");
                    filtered.push(object.clone())
                }
            }
            PredicateOperator::NotEquals => {
                let search_val: String = get_single_search_value(search_values)?;
                let cmp_val: String;
                if let Some(string) = val.as_str() {
                    cmp_val = string.to_string();
                } else {
                    cmp_val = format!("{}", val);
                }
                println!(
                    "(filter_for_operator) (MOCK) NotEquals filter with value [{}] for val [{}].",
                    search_val, val
                );
                if cmp_val != *search_val {
                    println!("(filter_for_operator) (MOCK) NotEquals found!");
                    filtered.push(object.clone())
                }
            }
            PredicateOperator::In => {
                let cmp_val: String;
                if let Some(string) = val.as_str() {
                    cmp_val = string.to_string();
                } else {
                    cmp_val = format!("{}", val);
                }

                println!(
                    "(filter_for_operator) (MOCK) In filter with values [{:?}] for val [{}].",
                    search_values, cmp_val
                );

                let _ = search_values
                    .iter()
                    .find(|&search_val| search_val == &cmp_val)
                    .map(|_| {
                        println!("(filter_for_operator) (MOCK) In found!");
                        filtered.push(object.clone())
                    });
            }
            PredicateOperator::NotIn => {
                let cmp_val: String;
                if let Some(string) = val.as_str() {
                    cmp_val = string.to_string();
                } else {
                    cmp_val = format!("{}", val);
                }

                println!(
                    "(filter_for_operator) (MOCK) NotIn filter with values [{:?}] for val [{}].",
                    search_values, cmp_val
                );

                let _ = search_values
                    .iter()
                    .find(|&search_val| search_val == &cmp_val)
                    .ok_or_else(|| {
                        println!("(filter_for_operator) (MOCK) NotIn found!");
                        filtered.push(object.clone());
                    });
            }
            PredicateOperator::Between => {
                println!(
                    "(filter_for_operator) (MOCK) Between filter with values [{:?}] for val [{}].",
                    search_values, val
                );
                let mut values: std::collections::VecDeque<String> = search_values.clone().into();

                let min = match values.pop_front() {
                    Some(val) => val,
                    None => {
                        return Err("Error in advanced search parameters. Between operator is expecting 2 values but got zero.".to_string());
                    }
                };
                let max = match values.pop_front() {
                    Some(val) => val,
                    None => {
                        return Err("Error in advanced search parameters. Between operator is expecting 2 values but got only one.".to_string());
                    }
                };
                println!(
                    "(filter_for_operator) (MOCK) Found min [{}] and max [{}] values to compare with.",
                    min,
                    max
                );

                if let Some(num_val) = val.as_f64() {
                    println!(
                        "(filter_for_operator) (MOCK) Can convert val to number, got [{}].",
                        num_val
                    );
                    let num_min = min.parse::<f64>().map_err(|e| {
                        format!("Could not convert search_value min [{}] to f64: {}", min, e)
                    })?;
                    let num_max = max.parse::<f64>().map_err(|e| {
                        format!("Could not convert search_value max [{}] to f64: {}", max, e)
                    })?;
                    if num_val >= num_min && num_val <= num_max {
                        println!("(filter_for_operator) (MOCK) Between found!");
                        filtered.push(object.clone())
                    }
                } else if let Ok(date_val) = lib_common::time::DateTime::parse_from_rfc3339(
                    val.as_str()
                    .ok_or("Could not convert provided value to string.")?,
                ) {
                    println!(
                        "(filter_for_operator) (MOCK) Can convert val to date, got [{}].",
                        date_val
                    );
                    let date_min =
                        lib_common::time::DateTime::parse_from_rfc3339(&min).map_err(|e| {
                            format!(
                                "Could not convert search_value min [{}] to date: {}",
                                min, e
                            )
                        })?;

                    let date_max =
                        lib_common::time::DateTime::parse_from_rfc3339(&max).map_err(|e| {
                            format!(
                                "Could not convert search_value max [{}] to date: {}",
                                min, e
                            )
                        })?;
                    if date_val >= date_min && date_val <= date_max {
                        println!("(filter_for_operator) (MOCK) Between found!");
                        filtered.push(object.clone())
                    }
                } else {
                    grpc_warn!(
                        "(filter_for_operator) (MOCK) Can't convert val [{}] to number or date, don't know what to do.",
                        &val.to_string()
                    );
                }
            }
            PredicateOperator::IsNull => {
                println!(
                    "(filter_for_operator) (MOCK) IsNull filter for value [{}].",
                    val
                );
                if val.is_null() {
                    println!("(filter_for_operator) (MOCK) IsNull found!");
                    filtered.push(object.clone())
                }
            }
            PredicateOperator::IsNotNull => {
                println!(
                    "(filter_for_operator) (MOCK) IsNotNull filter for value [{}].",
                    val
                );
                if !val.is_null() {
                    println!("(filter_for_operator) (MOCK) IsNotNull found!");
                    filtered.push(object.clone())
                }
            }
            PredicateOperator::Ilike => {
                let search_val: String = get_single_search_value(search_values)?;
                let cmp_val: String;
                if let Some(string) = val.as_str() {
                    cmp_val = string.to_string();
                } else {
                    cmp_val = format!("{}", val);
                }
                println!(
                    "(filter_for_operator) (MOCK) Ilike filter with value [{}] for val [{}].",
                    search_val, cmp_val
                );
                if cmp_val.to_lowercase().contains(&search_val.to_lowercase()) {
                    println!("(filter_for_operator) (MOCK) Ilike found!");
                    filtered.push(object.clone())
                }
            }
            PredicateOperator::Like => {
                let search_val: String = get_single_search_value(search_values)?;
                let cmp_val: String;
                if let Some(string) = val.as_str() {
                    cmp_val = string.to_string();
                } else {
                    cmp_val = format!("{}", val);
                }
                println!(
                    "(filter_for_operator) (MOCK) Like filter with value [{}] for val [{}].",
                    search_val, cmp_val
                );
                if cmp_val.contains(&search_val) {
                    println!("(filter_for_operator) (MOCK) Like found!");
                    filtered.push(object.clone())
                }
            }
            PredicateOperator::Greater => {
                let search_val: String = get_single_search_value(search_values)?;
                println!(
                    "(filter_for_operator) (MOCK) Greater filter with value [{:?}] for val [{}].",
                    search_val, val
                );
                if let Some(num_val) = val.as_f64() {
                    println!(
                        "(filter_for_operator) (MOCK) Can convert val to number, got [{}].",
                        num_val
                    );
                    let num_search_val = search_val.parse::<f64>().map_err(|e| {
                        format!(
                            "Could not convert search_value [{}] to f64: {}",
                            search_val, e
                        )
                    })?;
                    if num_val > num_search_val {
                        println!("(filter_for_operator) (MOCK) Greater found!");
                        filtered.push(object.clone())
                    }
                } else if let Ok(date_val) = lib_common::time::DateTime::parse_from_rfc3339(
                    val.as_str()
                    .ok_or("Could not convert provided value to string.")?,
                ) {
                    println!(
                        "(filter_for_operator) (MOCK) Can convert val to date, got [{}].",
                        date_val
                    );
                    let search_date = lib_common::time::DateTime::parse_from_rfc3339(&search_val)
                        .map_err(|e| {
                            format!(
                                "Could not convert search_value [{}] to date: {}",
                                search_val, e
                            )
                        })?;
                    if date_val > search_date {
                        println!("(filter_for_operator) (MOCK) Greater found!");
                        filtered.push(object.clone())
                    }
                } else {
                    grpc_warn!(
                        "(filter_for_operator) (MOCK) Can't convert val [{}] to number or date, don't know what to do.",
                        &val.to_string()
                    );
                }
            }
            PredicateOperator::GreaterOrEqual => {
                let search_val: String = get_single_search_value(search_values)?;
                println!(
                    "(filter_for_operator) (MOCK) GreaterOrEqual filter with value [{:?}] for val [{}].",
                    search_val,
                    val
                );
                if let Some(num_val) = val.as_f64() {
                    println!(
                        "(filter_for_operator) (MOCK) Can convert val to number, got [{}].",
                        num_val
                    );
                    let num_search_val = search_val.parse::<f64>().map_err(|e| {
                        format!(
                            "Could not convert search_value [{}] to f64: {}",
                            search_val, e
                        )
                    })?;
                    if num_val >= num_search_val {
                        println!("(filter_for_operator) (MOCK) GreaterOrEqual found!");
                        filtered.push(object.clone())
                    }
                } else if let Ok(date_val) = lib_common::time::DateTime::parse_from_rfc3339(
                    val.as_str()
                    .ok_or("Could not convert provided value to string.")?,
                ) {
                    println!(
                        "(filter_for_operator) (MOCK) Can convert val to date, got [{}].",
                        date_val
                    );
                    let search_date = lib_common::time::DateTime::parse_from_rfc3339(&search_val)
                        .map_err(|e| {
                            format!(
                                "Could not convert search_value [{}] to date: {}",
                                search_val, e
                            )
                        })?;
                    if date_val >= search_date {
                        println!("(filter_for_operator) (MOCK) GreaterOrEqual found!");
                        filtered.push(object.clone())
                    }
                } else {
                    grpc_warn!(
                        "(filter_for_operator) (MOCK) Can't convert val [{}] to number or date, don't know what to do.",
                        &val.to_string()
                    );
                }
            }
            PredicateOperator::Less => {
                let search_val: String = get_single_search_value(search_values)?;
                println!(
                    "(filter_for_operator) (MOCK) Less filter with value [{:?}] for val [{}].",
                    search_val, val
                );
                if let Some(num_val) = val.as_f64() {
                    println!(
                        "(filter_for_operator) (MOCK) Can convert val to number, got [{}].",
                        num_val
                    );
                    let num_search_val = search_val.parse::<f64>().map_err(|e| {
                        format!(
                            "Could not convert search_value [{}] to f64: {}",
                            search_val, e
                        )
                    })?;
                    if num_val < num_search_val {
                        println!("(filter_for_operator) (MOCK) Less found!");
                        filtered.push(object.clone())
                    }
                } else if let Ok(date_val) = lib_common::time::DateTime::parse_from_rfc3339(
                    val.as_str()
                    .ok_or("Could not convert provided value to string.")?,
                ) {
                    println!(
                        "(filter_for_operator) (MOCK) Can convert val to date, got [{}].",
                        date_val
                    );
                    let search_date = lib_common::time::DateTime::parse_from_rfc3339(&search_val)
                        .map_err(|e| {
                            format!(
                                "Could not convert search_value [{}] to date: {}",
                                search_val, e
                            )
                        })?;
                    if date_val < search_date {
                        println!("(filter_for_operator) (MOCK) Less found!");
                        filtered.push(object.clone())
                    }
                } else {
                    grpc_warn!(
                        "(filter_for_operator) (MOCK) Can't convert val [{}] to number or date, don't know what to do.",
                        &val.to_string()
                    );
                }
            }
            PredicateOperator::LessOrEqual => {
                let search_val: String = get_single_search_value(search_values)?;
                println!(
                    "(filter_for_operator) (MOCK) LessOrEqual filter with value [{:?}] for val [{}].",
                    search_val,
                    val
                );
                if let Some(num_val) = val.as_f64() {
                    println!(
                        "(filter_for_operator) (MOCK) Can convert val to number, got [{}].",
                        num_val
                    );
                    let num_search_val = search_val.parse::<f64>().map_err(|e| {
                        format!(
                            "Could not convert search_value [{}] to f64: {}",
                            search_val, e
                        )
                    })?;
                    if num_val <= num_search_val {
                        println!("(filter_for_operator) (MOCK) LessOrEqual found!");
                        filtered.push(object.clone())
                    }
                } else if let Ok(date_val) = lib_common::time::DateTime::parse_from_rfc3339(
                    val.as_str()
                    .ok_or("Could not convert provided value to string.")?,
                ) {
                    println!(
                        "(filter_for_operator) (MOCK) Can convert val to date, got [{}].",
                        date_val
                    );
                    let search_date = lib_common::time::DateTime::parse_from_rfc3339(&search_val)
                        .map_err(|e| {
                            format!(
                                "Could not convert search_value [{}] to date: {}",
                                search_val, e
                            )
                        })?;
                    if date_val <= search_date {
                        println!("(filter_for_operator) (MOCK) LessOrEqual found!");
                        filtered.push(object.clone())
                    }
                } else {
                    grpc_warn!(
                        "(filter_for_operator) (MOCK) Can't convert val [{}] to number or date, don't know what to do.",
                        &val.to_string()
                    );
                }
            }
            PredicateOperator::GeoIntersect => {
                /*
                filter_str = format!(
                r#" st_intersect(st_geomfromtext(${}), "{}")"#,
                next_param_index, search_col.col_name,
                );
                search_col.set_value(get_single_search_value(values)?);
                params.push(search_col.clone());
                next_param_index += 1;
                */
            }
            PredicateOperator::GeoWithin => {
                /*
                filter_str = format!(
                r#" st_within(st_geomfromtext(${}), "{}")"#,
                next_param_index, search_col.col_name,
                );
                search_col.set_value(get_single_search_value(values)?);
                params.push(search_col.clone());
                next_param_index += 1;
                */
            }
            PredicateOperator::GeoDisjoint => {
                /*
                filter_str = format!(
                r#" st_disjoint(st_geomfromtext(${}), "{}")"#,
                next_param_index, search_col.col_name,
                );
                search_col.set_value(get_single_search_value(values)?);
                params.push(search_col.clone());
                next_param_index += 1;
                */
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::SortOrder;
    use super::*;

    // Test all of search, and, or options for predicate operator; equals
    #[test]
    fn test_search_equals() {
        let filter =
            AdvancedSearchFilter::search_equals(String::from("equals"), String::from("test"))
                .and_equals(String::from("and_equals"), String::from("test"))
                .or_equals(String::from("or_equals"), String::from("test"));

        assert_eq!(filter.filters.len(), 3);

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "equals");
        assert_eq!(filter_option1.search_value, vec!["test"]);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::Equals as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_equals");
        assert_eq!(filter_option2.search_value, vec!["test"]);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::Equals as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_equals");
        assert_eq!(filter_option3.search_value, vec!["test"]);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::Equals as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; not_equals
    #[test]
    fn test_search_not_equals() {
        let filter = AdvancedSearchFilter::search_not_equals(
            String::from("not_equals"),
            String::from("test"),
        )
        .and_not_equals(String::from("and_not_equals"), String::from("test"))
        .or_not_equals(String::from("or_not_equals"), String::from("test"));

        assert_eq!(filter.filters.len(), 3);

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "not_equals");
        assert_eq!(filter_option1.search_value, vec!["test"]);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::NotEquals as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_not_equals");
        assert_eq!(filter_option2.search_value, vec!["test"]);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::NotEquals as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_not_equals");
        assert_eq!(filter_option3.search_value, vec!["test"]);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::NotEquals as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; in
    #[test]
    fn test_search_in() {
        let filter =
            AdvancedSearchFilter::search_in(String::from("in"), vec![String::from("test")])
                .and_in(String::from("and_in"), vec![String::from("test")])
                .or_in(String::from("or_in"), vec![String::from("test")]);

        assert_eq!(filter.filters.len(), 3);

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "in");
        assert_eq!(filter_option1.search_value, vec!["test"]);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::In as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_in");
        assert_eq!(filter_option2.search_value, vec!["test"]);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::In as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_in");
        assert_eq!(filter_option3.search_value, vec!["test"]);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::In as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; not_in
    #[test]
    fn test_search_not_in() {
        let filter =
            AdvancedSearchFilter::search_not_in(String::from("not_in"), vec![String::from("test")])
                .and_not_in(String::from("and_not_in"), vec![String::from("test")])
                .or_not_in(String::from("or_not_in"), vec![String::from("test")]);

        assert_eq!(filter.filters.len(), 3);

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "not_in");
        assert_eq!(filter_option1.search_value, vec!["test"]);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::NotIn as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_not_in");
        assert_eq!(filter_option2.search_value, vec!["test"]);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::NotIn as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_not_in");
        assert_eq!(filter_option3.search_value, vec!["test"]);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::NotIn as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; between
    #[test]
    fn test_search_between() {
        let filter = AdvancedSearchFilter::search_between(
            String::from("between"),
            1.to_string(),
            10.to_string(),
        )
        .and_between(String::from("and_between"), 1.to_string(), 5.to_string())
        .or_between(String::from("or_between"), 7.to_string(), 5.to_string());

        assert_eq!(filter.filters.len(), 3);

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "between");
        assert_eq!(filter_option1.search_value, vec!["1", "10"]);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::Between as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_between");
        assert_eq!(filter_option2.search_value, vec!["1", "5"]);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::Between as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_between");
        assert_eq!(filter_option3.search_value, vec!["7", "5"]);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::Between as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; is_null
    #[test]
    fn test_search_is_null() {
        let filter = AdvancedSearchFilter::search_is_null(String::from("is_null"))
            .and_is_null(String::from("and_is_null"))
            .or_is_null(String::from("or_is_null"));

        assert_eq!(filter.filters.len(), 3);

        let expected_values: Vec<String> = vec![];

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "is_null");
        assert_eq!(filter_option1.search_value, expected_values);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::IsNull as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_is_null");
        assert_eq!(filter_option2.search_value, expected_values);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::IsNull as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_is_null");
        assert_eq!(filter_option3.search_value, expected_values);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::IsNull as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; is_not_null
    #[test]
    fn test_search_is_not_null() {
        let filter = AdvancedSearchFilter::search_is_not_null(String::from("is_not_null"))
            .and_is_not_null(String::from("and_is_not_null"))
            .or_is_not_null(String::from("or_is_not_null"));

        assert_eq!(filter.filters.len(), 3);

        let expected_values: Vec<String> = vec![];

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "is_not_null");
        assert_eq!(filter_option1.search_value, expected_values);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::IsNotNull as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_is_not_null");
        assert_eq!(filter_option2.search_value, expected_values);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::IsNotNull as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_is_not_null");
        assert_eq!(filter_option3.search_value, expected_values);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::IsNotNull as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; ilike
    #[test]
    fn test_search_ilike() {
        let filter =
            AdvancedSearchFilter::search_ilike(String::from("ilike"), String::from("test1"))
                .and_ilike(String::from("and_ilike"), String::from("test2"))
                .or_ilike(String::from("or_ilike"), String::from("test3"));

        assert_eq!(filter.filters.len(), 3);

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "ilike");
        assert_eq!(filter_option1.search_value, vec!["test1"]);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::Ilike as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_ilike");
        assert_eq!(filter_option2.search_value, vec!["test2"]);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::Ilike as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_ilike");
        assert_eq!(filter_option3.search_value, vec!["test3"]);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::Ilike as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; like
    #[test]
    fn test_search_like() {
        let filter = AdvancedSearchFilter::search_like(String::from("like"), String::from("test1"))
            .and_like(String::from("and_like"), String::from("test2"))
            .or_like(String::from("or_like"), String::from("test3"));

        assert_eq!(filter.filters.len(), 3);

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "like");
        assert_eq!(filter_option1.search_value, vec!["test1"]);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::Like as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_like");
        assert_eq!(filter_option2.search_value, vec!["test2"]);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::Like as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_like");
        assert_eq!(filter_option3.search_value, vec!["test3"]);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::Like as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; greater
    #[test]
    fn test_search_greater() {
        let filter =
            AdvancedSearchFilter::search_greater(String::from("greater"), String::from("1"))
                .and_greater(String::from("and_greater"), String::from("2"))
                .or_greater(String::from("or_greater"), String::from("3"));

        assert_eq!(filter.filters.len(), 3);

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "greater");
        assert_eq!(filter_option1.search_value, vec!["1"]);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::Greater as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_greater");
        assert_eq!(filter_option2.search_value, vec!["2"]);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::Greater as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_greater");
        assert_eq!(filter_option3.search_value, vec!["3"]);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::Greater as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; greater_or_equal
    #[test]
    fn test_search_greater_or_equal() {
        let filter = AdvancedSearchFilter::search_greater_or_equal(
            String::from("greater_or_equal"),
            String::from("1"),
        )
        .and_greater_or_equal(String::from("and_greater_or_equal"), String::from("2"))
        .or_greater_or_equal(String::from("or_greater_or_equal"), String::from("3"));

        assert_eq!(filter.filters.len(), 3);

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "greater_or_equal");
        assert_eq!(filter_option1.search_value, vec!["1"]);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::GreaterOrEqual as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_greater_or_equal");
        assert_eq!(filter_option2.search_value, vec!["2"]);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::GreaterOrEqual as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_greater_or_equal");
        assert_eq!(filter_option3.search_value, vec!["3"]);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::GreaterOrEqual as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; less
    #[test]
    fn test_search_less() {
        let filter = AdvancedSearchFilter::search_less(String::from("less"), String::from("1"))
            .and_less(String::from("and_less"), String::from("2"))
            .or_less(String::from("or_less"), String::from("3"));

        assert_eq!(filter.filters.len(), 3);

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "less");
        assert_eq!(filter_option1.search_value, vec!["1"]);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::Less as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_less");
        assert_eq!(filter_option2.search_value, vec!["2"]);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::Less as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_less");
        assert_eq!(filter_option3.search_value, vec!["3"]);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::Less as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; less_or_equal
    #[test]
    fn test_search_less_or_equal() {
        let filter = AdvancedSearchFilter::search_less_or_equal(
            String::from("less_or_equal"),
            String::from("1"),
        )
        .and_less_or_equal(String::from("and_less_or_equal"), String::from("2"))
        .or_less_or_equal(String::from("or_less_or_equal"), String::from("3"));

        assert_eq!(filter.filters.len(), 3);

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "less_or_equal");
        assert_eq!(filter_option1.search_value, vec!["1"]);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::LessOrEqual as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_less_or_equal");
        assert_eq!(filter_option2.search_value, vec!["2"]);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::LessOrEqual as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_less_or_equal");
        assert_eq!(filter_option3.search_value, vec!["3"]);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::LessOrEqual as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; geo_intersect
    #[test]
    fn test_search_geo_intersect() {
        let filter = AdvancedSearchFilter::search_geo_intersect(
            String::from("geo_intersect"),
            String::from("POINT Z(1,2,3)"),
        )
        .and_geo_intersect(
            String::from("and_geo_intersect"),
            String::from("POINT Z(4,5,6)"),
        )
        .or_geo_intersect(
            String::from("or_geo_intersect"),
            String::from("POINT Z(7,8,9)"),
        );

        assert_eq!(filter.filters.len(), 3);

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "geo_intersect");
        assert_eq!(filter_option1.search_value, vec!["POINT Z(1,2,3)"]);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::GeoIntersect as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_geo_intersect");
        assert_eq!(filter_option2.search_value, vec!["POINT Z(4,5,6)"]);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::GeoIntersect as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_geo_intersect");
        assert_eq!(filter_option3.search_value, vec!["POINT Z(7,8,9)"]);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::GeoIntersect as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; geo_within
    #[test]
    fn test_search_geo_within() {
        let filter = AdvancedSearchFilter::search_geo_within(
            String::from("geo_within"),
            String::from("POINT Z(1,2,3)"),
        )
        .and_geo_within(
            String::from("and_geo_within"),
            String::from("POINT Z(4,5,6)"),
        )
        .or_geo_within(
            String::from("or_geo_within"),
            String::from("POINT Z(7,8,9)"),
        );

        assert_eq!(filter.filters.len(), 3);

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "geo_within");
        assert_eq!(filter_option1.search_value, vec!["POINT Z(1,2,3)"]);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::GeoWithin as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_geo_within");
        assert_eq!(filter_option2.search_value, vec!["POINT Z(4,5,6)"]);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::GeoWithin as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_geo_within");
        assert_eq!(filter_option3.search_value, vec!["POINT Z(7,8,9)"]);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::GeoWithin as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    // Test all of search, and, or options for predicate operator; geo_disjoint
    #[test]
    fn test_search_geo_disjoint() {
        let filter = AdvancedSearchFilter::search_geo_disjoint(
            String::from("geo_disjoint"),
            String::from("POINT Z(1,2,3)"),
        )
        .and_geo_disjoint(
            String::from("and_geo_disjoint"),
            String::from("POINT Z(4,5,6)"),
        )
        .or_geo_disjoint(
            String::from("or_geo_disjoint"),
            String::from("POINT Z(7,8,9)"),
        );

        assert_eq!(filter.filters.len(), 3);

        let filter_option1 = &filter.filters[0];
        assert_eq!(filter_option1.search_field, "geo_disjoint");
        assert_eq!(filter_option1.search_value, vec!["POINT Z(1,2,3)"]);
        assert_eq!(
            filter.filters[0].predicate_operator,
            PredicateOperator::GeoDisjoint as i32
        );

        let filter_option2 = &filter.filters[1];
        assert_eq!(filter_option2.search_field, "and_geo_disjoint");
        assert_eq!(filter_option2.search_value, vec!["POINT Z(4,5,6)"]);
        assert_eq!(
            filter_option2.predicate_operator,
            PredicateOperator::GeoDisjoint as i32
        );
        assert_eq!(
            filter_option2.comparison_operator,
            Some(ComparisonOperator::And as i32)
        );

        let filter_option3 = &filter.filters[2];
        assert_eq!(filter_option3.search_field, "or_geo_disjoint");
        assert_eq!(filter_option3.search_value, vec!["POINT Z(7,8,9)"]);
        assert_eq!(
            filter_option3.predicate_operator,
            PredicateOperator::GeoDisjoint as i32
        );
        assert_eq!(
            filter_option3.comparison_operator,
            Some(ComparisonOperator::Or as i32)
        );
    }

    #[test]
    fn test_predicate_operator_as_str_name() {
        assert_eq!(PredicateOperator::Equals.as_str_name(), "EQUALS");
        assert_eq!(PredicateOperator::NotEquals.as_str_name(), "NOT_EQUALS");
        assert_eq!(PredicateOperator::In.as_str_name(), "IN");
        assert_eq!(PredicateOperator::NotIn.as_str_name(), "NOT_IN");
        assert_eq!(PredicateOperator::Between.as_str_name(), "BETWEEN");
        assert_eq!(PredicateOperator::IsNull.as_str_name(), "IS_NULL");
        assert_eq!(PredicateOperator::IsNotNull.as_str_name(), "IS_NOT_NULL");
        assert_eq!(PredicateOperator::Ilike.as_str_name(), "ILIKE");
        assert_eq!(PredicateOperator::Like.as_str_name(), "LIKE");
        assert_eq!(PredicateOperator::Greater.as_str_name(), "GREATER");
        assert_eq!(
            PredicateOperator::GreaterOrEqual.as_str_name(),
            "GREATER_OR_EQUAL"
        );
        assert_eq!(PredicateOperator::Less.as_str_name(), "LESS");
        assert_eq!(
            PredicateOperator::LessOrEqual.as_str_name(),
            "LESS_OR_EQUAL"
        );
        assert_eq!(
            PredicateOperator::GeoIntersect.as_str_name(),
            "GEO_INTERSECT"
        );
        assert_eq!(PredicateOperator::GeoWithin.as_str_name(), "GEO_WITHIN");
        assert_eq!(PredicateOperator::GeoDisjoint.as_str_name(), "GEO_DISJOINT");
    }

    #[test]
    fn test_predicate_operator_from_str_name() {
        assert_eq!(
            PredicateOperator::from_str_name("EQUALS"),
            Some(PredicateOperator::Equals)
        );
        assert_eq!(
            PredicateOperator::from_str_name("NOT_EQUALS"),
            Some(PredicateOperator::NotEquals)
        );
        assert_eq!(
            PredicateOperator::from_str_name("IN"),
            Some(PredicateOperator::In)
        );
        assert_eq!(
            PredicateOperator::from_str_name("NOT_IN"),
            Some(PredicateOperator::NotIn)
        );
        assert_eq!(
            PredicateOperator::from_str_name("BETWEEN"),
            Some(PredicateOperator::Between)
        );
        assert_eq!(
            PredicateOperator::from_str_name("IS_NULL"),
            Some(PredicateOperator::IsNull)
        );
        assert_eq!(
            PredicateOperator::from_str_name("IS_NOT_NULL"),
            Some(PredicateOperator::IsNotNull)
        );
        assert_eq!(
            PredicateOperator::from_str_name("ILIKE"),
            Some(PredicateOperator::Ilike)
        );
        assert_eq!(
            PredicateOperator::from_str_name("LIKE"),
            Some(PredicateOperator::Like)
        );
        assert_eq!(
            PredicateOperator::from_str_name("GREATER"),
            Some(PredicateOperator::Greater)
        );
        assert_eq!(
            PredicateOperator::from_str_name("GREATER_OR_EQUAL"),
            Some(PredicateOperator::GreaterOrEqual)
        );
        assert_eq!(
            PredicateOperator::from_str_name("LESS"),
            Some(PredicateOperator::Less)
        );
        assert_eq!(
            PredicateOperator::from_str_name("LESS_OR_EQUAL"),
            Some(PredicateOperator::LessOrEqual)
        );
        assert_eq!(
            PredicateOperator::from_str_name("GEO_INTERSECT"),
            Some(PredicateOperator::GeoIntersect)
        );
        assert_eq!(
            PredicateOperator::from_str_name("GEO_WITHIN"),
            Some(PredicateOperator::GeoWithin)
        );
        assert_eq!(
            PredicateOperator::from_str_name("GEO_DISJOINT"),
            Some(PredicateOperator::GeoDisjoint)
        );

        assert_eq!(PredicateOperator::from_str_name("INVALID"), None);
    }

    #[test]
    fn test_comparison_operator_as_str_name() {
        assert_eq!(ComparisonOperator::And.as_str_name(), "AND");
        assert_eq!(ComparisonOperator::Or.as_str_name(), "OR");
    }

    #[test]
    fn test_comparison_operator_from_str_name() {
        assert_eq!(
            ComparisonOperator::from_str_name("AND"),
            Some(ComparisonOperator::And)
        );
        assert_eq!(
            ComparisonOperator::from_str_name("OR"),
            Some(ComparisonOperator::Or)
        );
        assert_eq!(ComparisonOperator::from_str_name("INVALID"), None);
    }

    #[test]
    fn test_sort_order_as_str_name() {
        assert_eq!(SortOrder::Asc.as_str_name(), "ASC");
        assert_eq!(SortOrder::Desc.as_str_name(), "DESC");
    }

    #[test]
    fn test_sort_order_from_str_name() {
        assert_eq!(SortOrder::from_str_name("ASC"), Some(SortOrder::Asc));
        assert_eq!(SortOrder::from_str_name("DESC"), Some(SortOrder::Desc));
        assert_eq!(SortOrder::from_str_name("INVALID"), None);
    }
}
