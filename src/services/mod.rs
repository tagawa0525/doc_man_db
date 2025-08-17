// Document Management System Services

pub mod document_number_generator;
pub mod document_service;

// Re-export all services
pub use document_number_generator::*;
pub use document_service::*;
