use crate::error::BatchError;
use crate::batch::{BatchExecution, BatchType, BatchStatus};
use crate::models::Employee;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error};

/// Active Directory同期サービス
pub struct AdSyncService {
    // TODO: AD接続設定を追加
}

/// AD同期結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdSyncResult {
    pub sync_type: AdSyncType,
    pub total_ad_users: i32,
    pub new_users: i32,
    pub updated_users: i32,
    pub deactivated_users: i32,
    pub sync_errors: i32,
    pub sync_time: DateTime<Utc>,
    pub error_details: Vec<AdSyncError>,
}

/// AD同期タイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdSyncType {
    Full,        // 全体同期
    Incremental, // 差分同期
    Manual,      // 手動同期
}

/// AD同期エラー
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdSyncError {
    pub ad_username: String,
    pub error_type: String,
    pub error_message: String,
    pub occurred_at: DateTime<Utc>,
}

/// ADユーザー情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdUser {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub employee_number: Option<String>,
    pub department: Option<String>,
    pub is_enabled: bool,
    pub last_logon: Option<DateTime<Utc>>,
    pub created_date: DateTime<Utc>,
    pub modified_date: DateTime<Utc>,
}

impl AdSyncService {
    pub fn new() -> Self {
        Self {}
    }
    
    /// AD同期を実行
    pub async fn run_sync(&self, execution_id: Uuid) -> Result<BatchExecution, BatchError> {
        info!("Starting AD sync: {}", execution_id);
        
        let start_time = Utc::now();
        let mut execution = BatchExecution {
            id: execution_id,
            batch_type: BatchType::AdSync,
            status: BatchStatus::Running,
            total_items: 0,
            processed_items: 0,
            success_count: 0,
            error_count: 0,
            start_time,
            end_time: None,
            started_by: None,
            result_summary: None,
            error_details: None,
        };
        
        // AD同期実行
        let sync_result = self.perform_ad_sync(AdSyncType::Incremental).await?;
        
        // 実行結果の更新
        execution.total_items = sync_result.total_ad_users;
        execution.processed_items = sync_result.new_users + sync_result.updated_users + sync_result.deactivated_users;
        execution.success_count = sync_result.new_users + sync_result.updated_users;
        execution.error_count = sync_result.sync_errors;
        execution.end_time = Some(Utc::now());
        
        execution.status = if execution.error_count == 0 {
            BatchStatus::Completed
        } else if execution.success_count > 0 {
            BatchStatus::Completed
        } else {
            BatchStatus::Failed
        };
        
        execution.result_summary = Some(serde_json::to_string(&sync_result)?);
        
        info!(
            "AD sync completed: {}/{} successful, {} errors, duration: {:?}",
            execution.success_count,
            execution.total_items,
            execution.error_count,
            execution.end_time.unwrap() - execution.start_time
        );
        
        Ok(execution)
    }
    
    /// AD同期処理を実行
    async fn perform_ad_sync(&self, sync_type: AdSyncType) -> Result<AdSyncResult, BatchError> {
        info!("Performing AD sync: {:?}", sync_type);
        
        let start_time = Utc::now();
        let mut sync_result = AdSyncResult {
            sync_type,
            total_ad_users: 0,
            new_users: 0,
            updated_users: 0,
            deactivated_users: 0,
            sync_errors: 0,
            sync_time: start_time,
            error_details: Vec::new(),
        };
        
        // ADからユーザー情報を取得
        let ad_users = self.fetch_ad_users().await?;
        sync_result.total_ad_users = ad_users.len() as i32;
        
        info!("Fetched {} users from AD", ad_users.len());
        
        // 既存のローカルユーザーを取得
        let local_users = self.get_local_users().await?;
        
        // AD ユーザーごとに同期処理
        for ad_user in &ad_users {
            match self.sync_user(ad_user, &local_users).await {
                Ok(sync_action) => {
                    match sync_action {
                        UserSyncAction::Created => sync_result.new_users += 1,
                        UserSyncAction::Updated => sync_result.updated_users += 1,
                        UserSyncAction::NoChange => {},
                    }
                }
                Err(error) => {
                    sync_result.sync_errors += 1;
                    sync_result.error_details.push(AdSyncError {
                        ad_username: ad_user.username.clone(),
                        error_type: "sync_error".to_string(),
                        error_message: error.to_string(),
                        occurred_at: Utc::now(),
                    });
                    
                    warn!("Failed to sync user {}: {}", ad_user.username, error);
                }
            }
        }
        
        // 非アクティブユーザーの検出と処理
        let deactivated_count = self.deactivate_removed_users(&ad_users, &local_users).await?;
        sync_result.deactivated_users = deactivated_count;
        
        info!(
            "AD sync completed: {} total, {} new, {} updated, {} deactivated, {} errors",
            sync_result.total_ad_users,
            sync_result.new_users,
            sync_result.updated_users,
            sync_result.deactivated_users,
            sync_result.sync_errors
        );
        
        Ok(sync_result)
    }
    
