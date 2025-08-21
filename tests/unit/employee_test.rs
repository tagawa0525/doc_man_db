use chrono::Utc;
use doc_man_db::models::{
    CreateEmployeeRequest, Employee, EmployeeSearchQuery, UpdateEmployeeRequest,
};

#[test]
fn test_employee_creation() {
    let now = Utc::now();
    let employee = Employee {
        id: 1,
        employee_number: Some("E2025001".to_string()),
        name: "田中太郎".to_string(),
        email: Some("tanaka@example.com".to_string()),
        ad_username: Some("tanaka.taro".to_string()),
        department_id: Some(10),
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    assert_eq!(employee.id, 1);
    assert_eq!(employee.employee_number, Some("E2025001".to_string()));
    assert_eq!(employee.name, "田中太郎");
    assert_eq!(employee.email, Some("tanaka@example.com".to_string()));
    assert_eq!(employee.ad_username, Some("tanaka.taro".to_string()));
    assert_eq!(employee.department_id, Some(10));
    assert!(employee.is_active);
    assert_eq!(employee.created_at, now);
    assert_eq!(employee.updated_at, now);
}

#[test]
fn test_employee_minimal_creation() {
    let now = Utc::now();
    let employee = Employee {
        id: 2,
        employee_number: None,
        name: "佐藤花子".to_string(),
        email: None,
        ad_username: None,
        department_id: None,
        is_active: false,
        created_at: now,
        updated_at: now,
    };

    assert_eq!(employee.id, 2);
    assert_eq!(employee.employee_number, None);
    assert_eq!(employee.name, "佐藤花子");
    assert_eq!(employee.email, None);
    assert_eq!(employee.ad_username, None);
    assert_eq!(employee.department_id, None);
    assert!(!employee.is_active);
}

#[test]
fn test_employee_serialization() {
    let now = Utc::now();
    let employee = Employee {
        id: 1,
        employee_number: Some("E2025001".to_string()),
        name: "田中太郎".to_string(),
        email: Some("tanaka@example.com".to_string()),
        ad_username: Some("tanaka.taro".to_string()),
        department_id: Some(10),
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    let serialized = serde_json::to_string(&employee).expect("Failed to serialize");
    assert!(serialized.contains("田中太郎"));
    assert!(serialized.contains("E2025001"));
    assert!(serialized.contains("tanaka@example.com"));
    assert!(serialized.contains("tanaka.taro"));
}

#[test]
fn test_employee_deserialization() {
    let json = r#"{
        "id": 1,
        "employee_number": "E2025001",
        "name": "田中太郎",
        "email": "tanaka@example.com",
        "ad_username": "tanaka.taro",
        "department_id": 10,
        "is_active": true,
        "created_at": "2025-01-01T00:00:00Z",
        "updated_at": "2025-01-01T00:00:00Z"
    }"#;

    let employee: Employee = serde_json::from_str(json).expect("Failed to deserialize");
    assert_eq!(employee.id, 1);
    assert_eq!(employee.employee_number, Some("E2025001".to_string()));
    assert_eq!(employee.name, "田中太郎");
    assert_eq!(employee.email, Some("tanaka@example.com".to_string()));
    assert_eq!(employee.ad_username, Some("tanaka.taro".to_string()));
    assert_eq!(employee.department_id, Some(10));
    assert!(employee.is_active);
}

#[test]
fn test_create_employee_request_full() {
    let request = CreateEmployeeRequest {
        employee_number: Some("E2025002".to_string()),
        name: "山田次郎".to_string(),
        email: Some("yamada@example.com".to_string()),
        ad_username: Some("yamada.jiro".to_string()),
        department_id: Some(20),
    };

    assert_eq!(request.employee_number, Some("E2025002".to_string()));
    assert_eq!(request.name, "山田次郎");
    assert_eq!(request.email, Some("yamada@example.com".to_string()));
    assert_eq!(request.ad_username, Some("yamada.jiro".to_string()));
    assert_eq!(request.department_id, Some(20));
}

#[test]
fn test_create_employee_request_minimal() {
    let request = CreateEmployeeRequest {
        employee_number: None,
        name: "鈴木三郎".to_string(),
        email: None,
        ad_username: None,
        department_id: None,
    };

    assert_eq!(request.employee_number, None);
    assert_eq!(request.name, "鈴木三郎");
    assert_eq!(request.email, None);
    assert_eq!(request.ad_username, None);
    assert_eq!(request.department_id, None);
}

#[test]
fn test_create_employee_request_deserialization() {
    let json = r#"{
        "employee_number": "E2025003",
        "name": "高橋四郎",
        "email": "takahashi@example.com",
        "ad_username": "takahashi.shiro",
        "department_id": 30
    }"#;

    let request: CreateEmployeeRequest = serde_json::from_str(json).expect("Failed to deserialize");
    assert_eq!(request.employee_number, Some("E2025003".to_string()));
    assert_eq!(request.name, "高橋四郎");
    assert_eq!(request.email, Some("takahashi@example.com".to_string()));
    assert_eq!(request.ad_username, Some("takahashi.shiro".to_string()));
    assert_eq!(request.department_id, Some(30));
}

#[test]
fn test_update_employee_request_full() {
    let request = UpdateEmployeeRequest {
        name: Some("田中太郎（更新）".to_string()),
        email: Some("tanaka.updated@example.com".to_string()),
        ad_username: Some("tanaka.taro.updated".to_string()),
        department_id: Some(15),
        is_active: Some(false),
    };

    assert_eq!(request.name, Some("田中太郎（更新）".to_string()));
    assert_eq!(
        request.email,
        Some("tanaka.updated@example.com".to_string())
    );
    assert_eq!(request.ad_username, Some("tanaka.taro.updated".to_string()));
    assert_eq!(request.department_id, Some(15));
    assert_eq!(request.is_active, Some(false));
}

