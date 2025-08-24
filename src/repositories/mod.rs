// Document Management System Repositories

pub mod advanced_search_repository;
pub mod business_repository;
pub mod business_search_repository;
pub mod circulation_repository;
pub mod department_repository;
pub mod document_number_rule_repository;
pub mod document_repository;

// Re-export all repositories
pub use advanced_search_repository::*;
pub use business_repository::*;
pub use business_search_repository::*;
pub use circulation_repository::*;
pub use department_repository::*;
pub use document_number_rule_repository::*;
pub use document_repository::*;
