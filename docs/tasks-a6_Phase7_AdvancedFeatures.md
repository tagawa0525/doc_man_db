# Phase 7: 業務従事者・検索機能実装 (Week 13-14)

## フェーズ概要

- **期間**: Week 13-14 (2週間)
- **目標**: 業務従事者管理機能と高度検索機能の実装
- **成果物**: 業務従事者管理機能、高度検索機能（社員・業務）、検索支援機能

## タスク一覧

### TASK-046: 業務関連テーブル作成

- **説明**: business_members/external_contactsテーブル
- **優先度**: High
- **見積工数**: 6h
- **状態**: 未着手
- **依存関係**: TASK-003

#### 実装内容(TASK-046)

1. 業務従事者テーブル設計
2. 外部連絡先テーブル設計
3. 業務・従事者関連テーブル
4. マイグレーション作成

#### データベーススキーマ(TASK-046)

```sql
-- migrations/010_create_business_tables.sql

-- 業務テーブル
CREATE TABLE businesses (
    id INTEGER PRIMARY KEY,
    business_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    customer_name TEXT,
    start_date DATE,
    end_date DATE,
    status TEXT CHECK(status IN ('active', 'completed', 'cancelled')) DEFAULT 'active',
    created_by INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES employees (id)
);

-- 業務従事者テーブル
CREATE TABLE business_members (
    id INTEGER PRIMARY KEY,
    business_id INTEGER NOT NULL,
    employee_id INTEGER NOT NULL,
    role TEXT NOT NULL, -- 'leader', 'member', 'advisor'
    participation_level TEXT CHECK(participation_level IN ('full', 'partial', 'support')) DEFAULT 'full',
    start_date DATE NOT NULL,
    end_date DATE,
    notes TEXT,
    created_by INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (business_id) REFERENCES businesses (id),
    FOREIGN KEY (employee_id) REFERENCES employees (id),
    FOREIGN KEY (created_by) REFERENCES employees (id),
    UNIQUE(business_id, employee_id, start_date)
);

-- 外部連絡先テーブル
CREATE TABLE external_contacts (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    company_name TEXT,
    email TEXT,
    phone TEXT,
    address TEXT,
    contact_type TEXT CHECK(contact_type IN ('customer', 'vendor', 'partner', 'other')),
    is_active BOOLEAN DEFAULT 1,
    created_by INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES employees (id)
);

-- 業務外部連絡先関連テーブル
CREATE TABLE business_external_contacts (
    id INTEGER PRIMARY KEY,
    business_id INTEGER NOT NULL,
    external_contact_id INTEGER NOT NULL,
    relationship TEXT, -- 'primary_contact', 'stakeholder', 'reviewer'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (business_id) REFERENCES businesses (id),
    FOREIGN KEY (external_contact_id) REFERENCES external_contacts (id),
    UNIQUE(business_id, external_contact_id)
);

-- インデックス作成
CREATE INDEX idx_business_members_business ON business_members(business_id);
CREATE INDEX idx_business_members_employee ON business_members(employee_id);
CREATE INDEX idx_business_members_period ON business_members(start_date, end_date);
CREATE INDEX idx_businesses_number ON businesses(business_number);
CREATE INDEX idx_businesses_customer ON businesses(customer_name);
CREATE INDEX idx_external_contacts_company ON external_contacts(company_name);
```

#### 成果物(TASK-046)

- 業務管理テーブル
- 業務従事者管理テーブル
- 外部連絡先テーブル
- 関連テーブル・インデックス

---

### TASK-047: 業務従事者管理API実装

- **説明**: 業務メンバーCRUD・役割管理API
- **優先度**: High
- **見積工数**: 12h
- **状態**: 未着手
- **依存関係**: TASK-046

#### 実装内容(TASK-047)

1. 業務CRUD API
2. 業務従事者管理API
3. 役割・参加レベル管理
4. 期間管理機能

#### GraphQL スキーマ(TASK-047)

