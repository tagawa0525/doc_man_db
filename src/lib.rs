pub mod app;
pub mod error;
pub mod graphql;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;

// Re-export main components for easy access
pub use app::{create_app, AppState};
