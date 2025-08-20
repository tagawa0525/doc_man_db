use async_trait::async_trait;
use axum::{
    Json,
    extract::{Path, Query, State},
};
use chrono::Utc;
use doc_man_db::error::DeduplicationError;
use doc_man_db::handlers::deduplication::{
    DuplicationQuery, FindDuplicatesRequest, MergeDataRequest, UpdateDuplicationStatusRequest,
    find_business_number_duplicates, find_customer_duplicates, find_employee_duplicates,
};
use doc_man_db::services::{
    DeduplicationService, DuplicationCandidate, DuplicationStatus, DuplicationType, MergeRecord,
    MergeResult,
};
use std::sync::Arc;
use tokio;
use uuid::Uuid;

// モック重複除去サービス
#[derive(Debug)]
struct MockDeduplicationService {
    should_fail: bool,
}

impl MockDeduplicationService {
    fn new(should_fail: bool) -> Self {
        Self { should_fail }
    }
}

#[async_trait]
impl DeduplicationService for MockDeduplicationService {
    async fn find_employee_duplicates(
        &self,
        _threshold: f64,
    ) -> Result<Vec<DuplicationCandidate>, DeduplicationError> {
        if self.should_fail {
            return Err(DeduplicationError::InvalidThreshold { threshold: 0.0 });
        }

        Ok(vec![DuplicationCandidate {
            id: Uuid::new_v4(),
            candidate_type: DuplicationType::Employee,
            primary_id: 1,
            duplicate_id: 2,
            similarity_score: 0.95,
            field_name: "name".to_string(),
            primary_value: "田中太郎".to_string(),
            duplicate_value: "田中 太郎".to_string(),
            status: DuplicationStatus::Pending,
            created_at: Utc::now(),
            reviewed_at: None,
            reviewed_by: None,
        }])
    }

    async fn find_customer_duplicates(
        &self,
        _threshold: f64,
    ) -> Result<Vec<DuplicationCandidate>, DeduplicationError> {
        if self.should_fail {
            return Err(DeduplicationError::InvalidThreshold { threshold: -1.0 });
        }

        Ok(vec![DuplicationCandidate {
            id: Uuid::new_v4(),
            candidate_type: DuplicationType::Customer,
            primary_id: 1,
            duplicate_id: 2,
            similarity_score: 0.95,
            field_name: "name".to_string(),
            primary_value: "株式会社テスト".to_string(),
            duplicate_value: "テスト株式会社".to_string(),
            status: DuplicationStatus::Pending,
            created_at: Utc::now(),
            reviewed_at: None,
            reviewed_by: None,
        }])
    }

    async fn find_business_number_duplicates(
        &self,
    ) -> Result<Vec<DuplicationCandidate>, DeduplicationError> {
        if self.should_fail {
            return Err(DeduplicationError::InvalidThreshold { threshold: 2.0 });
        }

        Ok(vec![DuplicationCandidate {
            id: Uuid::new_v4(),
            candidate_type: DuplicationType::BusinessNumber,
            primary_id: 101,
            duplicate_id: 102,
            similarity_score: 1.0,
            field_name: "business_number".to_string(),
            primary_value: "BIZ-2024-001".to_string(),
            duplicate_value: "BIZ-2024-001".to_string(),
            status: DuplicationStatus::Pending,
            created_at: Utc::now(),
            reviewed_at: None,
            reviewed_by: None,
        }])
    }

    async fn find_document_duplicates(
        &self,
        _threshold: f64,
    ) -> Result<Vec<DuplicationCandidate>, DeduplicationError> {
        if self.should_fail {
            return Err(DeduplicationError::InvalidThreshold { threshold: 3.0 });
        }
        Ok(vec![])
    }

    async fn update_duplication_status(
        &self,
        _candidate_id: Uuid,
        _status: DuplicationStatus,
        _reviewed_by: i32,
    ) -> Result<(), DeduplicationError> {
        if self.should_fail {
            return Err(DeduplicationError::InvalidThreshold { threshold: 4.0 });
        }
        Ok(())
    }

    async fn merge_employees(
        &self,
        _primary_id: i32,
        _duplicate_ids: Vec<i32>,
        _merged_by: i32,
    ) -> Result<MergeResult, DeduplicationError> {
        if self.should_fail {
            return Err(DeduplicationError::InvalidThreshold { threshold: 5.0 });
        }
        Ok(MergeResult {
            merge_id: Uuid::new_v4(),
            primary_id: 1,
            merged_ids: vec![2, 3],
            merge_type: DuplicationType::Employee,
            affected_documents: 5,
            merged_at: Utc::now(),
            merged_by: 1,
        })
    }

    async fn get_merge_history(
        &self,
        _limit: Option<i32>,
    ) -> Result<Vec<MergeRecord>, DeduplicationError> {
        if self.should_fail {
            return Err(DeduplicationError::InvalidThreshold { threshold: 6.0 });
        }
        Ok(vec![])
    }
}

#[tokio::test]
async fn test_find_employee_duplicates_success() {
    let service = Arc::new(MockDeduplicationService::new(false));

    let query = DuplicationQuery {
        r#type: Some("employee".to_string()),
        threshold: Some(0.8),
        status: None,
        limit: None,
    };

    let result = find_employee_duplicates(State(service), Query(query)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.0["duplicate_type"], "employee");
    assert_eq!(response.0["threshold"], 0.8);
    assert_eq!(response.0["total_count"], 1);
}

#[tokio::test]
async fn test_find_employee_duplicates_default_threshold() {
    let service = Arc::new(MockDeduplicationService::new(false));

    let query = DuplicationQuery {
        r#type: Some("employee".to_string()),
        threshold: None, // デフォルトの0.8を使用
        status: None,
        limit: None,
    };

    let result = find_employee_duplicates(State(service), Query(query)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.0["threshold"], 0.8); // デフォルト値
}

#[tokio::test]
async fn test_find_employee_duplicates_failure() {
    let service = Arc::new(MockDeduplicationService::new(true));

    let query = DuplicationQuery {
        r#type: Some("employee".to_string()),
        threshold: Some(0.9),
        status: None,
        limit: None,
    };

    let result = find_employee_duplicates(State(service), Query(query)).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_find_customer_duplicates_success() {
    let service = Arc::new(MockDeduplicationService::new(false));

    let query = DuplicationQuery {
        r#type: Some("customer".to_string()),
        threshold: Some(0.85),
        status: None,
        limit: None,
    };

    let result = find_customer_duplicates(State(service), Query(query)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.0["duplicate_type"], "customer");
    assert_eq!(response.0["threshold"], 0.85);
    assert_eq!(response.0["total_count"], 1);
}

#[tokio::test]
async fn test_find_customer_duplicates_failure() {
    let service = Arc::new(MockDeduplicationService::new(true));

    let query = DuplicationQuery {
        r#type: Some("customer".to_string()),
        threshold: Some(0.9),
        status: None,
        limit: None,
    };

    let result = find_customer_duplicates(State(service), Query(query)).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_find_business_number_duplicates_success() {
    let service = Arc::new(MockDeduplicationService::new(false));

    let result = find_business_number_duplicates(State(service)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.0["duplicate_type"], "business_number");
    assert_eq!(response.0["total_count"], 1);

    let candidates = response.0["candidates"].as_array().unwrap();
    assert_eq!(candidates.len(), 1);
}

#[tokio::test]
async fn test_find_business_number_duplicates_failure() {
    let service = Arc::new(MockDeduplicationService::new(true));

    let result = find_business_number_duplicates(State(service)).await;

    assert!(result.is_err());
}