```graphql
# 業務関連の型定義
type Business {
  id: ID!
  businessNumber: String!
  name: String!
  description: String
  customerName: String
  startDate: Date
  endDate: Date
  status: BusinessStatus!
  members: [BusinessMember!]!
  externalContacts: [ExternalContact!]!
  documents: [Document!]!
  createdBy: Employee!
  createdAt: DateTime!
  updatedAt: DateTime!
}

type BusinessMember {
  id: ID!
  business: Business!
  employee: Employee!
  role: BusinessRole!
  participationLevel: ParticipationLevel!
  startDate: Date!
  endDate: Date
  notes: String
  createdBy: Employee!
  createdAt: DateTime!
}

type ExternalContact {
  id: ID!
  name: String!
  companyName: String
  email: String
  phone: String
  address: String
  contactType: ContactType!
  isActive: Boolean!
  businesses: [Business!]!
}

enum BusinessStatus {
  ACTIVE
  COMPLETED
  CANCELLED
}

enum BusinessRole {
  LEADER
  MEMBER
  ADVISOR
}

enum ParticipationLevel {
  FULL
  PARTIAL
  SUPPORT
}

enum ContactType {
  CUSTOMER
  VENDOR
  PARTNER
  OTHER
}

# 入力型
input CreateBusinessInput {
  businessNumber: String
  name: String!
  description: String
  customerName: String
  startDate: Date
  endDate: Date
}

input CreateBusinessMemberInput {
  businessId: ID!
  employeeId: ID!
  role: BusinessRole!
  participationLevel: ParticipationLevel!
  startDate: Date!
  endDate: Date
  notes: String
}

input BusinessSearchInput {
  businessNumber: String
  name: String
  customerName: String
  status: BusinessStatus
  memberEmployeeId: ID
  dateRange: DateRangeInput
  pagination: PaginationInput
}

# クエリ・ミューテーション
extend type Query {
  business(id: ID!): Business
  businesses(input: BusinessSearchInput!): BusinessSearchResult!
  businessMembers(businessId: ID!): [BusinessMember!]!
  externalContacts(input: ExternalContactSearchInput!): ExternalContactSearchResult!
}

extend type Mutation {
  createBusiness(input: CreateBusinessInput!): BusinessResponse!
  updateBusiness(id: ID!, input: UpdateBusinessInput!): BusinessResponse!
  deleteBusiness(id: ID!): DeleteResponse!
  
  addBusinessMember(input: CreateBusinessMemberInput!): BusinessMemberResponse!
  updateBusinessMember(id: ID!, input: UpdateBusinessMemberInput!): BusinessMemberResponse!
  removeBusinessMember(id: ID!): DeleteResponse!
  
  createExternalContact(input: CreateExternalContactInput!): ExternalContactResponse!
  updateExternalContact(id: ID!, input: UpdateExternalContactInput!): ExternalContactResponse!
}
```

#### Repository実装(TASK-047)

