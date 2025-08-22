-- Create document_types table
-- This table defines the types of documents that can be created
CREATE TABLE document_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    department_code TEXT,
    prefix TEXT NOT NULL,
    effective_from DATE NOT NULL,
    effective_until DATE,
    is_active INTEGER DEFAULT 1,
    created_by INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES employees (id)
);

-- Create indexes for performance
CREATE INDEX idx_document_types_name ON document_types(name);
CREATE INDEX idx_document_types_prefix ON document_types(prefix);
CREATE INDEX idx_document_types_active ON document_types(is_active);
CREATE INDEX idx_document_types_effective ON document_types(effective_from, effective_until);

-- Insert some default document types for testing
INSERT INTO document_types (name, description, prefix, effective_from, created_by)
VALUES 
    ('技術文書', '技術仕様書、設計書等', 'TEC', '2024-01-01', 1),
    ('業務文書', '業務手順書、報告書等', 'BUS', '2024-01-01', 1),
    ('契約文書', '契約書、覚書等', 'CON', '2024-01-01', 1);