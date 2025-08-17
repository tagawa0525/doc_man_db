use crate::AppState;
use crate::batch::{BatchExecution, BatchStatus, BatchType};
use crate::error::AppError;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;

/// バッチ実行リクエスト
#[derive(Debug, Deserialize)]
pub struct BatchExecutionRequest {
    pub batch_type: BatchType,
    pub started_by: i32,
}

/// バッチ一覧取得クエリ
#[derive(Debug, Deserialize)]
pub struct BatchListQuery {
    pub status: Option<BatchStatus>,
    pub batch_type: Option<BatchType>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// バッチ統計レスポンス
#[derive(Debug, Serialize)]
pub struct BatchStatisticsResponse {
    pub total_executions: i32,
    pub running_batches: i32,
    pub completed_batches: i32,
    pub failed_batches: i32,
    pub avg_execution_time_minutes: f64,
    pub last_execution: Option<BatchExecution>,
}

/// 手動バッチ実行
pub async fn execute_batch_manually(
    State(_app_state): State<AppState>,
    Json(request): Json<BatchExecutionRequest>,
) -> Result<Json<BatchExecution>, AppError> {
    info!(
        "Manual batch execution requested: {:?} by user {}",
        request.batch_type, request.started_by
    );

    // TODO: 実際のスケジューラー実装に置き換える
    let result = create_mock_batch_execution(request.batch_type, request.started_by);

    Ok(Json(result))
}

/// バッチ実行履歴一覧取得
pub async fn get_batch_executions(
    State(_app_state): State<AppState>,
    Query(query): Query<BatchListQuery>,
) -> Result<Json<Vec<BatchExecution>>, AppError> {
    info!("Getting batch executions: {:?}", query);

    // TODO: 実際のデータベースから取得
    let executions = get_mock_batch_executions(query).await;

    Ok(Json(executions))
}

/// 特定バッチ実行詳細取得
pub async fn get_batch_execution(
    State(_app_state): State<AppState>,
    Path(execution_id): Path<Uuid>,
) -> Result<Json<BatchExecution>, AppError> {
    info!("Getting batch execution: {}", execution_id);

    // TODO: 実際のデータベースから取得
    let execution = get_mock_batch_execution(execution_id)
        .await
        .ok_or_else(|| {
            AppError::NotFound(format!("Batch execution not found: {}", execution_id))
        })?;

    Ok(Json(execution))
}

/// バッチ統計取得
pub async fn get_batch_statistics(
    State(_app_state): State<AppState>,
) -> Result<Json<BatchStatisticsResponse>, AppError> {
    info!("Getting batch statistics");

    // TODO: 実際のデータベースから統計を計算
    let statistics = BatchStatisticsResponse {
        total_executions: 45,
        running_batches: 1,
        completed_batches: 42,
        failed_batches: 2,
        avg_execution_time_minutes: 12.5,
        last_execution: get_mock_latest_execution().await,
    };

    Ok(Json(statistics))
}

/// 実行中バッチ一覧取得
pub async fn get_running_batches(
    State(_app_state): State<AppState>,
) -> Result<Json<Vec<BatchExecution>>, AppError> {
    info!("Getting running batches");

    // TODO: 実際のデータベースから実行中バッチを取得
    let running_batches = get_mock_running_batches().await;

    Ok(Json(running_batches))
}

/// バッチタイプ一覧取得
pub async fn get_batch_types(
    State(_app_state): State<AppState>,
) -> Result<Json<Vec<BatchType>>, AppError> {
    let batch_types = vec![
        BatchType::FileCheck,
        BatchType::AdSync,
        BatchType::DataCleanup,
        BatchType::DocumentBackup,
        BatchType::SystemMaintenance,
    ];

    Ok(Json(batch_types))
}

/// バッチスケジュール情報取得
pub async fn get_batch_schedules(
    State(_app_state): State<AppState>,
) -> Result<Json<HashMap<String, String>>, AppError> {
    let mut schedules = HashMap::new();
    schedules.insert("file_check".to_string(), "毎月1日 9:00".to_string());
    schedules.insert("ad_sync".to_string(), "毎週月曜日 6:00".to_string());
    schedules.insert("data_cleanup".to_string(), "毎日 2:00".to_string());
    schedules.insert(
        "system_maintenance".to_string(),
        "毎月第1日曜日 1:00".to_string(),
    );

    Ok(Json(schedules))
}

/// バッチキャンセル（将来実装）
pub async fn cancel_batch(
    State(_app_state): State<AppState>,
    Path(execution_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    info!("Cancelling batch execution: {}", execution_id);

    // TODO: バッチキャンセル機能の実装
    // 現在はサポートされていない旨を返す
    Err(AppError::InternalError(
        "Batch cancellation not yet implemented".to_string(),
    ))
}

// モックデータ生成関数（実装時はデータベースアクセスに置き換える）

async fn get_mock_batch_executions(_query: BatchListQuery) -> Vec<BatchExecution> {
    use chrono::Utc;

    vec![
        BatchExecution {
            id: Uuid::new_v4(),
            batch_type: BatchType::FileCheck,
            status: BatchStatus::Completed,
            total_items: 100,
            processed_items: 100,
            success_count: 95,
            error_count: 5,
            start_time: Utc::now() - chrono::Duration::hours(2),
            end_time: Some(Utc::now() - chrono::Duration::hours(1)),
            started_by: Some(1),
            result_summary: Some("{\"checked_files\": 100, \"missing_files\": 5}".to_string()),
            error_details: None,
        },
        BatchExecution {
            id: Uuid::new_v4(),
            batch_type: BatchType::AdSync,
            status: BatchStatus::Running,
            total_items: 50,
            processed_items: 25,
            success_count: 24,
            error_count: 1,
            start_time: Utc::now() - chrono::Duration::minutes(30),
            end_time: None,
            started_by: None,
            result_summary: None,
            error_details: None,
        },
    ]
}

async fn get_mock_batch_execution(execution_id: Uuid) -> Option<BatchExecution> {
    use chrono::Utc;

    Some(BatchExecution {
        id: execution_id,
        batch_type: BatchType::FileCheck,
        status: BatchStatus::Completed,
        total_items: 100,
        processed_items: 100,
        success_count: 95,
        error_count: 5,
        start_time: Utc::now() - chrono::Duration::hours(2),
        end_time: Some(Utc::now() - chrono::Duration::hours(1)),
        started_by: Some(1),
        result_summary: Some("{\"checked_files\": 100, \"missing_files\": 5}".to_string()),
        error_details: None,
    })
}

async fn get_mock_latest_execution() -> Option<BatchExecution> {
    use chrono::Utc;

    Some(BatchExecution {
        id: Uuid::new_v4(),
        batch_type: BatchType::FileCheck,
        status: BatchStatus::Completed,
        total_items: 100,
        processed_items: 100,
        success_count: 95,
        error_count: 5,
        start_time: Utc::now() - chrono::Duration::hours(2),
        end_time: Some(Utc::now() - chrono::Duration::hours(1)),
        started_by: Some(1),
        result_summary: Some("{\"checked_files\": 100, \"missing_files\": 5}".to_string()),
        error_details: None,
    })
}

async fn get_mock_running_batches() -> Vec<BatchExecution> {
    use chrono::Utc;

    vec![BatchExecution {
        id: Uuid::new_v4(),
        batch_type: BatchType::AdSync,
        status: BatchStatus::Running,
        total_items: 50,
        processed_items: 25,
        success_count: 24,
        error_count: 1,
        start_time: Utc::now() - chrono::Duration::minutes(30),
        end_time: None,
        started_by: None,
        result_summary: None,
        error_details: None,
    }]
}

fn create_mock_batch_execution(batch_type: BatchType, started_by: i32) -> BatchExecution {
    use chrono::Utc;

    BatchExecution {
        id: uuid::Uuid::new_v4(),
        batch_type,
        status: BatchStatus::Running,
        total_items: 100,
        processed_items: 0,
        success_count: 0,
        error_count: 0,
        start_time: Utc::now(),
        end_time: None,
        started_by: Some(started_by),
        result_summary: None,
        error_details: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Router;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_get_batch_types() {
        let response = get_batch_types().await.unwrap();
        assert_eq!(response.0.len(), 5);
        assert!(response.0.contains(&BatchType::FileCheck));
    }

    #[tokio::test]
    async fn test_get_batch_schedules() {
        let response = get_batch_schedules().await.unwrap();
        assert!(response.0.contains_key("file_check"));
        assert!(response.0.contains_key("ad_sync"));
    }

    #[tokio::test]
    async fn test_mock_data_generation() {
        let query = BatchListQuery {
            status: None,
            batch_type: None,
            limit: None,
            offset: None,
        };

        let executions = get_mock_batch_executions(query).await;
        assert!(!executions.is_empty());

        let execution_id = executions[0].id;
        let execution = get_mock_batch_execution(execution_id).await;
        assert!(execution.is_some());
    }
}
