use crate::error::BusinessError;
use crate::models::{
    Business, BusinessMember, BusinessSearchFilters, CirculationCandidate, CirculationCandidates,
    CreateBusinessMemberRequest, CreateBusinessRequest, UpdateBusinessMemberRequest,
    UpdateBusinessRequest,
};
use crate::repositories::BusinessRepository;
use std::sync::Arc;

// 権限管理用の構造体
#[derive(Debug, Clone, Default)]
pub struct UserPermissions {
    pub employee_id: i32,
    pub can_create_businesses: bool,
    pub can_manage_business_members: bool,
    pub can_view_all_businesses: bool,
    pub accessible_departments: Vec<i32>,
}

pub struct BusinessService {
    repository: Arc<dyn BusinessRepository>,
}

impl BusinessService {
    pub fn new(repository: Arc<dyn BusinessRepository>) -> Self {
        Self { repository }
    }

    /// 業務を作成する
    pub async fn create_business(
        &self,
        request: CreateBusinessRequest,
        user_permissions: &UserPermissions,
    ) -> Result<Business, BusinessError> {
        // 権限チェック
        if !user_permissions.can_create_businesses {
            return Err(BusinessError::PermissionDenied);
        }

        self.repository.create(request).await
    }

    /// 業務を取得する
    pub async fn get_business(
        &self,
        business_id: i32,
        user_permissions: &UserPermissions,
    ) -> Result<Option<Business>, BusinessError> {
        let business = self.repository.get_by_id(business_id).await?;

        if let Some(ref _business) = business {
            // 権限チェック: 自分が関与している業務または全業務参照権限
            if !user_permissions.can_view_all_businesses
                && !self
                    .is_user_involved(business_id, user_permissions.employee_id)
                    .await?
            {
                return Ok(None);
            }
        }

        Ok(business)
    }

    /// 業務を検索する
    pub async fn search_businesses(
        &self,
        mut filters: BusinessSearchFilters,
        user_permissions: &UserPermissions,
    ) -> Result<(Vec<Business>, i64), BusinessError> {
        // 権限による制限
        if !user_permissions.can_view_all_businesses {
            filters.member_employee_id = Some(user_permissions.employee_id);
        }

        self.repository.search(filters).await
    }

    /// 業務を更新する
    pub async fn update_business(
        &self,
        business_id: i32,
        request: UpdateBusinessRequest,
        user_permissions: &UserPermissions,
    ) -> Result<Business, BusinessError> {
        // 権限チェック
        if !self
            .can_user_edit_business(business_id, user_permissions)
            .await?
        {
            return Err(BusinessError::PermissionDenied);
        }

        self.repository.update(business_id, request).await
    }

    /// 業務を削除する
    pub async fn delete_business(
        &self,
        business_id: i32,
        user_permissions: &UserPermissions,
    ) -> Result<(), BusinessError> {
        // 権限チェック
        if !self
            .can_user_edit_business(business_id, user_permissions)
            .await?
        {
            return Err(BusinessError::PermissionDenied);
        }

        self.repository.delete(business_id).await
    }

    /// 業務従事者を追加する
    pub async fn add_business_member(
        &self,
        request: CreateBusinessMemberRequest,
        user_permissions: &UserPermissions,
    ) -> Result<BusinessMember, BusinessError> {
        // 権限チェック
        if !user_permissions.can_manage_business_members {
            return Err(BusinessError::PermissionDenied);
        }

        // 業務存在チェック
        let _business = self
            .repository
            .get_by_id(request.business_id)
            .await?
            .ok_or(BusinessError::BusinessNotFound)?;

        self.repository.add_member(request).await
    }

    /// 業務従事者を更新する
    pub async fn update_business_member(
        &self,
        member_id: i32,
        request: UpdateBusinessMemberRequest,
        user_permissions: &UserPermissions,
    ) -> Result<BusinessMember, BusinessError> {
        // 権限チェック
        if !user_permissions.can_manage_business_members {
            return Err(BusinessError::PermissionDenied);
        }

        self.repository.update_member(member_id, request).await
    }

    /// 業務従事者を削除する
    pub async fn remove_business_member(
        &self,
        member_id: i32,
        user_permissions: &UserPermissions,
    ) -> Result<(), BusinessError> {
        // 権限チェック
        if !user_permissions.can_manage_business_members {
            return Err(BusinessError::PermissionDenied);
        }

        self.repository.remove_member(member_id).await
    }

