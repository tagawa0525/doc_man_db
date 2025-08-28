use super::super::advanced_search::query_params_to_search_filters;
use std::collections::HashMap;

#[test]
fn test_query_params_to_search_filters() {
    let mut params = HashMap::new();
    params.insert("name".to_string(), "田中".to_string());
    params.insert("limit".to_string(), "50".to_string());
    params.insert("is_active".to_string(), "true".to_string());

    let filters = query_params_to_search_filters(&params);

    assert_eq!(filters.name, Some("田中".to_string()));
    assert_eq!(filters.pagination.limit, 50);
    assert_eq!(filters.is_active, Some(true));
}
