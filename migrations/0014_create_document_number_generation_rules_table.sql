-- Create document_number_generation_rules table
-- This table defines rules for generating document numbers
CREATE TABLE document_number_generation_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rule_name TEXT NOT NULL,
    template TEXT NOT NULL,
    sequence_digits INTEGER NOT NULL DEFAULT 3,
    department_code TEXT,
    document_type_codes TEXT NOT NULL, -- JSON array of document type codes
    effective_from DATE NOT NULL,
    effective_until DATE,
    priority INTEGER NOT NULL DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for performance
CREATE INDEX idx_document_number_rules_department ON document_number_generation_rules(department_code);
CREATE INDEX idx_document_number_rules_effective ON document_number_generation_rules(effective_from, effective_until);
CREATE INDEX idx_document_number_rules_priority ON document_number_generation_rules(priority);
