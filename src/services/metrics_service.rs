use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::time::interval;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: u64,
    pub memory_usage: MemoryUsage,
    pub performance: PerformanceMetrics,
    pub database: DatabaseMetrics,
    pub cache: CacheMetrics,
    pub api: ApiMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsage {
    pub heap_used: u64,
    pub heap_total: u64,
    pub rss: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub requests_per_second: f64,
    pub average_response_time_ms: f64,
    pub error_rate: f64,
    pub active_connections: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMetrics {
    pub total_queries: u64,
    pub slow_queries: u64,
    pub average_query_time_ms: f64,
    pub connection_pool_size: u32,
    pub active_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetrics {
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub size: usize,
    pub memory_usage_mb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub endpoints: HashMap<String, EndpointMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointMetrics {
    pub requests: u64,
    pub average_response_time_ms: f64,
    pub success_rate: f64,
    pub last_accessed: u64,
}

#[derive(Debug, Clone)]
struct RequestMetric {
    endpoint: String,
    start_time: Instant,
    success: bool,
}

#[derive(Debug)]
struct MetricsData {
    request_metrics: Vec<RequestMetric>,
    database_queries: Vec<QueryMetric>,
    cache_operations: Vec<CacheOperation>,
    start_time: Instant,
}

#[derive(Debug, Clone)]
struct QueryMetric {
    duration: Duration,
    query_type: String,
    success: bool,
}

#[derive(Debug, Clone)]
struct CacheOperation {
    operation: String,
    hit: bool,
}

pub struct MetricsService {
    data: Arc<RwLock<MetricsData>>,
    collection_interval: Duration,
}

impl MetricsService {
    pub fn new(collection_interval_seconds: u64) -> Self {
        Self {
            data: Arc::new(RwLock::new(MetricsData {
                request_metrics: Vec::new(),
                database_queries: Vec::new(),
                cache_operations: Vec::new(),
                start_time: Instant::now(),
            })),
            collection_interval: Duration::from_secs(collection_interval_seconds),
        }
    }

    pub async fn start_collection(&self) {
        let data = Arc::clone(&self.data);
        let interval_duration = self.collection_interval;
        
        tokio::spawn(async move {
            let mut interval = interval(interval_duration);
            
            loop {
                interval.tick().await;
                Self::cleanup_old_metrics(&data).await;
            }
        });
    }

    pub fn record_request(&self, endpoint: &str, start_time: Instant, success: bool) {
        let mut data = self.data.write().unwrap();
        data.request_metrics.push(RequestMetric {
            endpoint: endpoint.to_string(),
            start_time,
            success,
        });
    }

    pub fn record_database_query(&self, query_type: &str, duration: Duration, success: bool) {
        let mut data = self.data.write().unwrap();
        data.database_queries.push(QueryMetric {
            duration,
            query_type: query_type.to_string(),
            success,
        });
    }

    pub fn record_cache_operation(&self, operation: &str, hit: bool) {
        let mut data = self.data.write().unwrap();
        data.cache_operations.push(CacheOperation {
            operation: operation.to_string(),
            hit,
        });
    }

    pub fn get_metrics(&self) -> SystemMetrics {
        let data = self.data.read().unwrap();
        let now = Instant::now();
        let window = Duration::from_secs(300); // 5 minute window
        let cutoff = now - window;

        // Filter recent metrics
        let recent_requests: Vec<_> = data
            .request_metrics
            .iter()
            .filter(|r| r.start_time > cutoff)
            .collect();

        let recent_queries: Vec<_> = data
            .database_queries
            .iter()
            .filter(|q| now - q.duration > cutoff)
            .collect();

        let recent_cache_ops: Vec<_> = data
            .cache_operations
            .iter()
            .collect();

        // Calculate performance metrics
        let performance = self.calculate_performance_metrics(&recent_requests, window);
        let database = self.calculate_database_metrics(&recent_queries);
        let cache = self.calculate_cache_metrics(&recent_cache_ops);
        let api = self.calculate_api_metrics(&recent_requests);

        SystemMetrics {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            memory_usage: self.get_memory_usage(),
            performance,
            database,
            cache,
            api,
        }
    }

    fn calculate_performance_metrics(
        &self,
        requests: &[&RequestMetric],
        window: Duration,
    ) -> PerformanceMetrics {
        let total_requests = requests.len() as u64;
        let successful_requests = requests.iter().filter(|r| r.success).count() as u64;
        let failed_requests = total_requests - successful_requests;

        let requests_per_second = total_requests as f64 / window.as_secs_f64();
        let error_rate = if total_requests > 0 {
            failed_requests as f64 / total_requests as f64
        } else {
            0.0
        };

        let average_response_time_ms = if !requests.is_empty() {
            let total_time: Duration = requests
                .iter()
                .map(|r| Instant::now().duration_since(r.start_time))
                .sum();
            total_time.as_millis() as f64 / requests.len() as f64
        } else {
            0.0
        };

        PerformanceMetrics {
            requests_per_second,
            average_response_time_ms,
            error_rate,
            active_connections: 0, // This would need to be tracked separately
        }
    }

    fn calculate_database_metrics(&self, queries: &[&QueryMetric]) -> DatabaseMetrics {
        let total_queries = queries.len() as u64;
        let slow_queries = queries
            .iter()
            .filter(|q| q.duration > Duration::from_millis(1000))
            .count() as u64;

        let average_query_time_ms = if !queries.is_empty() {
            let total_time: Duration = queries.iter().map(|q| q.duration).sum();
            total_time.as_millis() as f64 / queries.len() as f64
        } else {
            0.0
        };

        DatabaseMetrics {
            total_queries,
            slow_queries,
            average_query_time_ms,
            connection_pool_size: 10, // This should come from actual pool
            active_connections: 1,    // This should come from actual pool
        }
    }

    fn calculate_cache_metrics(&self, operations: &[&CacheOperation]) -> CacheMetrics {
        let total_ops = operations.len() as u64;
        let hits = operations.iter().filter(|op| op.hit).count() as u64;
        let misses = total_ops - hits;

        let hit_rate = if total_ops > 0 {
            hits as f64 / total_ops as f64
        } else {
            0.0
        };

        CacheMetrics {
            hits,
            misses,
            hit_rate,
            size: 0,    // This should come from actual cache
            memory_usage_mb: 0.0, // This should come from actual cache
        }
    }

    fn calculate_api_metrics(&self, requests: &[&RequestMetric]) -> ApiMetrics {
        let total_requests = requests.len() as u64;
        let successful_requests = requests.iter().filter(|r| r.success).count() as u64;
        let failed_requests = total_requests - successful_requests;

        let mut endpoints: HashMap<String, Vec<&RequestMetric>> = HashMap::new();
        for request in requests {
            endpoints
                .entry(request.endpoint.clone())
                .or_default()
                .push(request);
        }

        let endpoint_metrics: HashMap<String, EndpointMetrics> = endpoints
            .into_iter()
            .map(|(endpoint, reqs)| {
                let total = reqs.len() as u64;
                let successful = reqs.iter().filter(|r| r.success).count() as u64;
                let success_rate = if total > 0 {
                    successful as f64 / total as f64
                } else {
                    0.0
                };

                let avg_response_time = if !reqs.is_empty() {
                    let total_time: Duration = reqs
                        .iter()
                        .map(|r| Instant::now().duration_since(r.start_time))
                        .sum();
                    total_time.as_millis() as f64 / reqs.len() as f64
                } else {
                    0.0
                };

                let last_accessed = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                (
                    endpoint,
                    EndpointMetrics {
                        requests: total,
                        average_response_time_ms: avg_response_time,
                        success_rate,
                        last_accessed,
                    },
                )
            })
            .collect();

        ApiMetrics {
            total_requests,
            successful_requests,
            failed_requests,
            endpoints: endpoint_metrics,
        }
    }

    fn get_memory_usage(&self) -> MemoryUsage {
        // In a real implementation, this would use system APIs
        // For now, return placeholder values
        MemoryUsage {
            heap_used: 50 * 1024 * 1024,  // 50MB
            heap_total: 100 * 1024 * 1024, // 100MB
            rss: 120 * 1024 * 1024,       // 120MB
        }
    }

    async fn cleanup_old_metrics(data: &Arc<RwLock<MetricsData>>) {
        let mut data = data.write().unwrap();
        let now = Instant::now();
        let cutoff = now - Duration::from_secs(3600); // Keep 1 hour of data

        data.request_metrics.retain(|r| r.start_time > cutoff);
        // For queries and cache ops, we don't have timestamps, so just limit the size
        if data.database_queries.len() > 10000 {
            data.database_queries.drain(0..5000);
        }
        if data.cache_operations.len() > 10000 {
            data.cache_operations.drain(0..5000);
        }
    }

    pub fn get_health_status(&self) -> HealthStatus {
        let metrics = self.get_metrics();
        
        let mut status = HealthStatus {
            overall: HealthState::Healthy,
            database: HealthState::Healthy,
            cache: HealthState::Healthy,
            api: HealthState::Healthy,
            details: HashMap::new(),
        };

        // Check API health
        if metrics.performance.error_rate > 0.1 {
            status.api = HealthState::Degraded;
            status.details.insert(
                "api".to_string(),
                format!("High error rate: {:.1}%", metrics.performance.error_rate * 100.0),
            );
        }

        if metrics.performance.average_response_time_ms > 5000.0 {
            status.api = HealthState::Unhealthy;
            status.details.insert(
                "api".to_string(),
                format!("High response time: {:.1}ms", metrics.performance.average_response_time_ms),
            );
        }

        // Check database health
        if metrics.database.average_query_time_ms > 1000.0 {
            status.database = HealthState::Degraded;
            status.details.insert(
                "database".to_string(),
                format!("Slow queries: {:.1}ms avg", metrics.database.average_query_time_ms),
            );
        }

        // Check cache health
        if metrics.cache.hit_rate < 0.5 {
            status.cache = HealthState::Degraded;
            status.details.insert(
                "cache".to_string(),
                format!("Low hit rate: {:.1}%", metrics.cache.hit_rate * 100.0),
            );
        }

        // Determine overall health
        if status.api == HealthState::Unhealthy || status.database == HealthState::Unhealthy {
            status.overall = HealthState::Unhealthy;
        } else if status.api == HealthState::Degraded 
            || status.database == HealthState::Degraded 
            || status.cache == HealthState::Degraded {
            status.overall = HealthState::Degraded;
        }

        status
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub overall: HealthState,
    pub database: HealthState,
    pub cache: HealthState,
    pub api: HealthState,
    pub details: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthState {
    Healthy,
    Degraded,
    Unhealthy,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_service() {
        let service = MetricsService::new(60);
        
        // Record some metrics
        service.record_request("/api/documents", Instant::now(), true);
        service.record_request("/api/users", Instant::now(), false);
        service.record_database_query("SELECT", Duration::from_millis(50), true);
        service.record_cache_operation("get", true);
        service.record_cache_operation("get", false);

        let metrics = service.get_metrics();
        
        assert!(metrics.api.total_requests >= 2);
        assert!(metrics.database.total_queries >= 1);
        assert!(metrics.cache.hits >= 1);
        assert!(metrics.cache.misses >= 1);
    }

    #[test]
    fn test_health_status() {
        let service = MetricsService::new(60);
        let health = service.get_health_status();
        
        assert_eq!(health.overall, HealthState::Healthy);
        assert_eq!(health.api, HealthState::Healthy);
        assert_eq!(health.database, HealthState::Healthy);
        assert_eq!(health.cache, HealthState::Healthy);
    }
}