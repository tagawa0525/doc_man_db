-- Create documents table
-- This is the core table for document management
CREATE TABLE documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    document_type_id INTEGER NOT NULL,
    created_by INTEGER NOT NULL,
    created_date DATE NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for performance
CREATE INDEX idx_documents_title ON documents(title);
CREATE INDEX idx_documents_type_date ON documents(document_type_id, created_date);
CREATE INDEX idx_documents_created_by ON documents(created_by);
CREATE INDEX idx_documents_created_date ON documents(created_date);