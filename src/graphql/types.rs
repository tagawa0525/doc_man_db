use async_graphql::{InputObject, SimpleObject};

/// GraphQL Document type
#[derive(SimpleObject)]
pub struct Document {
    pub id: i32,
    pub title: String,
    pub document_type_id: i32,
    pub created_by: i32,
    pub created_date: String, // NaiveDate as ISO string
    pub created_at: String,   // NaiveDateTime as ISO string
    pub updated_at: String,   // NaiveDateTime as ISO string
}

impl From<crate::models::Document> for Document {
    fn from(doc: crate::models::Document) -> Self {
        Self {
            id: doc.id,
            title: doc.title,
            document_type_id: doc.document_type_id,
            created_by: doc.created_by,
            created_date: doc.created_date.format("%Y-%m-%d").to_string(),
            created_at: doc.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            updated_at: doc.updated_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        }
    }
}

/// GraphQL CreateDocumentInput type
#[derive(InputObject)]
pub struct CreateDocumentInput {
    pub title: String,
    pub document_type_code: String,
    pub department_code: String,
    pub created_by: i32,
    pub created_date: String, // NaiveDate as string in GraphQL
}

impl From<CreateDocumentInput> for crate::models::CreateDocumentWithNumberRequest {
    fn from(val: CreateDocumentInput) -> Self {
        crate::models::CreateDocumentWithNumberRequest {
            title: val.title,
            document_type_code: val.document_type_code,
            department_code: val.department_code,
            created_by: val.created_by,
            created_date: chrono::NaiveDate::parse_from_str(&val.created_date, "%Y-%m-%d")
                .expect("Invalid date format"),
        }
    }
}

/// GraphQL DocumentSearchFilters type
#[derive(InputObject)]
pub struct DocumentSearchFilters {
    pub title: Option<String>,
    pub document_type_id: Option<i32>,
    pub created_by: Option<i32>,
    pub created_date_from: Option<String>,
    pub created_date_to: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl From<DocumentSearchFilters> for crate::models::DocumentSearchFilters {
    fn from(val: DocumentSearchFilters) -> Self {
        crate::models::DocumentSearchFilters {
            title: val.title,
            document_type_id: val.document_type_id,
            created_by: val.created_by,
            created_date_from: val
                .created_date_from
                .and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            created_date_to: val
                .created_date_to
                .and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            limit: val.limit.unwrap_or(10),
            offset: val.offset.unwrap_or(0),
        }
    }
}

/// GraphQL GeneratedDocumentNumber type
#[derive(SimpleObject)]
pub struct GeneratedDocumentNumber {
    pub rule_id: i32,
    pub sequence_number: i32,
    pub template_used: String,
}

impl From<crate::models::GeneratedDocumentNumber> for GeneratedDocumentNumber {
    fn from(generated: crate::models::GeneratedDocumentNumber) -> Self {
        Self {
            rule_id: generated.rule_id,
            sequence_number: generated.sequence_number,
            template_used: generated.template_used,
        }
    }
}

/// GraphQL CreatedDocumentWithNumber type
#[derive(SimpleObject)]
pub struct CreatedDocumentWithNumber {
    pub document: Document,
    pub document_number: String,
    pub generated_number: GeneratedDocumentNumber,
}

impl From<crate::models::CreatedDocumentWithNumber> for CreatedDocumentWithNumber {
    fn from(created: crate::models::CreatedDocumentWithNumber) -> Self {
        Self {
            document: created.document.into(),
            document_number: created.document_number,
            generated_number: created.generated_number.into(),
        }
    }
}

/// GraphQL SearchDocumentsResult type
#[derive(SimpleObject)]
pub struct SearchDocumentsResult {
    pub documents: Vec<Document>,
    pub total: i64,
}