    /// ADからユーザー情報を取得
    async fn fetch_ad_users(&self) -> Result<Vec<AdUser>, BatchError> {
        // TODO: 実際のAD接続実装
        // 現在は仮のデータを返す
        info!("Fetching users from Active Directory");
        
        Ok(vec![
            AdUser {
                username: "yamada.taro".to_string(),
                display_name: "山田太郎".to_string(),
                email: "yamada@company.com".to_string(),
                employee_number: Some("001".to_string()),
                department: Some("情報システム部".to_string()),
                is_enabled: true,
                last_logon: Some(Utc::now()),
                created_date: Utc::now(),
                modified_date: Utc::now(),
            },
            AdUser {
                username: "sato.hanako".to_string(),
                display_name: "佐藤花子".to_string(),
                email: "sato@company.com".to_string(),
                employee_number: Some("002".to_string()),
                department: Some("システム開発部".to_string()),
                is_enabled: true,
                last_logon: Some(Utc::now()),
                created_date: Utc::now(),
                modified_date: Utc::now(),
            },
            AdUser {
                username: "tanaka.ichiro".to_string(),
                display_name: "田中一郎".to_string(),
                email: "tanaka@company.com".to_string(),
                employee_number: Some("003".to_string()),
                department: Some("運用管理部".to_string()),
                is_enabled: false, // 無効化されたユーザー
                last_logon: None,
                created_date: Utc::now(),
                modified_date: Utc::now(),
            },
        ])
    }
    