```rust
// src/repositories/business_repository.rs
use sqlx::{SqlitePool, Row};
use async_trait::async_trait;

#[async_trait]
pub trait BusinessRepository: Send + Sync {
    async fn create(&self, business: CreateBusinessRequest) -> Result<Business, BusinessError>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Business>, BusinessError>;
    async fn search(&self, filters: BusinessSearchFilters) -> Result<(Vec<Business>, i64), BusinessError>;
    async fn update(&self, id: i32, business: UpdateBusinessRequest) -> Result<Business, BusinessError>;
    async fn delete(&self, id: i32) -> Result<(), BusinessError>;
    
    // 業務従事者関連
    async fn get_members(&self, business_id: i32) -> Result<Vec<BusinessMember>, BusinessError>;
    async fn add_member(&self, member: CreateBusinessMemberRequest) -> Result<BusinessMember, BusinessError>;
    async fn remove_member(&self, member_id: i32) -> Result<(), BusinessError>;
    async fn get_member_history(&self, employee_id: i32) -> Result<Vec<BusinessMember>, BusinessError>;
}

pub struct SqliteBusinessRepository {
    pool: SqlitePool,
}

impl SqliteBusinessRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BusinessRepository for SqliteBusinessRepository {
    async fn create(&self, request: CreateBusinessRequest) -> Result<Business, BusinessError> {
        let mut tx = self.pool.begin().await?;
        
        // 業務番号生成（未指定の場合）
        let business_number = if let Some(number) = request.business_number {
            number
        } else {
            self.generate_business_number().await?
        };
        
        let business_id = sqlx::query!(
            r#"
            INSERT INTO businesses (
                business_number, name, description, customer_name, 
                start_date, end_date, status, created_by
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            business_number,
            request.name,
            request.description,
            request.customer_name,
            request.start_date,
            request.end_date,
            request.status.unwrap_or_else(|| "active".to_string()),
            request.created_by
        )
        .execute(&mut *tx)
        .await?
        .last_insert_rowid();
        
        tx.commit().await?;
        
        self.get_by_id(business_id as i32).await?
            .ok_or(BusinessError::NotFound)
    }
    
    async fn search(&self, filters: BusinessSearchFilters) -> Result<(Vec<Business>, i64), BusinessError> {
        let mut conditions = Vec::new();
        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Sqlite> + Send + Sync>> = Vec::new();
        
        if let Some(business_number) = &filters.business_number {
            conditions.push("b.business_number LIKE ?");
            params.push(Box::new(format!("%{}%", business_number)));
        }
        
        if let Some(name) = &filters.name {
            conditions.push("b.name LIKE ?");
            params.push(Box::new(format!("%{}%", name)));
        }
        
        if let Some(customer_name) = &filters.customer_name {
            conditions.push("b.customer_name LIKE ?");
            params.push(Box::new(format!("%{}%", customer_name)));
        }
        
        if let Some(status) = &filters.status {
            conditions.push("b.status = ?");
            params.push(Box::new(status.clone()));
        }
        
        if let Some(member_employee_id) = filters.member_employee_id {
            conditions.push(
                "EXISTS (
                    SELECT 1 FROM business_members bm 
                    WHERE bm.business_id = b.id 
                    AND bm.employee_id = ?
                    AND (bm.end_date IS NULL OR bm.end_date >= DATE('now'))
                )"
            );
            params.push(Box::new(member_employee_id));
        }
        
        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };
        
        // 総件数取得
        let count_query = format!(
            "SELECT COUNT(*) as count FROM businesses b {}",
            where_clause
        );
        
        let total: i64 = sqlx::query(&count_query)
            .bind_all(params.iter().map(|p| p.as_ref()))
            .fetch_one(&self.pool)
            .await?
            .get("count");
        
        // データ取得
        let data_query = format!(
            r#"
            SELECT b.*, e.name as creator_name
            FROM businesses b
            JOIN employees e ON b.created_by = e.id
            {}
            ORDER BY b.created_at DESC
            LIMIT ? OFFSET ?
            "#,
            where_clause
        );
        
        let rows = sqlx::query(&data_query)
            .bind_all(params.iter().map(|p| p.as_ref()))
            .bind(filters.pagination.limit)
            .bind(filters.pagination.offset)
            .fetch_all(&self.pool)
            .await?;
        
        let businesses = rows.into_iter()
            .map(|row| Business::from_row(&row))
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok((businesses, total))
    }
    
    async fn add_member(&self, request: CreateBusinessMemberRequest) -> Result<BusinessMember, BusinessError> {
        // 重複チェック
        let existing = sqlx::query!(
            r#"
            SELECT id FROM business_members
            WHERE business_id = ? AND employee_id = ? 
            AND (end_date IS NULL OR end_date >= ?)
            "#,
            request.business_id,
            request.employee_id,
            request.start_date
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if existing.is_some() {
            return Err(BusinessError::MemberAlreadyExists);
        }
        
        let member_id = sqlx::query!(
            r#"
            INSERT INTO business_members (
                business_id, employee_id, role, participation_level,
                start_date, end_date, notes, created_by
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            request.business_id,
            request.employee_id,
            request.role,
            request.participation_level,
            request.start_date,
            request.end_date,
            request.notes,
            request.created_by
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        
        self.get_member_by_id(member_id as i32).await
    }
    
    async fn get_member_history(&self, employee_id: i32) -> Result<Vec<BusinessMember>, BusinessError> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                bm.*,
                b.business_number,
                b.name as business_name,
                e.name as employee_name
            FROM business_members bm
            JOIN businesses b ON bm.business_id = b.id
            JOIN employees e ON bm.employee_id = e.id
            WHERE bm.employee_id = ?
            ORDER BY bm.start_date DESC
            "#,
            employee_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let members = rows.into_iter()
            .map(|row| BusinessMember::from_row(&row))
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(members)
    }
}
```

