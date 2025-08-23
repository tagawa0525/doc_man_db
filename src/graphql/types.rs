use async_graphql::{Enum, InputObject, SimpleObject};

/// GraphQL Document type
#[derive(SimpleObject)]
pub struct Document {
    pub id: i32,
    pub number: String,
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
            number: doc.number,
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

// ========== Circulation Types ==========

/// GraphQL CirculationWorkflow type
#[derive(SimpleObject)]
pub struct CirculationWorkflow {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub steps: Vec<WorkflowStep>,
    pub is_active: bool,
    pub created_by: i32,
    pub created_at: String,
}

impl From<crate::models::CirculationWorkflow> for CirculationWorkflow {
    fn from(workflow: crate::models::CirculationWorkflow) -> Self {
        let steps: Vec<crate::models::WorkflowStep> =
            serde_json::from_str(&workflow.steps).unwrap_or_default();

        Self {
            id: workflow.id,
            name: workflow.name,
            description: workflow.description,
            steps: steps.into_iter().map(|s| s.into()).collect(),
            is_active: workflow.is_active,
            created_by: workflow.created_by,
            created_at: workflow.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        }
    }
}

/// GraphQL WorkflowStep type
#[derive(SimpleObject)]
pub struct WorkflowStep {
    pub step_number: i32,
    pub assignee_role: String,
    pub action_required: ActionType,
    pub is_optional: bool,
    pub timeout_hours: Option<i32>,
}

impl From<crate::models::WorkflowStep> for WorkflowStep {
    fn from(step: crate::models::WorkflowStep) -> Self {
        Self {
            step_number: step.step_number,
            assignee_role: step.assignee_role,
            action_required: step.action_required.into(),
            is_optional: step.is_optional,
            timeout_hours: step.timeout_hours,
        }
    }
}

/// GraphQL DocumentCirculation type
#[derive(SimpleObject)]
pub struct DocumentCirculation {
    pub id: i32,
    pub document_id: i32,
    pub workflow_id: i32,
    pub initiated_by: i32,
    pub current_step: i32,
    pub status: CirculationStatus,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub notes: Option<String>,
}

impl From<crate::models::DocumentCirculation> for DocumentCirculation {
    fn from(circulation: crate::models::DocumentCirculation) -> Self {
        Self {
            id: circulation.id,
            document_id: circulation.document_id,
            workflow_id: circulation.workflow_id,
            initiated_by: circulation.initiated_by,
            current_step: circulation.current_step,
            status: circulation.status.into(),
            started_at: circulation
                .started_at
                .format("%Y-%m-%dT%H:%M:%S")
                .to_string(),
            completed_at: circulation
                .completed_at
                .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S").to_string()),
            notes: circulation.notes,
        }
    }
}

/// GraphQL CirculationStep type
#[derive(SimpleObject)]
pub struct CirculationStep {
    pub id: i32,
    pub circulation_id: i32,
    pub step_number: i32,
    pub assignee_id: i32,
    pub action_required: ActionType,
    pub status: StepStatus,
    pub assigned_at: String,
    pub completed_at: Option<String>,
    pub comments: Option<String>,
}

impl From<crate::models::CirculationStep> for CirculationStep {
    fn from(step: crate::models::CirculationStep) -> Self {
        Self {
            id: step.id,
            circulation_id: step.circulation_id,
            step_number: step.step_number,
            assignee_id: step.assignee_id,
            action_required: step.action_required.into(),
            status: step.status.into(),
            assigned_at: step.assigned_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            completed_at: step
                .completed_at
                .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S").to_string()),
            comments: step.comments,
        }
    }
}

// Enums
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum CirculationStatus {
    Active,
    Completed,
    Cancelled,
}

impl From<crate::models::CirculationStatus> for CirculationStatus {
    fn from(status: crate::models::CirculationStatus) -> Self {
        match status {
            crate::models::CirculationStatus::Active => Self::Active,
            crate::models::CirculationStatus::Completed => Self::Completed,
            crate::models::CirculationStatus::Cancelled => Self::Cancelled,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum StepStatus {
    Pending,
    Completed,
    Skipped,
}

impl From<crate::models::StepStatus> for StepStatus {
    fn from(status: crate::models::StepStatus) -> Self {
        match status {
            crate::models::StepStatus::Pending => Self::Pending,
            crate::models::StepStatus::Completed => Self::Completed,
            crate::models::StepStatus::Skipped => Self::Skipped,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ActionType {
    Review,
    Approve,
    Acknowledge,
}

impl From<crate::models::ActionType> for ActionType {
    fn from(action: crate::models::ActionType) -> Self {
        match action {
            crate::models::ActionType::Review => Self::Review,
            crate::models::ActionType::Approve => Self::Approve,
            crate::models::ActionType::Acknowledge => Self::Acknowledge,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum StepAction {
    Approve,
    Reject,
    RequestChanges,
}

impl From<StepAction> for crate::models::StepAction {
    fn from(action: StepAction) -> Self {
        match action {
            StepAction::Approve => Self::Approve,
            StepAction::Reject => Self::Reject,
            StepAction::RequestChanges => Self::RequestChanges,
        }
    }
}

// Input Types
#[derive(InputObject)]
pub struct CreateCirculationInput {
    pub document_id: i32,
    pub workflow_id: i32,
    pub notes: Option<String>,
}

impl From<CreateCirculationInput> for crate::models::CreateCirculationInput {
    fn from(input: CreateCirculationInput) -> Self {
        Self {
            document_id: input.document_id,
            workflow_id: input.workflow_id,
            notes: input.notes,
        }
    }
}

#[derive(InputObject)]
pub struct CompleteStepInput {
    pub circulation_id: i32,
    pub step_id: i32,
    pub action: StepAction,
    pub comments: Option<String>,
}

impl From<CompleteStepInput> for crate::models::CompleteStepInput {
    fn from(input: CompleteStepInput) -> Self {
        Self {
            circulation_id: input.circulation_id,
            step_id: input.step_id,
            action: input.action.into(),
            comments: input.comments,
        }
    }
}

// Response Types
#[derive(SimpleObject)]
pub struct CirculationResponse {
    pub success: bool,
    pub circulation: Option<DocumentCirculation>,
    pub message: String,
}

impl From<crate::models::CirculationResponse> for CirculationResponse {
    fn from(response: crate::models::CirculationResponse) -> Self {
        Self {
            success: response.success,
            circulation: response.circulation.map(|c| c.into()),
            message: response.message,
        }
    }
}

#[derive(SimpleObject)]
pub struct StepResponse {
    pub success: bool,
    pub step: Option<CirculationStep>,
    pub message: String,
}

impl From<crate::models::StepResponse> for StepResponse {
    fn from(response: crate::models::StepResponse) -> Self {
        Self {
            success: response.success,
            step: response.step.map(|s| s.into()),
            message: response.message,
        }
    }
}

// ========== Dashboard Types ==========

/// GraphQL DashboardStats type
#[derive(SimpleObject)]
pub struct DashboardStats {
    pub total_documents: i32,
    pub monthly_created: i32,
    pub missing_files: i32,
    pub active_users: i32,
    pub pending_approvals: i32,
    pub system_uptime: f64,
}

/// GraphQL SystemStatus type
#[derive(SimpleObject)]
pub struct SystemStatus {
    pub api_status: String,
    pub database_status: String,
    pub file_system_status: String,
    pub last_backup: String,
    pub server_uptime: String,
    pub memory_usage: f64,
    pub disk_usage: f64,
}

/// GraphQL Activity type
#[derive(SimpleObject)]
pub struct Activity {
    pub id: String,
    #[graphql(name = "type")]
    pub activity_type: String,
    pub message: String,
    pub user: String,
    pub timestamp: String,
    pub document_id: Option<String>,
    pub document_title: Option<String>,
}

/// GraphQL PendingApproval type
#[derive(SimpleObject)]
pub struct PendingApproval {
    pub id: String,
    pub document_id: String,
    pub document_title: String,
    pub requester_name: String,
    pub requested_at: String,
    pub approval_type: String,
}