    /// 業務の従事者一覧を取得する
    pub async fn get_business_members(
        &self,
        business_id: i32,
        user_permissions: &UserPermissions,
    ) -> Result<Vec<BusinessMember>, BusinessError> {
        // 業務参照権限チェック
        if !user_permissions.can_view_all_businesses
            && !self
                .is_user_involved(business_id, user_permissions.employee_id)
                .await?
        {
            return Err(BusinessError::PermissionDenied);
        }

        self.repository.get_members(business_id).await
    }

    /// 社員の業務従事履歴を取得する
    pub async fn get_employee_business_history(
        &self,
        employee_id: i32,
        user_permissions: &UserPermissions,
    ) -> Result<Vec<BusinessMember>, BusinessError> {
        // 自分の履歴または管理者権限チェック
        if employee_id != user_permissions.employee_id && !user_permissions.can_view_all_businesses
        {
            return Err(BusinessError::PermissionDenied);
        }

        self.repository.get_member_history(employee_id).await
    }

    /// 回覧候補を提案する
    pub async fn suggest_circulation_candidates(
        &self,
        business_id: i32,
        user_permissions: &UserPermissions,
    ) -> Result<CirculationCandidates, BusinessError> {
        // 権限チェック
        if !self
            .can_user_edit_business(business_id, user_permissions)
            .await?
        {
            return Err(BusinessError::PermissionDenied);
        }

        let business = self
            .repository
            .get_by_id(business_id)
            .await?
            .ok_or(BusinessError::BusinessNotFound)?;

        let members = self.repository.get_members(business_id).await?;
        let external_contacts = self.repository.get_external_contacts(business_id).await?;

        // 階層化された候補を作成
        let candidates = CirculationCandidates {
            business_members: members
                .into_iter()
                .filter(|m| m.is_active())
                .map(|m| CirculationCandidate {
                    id: format!("employee_{}", m.employee_id),
                    name: format!("Employee {}", m.employee_id), // 実際の名前取得は別途実装
                    email: None,                                 // 実際のメール取得は別途実装
                    category: "業務従事者".to_string(),
                    priority: match m.role {
                        crate::models::BusinessRole::Leader => 1,
                        crate::models::BusinessRole::Member => 2,
                        crate::models::BusinessRole::Advisor => 3,
                    },
                })
                .collect(),

            external_contacts: external_contacts
                .into_iter()
                .filter(|c| c.is_active)
                .map(|c| CirculationCandidate {
                    id: format!("external_{}", c.id),
                    name: c.name.clone(),
                    email: c.email.clone(),
                    category: "外部連絡先".to_string(),
                    priority: 5,
                })
                .collect(),

            department_members: self
                .get_department_members(&business, user_permissions)
                .await?,
        };

        Ok(candidates)
    }

    /// 業務番号を生成する
    pub async fn generate_business_number(&self) -> Result<String, BusinessError> {
        self.repository.generate_business_number().await
    }

    // プライベートメソッド

    /// ユーザーが業務に関与しているかチェック
    async fn is_user_involved(
        &self,
        business_id: i32,
        employee_id: i32,
    ) -> Result<bool, BusinessError> {
        let members = self.repository.get_members(business_id).await?;
        Ok(members
            .iter()
            .any(|m| m.employee_id == employee_id && m.is_active()))
    }

    /// ユーザーが業務を編集できるかチェック
    async fn can_user_edit_business(
        &self,
        business_id: i32,
        user_permissions: &UserPermissions,
    ) -> Result<bool, BusinessError> {
        if user_permissions.can_manage_business_members {
            return Ok(true);
        }

        // 業務のリーダーかチェック
        let members = self.repository.get_members(business_id).await?;
        Ok(members.iter().any(|m| {
            m.employee_id == user_permissions.employee_id
                && m.role == crate::models::BusinessRole::Leader
                && m.is_active()
        }))
    }

    /// 部署メンバーを取得する（実装は簡略化）
    async fn get_department_members(
        &self,
        _business: &Business,
        _user_permissions: &UserPermissions,
    ) -> Result<Vec<CirculationCandidate>, BusinessError> {
        // TODO: 部署メンバー取得の実装
        // 現在は空のVecを返す
        Ok(vec![])
    }
}

// 業務関連のレスポンス型
#[derive(Debug, Clone, serde::Serialize)]
pub struct BusinessResponse {
    pub business: Business,
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BusinessMemberResponse {
    pub member: BusinessMember,
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub message: Option<String>,
}

impl From<Business> for BusinessResponse {
    fn from(business: Business) -> Self {
        Self {
            business,
            success: true,
            message: None,
        }
    }
}

impl From<BusinessMember> for BusinessMemberResponse {
    fn from(member: BusinessMember) -> Self {
        Self {
            member,
            success: true,
            message: None,
        }
    }
}