#### Service実装

```rust
// src/services/business_service.rs
pub struct BusinessService {
    repository: Box<dyn BusinessRepository>,
    employee_service: Box<dyn EmployeeService>,
}

impl BusinessService {
    pub async fn create_business(
        &self,
        request: CreateBusinessRequest,
        user_permissions: &UserPermissions,
    ) -> Result<Business, BusinessError> {
        // 権限チェック
        if !user_permissions.can_create_businesses {
            return Err(BusinessError::PermissionDenied);
        }
        
        // 業務番号重複チェック
        if let Some(business_number) = &request.business_number {
            if let Ok(Some(_)) = self.repository.get_by_business_number(business_number).await {
                return Err(BusinessError::BusinessNumberExists);
            }
        }
        
        self.repository.create(request).await
    }
    
    pub async fn add_business_member(
        &self,
        request: CreateBusinessMemberRequest,
        user_permissions: &UserPermissions,
    ) -> Result<BusinessMember, BusinessError> {
        // 権限チェック
        if !user_permissions.can_manage_business_members {
            return Err(BusinessError::PermissionDenied);
        }
        
        // 社員存在チェック
        let employee = self.employee_service
            .get_by_id(request.employee_id).await?
            .ok_or(BusinessError::EmployeeNotFound)?;
        
        // 業務存在チェック
        let business = self.repository
            .get_by_id(request.business_id).await?
            .ok_or(BusinessError::BusinessNotFound)?;
        
        self.repository.add_member(request).await
    }
    
    pub async fn suggest_circulation_candidates(
        &self,
        business_id: i32,
        user_permissions: &UserPermissions,
    ) -> Result<CirculationCandidates, BusinessError> {
        let business = self.repository
            .get_by_id(business_id).await?
            .ok_or(BusinessError::BusinessNotFound)?;
        
        let members = self.repository.get_members(business_id).await?;
        let external_contacts = self.repository.get_external_contacts(business_id).await?;
        
        // 階層化された候補を作成
        let candidates = CirculationCandidates {
            business_members: members.into_iter()
                .filter(|m| m.is_active())
                .map(|m| CirculationCandidate {
                    id: format!("employee_{}", m.employee_id),
                    name: m.employee_name.clone(),
                    email: m.employee_email.clone(),
                    category: "業務従事者".to_string(),
                    priority: match m.role.as_str() {
                        "leader" => 1,
                        "member" => 2,
                        "advisor" => 3,
                        _ => 4,
                    },
                })
                .collect(),
            
            external_contacts: external_contacts.into_iter()
                .filter(|c| c.is_active)
                .map(|c| CirculationCandidate {
                    id: format!("external_{}", c.id),
                    name: c.name.clone(),
                    email: c.email.clone(),
                    category: "外部連絡先".to_string(),
                    priority: 5,
                })
                .collect(),
                
            department_members: self.get_department_members(&business, user_permissions).await?,
        };
        
        Ok(candidates)
    }
}
```

#### 成果物(TASK-047)

- 業務管理API
- 業務従事者管理API
- 回覧候補提案機能
- 外部連絡先管理