    /// ローカルユーザーを取得
    async fn get_local_users(&self) -> Result<Vec<Employee>, BatchError> {
        // TODO: 実際のデータベースから取得
        Ok(vec![
            Employee {
                id: 1,
                employee_number: Some("001".to_string()),
                name: "山田太郎".to_string(),
                email: Some("yamada@company.com".to_string()),
                ad_username: Some("yamada.taro".to_string()),
                department_id: Some(1),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ])
    }
    
    /// ユーザー同期処理
    async fn sync_user(&self, ad_user: &AdUser, local_users: &[Employee]) -> Result<UserSyncAction, BatchError> {
        // ADユーザー名で既存ユーザーを検索
        let existing_user = local_users.iter()
            .find(|u| u.ad_username.as_ref().map_or(false, |ad| ad == &ad_user.username));
        
        match existing_user {
            Some(user) => {
                // 既存ユーザーの更新
                if self.user_needs_update(user, ad_user) {
                    self.update_local_user(user.id, ad_user).await?;
                    Ok(UserSyncAction::Updated)
                } else {
                    Ok(UserSyncAction::NoChange)
                }
            }
            None => {
                // 新規ユーザーの作成
                if ad_user.is_enabled {
                    self.create_local_user(ad_user).await?;
                    Ok(UserSyncAction::Created)
                } else {
                    // 無効なユーザーは作成しない
                    Ok(UserSyncAction::NoChange)
                }
            }
        }
    }
    
    /// ユーザーの更新が必要かチェック
    fn user_needs_update(&self, local_user: &Employee, ad_user: &AdUser) -> bool {
        local_user.name != ad_user.display_name
            || local_user.email.as_ref() != Some(&ad_user.email)
            || local_user.is_active != ad_user.is_enabled
    }
    
    /// ローカルユーザーを更新
    async fn update_local_user(&self, user_id: i32, ad_user: &AdUser) -> Result<(), BatchError> {
        info!("Updating local user {}: {}", user_id, ad_user.username);
        
        // TODO: 実際のデータベース更新
        Ok(())
    }
    
    /// 新規ローカルユーザーを作成
    async fn create_local_user(&self, ad_user: &AdUser) -> Result<(), BatchError> {
        info!("Creating new local user: {}", ad_user.username);
        
        // TODO: 実際のデータベース作成
        Ok(())
    }
    
    /// ADから削除されたユーザーを非アクティブ化
    async fn deactivate_removed_users(&self, ad_users: &[AdUser], local_users: &[Employee]) -> Result<i32, BatchError> {
        let mut deactivated_count = 0;
        
        for local_user in local_users {
            if !local_user.is_active {
                continue; // 既に非アクティブ
            }
            
            if let Some(ad_username) = &local_user.ad_username {
                // ADに存在しない、または無効化されたユーザーを非アクティブ化
                let ad_user = ad_users.iter().find(|ad| &ad.username == ad_username);
                
                match ad_user {
                    Some(ad) if !ad.is_enabled => {
                        // ADで無効化されている
                        self.deactivate_local_user(local_user.id).await?;
                        deactivated_count += 1;
                        info!("Deactivated user {} (disabled in AD)", ad_username);
                    }
                    None => {
                        // ADに存在しない
                        self.deactivate_local_user(local_user.id).await?;
                        deactivated_count += 1;
                        info!("Deactivated user {} (removed from AD)", ad_username);
                    }
                    _ => {} // アクティブなユーザー
                }
            }
        }
        
        Ok(deactivated_count)
    }
    
    /// ローカルユーザーを非アクティブ化
    async fn deactivate_local_user(&self, _user_id: i32) -> Result<(), BatchError> {
        // TODO: 実際のデータベース更新
        Ok(())
    }
    
    /// 手動同期の実行
    pub async fn run_manual_sync(&self, started_by: i32) -> Result<AdSyncResult, BatchError> {
        info!("Starting manual AD sync by user {}", started_by);
        self.perform_ad_sync(AdSyncType::Manual).await
    }
    
    /// 完全同期の実行
    pub async fn run_full_sync(&self) -> Result<AdSyncResult, BatchError> {
        info!("Starting full AD sync");
        self.perform_ad_sync(AdSyncType::Full).await
    }
}

/// ユーザー同期アクション
#[derive(Debug)]
enum UserSyncAction {
    Created,
    Updated,
    NoChange,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_needs_update() {
        let service = AdSyncService::new();
        
        let local_user = Employee {
            id: 1,
            employee_number: Some("001".to_string()),
            name: "山田太郎".to_string(),
            email: Some("yamada@company.com".to_string()),
            ad_username: Some("yamada.taro".to_string()),
            department_id: Some(1),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let ad_user_same = AdUser {
            username: "yamada.taro".to_string(),
            display_name: "山田太郎".to_string(),
            email: "yamada@company.com".to_string(),
            employee_number: Some("001".to_string()),
            department: Some("情報システム部".to_string()),
            is_enabled: true,
            last_logon: Some(Utc::now()),
            created_date: Utc::now(),
            modified_date: Utc::now(),
        };
        
        assert!(!service.user_needs_update(&local_user, &ad_user_same));
        
        let ad_user_different = AdUser {
            display_name: "山田太郎（更新）".to_string(),
            ..ad_user_same
        };
        
        assert!(service.user_needs_update(&local_user, &ad_user_different));
    }
    
    #[tokio::test]
    async fn test_ad_sync_service_creation() {
        let service = AdSyncService::new();
        let ad_users = service.fetch_ad_users().await.unwrap();
        
        assert!(!ad_users.is_empty());
        assert!(ad_users.iter().any(|u| u.username == "yamada.taro"));
    }
}