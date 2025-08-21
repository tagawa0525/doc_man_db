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

    /// Get circulation workflows
    async fn circulation_workflows(&self, ctx: &Context<'_>) -> Result<Vec<CirculationWorkflow>> {
        let state = ctx.data::<AppState>()?;

        match state.circulation_service.get_workflows().await {
            Ok(workflows) => {
                let graphql_workflows: Vec<CirculationWorkflow> =
                    workflows.into_iter().map(|w| w.into()).collect();
                Ok(graphql_workflows)
            }
            Err(e) => Err(async_graphql::Error::new(format!("Workflow error: {e}"))),
        }
    }

    /// Get pending circulations for current user
    async fn my_pending_circulations(&self, ctx: &Context<'_>) -> Result<Vec<CirculationStep>> {
        let state = ctx.data::<AppState>()?;
        // In a real implementation, you would get the user ID from the authentication context
        let user_id = 1; // Placeholder

        match state.circulation_service.get_pending_circulations_for_user(user_id).await {
            Ok(steps) => {
                let graphql_steps: Vec<CirculationStep> =
                    steps.into_iter().map(|s| s.into()).collect();
                Ok(graphql_steps)
            }
            Err(e) => Err(async_graphql::Error::new(format!("Circulation error: {e}"))),
        }
    }

    /// Get circulations for a document
    async fn document_circulations(&self, ctx: &Context<'_>, document_id: i32) -> Result<Vec<DocumentCirculation>> {
        let state = ctx.data::<AppState>()?;

        match state.circulation_service.get_document_circulations(document_id).await {
            Ok(circulations) => {
                let graphql_circulations: Vec<DocumentCirculation> =
                    circulations.into_iter().map(|c| c.into()).collect();
                Ok(graphql_circulations)
            }
            Err(e) => Err(async_graphql::Error::new(format!("Circulation error: {e}"))),
        }
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

    /// Create a new circulation
    async fn create_circulation(
        &self,
        ctx: &Context<'_>,
        input: CreateCirculationInput,
    ) -> Result<CirculationResponse> {
        let state = ctx.data::<AppState>()?;
        let circulation_input = input.into();
        
        // In a real implementation, you would get user permissions from the authentication context
        let user_permissions = crate::models::UserPermissions {
            user_id: 1,
            is_admin: false,
            department_id: Some(1),
            business_id: Some(1),
        };

        match state.circulation_service.create_circulation(circulation_input, &user_permissions).await {
            Ok(circulation) => Ok(CirculationResponse {
                success: true,
                circulation: Some(circulation.into()),
                message: "回覧が正常に開始されました".to_string(),
            }),
            Err(e) => Ok(CirculationResponse {
                success: false,
                circulation: None,
                message: format!("回覧の開始に失敗しました: {e}"),
            }),
        }
    }

    /// Complete a circulation step
    async fn complete_circulation_step(
        &self,
        ctx: &Context<'_>,
        input: CompleteStepInput,
    ) -> Result<StepResponse> {
        let state = ctx.data::<AppState>()?;
        let step_input = input.into();
        
        // In a real implementation, you would get user permissions from the authentication context
        let user_permissions = crate::models::UserPermissions {
            user_id: 1,
            is_admin: false,
            department_id: Some(1),
            business_id: Some(1),
        };

        match state.circulation_service.complete_step(step_input, &user_permissions).await {
            Ok(step) => Ok(StepResponse {
                success: true,
                step: Some(step.into()),
                message: "ステップが正常に完了しました".to_string(),
            }),
            Err(e) => Ok(StepResponse {
                success: false,
                step: None,
                message: format!("ステップの完了に失敗しました: {e}"),
            }),
        }
    }

    /// Cancel a circulation
    async fn cancel_circulation(
        &self,
        ctx: &Context<'_>,
        id: i32,
        reason: Option<String>,
    ) -> Result<CirculationResponse> {
        let state = ctx.data::<AppState>()?;
        
        // In a real implementation, you would get user permissions from the authentication context
        let user_permissions = crate::models::UserPermissions {
            user_id: 1,
            is_admin: false,
            department_id: Some(1),
            business_id: Some(1),
        };

        match state.circulation_service.cancel_circulation(id, reason, &user_permissions).await {
            Ok(_) => Ok(CirculationResponse {
                success: true,
                circulation: None,
                message: "回覧が正常にキャンセルされました".to_string(),
            }),
            Err(e) => Ok(CirculationResponse {
                success: false,
                circulation: None,
                message: format!("回覧のキャンセルに失敗しました: {e}"),
            }),
        }
    }
}