---

### TASK-048: 高度社員検索API実装

- **説明**: 複合条件検索・オートコンプリート
- **優先度**: High
- **見積工数**: 14h
- **状態**: 未着手
- **依存関係**: TASK-010

#### 実装内容(TASK-048)

1. 複合条件社員検索
2. オートコンプリート機能
3. 業務従事履歴検索
4. 部署・役職検索

#### GraphQL スキーマ拡張(TASK-048)

```graphql
# 高度検索用の型定義
input AdvancedEmployeeSearchInput {
  name: String
  employeeId: String
  email: String
  departmentId: ID
  currentPosition: String
  hasBusinessExperience: String  # 業務番号
  joiningDateFrom: Date
  joiningDateTo: Date
  isActive: Boolean
  skillKeywords: [String!]
  sortBy: EmployeeSortField
  sortOrder: SortOrder
  pagination: PaginationInput!
}

input EmployeeAutocompleteInput {
  query: String!
  field: EmployeeSearchField!
  limit: Int = 10
  includeInactive: Boolean = false
}

enum EmployeeSearchField {
  NAME
  EMPLOYEE_ID
  EMAIL
  DEPARTMENT
}

enum EmployeeSortField {
  NAME
  EMPLOYEE_ID
  JOINING_DATE
  DEPARTMENT
  LAST_UPDATED
}

type EmployeeSearchResult {
  employees: [Employee!]!
  totalCount: Int!
  hasNextPage: Boolean!
  aggregations: EmployeeSearchAggregations
}

type EmployeeSearchAggregations {
  departmentCounts: [DepartmentCount!]!
  positionCounts: [PositionCount!]!
  statusCounts: StatusCount!
}

type DepartmentCount {
  department: Department!
  count: Int!
}

type AutocompleteResult {
  suggestions: [AutocompleteSuggestion!]!
}

type AutocompleteSuggestion {
  value: String!
  label: String!
  category: String
  metadata: JSON
}

extend type Query {
  advancedEmployeeSearch(input: AdvancedEmployeeSearchInput!): EmployeeSearchResult!
  employeeAutocomplete(input: EmployeeAutocompleteInput!): AutocompleteResult!
  employeeBusinessHistory(employeeId: ID!): [BusinessMember!]!
}
```

#### Repository実装(TASK-048)

