pub mod advanced_search;
pub mod backup;
pub mod batch;
pub mod business;
pub mod business_management;
pub mod business_search;
pub mod circulation;
pub mod deduplication;
pub mod graphql;
pub mod http;
pub mod migration;
pub mod monitoring;
pub mod validation;

// Re-export business handlers for compatibility
pub use advanced_search::*;
pub use backup::*;
pub use batch::*;
pub use business::{DocumentHandlers, HealthHandler};
pub use business_management::*;
pub use business_search::*;
pub use circulation::*;
pub use deduplication::*;
pub use migration::*;
pub use monitoring::*;
pub use validation::*;
