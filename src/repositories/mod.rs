// Document Management System Repositories

pub mod document_number_rule_repository;
pub mod document_repository;

// Re-export all repositories
pub use document_number_rule_repository::*;
pub use document_repository::*;
