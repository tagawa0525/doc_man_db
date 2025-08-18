use crate::models::migration::{
    MigrationJob, MigrationPlan, MigrationRequest, MigrationStatistics, RollbackRequest,
    ValidationType,
};
use crate::services::MigrationService;
use axum::{
    Json as JsonBody,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// 移行計画作成レスポンス
#[derive(Debug, Serialize)]
pub struct CreateMigrationPlanResponse {
    pub plan_id: Uuid,
    pub status: String,
    pub message: String,
}

/// 移行実行レスポンス
#[derive(Debug, Serialize)]
pub struct ExecuteMigrationResponse {
    pub job_id: Uuid,
    pub status: String,
    pub message: String,
    pub dry_run: bool,
}

/// 移行停止レスポンス
#[derive(Debug, Serialize)]
pub struct StopMigrationResponse {
    pub job_id: Uuid,
    pub message: String,
}

/// ロールバックレスポンス
#[derive(Debug, Serialize)]
pub struct RollbackResponse {
    pub job_id: Uuid,
    pub status: String,
    pub message: String,
}

/// 移行検証レスポンス
#[derive(Debug, Serialize)]
pub struct ValidationResponse {
    pub validation_id: Uuid,
    pub status: String,
    pub risk_level: String,
    pub issues_count: usize,
    pub message: String,
}

/// 移行計画一覧レスポンス
#[derive(Debug, Serialize)]
pub struct MigrationPlansResponse {
    pub plans: Vec<MigrationPlan>,
    pub total_count: usize,
}

/// 移行ジョブ一覧レスポンス
#[derive(Debug, Serialize)]
pub struct MigrationJobsResponse {
    pub jobs: Vec<MigrationJob>,
    pub total_count: usize,
}

/// 移行クエリパラメータ
#[derive(Debug, Deserialize)]
pub struct MigrationQueryParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub status: Option<String>,
    pub environment: Option<String>,
}

/// 検証リクエスト
#[derive(Debug, Deserialize)]
pub struct ValidationRequest {
    pub validation_type: ValidationType,
    pub validator: Option<String>,
}

/// 移行停止リクエスト
#[derive(Debug, Deserialize)]
pub struct StopMigrationRequest {
    pub reason: String,
    pub force_stop: Option<bool>,
}

pub struct MigrationHandlers {
    migration_service: Arc<dyn MigrationService>,
}

impl MigrationHandlers {
    pub fn new(migration_service: Arc<dyn MigrationService>) -> Self {
        Self { migration_service }
    }

