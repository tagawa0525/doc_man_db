use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use crate::models::circulation::*;
use crate::error::AppError;

#[async_trait]
pub trait CirculationRepository: Send + Sync {
    async fn get_workflow(&self, id: i32) -> Result<Option<CirculationWorkflow>, AppError>;
    async fn list_workflows(&self) -> Result<Vec<CirculationWorkflow>, AppError>;
    async fn create_circulation(&self, circulation: NewDocumentCirculation) -> Result<DocumentCirculation, AppError>;
    async fn get_circulation(&self, id: i32) -> Result<Option<DocumentCirculation>, AppError>;
    async fn get_circulation_with_details(&self, id: i32) -> Result<Option<CirculationWithDetails>, AppError>;
    async fn update_circulation_status(&self, id: i32, status: CirculationStatus) -> Result<(), AppError>;
    async fn create_step(&self, step: NewCirculationStep) -> Result<CirculationStep, AppError>;
    async fn get_step(&self, id: i32) -> Result<Option<CirculationStep>, AppError>;
    async fn complete_step(&self, step_id: i32, action: StepAction, comments: Option<String>) -> Result<CirculationStep, AppError>;
    async fn get_pending_steps_for_user(&self, user_id: i32) -> Result<Vec<CirculationStep>, AppError>;
    async fn get_circulation_steps(&self, circulation_id: i32) -> Result<Vec<CirculationStep>, AppError>;
    async fn get_document_circulations(&self, document_id: i32) -> Result<Vec<DocumentCirculation>, AppError>;
    async fn advance_circulation(&self, circulation_id: i32) -> Result<(), AppError>;
}

pub struct SqliteCirculationRepository {
    pool: Pool<Sqlite>,
}

