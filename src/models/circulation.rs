use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CirculationWorkflow {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub steps: String, // JSON format
    pub is_active: bool,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub step_number: i32,
    pub assignee_role: String,
    pub action_required: ActionType,
    pub is_optional: bool,
    pub timeout_hours: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DocumentCirculation {
    pub id: i32,
    pub document_id: i32,
    pub workflow_id: i32,
    pub initiated_by: i32,
    pub current_step: i32,
    pub status: CirculationStatus,
    pub started_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CirculationStep {
    pub id: i32,
    pub circulation_id: i32,
    pub step_number: i32,
    pub assignee_id: i32,
    pub action_required: ActionType,
    pub status: StepStatus,
    pub assigned_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
    pub comments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CirculationStatus {
    Active,
    Completed,
    Cancelled,
}

impl From<String> for CirculationStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "active" => Self::Active,
            "completed" => Self::Completed,
            "cancelled" => Self::Cancelled,
            _ => Self::Active,
        }
    }
}

impl From<CirculationStatus> for String {
    fn from(status: CirculationStatus) -> Self {
        match status {
            CirculationStatus::Active => "active".to_string(),
            CirculationStatus::Completed => "completed".to_string(),
            CirculationStatus::Cancelled => "cancelled".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepStatus {
    Pending,
    Completed,
    Skipped,
}

impl From<String> for StepStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "pending" => Self::Pending,
            "completed" => Self::Completed,
            "skipped" => Self::Skipped,
            _ => Self::Pending,
        }
    }
}

impl From<StepStatus> for String {
    fn from(status: StepStatus) -> Self {
        match status {
            StepStatus::Pending => "pending".to_string(),
            StepStatus::Completed => "completed".to_string(),
            StepStatus::Skipped => "skipped".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionType {
    Review,
    Approve,
    Acknowledge,
}

impl From<String> for ActionType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "review" => Self::Review,
            "approve" => Self::Approve,
            "acknowledge" => Self::Acknowledge,
            _ => Self::Review,
        }
    }
}

impl From<ActionType> for String {
    fn from(action: ActionType) -> Self {
        match action {
            ActionType::Review => "review".to_string(),
            ActionType::Approve => "approve".to_string(),
            ActionType::Acknowledge => "acknowledge".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepAction {
    Approve,
    Reject,
    RequestChanges,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCirculationInput {
    pub document_id: i32,
    pub workflow_id: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteStepInput {
    pub circulation_id: i32,
    pub step_id: i32,
    pub action: StepAction,
    pub comments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewDocumentCirculation {
    pub document_id: i32,
    pub workflow_id: i32,
    pub initiated_by: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCirculationStep {
    pub circulation_id: i32,
    pub step_number: i32,
    pub assignee_id: i32,
    pub action_required: ActionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CirculationWithDetails {
    pub circulation: DocumentCirculation,
    pub workflow: CirculationWorkflow,
    pub steps: Vec<CirculationStep>,
    pub document_title: String,
    pub initiated_by_name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CirculationError {
    #[error("Unauthorized: User does not have permission to perform this action")]
    Unauthorized,
    #[error("Workflow not found")]
    WorkflowNotFound,
    #[error("Circulation not found")]
    CirculationNotFound,
    #[error("Step not found")]
    StepNotFound,
    #[error("Invalid step status")]
    InvalidStepStatus,
    #[error("Document not found")]
    DocumentNotFound,
    #[error("Database error: {0}")]
    Database(#[from] crate::error::AppError),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Notification error: {0}")]
    Notification(String),
}

pub type CirculationResult<T> = Result<T, CirculationError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissions {
    pub user_id: i32,
    pub is_admin: bool,
    pub department_id: Option<i32>,
    pub business_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CirculationResponse {
    pub success: bool,
    pub circulation: Option<DocumentCirculation>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResponse {
    pub success: bool,
    pub step: Option<CirculationStep>,
    pub message: String,
}
