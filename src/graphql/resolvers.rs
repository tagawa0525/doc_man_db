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
        println!("DEBUG: search_documents called with filters: {:?}", filters);
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
    async fn dashboard_stats(&self, ctx: &Context<'_>) -> Result<DashboardStats> {
        let state = ctx.data::<AppState>()?;
        
        // Get total documents count by searching with no filters
        let filters = crate::models::DocumentSearchFilters {
            title: None,
            document_type_id: None,
            created_by: None,
            created_date_from: None,
            created_date_to: None,
            limit: 1, // We only need the count
            offset: 0,
        };
        
        let (_documents, total_documents) = state.document_handlers
            .search_documents(filters)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get document count: {e}")))?;
        
        // Get monthly created documents (current month)
        use chrono::{Datelike, Local, NaiveDate};
        let now = Local::now().naive_local().date();
        let month_start = NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
            .ok_or_else(|| async_graphql::Error::new("Invalid date"))?;
        
        let monthly_filters = crate::models::DocumentSearchFilters {
            title: None,
            document_type_id: None,
            created_by: None,
            created_date_from: Some(month_start),
            created_date_to: Some(now),
            limit: 1,
            offset: 0,
        };
        
        let (_monthly_docs, monthly_created) = state.document_handlers
            .search_documents(monthly_filters)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get monthly count: {e}")))?;
        
        Ok(DashboardStats {
            total_documents: total_documents as i32,
            monthly_created: monthly_created as i32,
            missing_files: 0, // TODO: Implement file existence checking
            active_users: 1, // TODO: Implement user activity tracking
            pending_approvals: 0, // TODO: Implement approval system
            system_uptime: 99.9,
        })
    }

    /// Get system status
    async fn system_status(&self, ctx: &Context<'_>) -> Result<SystemStatus> {
        let state = ctx.data::<AppState>()?;
        
        // Test database connectivity by performing a simple query
        let database_status = match state.document_handlers
            .search_documents(crate::models::DocumentSearchFilters {
                title: None,
                document_type_id: None,
                created_by: None,
                created_date_from: None,
                created_date_to: None,
                limit: 1,
                offset: 0,
            })
            .await
        {
            Ok(_) => "healthy",
            Err(_) => "error",
        };
        
        // Test API status by checking health endpoint availability
        let api_status = "healthy"; // If we're here, API is working
        
        // Get current timestamp for consistent formatting
        use chrono::Utc;
        let now = Utc::now();
        
        Ok(SystemStatus {
            api_status: api_status.to_string(),
            database_status: database_status.to_string(),
            file_system_status: "healthy".to_string(), // TODO: Implement file system checks
            last_backup: "Not implemented".to_string(), // TODO: Implement backup tracking
            server_uptime: format!("Running since {}", now.format("%Y-%m-%d %H:%M:%S")),
            memory_usage: 0.0, // TODO: Implement memory monitoring
            disk_usage: 0.0,   // TODO: Implement disk monitoring
        })
    }

    /// Get recent activities
    async fn recent_activities(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
    ) -> Result<Vec<Activity>> {
        let limit = limit.unwrap_or(10);
        let state = ctx.data::<AppState>()?;
        
        // Get recent documents (ordered by creation time) to simulate activities
        let filters = crate::models::DocumentSearchFilters {
            title: None,
            document_type_id: None,
            created_by: None,
            created_date_from: None,
            created_date_to: None,
            limit: limit as i64,
            offset: 0,
        };
        
        let (documents, _total) = state.document_handlers
            .search_documents(filters)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get recent documents: {e}")))?;
        
        let mut activities = Vec::new();
        
        for (index, doc) in documents.iter().enumerate() {
            // Create activity for document creation
            activities.push(Activity {
                id: (index + 1).to_string(),
                activity_type: "create".to_string(),
                message: format!("文書「{}」を作成しました", doc.title),
                user: match doc.created_by {
                    1 => "システム管理者",
                    2 => "テストユーザー", 
                    _ => "不明なユーザー",
                }.to_string(),
                timestamp: doc.created_at.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                document_id: Some(doc.number.clone()),
                document_title: Some(doc.title.clone()),
            });
        }
        
        // If no documents exist, show a placeholder activity
        if activities.is_empty() {
            activities.push(Activity {
                id: "1".to_string(),
                activity_type: "system".to_string(),
                message: "システムが正常に起動しました".to_string(),
                user: "システム".to_string(),
                timestamp: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                document_id: None,
                document_title: None,
            });
        }
        
        Ok(activities)
    }

    /// Get pending approvals
    async fn pending_approvals(
        &self,
        _ctx: &Context<'_>,
        _limit: Option<i32>,
    ) -> Result<Vec<PendingApproval>> {
        // Return empty list since approval system is not yet implemented
        // TODO: Implement real approval system with database storage
        Ok(vec![])
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
