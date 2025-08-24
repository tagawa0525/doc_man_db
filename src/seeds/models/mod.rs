pub mod seed_department;
pub mod seed_document_number_rule;
pub mod seed_document_type;
pub mod seed_employee;

pub use seed_department::SeedDepartment;
pub use seed_document_number_rule::SeedDocumentNumberRule;
pub use seed_document_type::SeedDocumentType;
pub use seed_employee::SeedEmployee;

use serde::{Deserialize, Serialize};

/// Seedファイルの基本構造
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedFile<T> {
    pub version: String,
    pub environment: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub data: Vec<T>,
}
