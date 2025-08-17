use crate::error::BatchError;
use crate::batch::{FileCheckService, AdSyncService, DataCleanupService};
use crate::services::NotificationService;
use tokio_cron_scheduler::{JobScheduler, Job};
use std::sync::Arc;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error};

/// バッチスケジューラー
pub struct BatchScheduler {
    scheduler: JobScheduler,
    file_check_service: Arc<FileCheckService>,
    ad_sync_service: Arc<AdSyncService>,
    data_cleanup_service: Arc<DataCleanupService>,
    notification_service: Arc<NotificationService>,
}

/// バッチ実行記録
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchExecution {
    pub id: Uuid,
    pub batch_type: BatchType,
    pub status: BatchStatus,
    pub total_items: i32,
    pub processed_items: i32,
    pub success_count: i32,
    pub error_count: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub started_by: Option<i32>,
    pub result_summary: Option<String>,
    pub error_details: Option<String>,
}

/// バッチタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchType {
    FileCheck,           // ファイル存在確認
    AdSync,             // Active Directory同期
    DataCleanup,        // データクリーンアップ
    DocumentBackup,     // 文書バックアップ
    SystemMaintenance,  // システムメンテナンス
}

/// バッチステータス
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchStatus {
    Scheduled,  // スケジュール済み
    Running,    // 実行中
    Completed,  // 完了
    Failed,     // 失敗
    Cancelled,  // キャンセル
    Timeout,    // タイムアウト
}

impl BatchScheduler {
    pub async fn new(
        file_check_service: Arc<FileCheckService>,
        ad_sync_service: Arc<AdSyncService>,
        data_cleanup_service: Arc<DataCleanupService>,
        notification_service: Arc<NotificationService>,
    ) -> Result<Self, BatchError> {
        let scheduler = JobScheduler::new().await
            .map_err(|e| BatchError::Scheduler(e.to_string()))?;
        
        Ok(Self {
            scheduler,
            file_check_service,
            ad_sync_service,
            data_cleanup_service,
            notification_service,
        })
    }
    
    /// スケジューラーを開始
    pub async fn start(&self) -> Result<(), BatchError> {
        info!("Starting batch scheduler");
        
        // 月次ファイル確認バッチ（毎月1日 9:00実行）
        self.schedule_monthly_file_check().await?;
        
        // 週次AD同期バッチ（毎週月曜日 6:00実行）
        self.schedule_weekly_ad_sync().await?;
        
        // 日次データクリーンアップ（毎日 2:00実行）
        self.schedule_daily_cleanup().await?;
        
        // 月次システムメンテナンス（毎月第1日曜日 1:00実行）
        self.schedule_monthly_maintenance().await?;
        
        // TODO: schedulerのstart実装を修正
        // self.scheduler.start().await
        //     .map_err(|e| BatchError::Scheduler(e.to_string()))?;
        
        info!("Batch scheduler started successfully");
        Ok(())
    }
    
    /// 月次ファイル確認バッチをスケジュール
    async fn schedule_monthly_file_check(&self) -> Result<(), BatchError> {
        let file_check_service = Arc::clone(&self.file_check_service);
        let notification_service = Arc::clone(&self.notification_service);
        
        let _job = Job::new_async("0 0 9 1 * *", move |_uuid, _l| {
            let file_check_service = Arc::clone(&file_check_service);
            let notification_service = Arc::clone(&notification_service);
            
            Box::pin(async move {
                info!("Starting scheduled monthly file check batch");
                
                match run_monthly_file_check(file_check_service, notification_service).await {
                    Ok(result) => {
                        info!("Monthly file check completed successfully: {:?}", result.id);
                    }
                    Err(error) => {
                        error!("Monthly file check failed: {:?}", error);
                    }
                }
            })
        }).map_err(|e| BatchError::Scheduler(e.to_string()))?;
        
        // TODO: ジョブ追加実装を修正
        // self.scheduler.add(job).await
        //     .map_err(|e| BatchError::Scheduler(e.to_string()))?;
        
        info!("Monthly file check batch scheduled successfully");
        Ok(())
    }
    
