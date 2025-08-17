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
}
