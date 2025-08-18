use crate::models::backup::{
    BackupJob, BackupRequest, BackupStatistics, RestoreJob, RestoreRequest,
};
use crate::services::BackupService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Json as JsonBody,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// バックアップ作成レスポンス
#[derive(Debug, Serialize)]
pub struct CreateBackupResponse {
    pub job_id: Uuid,
    pub status: String,
    pub message: String,
}

/// リストア作成レスポンス
#[derive(Debug, Serialize)]
pub struct CreateRestoreResponse {
    pub job_id: Uuid,
    pub status: String,
    pub message: String,
}

/// バックアップジョブ一覧レスポンス
#[derive(Debug, Serialize)]
pub struct BackupJobsResponse {
    pub jobs: Vec<BackupJob>,
    pub total_count: usize,
}

/// バックアップクエリパラメータ
#[derive(Debug, Deserialize)]
pub struct BackupQueryParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub status: Option<String>,
}

/// クリーンアップリクエスト
#[derive(Debug, Deserialize)]
pub struct CleanupRequest {
    pub retention_days: Option<i32>,
    pub dry_run: Option<bool>,
}

/// クリーンアップレスポンス
#[derive(Debug, Serialize)]
pub struct CleanupResponse {
    pub deleted_count: i32,
    pub message: String,
}

pub struct BackupHandlers {
    backup_service: Arc<dyn BackupService>,
}

impl BackupHandlers {
    pub fn new(backup_service: Arc<dyn BackupService>) -> Self {
        Self { backup_service }
    }

    /// バックアップ作成
    pub async fn create_backup(
        State(handlers): State<Arc<BackupHandlers>>,
        JsonBody(request): JsonBody<BackupRequest>,
    ) -> Result<Json<CreateBackupResponse>, StatusCode> {
        match handlers.backup_service.create_backup(request).await {
            Ok(job) => Ok(Json(CreateBackupResponse {
                job_id: job.id,
                status: "started".to_string(),
                message: "バックアップを開始しました".to_string(),
            })),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// リストア実行
    pub async fn create_restore(
        State(handlers): State<Arc<BackupHandlers>>,
        JsonBody(request): JsonBody<RestoreRequest>,
    ) -> Result<Json<CreateRestoreResponse>, StatusCode> {
        match handlers.backup_service.restore_backup(request).await {
            Ok(job) => Ok(Json(CreateRestoreResponse {
                job_id: job.id,
                status: "started".to_string(),
                message: "リストアを開始しました".to_string(),
            })),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// バックアップジョブ詳細取得
    pub async fn get_backup_job(
        State(handlers): State<Arc<BackupHandlers>>,
        Path(job_id): Path<Uuid>,
    ) -> Result<Json<BackupJob>, StatusCode> {
        match handlers.backup_service.get_backup_job(job_id).await {
            Ok(Some(job)) => Ok(Json(job)),
            Ok(None) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// バックアップジョブ一覧取得
    pub async fn list_backup_jobs(
        State(handlers): State<Arc<BackupHandlers>>,
        Query(params): Query<BackupQueryParams>,
    ) -> Result<Json<BackupJobsResponse>, StatusCode> {
        let limit = params.limit.unwrap_or(50);

        match handlers.backup_service.list_backup_jobs(Some(limit)).await {
            Ok(jobs) => {
                let total_count = jobs.len();
                Ok(Json(BackupJobsResponse { jobs, total_count }))
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// バックアップ削除
    pub async fn delete_backup(
        State(handlers): State<Arc<BackupHandlers>>,
        Path(job_id): Path<Uuid>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        match handlers.backup_service.delete_backup(job_id).await {
            Ok(()) => Ok(Json(serde_json::json!({
                "message": "バックアップを削除しました",
                "job_id": job_id
            }))),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// バックアップ統計取得
    pub async fn get_backup_statistics(
        State(handlers): State<Arc<BackupHandlers>>,
    ) -> Result<Json<BackupStatistics>, StatusCode> {
        match handlers.backup_service.get_backup_statistics().await {
            Ok(stats) => Ok(Json(stats)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// 古いバックアップのクリーンアップ
    pub async fn cleanup_old_backups(
        State(handlers): State<Arc<BackupHandlers>>,
        JsonBody(request): JsonBody<CleanupRequest>,
    ) -> Result<Json<CleanupResponse>, StatusCode> {
        let retention_days = request.retention_days.unwrap_or(30);

        if request.dry_run.unwrap_or(false) {
            // ドライランモード（実際には削除しない）
            return Ok(Json(CleanupResponse {
                deleted_count: 0,
                message: format!("ドライランモード: {}日以前のバックアップが削除対象です", retention_days),
            }));
        }

        match handlers
            .backup_service
            .cleanup_old_backups(retention_days)
            .await
        {
            Ok(deleted_count) => Ok(Json(CleanupResponse {
                deleted_count,
                message: format!("{}個のバックアップファイルを削除しました", deleted_count),
            })),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// バックアップ設定取得
    pub async fn get_backup_config(
        State(_handlers): State<Arc<BackupHandlers>>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        // 実際の実装では設定サービスから取得
        Ok(Json(serde_json::json!({
            "backup_directory": "./backups",
            "max_concurrent_jobs": 2,
            "default_retention_days": 30,
            "compression_level": 6,
            "auto_cleanup_enabled": true
        })))
    }

    /// バックアップ設定更新
    pub async fn update_backup_config(
        State(_handlers): State<Arc<BackupHandlers>>,
        JsonBody(config): JsonBody<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        // 実際の実装では設定サービスで更新
        Ok(Json(serde_json::json!({
            "message": "バックアップ設定を更新しました",
            "config": config
        })))
    }

    /// ヘルスチェック
    pub async fn health_check(
        State(handlers): State<Arc<BackupHandlers>>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        // バックアップサービスの状態確認
        match handlers.backup_service.get_backup_statistics().await {
            Ok(_) => Ok(Json(serde_json::json!({
                "status": "healthy",
                "service": "backup",
                "timestamp": chrono::Utc::now()
            }))),
            Err(_) => Ok(Json(serde_json::json!({
                "status": "unhealthy",
                "service": "backup",
                "timestamp": chrono::Utc::now()
            }))),
        }
    }
}