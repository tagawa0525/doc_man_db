use crate::models::{
    AdvancedBusinessSearchInput, BusinessSortField, BusinessStatus, PaginationInput, SortOrder,
};
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

/// クエリパラメータを業務検索フィルターに変換
fn query_params_to_business_search_filters(
    params: &HashMap<String, String>,
) -> AdvancedBusinessSearchInput {
    AdvancedBusinessSearchInput {
        business_number: params.get("business_number").cloned(),
        name: params.get("name").cloned(),
        customer_name: params.get("customer_name").cloned(),
        description: params.get("description").cloned(),
        member_employee_id: params
            .get("member_employee_id")
            .and_then(|s| s.parse().ok()),
        member_role: params
            .get("member_role")
            .map(|s| crate::models::BusinessRole::from(s.clone())),
        status: params
            .get("status")
            .map(|s| crate::models::BusinessStatus::from(s.clone())),
        start_date_from: params.get("start_date_from").and_then(|s| s.parse().ok()),
        start_date_to: params.get("start_date_to").and_then(|s| s.parse().ok()),
        end_date_from: params.get("end_date_from").and_then(|s| s.parse().ok()),
        end_date_to: params.get("end_date_to").and_then(|s| s.parse().ok()),
        has_documents: params.get("has_documents").and_then(|s| s.parse().ok()),
        created_by: params.get("created_by").and_then(|s| s.parse().ok()),
        sort_by: params
            .get("sort_by")
            .map(|s| BusinessSortField::from(s.clone())),
        sort_order: params.get("sort_order").map(|s| SortOrder::from(s.clone())),
        pagination: PaginationInput {
            limit: params
                .get("limit")
                .and_then(|s| s.parse().ok())
                .unwrap_or(20),
            offset: params
                .get("offset")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
        },
    }
}
