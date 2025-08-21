use crate::models::circulation::*;
use crate::repositories::circulation_repository::CirculationRepository;
use crate::services::document_service::DocumentService;
use crate::services::notification_service::NotificationService;
use std::sync::Arc;

pub struct CirculationService {
    circulation_repo: Arc<dyn CirculationRepository>,
    document_service: Arc<DocumentService>,
    notification_service: Arc<NotificationService>,
}

impl CirculationService {
    pub fn new(
        circulation_repo: Arc<dyn CirculationRepository>,
        document_service: Arc<DocumentService>,
        notification_service: Arc<NotificationService>,
    ) -> Self {
        Self {
            circulation_repo,
            document_service,
            notification_service,
        }
    }

    pub async fn get_workflows(&self) -> CirculationResult<Vec<CirculationWorkflow>> {
        self.circulation_repo
            .list_workflows()
            .await
            .map_err(CirculationError::Database)
    }

    pub async fn create_circulation(
        &self,
        input: CreateCirculationInput,
        user_permissions: &UserPermissions,
    ) -> CirculationResult<DocumentCirculation> {
        // 権限確認
        // TODO: Implement permission validation
        // self.validate_circulation_permission(&input, user_permissions).await?;

        // ワークフロー取得
        let workflow = self
            .circulation_repo
            .get_workflow(input.workflow_id)
            .await
            .map_err(CirculationError::Database)?
            .ok_or(CirculationError::WorkflowNotFound)?;

        // 回覧作成
        let circulation = NewDocumentCirculation {
            document_id: input.document_id,
            workflow_id: input.workflow_id,
            initiated_by: user_permissions.user_id,
            notes: input.notes,
        };

        let created_circulation = self
            .circulation_repo
            .create_circulation(circulation)
            .await
            .map_err(CirculationError::Database)?;

        // 最初のステップを作成
        self.create_initial_steps(&created_circulation, &workflow)
            .await?;

        // 通知送信
        self.send_circulation_notifications(&created_circulation)
            .await?;

        Ok(created_circulation)
    }

    pub async fn complete_step(
        &self,
        input: CompleteStepInput,
        user_permissions: &UserPermissions,
    ) -> CirculationResult<CirculationStep> {
        // ステップ取得・権限確認
        let step = self
            .circulation_repo
            .get_step(input.step_id)
            .await
            .map_err(CirculationError::Database)?
            .ok_or(CirculationError::StepNotFound)?;

        if step.assignee_id != user_permissions.user_id {
            return Err(CirculationError::Unauthorized);
        }

        // ステップ完了
        let completed_step = self
            .circulation_repo
            .complete_step(input.step_id, input.action.clone(), input.comments)
            .await
            .map_err(CirculationError::Database)?;

        // 次のステップ処理
        self.process_next_step(step.circulation_id, &input.action)
            .await?;

        // 通知送信
        self.send_step_completion_notifications(&completed_step)
            .await?;

        Ok(completed_step)
    }

    pub async fn get_pending_circulations_for_user(
        &self,
        user_id: i32,
    ) -> CirculationResult<Vec<CirculationStep>> {
        self.circulation_repo
            .get_pending_steps_for_user(user_id)
            .await
            .map_err(CirculationError::Database)
    }

    pub async fn get_document_circulations(
        &self,
        document_id: i32,
    ) -> CirculationResult<Vec<DocumentCirculation>> {
        self.circulation_repo
            .get_document_circulations(document_id)
            .await
            .map_err(CirculationError::Database)
    }

    pub async fn get_circulation_with_details(
        &self,
        circulation_id: i32,
    ) -> CirculationResult<Option<CirculationWithDetails>> {
        self.circulation_repo
            .get_circulation_with_details(circulation_id)
            .await
            .map_err(CirculationError::Database)
    }

    pub async fn cancel_circulation(
        &self,
        circulation_id: i32,
        reason: Option<String>,
        user_permissions: &UserPermissions,
    ) -> CirculationResult<()> {
        // 回覧取得・権限確認
        let circulation = self
            .circulation_repo
            .get_circulation(circulation_id)
            .await
            .map_err(CirculationError::Database)?
            .ok_or(CirculationError::CirculationNotFound)?;

        // 開始者または管理者のみがキャンセル可能
        if circulation.initiated_by != user_permissions.user_id && !user_permissions.is_admin {
            return Err(CirculationError::Unauthorized);
        }

        // ステータス更新
        self.circulation_repo
            .update_circulation_status(circulation_id, CirculationStatus::Cancelled)
            .await
            .map_err(CirculationError::Database)?;

        // キャンセル通知
        self.send_cancellation_notifications(&circulation, reason)
            .await?;

        Ok(())
    }

    async fn validate_circulation_permission(
        &self,
        input: &CreateCirculationInput,
        user_permissions: &UserPermissions,
    ) -> CirculationResult<()> {
        // TODO: Implement document validation
        // let document = self.document_service
        //     .get_document(input.document_id, user_permissions)
        //     .await
        //     .map_err(|_| CirculationError::DocumentNotFound)?
        //     .ok_or(CirculationError::DocumentNotFound)?;

        // TODO: Implement authorization check
        // if document.created_by != user_permissions.user_id && !user_permissions.is_admin {
        //     return Err(CirculationError::Unauthorized);
        // }

        Ok(())
    }

