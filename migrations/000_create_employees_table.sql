-- Create employees table
-- This table must be created before other tables that reference it
CREATE TABLE employees (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    employee_number TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    department TEXT NOT NULL,
    position TEXT,
    email TEXT,
    phone TEXT,
    hire_date DATE,
    is_active INTEGER DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for performance
CREATE INDEX idx_employees_number ON employees(employee_number);
CREATE INDEX idx_employees_name ON employees(name);
CREATE INDEX idx_employees_department ON employees(department);
CREATE INDEX idx_employees_active ON employees(is_active);

-- Insert some default test data for testing
INSERT INTO employees (employee_number, name, department, position, email)
VALUES 
    ('EMP001', 'システム管理者', 'IT', 'Manager', 'admin@company.com'),
    ('EMP002', 'テストユーザー', 'General', 'Staff', 'test@company.com');