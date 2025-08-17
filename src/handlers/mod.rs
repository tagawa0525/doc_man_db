pub mod business;
pub mod graphql;
pub mod http;

// Re-export business handlers for compatibility
pub use business::{DocumentHandlers, HealthHandler};