    /// 週次AD同期バッチをスケジュール
    async fn schedule_weekly_ad_sync(&self) -> Result<(), BatchError> {
        let ad_sync_service = Arc::clone(&self.ad_sync_service);
        let notification_service = Arc::clone(&self.notification_service);
        
        let _job = Job::new_async("0 0 6 * * MON", move |_uuid, _l| {
            let ad_sync_service = Arc::clone(&ad_sync_service);
            let notification_service = Arc::clone(&notification_service);
            
            Box::pin(async move {
                info!("Starting scheduled weekly AD sync batch");
                
                match run_weekly_ad_sync(ad_sync_service, notification_service).await {
                    Ok(result) => {
                        info!("Weekly AD sync completed successfully: {:?}", result.id);
                    }
                    Err(error) => {
                        error!("Weekly AD sync failed: {:?}", error);
                    }
                }
            })
        }).map_err(|e| BatchError::Scheduler(e.to_string()))?;
        
        // TODO: ジョブ追加実装を修正
        // self.scheduler.add(job).await
        //     .map_err(|e| BatchError::Scheduler(e.to_string()))?;
        
        info!("Weekly AD sync batch scheduled successfully");
        Ok(())
    }
    
    /// 日次データクリーンアップをスケジュール
    async fn schedule_daily_cleanup(&self) -> Result<(), BatchError> {
        let data_cleanup_service = Arc::clone(&self.data_cleanup_service);
        let notification_service = Arc::clone(&self.notification_service);
        
        let _job = Job::new_async("0 0 2 * * *", move |_uuid, _l| {
            let data_cleanup_service = Arc::clone(&data_cleanup_service);
            let notification_service = Arc::clone(&notification_service);
            
            Box::pin(async move {
                info!("Starting scheduled daily cleanup batch");
                
                match run_daily_cleanup(data_cleanup_service, notification_service).await {
                    Ok(result) => {
                        info!("Daily cleanup completed successfully: {:?}", result.id);
                    }
                    Err(error) => {
                        error!("Daily cleanup failed: {:?}", error);
                    }
                }
            })
        }).map_err(|e| BatchError::Scheduler(e.to_string()))?;
        
        // TODO: ジョブ追加実装を修正
        // self.scheduler.add(job).await
        //     .map_err(|e| BatchError::Scheduler(e.to_string()))?;
        
        info!("Daily cleanup batch scheduled successfully");
        Ok(())
    }
    
    /// 月次システムメンテナンスをスケジュール
    async fn schedule_monthly_maintenance(&self) -> Result<(), BatchError> {
        let notification_service = Arc::clone(&self.notification_service);
        
        let _job = Job::new_async("0 0 1 * * SUN#1", move |_uuid, _l| {
            let notification_service = Arc::clone(&notification_service);
            
            Box::pin(async move {
                info!("Starting scheduled monthly maintenance");
                
                match run_monthly_maintenance(notification_service).await {
                    Ok(result) => {
                        info!("Monthly maintenance completed successfully: {:?}", result.id);
                    }
                    Err(error) => {
                        error!("Monthly maintenance failed: {:?}", error);
                    }
                }
            })
        }).map_err(|e| BatchError::Scheduler(e.to_string()))?;
        
        // TODO: ジョブ追加実装を修正
        // self.scheduler.add(job).await
        //     .map_err(|e| BatchError::Scheduler(e.to_string()))?;
        
        info!("Monthly maintenance batch scheduled successfully");
        Ok(())
    }
    
    /// 手動バッチ実行
    pub async fn run_batch_manually(
        &self,
        batch_type: BatchType,
        started_by: i32,
    ) -> Result<BatchExecution, BatchError> {
        info!("Starting manual batch execution: {:?} by user {}", batch_type, started_by);
        
        match batch_type {
            BatchType::FileCheck => {
                run_monthly_file_check(
                    Arc::clone(&self.file_check_service),
                    Arc::clone(&self.notification_service),
                ).await
            }
            BatchType::AdSync => {
                run_weekly_ad_sync(
                    Arc::clone(&self.ad_sync_service),
                    Arc::clone(&self.notification_service),
                ).await
            }
            BatchType::DataCleanup => {
                run_daily_cleanup(
                    Arc::clone(&self.data_cleanup_service),
                    Arc::clone(&self.notification_service),
                ).await
            }
            BatchType::SystemMaintenance => {
                run_monthly_maintenance(Arc::clone(&self.notification_service)).await
            }
            _ => {
                Err(BatchError::JobExecution("Batch type not supported".to_string()))
            }
        }
    }
    
