use crate::AppState;
use crate::graphql::types::*;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get a document by ID
    async fn document(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Document>> {
        let state = ctx.data::<AppState>()?;

        match state.document_handlers.get_document(id).await {
            Ok(doc) => Ok(Some(doc.into())),
            Err(crate::error::AppError::NotFound(_)) => Ok(None),
            Err(e) => Err(async_graphql::Error::new(format!("Database error: {e}"))),
        }
    }

    /// Search documents
    async fn search_documents(
        &self,
        ctx: &Context<'_>,
        filters: DocumentSearchFilters,
    ) -> Result<SearchDocumentsResult> {
        let state = ctx.data::<AppState>()?;
        let search_filters = filters.into();

        match state
            .document_handlers
            .search_documents(search_filters)
            .await
        {
            Ok((documents, total)) => {
                let graphql_documents: Vec<Document> =
                    documents.into_iter().map(|d| d.into()).collect();
                Ok(SearchDocumentsResult {
                    documents: graphql_documents,
                    total,
                })
            }
            Err(e) => Err(async_graphql::Error::new(format!("Search error: {e}"))),
        }
    }

    /// Get circulation workflows (placeholder implementation)
    async fn circulation_workflows(&self, _ctx: &Context<'_>) -> Result<Vec<CirculationWorkflow>> {
        // TODO: Implement when AppState includes circulation_service
        Ok(vec![])
    }

    /// Get pending circulations for current user (placeholder implementation)
    async fn my_pending_circulations(&self, _ctx: &Context<'_>) -> Result<Vec<CirculationStep>> {
        // TODO: Implement when AppState includes circulation_service
        Ok(vec![])
    }

    /// Get circulations for a document (placeholder implementation)
    async fn document_circulations(
        &self,
        _ctx: &Context<'_>,
        _document_id: i32,
    ) -> Result<Vec<DocumentCirculation>> {
        // TODO: Implement when AppState includes circulation_service
        Ok(vec![])
    }

    /// Get dashboard statistics
    async fn dashboard_stats(&self, _ctx: &Context<'_>) -> Result<DashboardStats> {
        // Mock data for now - TODO: Implement with real statistics
        Ok(DashboardStats {
            total_documents: 152,
            monthly_created: 23,
            missing_files: 3,
            active_users: 28,
            pending_approvals: 7,
            system_uptime: 99.8,
        })
    }

    /// Get system status
    async fn system_status(&self, _ctx: &Context<'_>) -> Result<SystemStatus> {
        // Mock data for now - TODO: Implement with real system monitoring
        Ok(SystemStatus {
            api_status: "healthy".to_string(),
            database_status: "healthy".to_string(),
            file_system_status: "healthy".to_string(),
            last_backup: "2024-08-23T02:00:00Z".to_string(),
            server_uptime: "7 days, 14 hours".to_string(),
            memory_usage: 68.5,
            disk_usage: 45.2,
        })
    }

    /// Get recent activities
    async fn recent_activities(
        &self,
        _ctx: &Context<'_>,
        limit: Option<i32>,
    ) -> Result<Vec<Activity>> {
        let limit = limit.unwrap_or(10) as usize;

        // Mock data for now - TODO: Implement with real activity tracking
        let all_activities = vec![
            Activity {
                id: "1".to_string(),
                activity_type: "create".to_string(),
                message: "システム設計書 v3.0を作成しました".to_string(),
                user: "山田太郎".to_string(),
                timestamp: "2024-08-23T14:30:00Z".to_string(),
                document_id: Some("DOC-001".to_string()),
                document_title: Some("システム設計書 v3.0".to_string()),
            },
            Activity {
                id: "2".to_string(),
                activity_type: "approval".to_string(),
                message: "データベース移行計画書の承認を依頼しました".to_string(),
                user: "佐藤花子".to_string(),
                timestamp: "2024-08-23T13:45:00Z".to_string(),
                document_id: Some("DOC-002".to_string()),
                document_title: Some("データベース移行計画書".to_string()),
            },
            Activity {
                id: "3".to_string(),
                activity_type: "update".to_string(),
                message: "運用手順書 v2.1を更新しました".to_string(),
                user: "田中一郎".to_string(),
                timestamp: "2024-08-23T11:15:00Z".to_string(),
                document_id: Some("DOC-003".to_string()),
                document_title: Some("運用手順書 v2.1".to_string()),
            },
        ];

        Ok(all_activities.into_iter().take(limit).collect())
    }

    /// Get pending approvals
    async fn pending_approvals(
        &self,
        _ctx: &Context<'_>,
        limit: Option<i32>,
    ) -> Result<Vec<PendingApproval>> {
        let limit = limit.unwrap_or(5) as usize;

        // Mock data for now - TODO: Implement with real approval system
        let all_approvals = vec![
            PendingApproval {
                id: "approval-1".to_string(),
                document_id: "DOC-002".to_string(),
                document_title: "データベース移行計画書".to_string(),
                requester_name: "佐藤花子".to_string(),
                requested_at: "2024-08-23T13:45:00Z".to_string(),
                approval_type: "approval".to_string(),
            },
            PendingApproval {
                id: "approval-2".to_string(),
                document_id: "DOC-005".to_string(),
                document_title: "セキュリティポリシー更新版".to_string(),
                requester_name: "鈴木太郎".to_string(),
                requested_at: "2024-08-23T10:30:00Z".to_string(),
                approval_type: "review".to_string(),
            },
        ];

        Ok(all_approvals.into_iter().take(limit).collect())
    }
}

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a document with auto-generated number
    async fn create_document(
        &self,
        ctx: &Context<'_>,
        input: CreateDocumentInput,
    ) -> Result<CreatedDocumentWithNumber> {
        let state = ctx.data::<AppState>()?;
        let request = input.into();

        match state.document_handlers.create_document(request).await {
            Ok(created) => Ok(created.into()),
            Err(e) => Err(async_graphql::Error::new(format!("Creation error: {e}"))),
        }
    }

    /// Create a new circulation (placeholder implementation)
    async fn create_circulation(
        &self,
        _ctx: &Context<'_>,
        _input: CreateCirculationInput,
    ) -> Result<CirculationResponse> {
        // TODO: Implement when AppState includes circulation_service
        Ok(CirculationResponse {
            success: false,
            circulation: None,
            message: "Not implemented yet".to_string(),
        })
    }

    /// Complete a circulation step (placeholder implementation)
    async fn complete_circulation_step(
        &self,
        _ctx: &Context<'_>,
        _input: CompleteStepInput,
    ) -> Result<StepResponse> {
        // TODO: Implement when AppState includes circulation_service
        Ok(StepResponse {
            success: false,
            step: None,
            message: "Not implemented yet".to_string(),
        })
    }

    /// Cancel a circulation (placeholder implementation)
    async fn cancel_circulation(
        &self,
        _ctx: &Context<'_>,
        _id: i32,
        _reason: Option<String>,
    ) -> Result<CirculationResponse> {
        // TODO: Implement when AppState includes circulation_service
        Ok(CirculationResponse {
            success: false,
            circulation: None,
            message: "Not implemented yet".to_string(),
        })
    }
}