#[test]
fn test_update_employee_request_partial() {
    let request = UpdateEmployeeRequest {
        name: Some("田中太郎（部分更新）".to_string()),
        email: None,
        ad_username: None,
        department_id: None,
        is_active: None,
    };

    assert_eq!(request.name, Some("田中太郎（部分更新）".to_string()));
    assert_eq!(request.email, None);
    assert_eq!(request.ad_username, None);
    assert_eq!(request.department_id, None);
    assert_eq!(request.is_active, None);
}

#[test]
fn test_update_employee_request_deserialization() {
    let json = r#"{
        "name": "田中太郎（JSON更新）",
        "email": "tanaka.json@example.com",
        "is_active": true
    }"#;

    let request: UpdateEmployeeRequest = serde_json::from_str(json).expect("Failed to deserialize");
    assert_eq!(request.name, Some("田中太郎（JSON更新）".to_string()));
    assert_eq!(request.email, Some("tanaka.json@example.com".to_string()));
    assert_eq!(request.ad_username, None);
    assert_eq!(request.department_id, None);
    assert_eq!(request.is_active, Some(true));
}

#[test]
fn test_employee_search_query_default() {
    let query = EmployeeSearchQuery::default();

    assert_eq!(query.name, None);
    assert_eq!(query.employee_number, None);
    assert_eq!(query.department_id, None);
    assert_eq!(query.is_active, Some(true)); // デフォルトでアクティブユーザーのみ
    assert_eq!(query.limit, Some(50));
    assert_eq!(query.offset, Some(0));
}

#[test]
fn test_employee_search_query_full() {
    let query = EmployeeSearchQuery {
        name: Some("田中".to_string()),
        employee_number: Some("E2025".to_string()),
        department_id: Some(10),
        is_active: Some(false),
        limit: Some(100),
        offset: Some(20),
    };

    assert_eq!(query.name, Some("田中".to_string()));
    assert_eq!(query.employee_number, Some("E2025".to_string()));
    assert_eq!(query.department_id, Some(10));
    assert_eq!(query.is_active, Some(false));
    assert_eq!(query.limit, Some(100));
    assert_eq!(query.offset, Some(20));
}

#[test]
fn test_employee_search_query_deserialization() {
    let json = r#"{
        "name": "佐藤",
        "employee_number": "E2025001",
        "department_id": 20,
        "is_active": true,
        "limit": 25,
        "offset": 10
    }"#;

    let query: EmployeeSearchQuery = serde_json::from_str(json).expect("Failed to deserialize");
    assert_eq!(query.name, Some("佐藤".to_string()));
    assert_eq!(query.employee_number, Some("E2025001".to_string()));
    assert_eq!(query.department_id, Some(20));
    assert_eq!(query.is_active, Some(true));
    assert_eq!(query.limit, Some(25));
    assert_eq!(query.offset, Some(10));
}

#[test]
fn test_employee_search_query_partial_deserialization() {
    let json = r#"{
        "name": "山田"
    }"#;

    let query: EmployeeSearchQuery = serde_json::from_str(json).expect("Failed to deserialize");
    assert_eq!(query.name, Some("山田".to_string()));
    assert_eq!(query.employee_number, None);
    assert_eq!(query.department_id, None);
    assert_eq!(query.is_active, None);
    assert_eq!(query.limit, None);
    assert_eq!(query.offset, None);
}

#[test]
fn test_employee_clone() {
    let now = Utc::now();
    let employee = Employee {
        id: 1,
        employee_number: Some("E2025001".to_string()),
        name: "田中太郎".to_string(),
        email: Some("tanaka@example.com".to_string()),
        ad_username: Some("tanaka.taro".to_string()),
        department_id: Some(10),
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    let cloned = employee.clone();
    assert_eq!(employee.id, cloned.id);
    assert_eq!(employee.name, cloned.name);
    assert_eq!(employee.email, cloned.email);
    assert_eq!(employee.is_active, cloned.is_active);
}

#[test]
fn test_employee_debug_format() {
    let now = Utc::now();
    let employee = Employee {
        id: 1,
        employee_number: Some("E2025001".to_string()),
        name: "田中太郎".to_string(),
        email: Some("tanaka@example.com".to_string()),
        ad_username: Some("tanaka.taro".to_string()),
        department_id: Some(10),
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    let debug_str = format!("{employee:?}");
    assert!(debug_str.contains("Employee"));
    assert!(debug_str.contains("田中太郎"));
    assert!(debug_str.contains("E2025001"));
    assert!(debug_str.contains("tanaka@example.com"));
}

#[test]
fn test_employee_validation_scenarios() {
    // 名前が空文字列の場合
    let request = CreateEmployeeRequest {
        employee_number: Some("E2025004".to_string()),
        name: "".to_string(),
        email: Some("empty.name@example.com".to_string()),
        ad_username: Some("empty.name".to_string()),
        department_id: Some(40),
    };

    // 構造体自体は作成できるが、名前が空であることを確認
    assert_eq!(request.name, "");

    // 非常に長い名前の場合
    let long_name = "あ".repeat(1000);
    let request_long = CreateEmployeeRequest {
        employee_number: None,
        name: long_name.clone(),
        email: None,
        ad_username: None,
        department_id: None,
    };

    assert_eq!(request_long.name.len(), 3000); // UTF-8で1文字3バイト
    assert_eq!(request_long.name, long_name);
}