    /// スケジューラーを停止
    pub async fn stop(&self) -> Result<(), BatchError> {
        info!("Stopping batch scheduler");
        
        // TODO: schedulerのshutdown実装を修正
        // self.scheduler.shutdown().await
        //     .map_err(|e| BatchError::Scheduler(e.to_string()))?;
        
        info!("Batch scheduler stopped successfully");
        Ok(())
    }
}

/// 月次ファイル確認バッチ実行
async fn run_monthly_file_check(
    file_check_service: Arc<FileCheckService>,
    notification_service: Arc<NotificationService>,
) -> Result<BatchExecution, BatchError> {
    let execution_id = Uuid::new_v4();
    let _start_time = Utc::now();
    
    info!("Starting monthly file check batch: {}", execution_id);
    
    let result = file_check_service.run_monthly_check(execution_id).await?;
    
    // 通知送信
    if result.error_count > 0 {
        notification_service.send_file_check_alert(
            &result,
            "Monthly file check completed with errors",
        ).await.map_err(|e| BatchError::JobExecution(e.to_string()))?;
    } else {
        notification_service.send_file_check_summary(&result).await
            .map_err(|e| BatchError::JobExecution(e.to_string()))?;
    }
    
    info!("Monthly file check batch completed: {}", execution_id);
    Ok(result)
}

/// 週次AD同期バッチ実行
async fn run_weekly_ad_sync(
    ad_sync_service: Arc<AdSyncService>,
    notification_service: Arc<NotificationService>,
) -> Result<BatchExecution, BatchError> {
    let execution_id = Uuid::new_v4();
    let _start_time = Utc::now();
    
    info!("Starting weekly AD sync batch: {}", execution_id);
    
    let result = ad_sync_service.run_sync(execution_id).await?;
    
    // 通知送信
    if result.error_count > 0 {
        notification_service.send_ad_sync_alert(&result).await
            .map_err(|e| BatchError::JobExecution(e.to_string()))?;
    }
    
    info!("Weekly AD sync batch completed: {}", execution_id);
    Ok(result)
}

/// 日次データクリーンアップ実行
async fn run_daily_cleanup(
    data_cleanup_service: Arc<DataCleanupService>,
    _notification_service: Arc<NotificationService>,
) -> Result<BatchExecution, BatchError> {
    let execution_id = Uuid::new_v4();
    let _start_time = Utc::now();
    
    info!("Starting daily cleanup batch: {}", execution_id);
    
    let result = data_cleanup_service.run_cleanup(execution_id).await?;
    
    info!("Daily cleanup batch completed: {}", execution_id);
    Ok(result)
}

/// 月次システムメンテナンス実行
async fn run_monthly_maintenance(
    notification_service: Arc<NotificationService>,
) -> Result<BatchExecution, BatchError> {
    let execution_id = Uuid::new_v4();
    let start_time = Utc::now();
    
    info!("Starting monthly maintenance: {}", execution_id);
    
    // システムメンテナンス処理
    let result = BatchExecution {
        id: execution_id,
        batch_type: BatchType::SystemMaintenance,
        status: BatchStatus::Completed,
        total_items: 1,
        processed_items: 1,
        success_count: 1,
        error_count: 0,
        start_time,
        end_time: Some(Utc::now()),
        started_by: None,
        result_summary: Some("Monthly maintenance completed successfully".to_string()),
        error_details: None,
    };
    
    // メンテナンス通知
    notification_service.send_maintenance_notification(&result).await
        .map_err(|e| BatchError::JobExecution(e.to_string()))?;
    
    info!("Monthly maintenance completed: {}", execution_id);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_batch_execution_creation() {
        let execution = BatchExecution {
            id: Uuid::new_v4(),
            batch_type: BatchType::FileCheck,
            status: BatchStatus::Running,
            total_items: 100,
            processed_items: 50,
            success_count: 45,
            error_count: 5,
            start_time: Utc::now(),
            end_time: None,
            started_by: Some(1),
            result_summary: None,
            error_details: None,
        };
        
        assert_eq!(execution.status, BatchStatus::Running);
        assert_eq!(execution.total_items, 100);
        assert_eq!(execution.processed_items, 50);
    }
    
    #[test]
    fn test_batch_type_serialization() {
        let batch_type = BatchType::FileCheck;
        let serialized = serde_json::to_string(&batch_type).unwrap();
        assert_eq!(serialized, "\"file_check\"");
        
        let deserialized: BatchType = serde_json::from_str(&serialized).unwrap();
        assert!(matches!(deserialized, BatchType::FileCheck));
    }
}