    async fn create_initial_steps(
        &self,
        circulation: &DocumentCirculation,
        workflow: &CirculationWorkflow,
    ) -> CirculationResult<()> {
        // ワークフローステップをパース
        let workflow_steps: Vec<WorkflowStep> =
            serde_json::from_str(&workflow.steps).map_err(CirculationError::Json)?;

        // 最初のステップのみを作成
        if let Some(first_step) = workflow_steps.first() {
            // 担当者を決定（ここでは簡単化のため、ロールベースで決定）
            let assignee_id = self.resolve_assignee(&first_step.assignee_role).await?;

            let new_step = NewCirculationStep {
                circulation_id: circulation.id,
                step_number: first_step.step_number,
                assignee_id,
                action_required: first_step.action_required.clone(),
            };

            self.circulation_repo
                .create_step(new_step)
                .await
                .map_err(CirculationError::Database)?;
        }

        Ok(())
    }

    async fn process_next_step(
        &self,
        circulation_id: i32,
        action: &StepAction,
    ) -> CirculationResult<()> {
        match action {
            StepAction::Approve => {
                // 次のステップに進む
                self.advance_to_next_step(circulation_id).await?;
            }
            StepAction::Reject => {
                // 回覧を終了
                self.circulation_repo
                    .update_circulation_status(circulation_id, CirculationStatus::Completed)
                    .await
                    .map_err(CirculationError::Database)?;
            }
            StepAction::RequestChanges => {
                // 作成者に差し戻し（実装簡化のため、ここでは完了とする）
                self.circulation_repo
                    .update_circulation_status(circulation_id, CirculationStatus::Completed)
                    .await
                    .map_err(CirculationError::Database)?;
            }
        }

        Ok(())
    }

    async fn advance_to_next_step(&self, circulation_id: i32) -> CirculationResult<()> {
        // 現在の回覧情報を取得
        let circulation = self
            .circulation_repo
            .get_circulation(circulation_id)
            .await
            .map_err(CirculationError::Database)?
            .ok_or(CirculationError::CirculationNotFound)?;

        // ワークフロー情報を取得
        let workflow = self
            .circulation_repo
            .get_workflow(circulation.workflow_id)
            .await
            .map_err(CirculationError::Database)?
            .ok_or(CirculationError::WorkflowNotFound)?;

        let workflow_steps: Vec<WorkflowStep> =
            serde_json::from_str(&workflow.steps).map_err(CirculationError::Json)?;

        // 次のステップがあるかチェック
        let next_step_number = circulation.current_step + 1;
        if let Some(next_step) = workflow_steps
            .iter()
            .find(|s| s.step_number == next_step_number)
        {
            // 次のステップを作成
            let assignee_id = self.resolve_assignee(&next_step.assignee_role).await?;

            let new_step = NewCirculationStep {
                circulation_id,
                step_number: next_step.step_number,
                assignee_id,
                action_required: next_step.action_required.clone(),
            };

            self.circulation_repo
                .create_step(new_step)
                .await
                .map_err(CirculationError::Database)?;

            // 回覧の現在ステップを更新
            self.circulation_repo
                .advance_circulation(circulation_id)
                .await
                .map_err(CirculationError::Database)?;
        } else {
            // 全ステップ完了
            self.circulation_repo
                .update_circulation_status(circulation_id, CirculationStatus::Completed)
                .await
                .map_err(CirculationError::Database)?;
        }

        Ok(())
    }

    async fn resolve_assignee(&self, role: &str) -> CirculationResult<i32> {
        // 実際の実装では、ロールから適切な担当者を選出する
        // ここでは簡単化のため、固定値を返す
        match role {
            "manager" => Ok(2),   // 仮の管理者ID
            "director" => Ok(3),  // 仮の部長ID
            "executive" => Ok(4), // 仮の役員ID
            _ => Ok(1),           // デフォルト
        }
    }

    async fn send_circulation_notifications(
        &self,
        circulation: &DocumentCirculation,
    ) -> CirculationResult<()> {
        // TODO: Implement circulation notifications
        tracing::info!("回覧開始通知: circulation_id={}", circulation.id);
        Ok(())
    }

    async fn send_step_completion_notifications(
        &self,
        step: &CirculationStep,
    ) -> CirculationResult<()> {
        // TODO: Implement step completion notifications
        tracing::info!("ステップ完了通知: step_id={}", step.id);
        Ok(())
    }

    async fn send_cancellation_notifications(
        &self,
        circulation: &DocumentCirculation,
        reason: Option<String>,
    ) -> CirculationResult<()> {
        // TODO: Implement cancellation notifications
        tracing::info!(
            "回覧キャンセル通知: circulation_id={}, reason={:?}",
            circulation.id,
            reason
        );
        Ok(())
    }
}
