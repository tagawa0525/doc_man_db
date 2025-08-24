-- Create departments table
CREATE TABLE departments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    code TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    parent_id INTEGER,
    level INTEGER DEFAULT 0,
    manager_id INTEGER,
    description TEXT,
    location TEXT,
    phone_number TEXT,
    email TEXT,
    budget REAL,
    is_active INTEGER DEFAULT 1,
    created_date DATE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES departments(id),
    FOREIGN KEY (manager_id) REFERENCES employees(id)
);

-- Create indexes for performance
CREATE INDEX idx_departments_code ON departments(code);
CREATE INDEX idx_departments_name ON departments(name);
CREATE INDEX idx_departments_parent ON departments(parent_id);
CREATE INDEX idx_departments_manager ON departments(manager_id);
CREATE INDEX idx_departments_active ON departments(is_active);

-- Insert test department data without manager_id first to avoid FK constraints
INSERT INTO departments (id, code, name, parent_id, level, description, location, phone_number, email, budget, is_active, created_date)
VALUES 
    (1, 'DEV', '開発部', NULL, 0, 'システム開発とメンテナンス', '本社3F', '03-1234-5600', 'dev@company.com', 50000000.00, 1, '2020-04-01'),
    (2, 'SALES', '営業部', NULL, 0, '営業活動と顧客対応', '本社2F', '03-1234-5602', 'sales@company.com', 30000000.00, 1, '2020-04-01'),
    (3, 'HR', '人事部', NULL, 0, '人事管理と組織運営', '本社1F', '03-1234-5603', 'hr@company.com', 20000000.00, 1, '2020-04-01'),
    (4, 'FIN', '財務部', NULL, 0, '財務管理と経理', '本社1F', '03-1234-5604', 'finance@company.com', 15000000.00, 0, '2020-04-01');

-- Update employees table to include test employees for each department
INSERT INTO employees (id, employee_number, name, department, position, email, hire_date)
VALUES 
    (1, 'EMP101', '山田太郎', 'DEV', '部長', 'yamada@company.com', '2018-04-01'),
    (2, 'EMP201', '佐藤花子', 'SALES', '部長', 'sato@company.com', '2019-04-01'),
    (3, 'EMP301', '田中一郎', 'HR', '部長', 'tanaka@company.com', '2017-04-01'),
    (4, 'EMP401', '鈴木二郎', 'FIN', '部長', 'suzuki.finance@company.com', '2016-04-01'),
    (11, 'EMP102', '鈴木次郎', 'DEV', '課長', 'suzuki@company.com', '2019-07-01'),
    (12, 'EMP103', '田村美子', 'DEV', '主任', 'tamura@company.com', '2021-04-01'),
    (13, 'EMP202', '高橋健太', 'SALES', '課長', 'takahashi@company.com', '2020-04-01'),
    (14, 'EMP203', '松本美香', 'SALES', '主任', 'matsumoto@company.com', '2021-07-01'),
    (15, 'EMP302', '渡辺聡子', 'HR', '課長', 'watanabe@company.com', '2019-04-01'),
    (16, 'EMP303', '小林達也', 'HR', '主任', 'kobayashi@company.com', '2020-07-01'),
    (17, 'EMP402', '伊藤明子', 'FIN', '課長', 'ito@company.com', '2018-04-01')
ON CONFLICT(id) DO UPDATE SET
    employee_number = excluded.employee_number,
    name = excluded.name,
    department = excluded.department,
    position = excluded.position,
    email = excluded.email,
    hire_date = excluded.hire_date;

-- Now update departments with manager_id after employees exist
UPDATE departments SET manager_id = 1 WHERE id = 1; -- DEV department manager
UPDATE departments SET manager_id = 2 WHERE id = 2; -- SALES department manager  
UPDATE departments SET manager_id = 3 WHERE id = 3; -- HR department manager
UPDATE departments SET manager_id = 4 WHERE id = 4; -- FIN department manager