    /// 移行計画作成
    pub async fn create_migration_plan(
        State(handlers): State<Arc<MigrationHandlers>>,
        JsonBody(plan): JsonBody<MigrationPlan>,
    ) -> Result<Json<CreateMigrationPlanResponse>, StatusCode> {
        match handlers.migration_service.create_migration_plan(plan).await {
            Ok(created_plan) => Ok(Json(CreateMigrationPlanResponse {
                plan_id: created_plan.id,
                status: "created".to_string(),
                message: "移行計画を作成しました".to_string(),
            })),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// 移行計画検証
    pub async fn validate_migration_plan(
        State(handlers): State<Arc<MigrationHandlers>>,
        Path(plan_id): Path<Uuid>,
        JsonBody(request): JsonBody<ValidationRequest>,
    ) -> Result<Json<ValidationResponse>, StatusCode> {
        let validator = request.validator.unwrap_or_else(|| "system".to_string());

        match handlers
            .migration_service
            .validate_migration_plan(plan_id, request.validation_type, validator)
            .await
        {
            Ok(validation) => Ok(Json(ValidationResponse {
                validation_id: validation.id,
                status: format!("{:?}", validation.status),
                risk_level: format!("{:?}", validation.risk_level),
                issues_count: validation.issues.len(),
                message: if validation.issues.is_empty() {
                    "検証が正常に完了しました".to_string()
                } else {
                    format!("{}件の問題が検出されました", validation.issues.len())
                },
            })),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// 移行実行
    pub async fn execute_migration(
        State(handlers): State<Arc<MigrationHandlers>>,
        JsonBody(request): JsonBody<MigrationRequest>,
    ) -> Result<Json<ExecuteMigrationResponse>, StatusCode> {
        let is_dry_run = request.dry_run.unwrap_or(false);

        match handlers.migration_service.execute_migration(request).await {
            Ok(job) => Ok(Json(ExecuteMigrationResponse {
                job_id: job.id,
                status: format!("{:?}", job.status),
                message: if is_dry_run {
                    "ドライラン移行を開始しました".to_string()
                } else {
                    "移行を開始しました".to_string()
                },
                dry_run: is_dry_run,
            })),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// 移行ジョブ詳細取得
    pub async fn get_migration_job(
        State(handlers): State<Arc<MigrationHandlers>>,
        Path(job_id): Path<Uuid>,
    ) -> Result<Json<MigrationJob>, StatusCode> {
        match handlers.migration_service.get_migration_job(job_id).await {
            Ok(Some(job)) => Ok(Json(job)),
            Ok(None) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// 移行停止
    pub async fn stop_migration(
        State(handlers): State<Arc<MigrationHandlers>>,
        Path(job_id): Path<Uuid>,
        JsonBody(_request): JsonBody<StopMigrationRequest>,
    ) -> Result<Json<StopMigrationResponse>, StatusCode> {
        match handlers.migration_service.stop_migration(job_id).await {
            Ok(()) => Ok(Json(StopMigrationResponse {
                job_id,
                message: "移行を停止しました".to_string(),
            })),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// ロールバック実行
    pub async fn rollback_migration(
        State(handlers): State<Arc<MigrationHandlers>>,
        JsonBody(request): JsonBody<RollbackRequest>,
    ) -> Result<Json<RollbackResponse>, StatusCode> {
        match handlers.migration_service.rollback_migration(request).await {
            Ok(job) => Ok(Json(RollbackResponse {
                job_id: job.id,
                status: format!("{:?}", job.status),
                message: "ロールバックを開始しました".to_string(),
            })),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// 移行計画一覧取得
    pub async fn list_migration_plans(
        State(handlers): State<Arc<MigrationHandlers>>,
        Query(_params): Query<MigrationQueryParams>,
    ) -> Result<Json<MigrationPlansResponse>, StatusCode> {
        match handlers.migration_service.list_migration_plans().await {
            Ok(plans) => {
                let total_count = plans.len();
                Ok(Json(MigrationPlansResponse { plans, total_count }))
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// 移行ジョブ一覧取得
    pub async fn list_migration_jobs(
        State(handlers): State<Arc<MigrationHandlers>>,
        Query(params): Query<MigrationQueryParams>,
    ) -> Result<Json<MigrationJobsResponse>, StatusCode> {
        let limit = params.limit.unwrap_or(50);

        match handlers
            .migration_service
            .list_migration_jobs(Some(limit))
            .await
        {
            Ok(jobs) => {
                let total_count = jobs.len();
                Ok(Json(MigrationJobsResponse { jobs, total_count }))
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// 移行統計取得
    pub async fn get_migration_statistics(
        State(handlers): State<Arc<MigrationHandlers>>,
    ) -> Result<Json<MigrationStatistics>, StatusCode> {
        match handlers.migration_service.get_migration_statistics().await {
            Ok(stats) => Ok(Json(stats)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// 移行計画承認
    pub async fn approve_migration_plan(
        State(_handlers): State<Arc<MigrationHandlers>>,
        Path(plan_id): Path<Uuid>,
        JsonBody(approval): JsonBody<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        // 実際の実装では、承認処理を行う
        let approver = approval
            .get("approver")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        Ok(Json(serde_json::json!({
            "plan_id": plan_id,
            "status": "approved",
            "approver": approver,
            "message": "移行計画を承認しました"
        })))
    }

    /// 移行計画削除
    pub async fn delete_migration_plan(
        State(_handlers): State<Arc<MigrationHandlers>>,
        Path(plan_id): Path<Uuid>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        // 実際の実装では、計画削除処理を行う
        Ok(Json(serde_json::json!({
            "plan_id": plan_id,
            "message": "移行計画を削除しました"
        })))
    }

    /// 移行ジョブのログ取得
    pub async fn get_migration_job_logs(
        State(_handlers): State<Arc<MigrationHandlers>>,
        Path(job_id): Path<Uuid>,
        Query(_params): Query<MigrationQueryParams>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        // 実際の実装では、ジョブのログを取得
        Ok(Json(serde_json::json!({
            "job_id": job_id,
            "logs": [],
            "total_count": 0
        })))
    }

    /// 移行設定取得
    pub async fn get_migration_config(
        State(_handlers): State<Arc<MigrationHandlers>>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        // 実際の実装では設定サービスから取得
        Ok(Json(serde_json::json!({
            "max_concurrent_migrations": 2,
            "default_timeout_minutes": 120,
            "auto_rollback_on_failure": true,
            "notification_enabled": true,
            "validation_required": true
        })))
    }

    /// 移行設定更新
    pub async fn update_migration_config(
        State(_handlers): State<Arc<MigrationHandlers>>,
        JsonBody(config): JsonBody<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        // 実際の実装では設定サービスで更新
        Ok(Json(serde_json::json!({
            "message": "移行設定を更新しました",
            "config": config
        })))
    }

    /// ヘルスチェック
    pub async fn health_check(
        State(handlers): State<Arc<MigrationHandlers>>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        // 移行サービスの状態確認
        match handlers.migration_service.get_migration_statistics().await {
            Ok(_) => Ok(Json(serde_json::json!({
                "status": "healthy",
                "service": "migration",
                "timestamp": chrono::Utc::now()
            }))),
            Err(_) => Ok(Json(serde_json::json!({
                "status": "unhealthy",
                "service": "migration",
                "timestamp": chrono::Utc::now()
            }))),
        }
    }

    /// 移行進捗監視
    pub async fn get_migration_progress(
        State(_handlers): State<Arc<MigrationHandlers>>,
        Path(job_id): Path<Uuid>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        // 実際の実装では、リアルタイムの進捗情報を取得
        Ok(Json(serde_json::json!({
            "job_id": job_id,
            "progress_percentage": 0.0,
            "current_step": null,
            "estimated_completion": null,
            "data_processed_mb": null
        })))
    }

    /// 移行スケジュール
    pub async fn schedule_migration(
        State(_handlers): State<Arc<MigrationHandlers>>,
        Path(plan_id): Path<Uuid>,
        JsonBody(schedule): JsonBody<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        // 実際の実装では、スケジューラーに登録
        let scheduled_at = schedule.get("scheduled_at");

        Ok(Json(serde_json::json!({
            "plan_id": plan_id,
            "scheduled_at": scheduled_at,
            "message": "移行をスケジュールしました"
        })))
    }
}
