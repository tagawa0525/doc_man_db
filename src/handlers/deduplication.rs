use crate::error::AppError;
use crate::services::{DeduplicationService, DuplicationStatus, DuplicationType};
use axum::{
    Extension,
    extract::{Path, Query, State},
    response::Json,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

/// 重複検索リクエスト
#[derive(Debug, Deserialize)]
pub struct FindDuplicatesRequest {
    pub duplicate_type: DuplicationType,
    pub threshold: Option<f64>,
}

/// 重複ステータス更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateDuplicationStatusRequest {
    pub status: DuplicationStatus,
}

/// データ統合リクエスト
#[derive(Debug, Deserialize)]
pub struct MergeDataRequest {
    pub primary_id: i32,
    pub duplicate_ids: Vec<i32>,
    pub merge_type: DuplicationType,
}

/// 重複検索クエリパラメータ
#[derive(Debug, Deserialize)]
pub struct DuplicationQuery {
    pub r#type: Option<String>,
    pub threshold: Option<f64>,
    pub status: Option<String>,
    pub limit: Option<i32>,
}

/// 社員の重複候補を検索
pub async fn find_employee_duplicates(
    State(deduplication_service): State<Arc<dyn DeduplicationService>>,
    Query(query): Query<DuplicationQuery>,
) -> Result<Json<Value>, AppError> {
    let threshold = query.threshold.unwrap_or(0.8);

    info!("Finding employee duplicates with threshold: {}", threshold);

    let candidates = deduplication_service
        .find_employee_duplicates(threshold)
        .await?;

    info!("Found {} employee duplicate candidates", candidates.len());

    Ok(Json(json!({
        "duplicate_type": "employee",
        "threshold": threshold,
        "candidates": candidates,
        "total_count": candidates.len()
    })))
}

/// 顧客の重複候補を検索
pub async fn find_customer_duplicates(
    State(deduplication_service): State<Arc<dyn DeduplicationService>>,
    Query(query): Query<DuplicationQuery>,
) -> Result<Json<Value>, AppError> {
    let threshold = query.threshold.unwrap_or(0.8);

    info!("Finding customer duplicates with threshold: {}", threshold);

    let candidates = deduplication_service
        .find_customer_duplicates(threshold)
        .await?;

    Ok(Json(json!({
        "duplicate_type": "customer",
        "threshold": threshold,
        "candidates": candidates,
        "total_count": candidates.len()
    })))
}

/// 業務番号の重複候補を検索
pub async fn find_business_number_duplicates(
    State(deduplication_service): State<Arc<dyn DeduplicationService>>,
) -> Result<Json<Value>, AppError> {
    info!("Finding business number duplicates");

    let candidates = deduplication_service
        .find_business_number_duplicates()
        .await?;

    Ok(Json(json!({
        "duplicate_type": "business_number",
        "candidates": candidates,
        "total_count": candidates.len()
    })))
}

/// 文書の重複候補を検索
pub async fn find_document_duplicates(
    State(deduplication_service): State<Arc<dyn DeduplicationService>>,
    Query(query): Query<DuplicationQuery>,
) -> Result<Json<Value>, AppError> {
    let threshold = query.threshold.unwrap_or(0.8);

    info!("Finding document duplicates with threshold: {}", threshold);

    let candidates = deduplication_service
        .find_document_duplicates(threshold)
        .await?;

    Ok(Json(json!({
        "duplicate_type": "document",
        "threshold": threshold,
        "candidates": candidates,
        "total_count": candidates.len()
    })))
}

/// 全タイプの重複候補を検索
pub async fn find_all_duplicates(
    State(deduplication_service): State<Arc<dyn DeduplicationService>>,
    Query(query): Query<DuplicationQuery>,
) -> Result<Json<Value>, AppError> {
    let threshold = query.threshold.unwrap_or(0.8);

    info!(
        "Finding all types of duplicates with threshold: {}",
        threshold
    );

    // 並列で全ての重複検索を実行
    let (employee_candidates, customer_candidates, business_candidates, document_candidates) = tokio::join!(
        deduplication_service.find_employee_duplicates(threshold),
        deduplication_service.find_customer_duplicates(threshold),
        deduplication_service.find_business_number_duplicates(),
        deduplication_service.find_document_duplicates(threshold)
    );

    let employee_candidates = employee_candidates?;
    let customer_candidates = customer_candidates?;
    let business_candidates = business_candidates?;
    let document_candidates = document_candidates?;

    let total_count = employee_candidates.len()
        + customer_candidates.len()
        + business_candidates.len()
        + document_candidates.len();

    Ok(Json(json!({
        "threshold": threshold,
        "results": {
            "employees": {
                "candidates": employee_candidates,
                "count": employee_candidates.len()
            },
            "customers": {
                "candidates": customer_candidates,
                "count": customer_candidates.len()
            },
            "business_numbers": {
                "candidates": business_candidates,
                "count": business_candidates.len()
            },
            "documents": {
                "candidates": document_candidates,
                "count": document_candidates.len()
            }
        },
        "total_count": total_count
    })))
}

/// 重複候補のステータス更新
pub async fn update_duplication_status(
    State(deduplication_service): State<Arc<dyn DeduplicationService>>,
    Path(candidate_id): Path<Uuid>,
    Extension(user_id): Extension<i32>,
    Json(request): Json<UpdateDuplicationStatusRequest>,
) -> Result<Json<Value>, AppError> {
    info!(
        "Updating duplication status for {}: {:?} by user {}",
        candidate_id, request.status, user_id
    );

    deduplication_service
        .update_duplication_status(candidate_id, request.status, user_id)
        .await?;

    Ok(Json(json!({
        "candidate_id": candidate_id,
        "status": "updated",
        "message": "Duplication status updated successfully"
    })))
}