```rust
// src/repositories/advanced_search_repository.rs
pub struct AdvancedSearchRepository {
    pool: SqlitePool,
}

impl AdvancedSearchRepository {
    pub async fn search_employees_advanced(
        &self,
        filters: AdvancedEmployeeSearchInput,
        user_permissions: &UserPermissions,
    ) -> Result<(Vec<Employee>, i64, EmployeeSearchAggregations), SearchError> {
        let mut conditions = Vec::new();
        let mut joins = Vec::new();
        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Sqlite> + Send + Sync>> = Vec::new();
        
        // 基本的な検索条件
        if let Some(name) = &filters.name {
            conditions.push("e.name LIKE ?");
            params.push(Box::new(format!("%{}%", name)));
        }
        
        if let Some(employee_id) = &filters.employee_id {
            conditions.push("e.employee_id LIKE ?");
            params.push(Box::new(format!("%{}%", employee_id)));
        }
        
        if let Some(email) = &filters.email {
            conditions.push("e.email LIKE ?");
            params.push(Box::new(format!("%{}%", email)));
        }
        
        if let Some(department_id) = filters.department_id {
            joins.push(
                "JOIN department_assignments da ON e.id = da.employee_id 
                 AND (da.end_date IS NULL OR da.end_date >= DATE('now'))"
            );
            conditions.push("da.department_id = ?");
            params.push(Box::new(department_id));
        }
        
        // 業務従事経験フィルター
        if let Some(business_number) = &filters.has_business_experience {
            joins.push(
                "JOIN business_members bm ON e.id = bm.employee_id
                 JOIN businesses b ON bm.business_id = b.id"
            );
            conditions.push("b.business_number LIKE ?");
            params.push(Box::new(format!("%{}%", business_number)));
        }
        
        // 入社日範囲
        if let Some(joining_date_from) = &filters.joining_date_from {
            conditions.push("e.joining_date >= ?");
            params.push(Box::new(*joining_date_from));
        }
        
        if let Some(joining_date_to) = &filters.joining_date_to {
            conditions.push("e.joining_date <= ?");
            params.push(Box::new(*joining_date_to));
        }
        
        // アクティブ状態
        if let Some(is_active) = filters.is_active {
            conditions.push("e.is_active = ?");
            params.push(Box::new(is_active));
        }
        
        // 権限による部署フィルター
        if !user_permissions.accessible_departments.is_empty() {
            let dept_placeholders = user_permissions.accessible_departments
                .iter()
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(",");
            
            if !joins.iter().any(|j| j.contains("department_assignments")) {
                joins.push(
                    "JOIN department_assignments da ON e.id = da.employee_id 
                     AND (da.end_date IS NULL OR da.end_date >= DATE('now'))"
                );
            }
            
            conditions.push(&format!("da.department_id IN ({})", dept_placeholders));
            for dept_id in &user_permissions.accessible_departments {
                params.push(Box::new(*dept_id));
            }
        }
        
        let joins_clause = joins.join(" ");
        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };
        
        // ソート順設定
        let order_by = match filters.sort_by.unwrap_or(EmployeeSortField::Name) {
            EmployeeSortField::Name => "e.name",
            EmployeeSortField::EmployeeId => "e.employee_id",
            EmployeeSortField::JoiningDate => "e.joining_date",
            EmployeeSortField::Department => "d.name",
            EmployeeSortField::LastUpdated => "e.updated_at",
        };
        
        let sort_order = match filters.sort_order.unwrap_or(SortOrder::Asc) {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        };
        
        // メインクエリ実行
        let query = format!(
            r#"
            SELECT DISTINCT e.*, d.name as department_name
            FROM employees e
            LEFT JOIN department_assignments da ON e.id = da.employee_id 
                AND (da.end_date IS NULL OR da.end_date >= DATE('now'))
            LEFT JOIN departments d ON da.department_id = d.id
            {}
            {}
            ORDER BY {} {}
            LIMIT ? OFFSET ?
            "#,
            joins_clause, where_clause, order_by, sort_order
        );
        
        let rows = sqlx::query(&query)
            .bind_all(params.iter().map(|p| p.as_ref()))
            .bind(filters.pagination.limit)
            .bind(filters.pagination.offset)
            .fetch_all(&self.pool)
            .await?;
        
        let employees = rows.into_iter()
            .map(|row| Employee::from_row(&row))
            .collect::<Result<Vec<_>, _>>()?;
        
        // 総件数取得
        let count_query = format!(
            "SELECT COUNT(DISTINCT e.id) as count FROM employees e {} {}",
            joins_clause, where_clause
        );
        
        let total: i64 = sqlx::query(&count_query)
            .bind_all(params.iter().map(|p| p.as_ref()))
            .fetch_one(&self.pool)
            .await?
            .get("count");
        
        // 集計情報取得
        let aggregations = self.get_employee_search_aggregations(&filters, user_permissions).await?;
        
        Ok((employees, total, aggregations))
    }
    
    pub async fn employee_autocomplete(
        &self,
        input: EmployeeAutocompleteInput,
        user_permissions: &UserPermissions,
    ) -> Result<Vec<AutocompleteSuggestion>, SearchError> {
        let mut suggestions = Vec::new();
        
        match input.field {
            EmployeeSearchField::Name => {
                let rows = sqlx::query!(
                    r#"
                    SELECT DISTINCT e.name, e.employee_id, d.name as department_name
                    FROM employees e
                    LEFT JOIN department_assignments da ON e.id = da.employee_id 
                        AND (da.end_date IS NULL OR da.end_date >= DATE('now'))
                    LEFT JOIN departments d ON da.department_id = d.id
                    WHERE e.name LIKE ? 
                    AND (? OR e.is_active = 1)
                    ORDER BY e.name
                    LIMIT ?
                    "#,
                    format!("%{}%", input.query),
                    input.include_inactive,
                    input.limit
                )
                .fetch_all(&self.pool)
                .await?;
                
                for row in rows {
                    suggestions.push(AutocompleteSuggestion {
                        value: row.name.clone(),
                        label: format!("{} ({})", row.name, row.employee_id),
                        category: row.department_name,
                        metadata: serde_json::json!({
                            "employeeId": row.employee_id
                        }),
                    });
                }
            },
            
            EmployeeSearchField::EmployeeId => {
                let rows = sqlx::query!(
                    r#"
                    SELECT DISTINCT e.employee_id, e.name, d.name as department_name
                    FROM employees e
                    LEFT JOIN department_assignments da ON e.id = da.employee_id 
                        AND (da.end_date IS NULL OR da.end_date >= DATE('now'))
                    LEFT JOIN departments d ON da.department_id = d.id
                    WHERE e.employee_id LIKE ?
                    AND (? OR e.is_active = 1)
                    ORDER BY e.employee_id
                    LIMIT ?
                    "#,
                    format!("%{}%", input.query),
                    input.include_inactive,
                    input.limit
                )
                .fetch_all(&self.pool)
                .await?;
                
                for row in rows {
                    suggestions.push(AutocompleteSuggestion {
                        value: row.employee_id.clone(),
                        label: format!("{} - {}", row.employee_id, row.name),
                        category: row.department_name,
                        metadata: serde_json::json!({
                            "name": row.name
                        }),
                    });
                }
            },
            
            EmployeeSearchField::Department => {
                let rows = sqlx::query!(
                    r#"
                    SELECT DISTINCT d.name, d.code, COUNT(da.employee_id) as employee_count
                    FROM departments d
                    LEFT JOIN department_assignments da ON d.id = da.department_id 
                        AND (da.end_date IS NULL OR da.end_date >= DATE('now'))
                    WHERE d.name LIKE ?
                    GROUP BY d.id, d.name, d.code
                    ORDER BY d.name
                    LIMIT ?
                    "#,
                    format!("%{}%", input.query),
                    input.limit
                )
                .fetch_all(&self.pool)
                .await?;
                
                for row in rows {
                    suggestions.push(AutocompleteSuggestion {
                        value: row.name.clone(),
                        label: format!("{} ({} 名)", row.name, row.employee_count),
                        category: "部署".to_string(),
                        metadata: serde_json::json!({
                            "code": row.code,
                            "employeeCount": row.employee_count
                        }),
                    });
                }
            },
            
            _ => {
                // その他のフィールドの実装
            }
        }
        
        Ok(suggestions)
    }
}
```

