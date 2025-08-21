use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Business {
    pub id: i32,
    pub business_number: String,
    pub name: String,
    pub description: Option<String>,
    pub customer_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: BusinessStatus,
    pub created_by: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BusinessMember {
    pub id: i32,
    pub business_id: i32,
    pub employee_id: i32,
    pub role: BusinessRole,
    pub participation_level: ParticipationLevel,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_by: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExternalContact {
    pub id: i32,
    pub name: String,
    pub company_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub contact_type: ContactType,
    pub is_active: bool,
    pub created_by: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BusinessExternalContact {
    pub id: i32,
    pub business_id: i32,
    pub external_contact_id: i32,
    pub relationship: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BusinessStatus {
    Active,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BusinessRole {
    Leader,
    Member,
    Advisor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParticipationLevel {
    Full,
    Partial,
    Support,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContactType {
    Customer,
    Vendor,
    Partner,
    Other,
}

// 入力データ構造
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBusinessRequest {
    pub business_number: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub customer_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub created_by: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBusinessRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub customer_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBusinessMemberRequest {
    pub business_id: i32,
    pub employee_id: i32,
    pub role: String,
    pub participation_level: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_by: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBusinessMemberRequest {
    pub role: Option<String>,
    pub participation_level: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExternalContactRequest {
    pub name: String,
    pub company_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub contact_type: String,
    pub created_by: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExternalContactRequest {
    pub name: Option<String>,
    pub company_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub contact_type: Option<String>,
    pub is_active: Option<bool>,
}

// 検索・フィルタリング用の構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSearchFilters {
    pub business_number: Option<String>,
    pub name: Option<String>,
    pub customer_name: Option<String>,
    pub status: Option<String>,
    pub member_employee_id: Option<i32>,
    pub date_range: Option<DateRange>,
    pub pagination: PaginationInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInput {
    pub limit: i32,
    pub offset: i32,
}

// 業務回覧候補
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CirculationCandidates {
    pub business_members: Vec<CirculationCandidate>,
    pub external_contacts: Vec<CirculationCandidate>,
    pub department_members: Vec<CirculationCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CirculationCandidate {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub category: String,
    pub priority: i32,
}

// 文字列からenumへの変換
impl From<String> for BusinessStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "active" => BusinessStatus::Active,
            "completed" => BusinessStatus::Completed,
            "cancelled" => BusinessStatus::Cancelled,
            _ => BusinessStatus::Active,
        }
    }
}

impl From<BusinessStatus> for String {
    fn from(status: BusinessStatus) -> Self {
        match status {
            BusinessStatus::Active => "active".to_string(),
            BusinessStatus::Completed => "completed".to_string(),
            BusinessStatus::Cancelled => "cancelled".to_string(),
        }
    }
}

impl From<String> for BusinessRole {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "leader" => BusinessRole::Leader,
            "member" => BusinessRole::Member,
            "advisor" => BusinessRole::Advisor,
            _ => BusinessRole::Member,
        }
    }
}

impl From<BusinessRole> for String {
    fn from(role: BusinessRole) -> Self {
        match role {
            BusinessRole::Leader => "leader".to_string(),
            BusinessRole::Member => "member".to_string(),
            BusinessRole::Advisor => "advisor".to_string(),
        }
    }
}

impl From<String> for ParticipationLevel {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "full" => ParticipationLevel::Full,
            "partial" => ParticipationLevel::Partial,
            "support" => ParticipationLevel::Support,
            _ => ParticipationLevel::Full,
        }
    }
}

impl From<ParticipationLevel> for String {
    fn from(level: ParticipationLevel) -> Self {
        match level {
            ParticipationLevel::Full => "full".to_string(),
            ParticipationLevel::Partial => "partial".to_string(),
            ParticipationLevel::Support => "support".to_string(),
        }
    }
}

impl From<String> for ContactType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "customer" => ContactType::Customer,
            "vendor" => ContactType::Vendor,
            "partner" => ContactType::Partner,
            "other" => ContactType::Other,
            _ => ContactType::Other,
        }
    }
}

impl From<ContactType> for String {
    fn from(contact_type: ContactType) -> Self {
        match contact_type {
            ContactType::Customer => "customer".to_string(),
            ContactType::Vendor => "vendor".to_string(),
            ContactType::Partner => "partner".to_string(),
            ContactType::Other => "other".to_string(),
        }
    }
}

// SQLXからの変換
impl Business {
    pub fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Business {
            id: row.try_get("id")?,
            business_number: row.try_get("business_number")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            customer_name: row.try_get("customer_name")?,
            start_date: row.try_get("start_date")?,
            end_date: row.try_get("end_date")?,
            status: BusinessStatus::from(row.try_get::<String, _>("status")?),
            created_by: row.try_get("created_by")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl BusinessMember {
    pub fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(BusinessMember {
            id: row.try_get("id")?,
            business_id: row.try_get("business_id")?,
            employee_id: row.try_get("employee_id")?,
            role: BusinessRole::from(row.try_get::<String, _>("role")?),
            participation_level: ParticipationLevel::from(row.try_get::<String, _>("participation_level")?),
            start_date: row.try_get("start_date")?,
            end_date: row.try_get("end_date")?,
            notes: row.try_get("notes")?,
            created_by: row.try_get("created_by")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    pub fn is_active(&self) -> bool {
        self.end_date.map_or(true, |end| end >= chrono::Utc::now().date_naive())
    }
}

impl ExternalContact {
    pub fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(ExternalContact {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            company_name: row.try_get("company_name")?,
            email: row.try_get("email")?,
            phone: row.try_get("phone")?,
            address: row.try_get("address")?,
            contact_type: ContactType::from(row.try_get::<String, _>("contact_type")?),
            is_active: row.try_get("is_active")?,
            created_by: row.try_get("created_by")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl BusinessExternalContact {
    pub fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(BusinessExternalContact {
            id: row.try_get("id")?,
            business_id: row.try_get("business_id")?,
            external_contact_id: row.try_get("external_contact_id")?,
            relationship: row.try_get("relationship")?,
            created_at: row.try_get("created_at")?,
        })
    }
}