/// 社員データの統合
pub async fn merge_employees(
    State(deduplication_service): State<Arc<dyn DeduplicationService>>,
    Extension(user_id): Extension<i32>,
    Json(request): Json<MergeDataRequest>,
) -> Result<Json<Value>, AppError> {
    info!(
        "Merging employees: primary={}, duplicates={:?}, by user={}",
        request.primary_id, request.duplicate_ids, user_id
    );

    // データ統合実行
    let merge_result = deduplication_service
        .merge_employees(request.primary_id, request.duplicate_ids, user_id)
        .await?;

    info!("Employee merge completed: {:?}", merge_result.merge_id);

    Ok(Json(json!({
        "merge_id": merge_result.merge_id,
        "primary_id": merge_result.primary_id,
        "merged_ids": merge_result.merged_ids,
        "affected_documents": merge_result.affected_documents,
        "merged_at": merge_result.merged_at,
        "status": "success",
        "message": format!("Successfully merged {} records into primary record {}",
                          merge_result.merged_ids.len(), merge_result.primary_id)
    })))
}

/// 一括データ統合
pub async fn bulk_merge_data(
    State(deduplication_service): State<Arc<dyn DeduplicationService>>,
    Extension(user_id): Extension<i32>,
    Json(requests): Json<Vec<MergeDataRequest>>,
) -> Result<Json<Value>, AppError> {
    info!(
        "Starting bulk merge operation for {} requests by user {}",
        requests.len(),
        user_id
    );

    let mut results = Vec::new();
    let mut errors = Vec::new();

    for (index, request) in requests.iter().enumerate() {
        match request.merge_type {
            DuplicationType::Employee => {
                match deduplication_service
                    .merge_employees(request.primary_id, request.duplicate_ids.clone(), user_id)
                    .await
                {
                    Ok(result) => results.push(json!({
                        "index": index,
                        "merge_id": result.merge_id,
                        "primary_id": result.primary_id,
                        "merged_ids": result.merged_ids,
                        "status": "success"
                    })),
                    Err(error) => errors.push(json!({
                        "index": index,
                        "primary_id": request.primary_id,
                        "error": error.to_string()
                    })),
                }
            }
            _ => {
                errors.push(json!({
                    "index": index,
                    "primary_id": request.primary_id,
                    "error": "Merge type not yet supported"
                }));
            }
        }
    }

    info!(
        "Bulk merge completed: {} successful, {} errors",
        results.len(),
        errors.len()
    );

    Ok(Json(json!({
        "total_requests": requests.len(),
        "successful_merges": results.len(),
        "failed_merges": errors.len(),
        "results": results,
        "errors": errors
    })))
}

/// 統合履歴の取得
pub async fn get_merge_history(
    State(deduplication_service): State<Arc<dyn DeduplicationService>>,
    Query(query): Query<DuplicationQuery>,
) -> Result<Json<Value>, AppError> {
    let limit = query.limit;

    info!("Getting merge history with limit: {:?}", limit);

    let history = deduplication_service.get_merge_history(limit).await?;

    Ok(Json(json!({
        "history": history,
        "count": history.len()
    })))
}

/// 重複データ統計の取得
pub async fn get_duplication_statistics(
    State(deduplication_service): State<Arc<dyn DeduplicationService>>,
) -> Result<Json<Value>, AppError> {
    info!("Getting duplication statistics");

    // 各タイプの重複候補数を取得
    let threshold = 0.8;

    let (employee_candidates, customer_candidates, business_candidates, document_candidates) = tokio::join!(
        deduplication_service.find_employee_duplicates(threshold),
        deduplication_service.find_customer_duplicates(threshold),
        deduplication_service.find_business_number_duplicates(),
        deduplication_service.find_document_duplicates(threshold)
    );

    let employee_count = employee_candidates.map(|c| c.len()).unwrap_or(0);
    let customer_count = customer_candidates.map(|c| c.len()).unwrap_or(0);
    let business_count = business_candidates.map(|c| c.len()).unwrap_or(0);
    let document_count = document_candidates.map(|c| c.len()).unwrap_or(0);

    let total_duplicates = employee_count + customer_count + business_count + document_count;

    Ok(Json(json!({
        "threshold": threshold,
        "statistics": {
            "total_duplicates": total_duplicates,
            "by_type": {
                "employees": employee_count,
                "customers": customer_count,
                "business_numbers": business_count,
                "documents": document_count
            }
        },
        "recommendations": {
            "high_priority": if total_duplicates > 100 { "Immediate attention required" } else { "Normal monitoring" },
            "suggested_actions": if total_duplicates > 50 {
                vec!["Run bulk deduplication", "Review merge policies", "Increase data validation"]
            } else {
                vec!["Regular monitoring", "Manual review"]
            }
        }
    })))
}

/// デモ用のサンプルデータ統合
pub async fn demo_merge_employees(
    State(deduplication_service): State<Arc<dyn DeduplicationService>>,
    Extension(user_id): Extension<i32>,
) -> Result<Json<Value>, AppError> {
    info!("Running demo employee merge for user {}", user_id);

    // デモ用の統合：ID 2をID 1に統合
    let merge_result = deduplication_service
        .merge_employees(1, vec![2], user_id)
        .await?;

    Ok(Json(json!({
        "demo": true,
        "merge_result": merge_result,
        "message": "Demo merge completed successfully"
    })))
}