#### 成果物(TASK-048)

- 高度社員検索API
- オートコンプリート機能
- 検索結果集計
- 業務従事履歴検索

---

### TASK-049: 高度業務検索API実装

- **説明**: 業務検索・従事者逆引き検索
- **優先度**: High
- **見積工数**: 12h
- **状態**: 未着手
- **依存関係**: TASK-047

#### 実装内容(TASK-049)

1. 業務複合条件検索
2. 従事者による逆引き検索
3. 顧客・期間検索
4. 業務ステータス検索

#### GraphQL スキーマ(TASK-049)

```graphql
input AdvancedBusinessSearchInput {
  businessNumber: String
  name: String
  customerName: String
  description: String
  memberEmployeeId: ID
  memberRole: BusinessRole
  status: BusinessStatus
  startDateFrom: Date
  startDateTo: Date
  endDateFrom: Date
  endDateTo: Date
  hasDocuments: Boolean
  createdBy: ID
  sortBy: BusinessSortField
  sortOrder: SortOrder
  pagination: PaginationInput!
}

enum BusinessSortField {
  BUSINESS_NUMBER
  NAME
  CUSTOMER_NAME
  START_DATE
  END_DATE
  CREATED_AT
}

type BusinessSearchResult {
  businesses: [Business!]!
  totalCount: Int!
  hasNextPage: Boolean!
  aggregations: BusinessSearchAggregations!
}

type BusinessSearchAggregations {
  statusCounts: [StatusCount!]!
  customerCounts: [CustomerCount!]!
  memberCounts: [MemberCount!]!
  yearCounts: [YearCount!]!
}

extend type Query {
  advancedBusinessSearch(input: AdvancedBusinessSearchInput!): BusinessSearchResult!
  businessAutocomplete(input: BusinessAutocompleteInput!): AutocompleteResult!
  employeeBusinesses(employeeId: ID!, includeCompleted: Boolean = false): [Business!]!
}
```

