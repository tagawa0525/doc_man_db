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