use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Cache miss")]
    Miss,
    #[error("Expired")]
    Expired,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    data: String,
    expires_at: Instant,
}

impl CacheEntry {
    fn new(data: String, ttl: Duration) -> Self {
        Self {
            data,
            expires_at: Instant::now() + ttl,
        }
    }

    fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }
}

/// In-memory cache service implementation
/// In production, this could be replaced with Redis or other external cache
pub struct CacheService {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl CacheService {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get<T>(&self, key: &str) -> Result<Option<T>, CacheError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let cache = self.cache.read().unwrap();
        
        if let Some(entry) = cache.get(key) {
            if entry.is_expired() {
                // Entry is expired, treat as cache miss
                drop(cache);
                // Remove expired entry asynchronously
                self.remove_expired_entry(key).await;
                return Ok(None);
            }
            
            let deserialized: T = serde_json::from_str(&entry.data)?;
            Ok(Some(deserialized))
        } else {
            Ok(None)
        }
    }

    pub async fn set<T>(
        &self,
        key: &str,
        value: &T,
        ttl_seconds: u64,
    ) -> Result<(), CacheError>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string(value)?;
        let entry = CacheEntry::new(serialized, Duration::from_secs(ttl_seconds));
        
        let mut cache = self.cache.write().unwrap();
        cache.insert(key.to_string(), entry);
        
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<(), CacheError> {
        let mut cache = self.cache.write().unwrap();
        cache.remove(key);
        Ok(())
    }

    pub async fn invalidate_pattern(&self, pattern: &str) -> Result<(), CacheError> {
        let mut cache = self.cache.write().unwrap();
        let keys_to_remove: Vec<String> = cache
            .keys()
            .filter(|key| key.contains(pattern))
            .cloned()
            .collect();
        
        for key in keys_to_remove {
            cache.remove(&key);
        }
        
        Ok(())
    }

    pub async fn clear(&self) -> Result<(), CacheError> {
        let mut cache = self.cache.write().unwrap();
        cache.clear();
        Ok(())
    }

    pub async fn size(&self) -> usize {
        let cache = self.cache.read().unwrap();
        cache.len()
    }

    pub async fn cleanup_expired(&self) -> usize {
        let mut cache = self.cache.write().unwrap();
        let expired_keys: Vec<String> = cache
            .iter()
            .filter(|(_, entry)| entry.is_expired())
            .map(|(key, _)| key.clone())
            .collect();
        
        let count = expired_keys.len();
        for key in expired_keys {
            cache.remove(&key);
        }
        
        count
    }

    async fn remove_expired_entry(&self, key: &str) {
        let mut cache = self.cache.write().unwrap();
        if let Some(entry) = cache.get(key) {
            if entry.is_expired() {
                cache.remove(key);
            }
        }
    }
}

impl Default for CacheService {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache key management utilities
pub struct CacheKeys;

impl CacheKeys {
    pub fn document(id: i32) -> String {
        format!("document:{}", id)
    }

    pub fn document_search(hash: &str) -> String {
        format!("search:documents:{}", hash)
    }

    pub fn user_permissions(user_id: i32) -> String {
        format!("permissions:user:{}", user_id)
    }

    pub fn workflow(id: i32) -> String {
        format!("workflow:{}", id)
    }

    pub fn circulation(id: i32) -> String {
        format!("circulation:{}", id)
    }

    pub fn department(id: i32) -> String {
        format!("department:{}", id)
    }

    pub fn document_types() -> String {
        "document_types:all".to_string()
    }

    pub fn system_stats() -> String {
        "stats:system".to_string()
    }

    pub fn generate_search_hash(query: &str, filters: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        query.hash(&mut hasher);
        filters.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        id: i32,
        name: String,
    }

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let cache = CacheService::new();
        let test_data = TestData {
            id: 1,
            name: "Test".to_string(),
        };

        // Test set and get
        cache.set("test_key", &test_data, 60).await.unwrap();
        let retrieved: Option<TestData> = cache.get("test_key").await.unwrap();
        
        assert_eq!(retrieved, Some(test_data));

        // Test delete
        cache.delete("test_key").await.unwrap();
        let retrieved: Option<TestData> = cache.get("test_key").await.unwrap();
        
        assert_eq!(retrieved, None);
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache = CacheService::new();
        let test_data = TestData {
            id: 1,
            name: "Test".to_string(),
        };

        // Set with 0 second TTL (should expire immediately)
        cache.set("test_key", &test_data, 0).await.unwrap();
        
        // Wait a bit to ensure expiration
        tokio::time::sleep(Duration::from_millis(1)).await;
        
        let retrieved: Option<TestData> = cache.get("test_key").await.unwrap();
        assert_eq!(retrieved, None);
    }

    #[tokio::test]
    async fn test_cache_pattern_invalidation() {
        let cache = CacheService::new();
        let test_data = TestData {
            id: 1,
            name: "Test".to_string(),
        };

        cache.set("user:1:profile", &test_data, 60).await.unwrap();
        cache.set("user:2:profile", &test_data, 60).await.unwrap();
        cache.set("document:1", &test_data, 60).await.unwrap();

        cache.invalidate_pattern("user:").await.unwrap();

        let user1: Option<TestData> = cache.get("user:1:profile").await.unwrap();
        let user2: Option<TestData> = cache.get("user:2:profile").await.unwrap();
        let doc: Option<TestData> = cache.get("document:1").await.unwrap();

        assert_eq!(user1, None);
        assert_eq!(user2, None);
        assert_eq!(doc, Some(test_data));
    }

    #[test]
    fn test_cache_keys() {
        assert_eq!(CacheKeys::document(123), "document:123");
        assert_eq!(CacheKeys::user_permissions(456), "permissions:user:456");
        assert_eq!(CacheKeys::workflow(789), "workflow:789");
        
        let hash1 = CacheKeys::generate_search_hash("title", "filters");
        let hash2 = CacheKeys::generate_search_hash("title", "filters");
        let hash3 = CacheKeys::generate_search_hash("different", "filters");
        
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}