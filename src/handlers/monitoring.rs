use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::AppError;
use crate::services::{CacheService, DatabaseOptimizer, MetricsService};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub timestamp: u64,
    pub version: String,
    pub uptime_seconds: u64,
    pub services: ServiceHealthStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceHealthStatus {
    pub database: String,
    pub cache: String,
    pub api: String,
    pub file_system: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsResponse {
    pub metrics: crate::services::SystemMetrics,
    pub health: crate::services::HealthStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationResponse {
    pub report: crate::services::DatabaseOptimizationReport,
}

/// Application state for monitoring handlers
pub struct MonitoringState {
    pub metrics_service: Arc<MetricsService>,
    pub cache_service: Arc<CacheService>,
    pub start_time: std::time::Instant,
}

/// Health check endpoint
pub async fn health_check(
    State(state): State<Arc<MonitoringState>>,
) -> Result<Json<HealthCheckResponse>, AppError> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let uptime_seconds = state.start_time.elapsed().as_secs();

    // Check service health
    let health_status = state.metrics_service.get_health_status();

    let services = ServiceHealthStatus {
        database: health_status.database.to_string(),
        cache: health_status.cache.to_string(),
        api: health_status.api.to_string(),
        file_system: "healthy".to_string(), // Placeholder
    };

    let overall_status = match health_status.overall {
        crate::services::HealthState::Healthy => "healthy",
        crate::services::HealthState::Degraded => "degraded",
        crate::services::HealthState::Unhealthy => "unhealthy",
    };

    Ok(Json(HealthCheckResponse {
        status: overall_status.to_string(),
        timestamp,
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds,
        services,
    }))
}

/// Readiness check endpoint (for Kubernetes)
pub async fn readiness_check(
    State(state): State<Arc<MonitoringState>>,
) -> Result<StatusCode, AppError> {
    let health_status = state.metrics_service.get_health_status();

    match health_status.overall {
        crate::services::HealthState::Healthy => Ok(StatusCode::OK),
        crate::services::HealthState::Degraded => Ok(StatusCode::OK), // Still ready, but degraded
        crate::services::HealthState::Unhealthy => Ok(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// Liveness check endpoint (for Kubernetes)
pub async fn liveness_check() -> StatusCode {
    // Simple liveness check - if the handler responds, the service is alive
    StatusCode::OK
}

/// Get system metrics
pub async fn get_metrics(
    State(state): State<Arc<MonitoringState>>,
) -> Result<Json<MetricsResponse>, AppError> {
    let metrics = state.metrics_service.get_metrics();
    let health = state.metrics_service.get_health_status();

    Ok(Json(MetricsResponse { metrics, health }))
}

/// Get cache statistics
pub async fn get_cache_stats(
    State(state): State<Arc<MonitoringState>>,
) -> Result<Json<CacheStatsResponse>, AppError> {
    let size = state.cache_service.size().await;
    let cleared_count = state.cache_service.cleanup_expired().await;

    Ok(Json(CacheStatsResponse {
        size,
        expired_entries_cleared: cleared_count,
        status: "healthy".to_string(),
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStatsResponse {
    pub size: usize,
    pub expired_entries_cleared: usize,
    pub status: String,
}

/// Clear cache
pub async fn clear_cache(
    State(state): State<Arc<MonitoringState>>,
) -> Result<Json<CacheOperationResponse>, AppError> {
    state
        .cache_service
        .clear()
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to clear cache: {e}")))?;

    Ok(Json(CacheOperationResponse {
        success: true,
        message: "Cache cleared successfully".to_string(),
    }))
}

/// Invalidate cache pattern
pub async fn invalidate_cache_pattern(
    State(state): State<Arc<MonitoringState>>,
    Json(request): Json<InvalidateCacheRequest>,
) -> Result<Json<CacheOperationResponse>, AppError> {
    state
        .cache_service
        .invalidate_pattern(&request.pattern)
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to invalidate cache pattern: {e}")))?;

    Ok(Json(CacheOperationResponse {
        success: true,
        message: format!(
            "Cache pattern '{}' invalidated successfully",
            request.pattern
        ),
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvalidateCacheRequest {
    pub pattern: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheOperationResponse {
    pub success: bool,
    pub message: String,
}

/// Get database optimization report
pub async fn get_optimization_report(
    State(optimizer): State<Arc<DatabaseOptimizer>>,
) -> Result<Json<OptimizationResponse>, AppError> {
    let report = optimizer.generate_optimization_report().await?;
    Ok(Json(OptimizationResponse { report }))
}

/// Apply database optimization
pub async fn apply_optimization(
    State(optimizer): State<Arc<DatabaseOptimizer>>,
    Json(request): Json<ApplyOptimizationRequest>,
) -> Result<Json<OptimizationApplyResponse>, AppError> {
    let result = optimizer
        .apply_optimization(&request.optimization_type)
        .await?;

    Ok(Json(OptimizationApplyResponse {
        success: true,
        message: result,
        optimization_type: request.optimization_type,
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyOptimizationRequest {
    pub optimization_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationApplyResponse {
    pub success: bool,
    pub message: String,
    pub optimization_type: String,
}

impl std::fmt::Display for crate::services::HealthState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            crate::services::HealthState::Healthy => write!(f, "healthy"),
            crate::services::HealthState::Degraded => write!(f, "degraded"),
            crate::services::HealthState::Unhealthy => write!(f, "unhealthy"),
        }
    }
}
