use super::super::business_search::query_params_to_business_search_filters;
use crate::models::BusinessStatus;
use std::collections::HashMap;

#[test]
fn test_query_params_to_business_search_filters() {
    let mut params = HashMap::new();
    params.insert("business_number".to_string(), "BUS-25001".to_string());
    params.insert("name".to_string(), "テスト業務".to_string());
    params.insert("status".to_string(), "active".to_string());
    params.insert("limit".to_string(), "50".to_string());

    let filters = query_params_to_business_search_filters(&params);

    assert_eq!(filters.business_number, Some("BUS-25001".to_string()));
    assert_eq!(filters.name, Some("テスト業務".to_string()));
    assert_eq!(filters.status, Some(BusinessStatus::Active));
    assert_eq!(filters.pagination.limit, 50);
}