#### 成果物(TASK-049)

- 高度業務検索API
- 従事者逆引き検索
- 業務オートコンプリート
- 検索結果集計

---

### TASK-050: 検索候補・履歴機能実装

- **説明**: オートコンプリート・お気に入り機能
- **優先度**: Medium
- **見積工数**: 8h
- **状態**: 未着手
- **依存関係**: TASK-048

#### 実装内容(TASK-050)

1. 検索履歴保存
2. お気に入り検索条件
3. 検索候補学習
4. 検索統計分析

#### データベーステーブル

```sql
-- 検索履歴テーブル
CREATE TABLE search_history (
    id INTEGER PRIMARY KEY,
    employee_id INTEGER NOT NULL,
    search_type TEXT NOT NULL, -- 'employee', 'business', 'document'
    search_query TEXT NOT NULL, -- JSON形式の検索条件
    result_count INTEGER,
    execution_time_ms INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (employee_id) REFERENCES employees (id)
);

-- お気に入り検索テーブル
CREATE TABLE favorite_searches (
    id INTEGER PRIMARY KEY,
    employee_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    search_type TEXT NOT NULL,
    search_query TEXT NOT NULL, -- JSON形式の検索条件
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (employee_id) REFERENCES employees (id)
);

-- 検索候補テーブル
CREATE TABLE search_suggestions (
    id INTEGER PRIMARY KEY,
    search_type TEXT NOT NULL,
    field_name TEXT NOT NULL,
    suggestion TEXT NOT NULL,
    frequency INTEGER DEFAULT 1,
    last_used DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_search_history_employee ON search_history(employee_id, created_at);
CREATE INDEX idx_favorite_searches_employee ON favorite_searches(employee_id, is_active);
CREATE INDEX idx_search_suggestions_type ON search_suggestions(search_type, field_name);
```

#### 成果物(TASK-050)

- 検索履歴管理
- お気に入り検索機能
- 自動候補生成
- 検索統計分析

## フェーズ完了基準

### 必須条件

- [ ] 業務従事者管理機能が動作する
- [ ] 高度社員検索が動作する
- [ ] 高度業務検索が動作する
- [ ] オートコンプリート機能が動作する
- [ ] 検索履歴・お気に入り機能が動作する

### 検証方法

```bash
# 業務管理テスト
curl -X POST /graphql -d '{"query":"mutation{createBusiness(...)}"}'

# 高度検索テスト
curl -X POST /graphql -d '{"query":"query{advancedEmployeeSearch(...)}"}'

# オートコンプリートテスト
curl -X POST /graphql -d '{"query":"query{employeeAutocomplete(...)}"}'
```

## 次フェーズへの引き継ぎ事項

- 業務従事者管理機能完成
- 高度検索機能完成
- 最終フェーズ実装準備
- UI統合準備

## リスク・課題

- **検索性能**: 複合条件での検索性能
- **データ整合性**: 業務・従事者関連の整合性
- **UI複雑性**: 高度検索UIの複雑化

## 対応策

- インデックス最適化
- データ整合性チェック強化
- 段階的UI公開・ユーザーテスト