impl SqliteCirculationRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CirculationRepository for SqliteCirculationRepository {
    async fn get_workflow(&self, id: i32) -> Result<Option<CirculationWorkflow>, AppError> {
        let workflow = sqlx::query_as!(
            CirculationWorkflow,
            r#"
            SELECT id, name, description, steps, is_active, created_by, created_at
            FROM circulation_workflows 
            WHERE id = ? AND is_active = 1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(workflow)
    }

    async fn list_workflows(&self) -> Result<Vec<CirculationWorkflow>, AppError> {
        let workflows = sqlx::query_as!(
            CirculationWorkflow,
            r#"
            SELECT id, name, description, steps, is_active, created_by, created_at
            FROM circulation_workflows 
            WHERE is_active = 1
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(workflows)
    }

    async fn create_circulation(&self, circulation: NewDocumentCirculation) -> Result<DocumentCirculation, AppError> {
        let result = sqlx::query!(
            r#"
            INSERT INTO document_circulations (document_id, workflow_id, initiated_by, notes)
            VALUES (?, ?, ?, ?)
            "#,
            circulation.document_id,
            circulation.workflow_id,
            circulation.initiated_by,
            circulation.notes
        )
        .execute(&self.pool)
        .await?;

        let circulation_id = result.last_insert_rowid() as i32;

        let created_circulation = sqlx::query_as!(
            DocumentCirculation,
            r#"
            SELECT id, document_id, workflow_id, initiated_by, current_step, 
                   status, started_at, completed_at, notes
            FROM document_circulations 
            WHERE id = ?
            "#,
            circulation_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(DocumentCirculation {
            id: created_circulation.id,
            document_id: created_circulation.document_id,
            workflow_id: created_circulation.workflow_id,
            initiated_by: created_circulation.initiated_by,
            current_step: created_circulation.current_step,
            status: CirculationStatus::from(created_circulation.status),
            started_at: created_circulation.started_at,
            completed_at: created_circulation.completed_at,
            notes: created_circulation.notes,
        })
    }

    async fn get_circulation(&self, id: i32) -> Result<Option<DocumentCirculation>, AppError> {
        let circulation = sqlx::query_as!(
            DocumentCirculation,
            r#"
            SELECT id, document_id, workflow_id, initiated_by, current_step, 
                   status, started_at, completed_at, notes
            FROM document_circulations 
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(circulation.map(|c| DocumentCirculation {
            id: c.id,
            document_id: c.document_id,
            workflow_id: c.workflow_id,
            initiated_by: c.initiated_by,
            current_step: c.current_step,
            status: CirculationStatus::from(c.status),
            started_at: c.started_at,
            completed_at: c.completed_at,
            notes: c.notes,
        }))
    }

    async fn get_circulation_with_details(&self, id: i32) -> Result<Option<CirculationWithDetails>, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT 
                dc.id, dc.document_id, dc.workflow_id, dc.initiated_by, dc.current_step,
                dc.status, dc.started_at, dc.completed_at, dc.notes,
                cw.name as workflow_name, cw.description as workflow_description, cw.steps as workflow_steps,
                cw.is_active as workflow_is_active, cw.created_by as workflow_created_by, cw.created_at as workflow_created_at,
                d.title as document_title,
                e.name as initiated_by_name
            FROM document_circulations dc
            JOIN circulation_workflows cw ON dc.workflow_id = cw.id
            JOIN documents d ON dc.document_id = d.id
            JOIN employees e ON dc.initiated_by = e.id
            WHERE dc.id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let circulation = DocumentCirculation {
                id: row.id,
                document_id: row.document_id,
                workflow_id: row.workflow_id,
                initiated_by: row.initiated_by,
                current_step: row.current_step,
                status: CirculationStatus::from(row.status),
                started_at: row.started_at,
                completed_at: row.completed_at,
                notes: row.notes,
            };

            let workflow = CirculationWorkflow {
                id: row.workflow_id,
                name: row.workflow_name,
                description: row.workflow_description,
                steps: row.workflow_steps,
                is_active: row.workflow_is_active,
                created_by: row.workflow_created_by,
                created_at: row.workflow_created_at,
            };

            let steps = self.get_circulation_steps(id).await?;

            Ok(Some(CirculationWithDetails {
                circulation,
                workflow,
                steps,
                document_title: row.document_title,
                initiated_by_name: row.initiated_by_name,
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_circulation_status(&self, id: i32, status: CirculationStatus) -> Result<(), AppError> {
        let status_str = String::from(status.clone());
        let completed_at = if status == CirculationStatus::Completed {
            Some(chrono::Utc::now().naive_utc())
        } else {
            None
        };

        sqlx::query!(
            r#"
            UPDATE document_circulations 
            SET status = ?, completed_at = ?
            WHERE id = ?
            "#,
            status_str,
            completed_at,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn create_step(&self, step: NewCirculationStep) -> Result<CirculationStep, AppError> {
        let action_str = String::from(step.action_required.clone());
        
        let result = sqlx::query!(
            r#"
            INSERT INTO circulation_steps (circulation_id, step_number, assignee_id, action_required)
            VALUES (?, ?, ?, ?)
            "#,
            step.circulation_id,
            step.step_number,
            step.assignee_id,
            action_str
        )
        .execute(&self.pool)
        .await?;

        let step_id = result.last_insert_rowid() as i32;

        let created_step = sqlx::query_as!(
            CirculationStep,
            r#"
            SELECT id, circulation_id, step_number, assignee_id, action_required,
                   status, assigned_at, completed_at, comments
            FROM circulation_steps 
            WHERE id = ?
            "#,
            step_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CirculationStep {
            id: created_step.id,
            circulation_id: created_step.circulation_id,
            step_number: created_step.step_number,
            assignee_id: created_step.assignee_id,
            action_required: ActionType::from(created_step.action_required),
            status: StepStatus::from(created_step.status),
            assigned_at: created_step.assigned_at,
            completed_at: created_step.completed_at,
            comments: created_step.comments,
        })
    }

    async fn get_step(&self, id: i32) -> Result<Option<CirculationStep>, AppError> {
        let step = sqlx::query_as!(
            CirculationStep,
            r#"
            SELECT id, circulation_id, step_number, assignee_id, action_required,
                   status, assigned_at, completed_at, comments
            FROM circulation_steps 
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(step.map(|s| CirculationStep {
            id: s.id,
            circulation_id: s.circulation_id,
            step_number: s.step_number,
            assignee_id: s.assignee_id,
            action_required: ActionType::from(s.action_required),
            status: StepStatus::from(s.status),
            assigned_at: s.assigned_at,
            completed_at: s.completed_at,
            comments: s.comments,
        }))
    }

    async fn complete_step(&self, step_id: i32, action: StepAction, comments: Option<String>) -> Result<CirculationStep, AppError> {
        let status = match action {
            StepAction::Approve | StepAction::Reject => StepStatus::Completed,
            StepAction::RequestChanges => StepStatus::Completed,
        };

        let status_str = String::from(status);
        let completed_at = chrono::Utc::now().naive_utc();

        sqlx::query!(
            r#"
            UPDATE circulation_steps 
            SET status = ?, completed_at = ?, comments = ?
            WHERE id = ?
            "#,
            status_str,
            completed_at,
            comments,
            step_id
        )
        .execute(&self.pool)
        .await?;

        self.get_step(step_id).await?.ok_or_else(|| AppError::NotFound("Step not found".to_string()))
    }

    async fn get_pending_steps_for_user(&self, user_id: i32) -> Result<Vec<CirculationStep>, AppError> {
        let steps = sqlx::query_as!(
            CirculationStep,
            r#"
            SELECT cs.id, cs.circulation_id, cs.step_number, cs.assignee_id, cs.action_required,
                   cs.status, cs.assigned_at, cs.completed_at, cs.comments
            FROM circulation_steps cs
            JOIN document_circulations dc ON cs.circulation_id = dc.id
            WHERE cs.assignee_id = ? 
              AND cs.status = 'pending'
              AND dc.status = 'active'
            ORDER BY cs.assigned_at
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(steps.into_iter().map(|s| CirculationStep {
            id: s.id,
            circulation_id: s.circulation_id,
            step_number: s.step_number,
            assignee_id: s.assignee_id,
            action_required: ActionType::from(s.action_required),
            status: StepStatus::from(s.status),
            assigned_at: s.assigned_at,
            completed_at: s.completed_at,
            comments: s.comments,
        }).collect())
    }

    async fn get_circulation_steps(&self, circulation_id: i32) -> Result<Vec<CirculationStep>, AppError> {
        let steps = sqlx::query_as!(
            CirculationStep,
            r#"
            SELECT id, circulation_id, step_number, assignee_id, action_required,
                   status, assigned_at, completed_at, comments
            FROM circulation_steps 
            WHERE circulation_id = ?
            ORDER BY step_number
            "#,
            circulation_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(steps.into_iter().map(|s| CirculationStep {
            id: s.id,
            circulation_id: s.circulation_id,
            step_number: s.step_number,
            assignee_id: s.assignee_id,
            action_required: ActionType::from(s.action_required),
            status: StepStatus::from(s.status),
            assigned_at: s.assigned_at,
            completed_at: s.completed_at,
            comments: s.comments,
        }).collect())
    }

    async fn get_document_circulations(&self, document_id: i32) -> Result<Vec<DocumentCirculation>, AppError> {
        let circulations = sqlx::query_as!(
            DocumentCirculation,
            r#"
            SELECT id, document_id, workflow_id, initiated_by, current_step, 
                   status, started_at, completed_at, notes
            FROM document_circulations 
            WHERE document_id = ?
            ORDER BY started_at DESC
            "#,
            document_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(circulations.into_iter().map(|c| DocumentCirculation {
            id: c.id,
            document_id: c.document_id,
            workflow_id: c.workflow_id,
            initiated_by: c.initiated_by,
            current_step: c.current_step,
            status: CirculationStatus::from(c.status),
            started_at: c.started_at,
            completed_at: c.completed_at,
            notes: c.notes,
        }).collect())
    }

    async fn advance_circulation(&self, circulation_id: i32) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE document_circulations 
            SET current_step = current_step + 1
            WHERE id = ?
            "#,
            circulation_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}