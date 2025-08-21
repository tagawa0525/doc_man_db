use sqlx::{Pool, Sqlite, Row};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPerformance {
    pub query_hash: String,
    pub query_type: String,
    pub average_duration_ms: f64,
    pub execution_count: u64,
    pub total_time_ms: f64,
    pub max_duration_ms: f64,
    pub min_duration_ms: f64,
    pub last_executed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseOptimizationReport {
    pub slow_queries: Vec<QueryPerformance>,
    pub index_recommendations: Vec<IndexRecommendation>,
    pub table_statistics: Vec<TableStatistic>,
    pub connection_pool_stats: ConnectionPoolStats,
    pub optimization_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexRecommendation {
    pub table_name: String,
    pub column_names: Vec<String>,
    pub reason: String,
    pub estimated_impact: String,
    pub create_statement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableStatistic {
    pub table_name: String,
    pub row_count: i64,
    pub size_mb: f64,
    pub last_updated: Option<String>,
    pub fragmentation_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolStats {
    pub total_connections: u32,
    pub active_connections: u32,
    pub idle_connections: u32,
    pub max_connections: u32,
    pub average_acquire_time_ms: f64,
    pub pool_efficiency: f64,
}

pub struct DatabaseOptimizer {
    pool: Pool<Sqlite>,
    query_performance: HashMap<String, QueryMetrics>,
}

#[derive(Debug, Clone)]
struct QueryMetrics {
    executions: Vec<Duration>,
    query_type: String,
    first_seen: Instant,
    last_executed: Instant,
}

impl DatabaseOptimizer {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            query_performance: HashMap::new(),
        }
    }

    pub fn record_query(&mut self, query: &str, duration: Duration) {
        let query_hash = self.hash_query(query);
        let query_type = self.determine_query_type(query);

        self.query_performance
            .entry(query_hash)
            .and_modify(|metrics| {
                metrics.executions.push(duration);
                metrics.last_executed = Instant::now();
                // Keep only last 1000 executions to prevent memory growth
                if metrics.executions.len() > 1000 {
                    metrics.executions.drain(0..500);
                }
            })
            .or_insert_with(|| QueryMetrics {
                executions: vec![duration],
                query_type,
                first_seen: Instant::now(),
                last_executed: Instant::now(),
            });
    }

    pub async fn generate_optimization_report(&self) -> Result<DatabaseOptimizationReport, AppError> {
        let slow_queries = self.identify_slow_queries();
        let index_recommendations = self.generate_index_recommendations().await?;
        let table_statistics = self.collect_table_statistics().await?;
        let connection_pool_stats = self.get_connection_pool_stats();
        let optimization_suggestions = self.generate_optimization_suggestions(&slow_queries, &table_statistics);

        Ok(DatabaseOptimizationReport {
            slow_queries,
            index_recommendations,
            table_statistics,
            connection_pool_stats,
            optimization_suggestions,
        })
    }

    fn identify_slow_queries(&self) -> Vec<QueryPerformance> {
        let mut slow_queries: Vec<QueryPerformance> = self
            .query_performance
            .iter()
            .filter_map(|(hash, metrics)| {
                if metrics.executions.is_empty() {
                    return None;
                }

                let total_time: Duration = metrics.executions.iter().sum();
                let average_duration = total_time / metrics.executions.len() as u32;
                
                // Consider queries slow if they take more than 100ms on average
                if average_duration > Duration::from_millis(100) {
                    let min_duration = metrics.executions.iter().min().unwrap_or(&Duration::ZERO);
                    let max_duration = metrics.executions.iter().max().unwrap_or(&Duration::ZERO);

                    Some(QueryPerformance {
                        query_hash: hash.clone(),
                        query_type: metrics.query_type.clone(),
                        average_duration_ms: average_duration.as_millis() as f64,
                        execution_count: metrics.executions.len() as u64,
                        total_time_ms: total_time.as_millis() as f64,
                        max_duration_ms: max_duration.as_millis() as f64,
                        min_duration_ms: min_duration.as_millis() as f64,
                        last_executed: format!("{:?}", metrics.last_executed),
                    })
                } else {
                    None
                }
            })
            .collect();

        // Sort by total time (impact)
        slow_queries.sort_by(|a, b| b.total_time_ms.partial_cmp(&a.total_time_ms).unwrap_or(std::cmp::Ordering::Equal));
        slow_queries.truncate(20); // Return top 20 slow queries
        slow_queries
    }

    async fn generate_index_recommendations(&self) -> Result<Vec<IndexRecommendation>, AppError> {
        let mut recommendations = Vec::new();

        // Check for missing indexes on frequently queried columns
        let tables_to_analyze = vec!["documents", "document_types", "circulations", "circulation_steps"];

        for table in tables_to_analyze {
            let indexes = self.analyze_table_indexes(table).await?;
            recommendations.extend(indexes);
        }

        Ok(recommendations)
    }

    async fn analyze_table_indexes(&self, table_name: &str) -> Result<Vec<IndexRecommendation>, AppError> {
        let mut recommendations = Vec::new();

        match table_name {
            "documents" => {
                // Check if we have index on title for search queries
                let has_title_index = self.check_index_exists(table_name, &["title"]).await?;
                if !has_title_index {
                    recommendations.push(IndexRecommendation {
                        table_name: table_name.to_string(),
                        column_names: vec!["title".to_string()],
                        reason: "Frequent text searches on title column".to_string(),
                        estimated_impact: "High - will improve search performance by 50-80%".to_string(),
                        create_statement: format!("CREATE INDEX idx_{}_title ON {} (title)", table_name, table_name),
                    });
                }

                // Check for compound index on document_type_id + created_date
                let has_compound_index = self.check_index_exists(table_name, &["document_type_id", "created_date"]).await?;
                if !has_compound_index {
                    recommendations.push(IndexRecommendation {
                        table_name: table_name.to_string(),
                        column_names: vec!["document_type_id".to_string(), "created_date".to_string()],
                        reason: "Frequent filtering by document type and date range".to_string(),
                        estimated_impact: "Medium - will improve filtered searches by 30-50%".to_string(),
                        create_statement: format!("CREATE INDEX idx_{}_type_date ON {} (document_type_id, created_date)", table_name, table_name),
                    });
                }
            }
            "circulations" => {
                let has_status_index = self.check_index_exists(table_name, &["status"]).await?;
                if !has_status_index {
                    recommendations.push(IndexRecommendation {
                        table_name: table_name.to_string(),
                        column_names: vec!["status".to_string()],
                        reason: "Frequent filtering by circulation status".to_string(),
                        estimated_impact: "Medium - will improve status-based queries by 40-60%".to_string(),
                        create_statement: format!("CREATE INDEX idx_{}_status ON {} (status)", table_name, table_name),
                    });
                }
            }
            "circulation_steps" => {
                let has_assignee_status_index = self.check_index_exists(table_name, &["assignee_id", "status"]).await?;
                if !has_assignee_status_index {
                    recommendations.push(IndexRecommendation {
                        table_name: table_name.to_string(),
                        column_names: vec!["assignee_id".to_string(), "status".to_string()],
                        reason: "Frequent queries for pending steps by user".to_string(),
                        estimated_impact: "High - will improve user task lists by 60-80%".to_string(),
                        create_statement: format!("CREATE INDEX idx_{}_assignee_status ON {} (assignee_id, status)", table_name, table_name),
                    });
                }
            }
            _ => {}
        }

        Ok(recommendations)
    }

    async fn check_index_exists(&self, table_name: &str, columns: &[&str]) -> Result<bool, AppError> {
        let query = "SELECT name FROM sqlite_master WHERE type='index' AND tbl_name=? AND sql LIKE ?";
        let column_pattern = format!("%{}%", columns.join("%"));
        
        let row = sqlx::query(query)
            .bind(table_name)
            .bind(column_pattern)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.is_some())
    }

    async fn collect_table_statistics(&self) -> Result<Vec<TableStatistic>, AppError> {
        let mut statistics = Vec::new();

        let tables_query = "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'";
        let rows = sqlx::query(tables_query).fetch_all(&self.pool).await?;

        for row in rows {
            let table_name: String = row.try_get("name")?;
            
            // Get row count
            let count_query = format!("SELECT COUNT(*) as count FROM {}", table_name);
            let count_row = sqlx::query(&count_query).fetch_one(&self.pool).await?;
            let row_count: i64 = count_row.try_get("count")?;

            // Estimate size (SQLite doesn't provide direct size info easily)
            let size_mb = (row_count as f64 * 1.0) / 1024.0; // Rough estimate

            statistics.push(TableStatistic {
                table_name,
                row_count,
                size_mb,
                last_updated: None, // SQLite doesn't track this easily
                fragmentation_level: 0.0, // Would need VACUUM analysis
            });
        }

        Ok(statistics)
    }

    fn get_connection_pool_stats(&self) -> ConnectionPoolStats {
        // In a real implementation, this would access actual pool statistics
        // For now, return estimated values
        ConnectionPoolStats {
            total_connections: 10,
            active_connections: 2,
            idle_connections: 8,
            max_connections: 10,
            average_acquire_time_ms: 5.0,
            pool_efficiency: 0.8,
        }
    }

    fn generate_optimization_suggestions(
        &self,
        slow_queries: &[QueryPerformance],
        table_statistics: &[TableStatistic],
    ) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Check for slow queries
        if !slow_queries.is_empty() {
            suggestions.push(format!(
                "Found {} slow queries. Consider adding indexes or optimizing query structure.",
                slow_queries.len()
            ));
        }

        // Check for large tables without pagination
        for stat in table_statistics {
            if stat.row_count > 10000 {
                suggestions.push(format!(
                    "Table '{}' has {} rows. Ensure queries use LIMIT/OFFSET for pagination.",
                    stat.table_name, stat.row_count
                ));
            }
        }

        // Generic optimizations
        suggestions.push("Consider running VACUUM periodically to reclaim space and improve performance.".to_string());
        suggestions.push("Use prepared statements to improve query compilation performance.".to_string());
        suggestions.push("Consider implementing read replicas for heavy read workloads.".to_string());

        suggestions
    }

    fn hash_query(&self, query: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Normalize query by removing whitespace and converting to lowercase
        let normalized = query
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();

        let mut hasher = DefaultHasher::new();
        normalized.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn determine_query_type(&self, query: &str) -> String {
        let query_lower = query.to_lowercase();
        
        if query_lower.starts_with("select") {
            "SELECT"
        } else if query_lower.starts_with("insert") {
            "INSERT"
        } else if query_lower.starts_with("update") {
            "UPDATE"
        } else if query_lower.starts_with("delete") {
            "DELETE"
        } else if query_lower.starts_with("create") {
            "CREATE"
        } else if query_lower.starts_with("drop") {
            "DROP"
        } else {
            "OTHER"
        }.to_string()
    }

    pub async fn apply_optimization(&self, optimization_type: &str) -> Result<String, AppError> {
        match optimization_type {
            "vacuum" => {
                sqlx::query("VACUUM").execute(&self.pool).await?;
                Ok("Database vacuum completed successfully".to_string())
            }
            "analyze" => {
                sqlx::query("ANALYZE").execute(&self.pool).await?;
                Ok("Database statistics updated successfully".to_string())
            }
            "reindex" => {
                sqlx::query("REINDEX").execute(&self.pool).await?;
                Ok("Database indexes rebuilt successfully".to_string())
            }
            _ => Err(AppError::BadRequest(format!("Unknown optimization type: {}", optimization_type))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_type_detection() {
        // Create a dummy pool for testing
        use sqlx::sqlite::SqlitePoolOptions;
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_lazy("sqlite::memory:")
            .unwrap();
        let optimizer = DatabaseOptimizer::new(pool);

        assert_eq!(optimizer.determine_query_type("SELECT * FROM users"), "SELECT");
        assert_eq!(optimizer.determine_query_type("INSERT INTO users VALUES (1, 'test')"), "INSERT");
        assert_eq!(optimizer.determine_query_type("UPDATE users SET name = 'test'"), "UPDATE");
        assert_eq!(optimizer.determine_query_type("DELETE FROM users WHERE id = 1"), "DELETE");
    }

    #[test]
    fn test_query_hashing() {
        use sqlx::sqlite::SqlitePoolOptions;
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_lazy("sqlite::memory:")
            .unwrap();
        let optimizer = DatabaseOptimizer::new(pool);

        let hash1 = optimizer.hash_query("SELECT * FROM users WHERE id = ?");
        let hash2 = optimizer.hash_query("SELECT   *   FROM   users   WHERE   id   =   ?");
        let hash3 = optimizer.hash_query("SELECT * FROM products WHERE id = ?");

        assert_eq!(hash1, hash2); // Same query, different formatting
        assert_ne!(hash1, hash3); // Different queries
    }
}