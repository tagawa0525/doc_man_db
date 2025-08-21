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
    async fn circulation_workflows(&self, ctx: &Context<'_>) -> Result<Vec<CirculationWorkflow>> {
        // TODO: Implement when AppState includes circulation_service
        Ok(vec![])
    }

    /// Get pending circulations for current user (placeholder implementation)
    async fn my_pending_circulations(&self, ctx: &Context<'_>) -> Result<Vec<CirculationStep>> {
        // TODO: Implement when AppState includes circulation_service
        Ok(vec![])
    }

    /// Get circulations for a document (placeholder implementation)
    async fn document_circulations(
        &self,
        ctx: &Context<'_>,
        document_id: i32,
    ) -> Result<Vec<DocumentCirculation>> {
        // TODO: Implement when AppState includes circulation_service
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
        ctx: &Context<'_>,
        input: CreateCirculationInput,
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
        ctx: &Context<'_>,
        input: CompleteStepInput,
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
        ctx: &Context<'_>,
        id: i32,
        reason: Option<String>,
    ) -> Result<CirculationResponse> {
        // TODO: Implement when AppState includes circulation_service
        Ok(CirculationResponse {
            success: false,
            circulation: None,
            message: "Not implemented yet".to_string(),
        })
    }
}
