//! Async resource cache service with TTL support
//!
//! This module provides a thread-safe, async cache for resources with
//! configurable time-to-live (TTL) for automatic expiration.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use avalonia_mcp_core::error::AvaloniaMcpError;

/// Cache entry with TTL support
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    pub value: T,
    pub expires_at: Option<Instant>,
    pub created_at: Instant,
    pub access_count: u64,
}

impl<T> CacheEntry<T> {
    pub fn new(value: T, ttl: Option<Duration>) -> Self {
        let now = Instant::now();
        Self {
            value,
            expires_at: ttl.map(|d| now + d),
            created_at: now,
            access_count: 0,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.map_or(false, |exp| Instant::now() > exp)
    }

    pub fn record_access(&mut self) {
        self.access_count += 1;
    }
}

/// Async resource cache with backpressure support
pub struct ResourceCacheService {
    cache: Arc<RwLock<HashMap<String, CacheEntry<serde_json::Value>>>>,
    default_ttl: Duration,
    max_entries: Option<usize>,
}

impl ResourceCacheService {
    /// Create a new cache service with default TTL
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            default_ttl,
            max_entries: None,
        }
    }

    /// Create a new cache service with max entries limit
    pub fn with_max_entries(mut self, max: usize) -> Self {
        self.max_entries = Some(max);
        self
    }

    /// Get value from cache (read lock, shared access)
    pub async fn get(&self, key: &str) -> Option<serde_json::Value> {
        let mut cache = self.cache.write().await;

        match cache.get_mut(key) {
            Some(entry) if !entry.is_expired() => {
                entry.record_access();
                debug!(key, "Cache hit");
                Some(entry.value.clone())
            }
            Some(_entry) => {
                debug!(key, "Cache entry expired");
                cache.remove(key);
                None
            }
            None => {
                debug!(key, "Cache miss");
                None
            }
        }
    }

    /// Set value in cache with default TTL (write lock, exclusive access)
    pub async fn set(
        &self,
        key: impl Into<String>,
        value: serde_json::Value,
    ) -> Result<(), AvaloniaMcpError> {
        self.set_with_ttl(key, value, self.default_ttl).await
    }

    /// Set value in cache with custom TTL
    pub async fn set_with_ttl(
        &self,
        key: impl Into<String>,
        value: serde_json::Value,
        ttl: Duration,
    ) -> Result<(), AvaloniaMcpError> {
        let key = key.into();
        let mut cache = self.cache.write().await;

        // Check max entries limit
        if let Some(max) = self.max_entries {
            if !cache.contains_key(&key) && cache.len() >= max {
                // Evict oldest or least accessed entry
                self.evict_one(&mut cache).await;
            }
        }

        cache.insert(key.clone(), CacheEntry::new(value, Some(ttl)));
        debug!(key, ttl = ?ttl, "Cache entry set");
        Ok(())
    }

    /// Remove a key from cache
    pub async fn remove(&self, key: &str) -> Option<serde_json::Value> {
        let mut cache = self.cache.write().await;
        cache.remove(key).map(|entry| entry.value)
    }

    /// Check if key exists in cache (without updating access count)
    pub async fn contains_key(&self, key: &str) -> bool {
        let cache = self.cache.read().await;
        cache.get(key).map_or(false, |entry| !entry.is_expired())
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        let now = Instant::now();

        let total_entries = cache.len();
        let expired_entries = cache
            .values()
            .filter(|entry| entry.is_expired())
            .count();
        let total_access_count: u64 = cache.values().map(|e| e.access_count).sum();
        let oldest_entry = cache
            .values()
            .map(|e| e.created_at)
            .min()
            .map(|t| now.duration_since(t));
        let newest_entry = cache
            .values()
            .map(|e| e.created_at)
            .max()
            .map(|t| now.duration_since(t));

        CacheStats {
            total_entries,
            expired_entries,
            total_access_count,
            oldest_entry,
            newest_entry,
        }
    }

    /// Clear all entries from cache
    pub async fn clear(&self) -> usize {
        let mut cache = self.cache.write().await;
        let count = cache.len();
        cache.clear();
        info!(count, "Cache cleared");
        count
    }

    /// Remove expired entries
    pub async fn remove_expired(&self) -> Vec<String> {
        let mut cache = self.cache.write().await;
        let expired_keys: Vec<String> = cache
            .iter()
            .filter(|(_, entry)| entry.is_expired())
            .map(|(key, _)| key.clone())
            .collect();

        for key in &expired_keys {
            cache.remove(key);
        }

        if !expired_keys.is_empty() {
            info!(count = expired_keys.len(), "Removed expired cache entries");
        }

        expired_keys
    }

    /// Preload common resources (to be implemented with actual resource loading)
    pub async fn preload_common_resources(&self) -> Result<(), AvaloniaMcpError> {
        info!("Preloading common resources into cache");
        // This will be implemented to load controls.json, xaml-patterns.json, etc.
        // For now, it's a placeholder
        Ok(())
    }

    /// Evict one entry (oldest or least accessed)
    async fn evict_one(&self, cache: &mut HashMap<String, CacheEntry<serde_json::Value>>) {
        if let Some((key, _)) = cache
            .iter()
            .min_by_key(|(_, entry)| (entry.access_count, entry.created_at))
        {
            let key = key.clone();
            cache.remove(&key);
            warn!(key, "Evicted cache entry due to max entries limit");
        }
    }
}

impl Clone for ResourceCacheService {
    fn clone(&self) -> Self {
        Self {
            cache: Arc::clone(&self.cache),
            default_ttl: self.default_ttl,
            max_entries: self.max_entries,
        }
    }
}

impl Default for ResourceCacheService {
    fn default() -> Self {
        Self::new(Duration::from_secs(300)) // 5 minutes default TTL
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub expired_entries: usize,
    pub total_access_count: u64,
    pub oldest_entry: Option<Duration>,
    pub newest_entry: Option<Duration>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_set_get() {
        let cache = ResourceCacheService::new(Duration::from_secs(60));

        cache.set("key1", serde_json::json!("value1")).await.unwrap();

        let result = cache.get("key1").await;
        assert_eq!(result, Some(serde_json::json!("value1")));
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache = ResourceCacheService::new(Duration::from_millis(100));

        cache.set("key2", serde_json::json!("value2")).await.unwrap();

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(150)).await;

        let result = cache.get("key2").await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_cache_remove() {
        let cache = ResourceCacheService::new(Duration::from_secs(60));

        cache.set("key3", serde_json::json!("value3")).await.unwrap();
        assert!(cache.contains_key("key3").await);

        let removed = cache.remove("key3").await;
        assert!(removed.is_some());
        assert!(!cache.contains_key("key3").await);
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let cache = ResourceCacheService::new(Duration::from_secs(60));

        cache.set("key1", serde_json::json!("value1")).await.unwrap();
        cache.set("key2", serde_json::json!("value2")).await.unwrap();

        let count = cache.clear().await;
        assert_eq!(count, 2);
        assert!(!cache.contains_key("key1").await);
        assert!(!cache.contains_key("key2").await);
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let cache = ResourceCacheService::new(Duration::from_secs(60));

        cache.set("key1", serde_json::json!("value1")).await.unwrap();
        cache.get("key1").await; // Access once

        let stats = cache.stats().await;
        assert_eq!(stats.total_entries, 1);
        assert_eq!(stats.total_access_count, 1);
    }

    #[tokio::test]
    async fn test_cache_concurrent_access() {
        let cache = Arc::new(ResourceCacheService::new(Duration::from_secs(60)));

        // Spawn multiple tasks to test concurrent access
        let mut handles = Vec::new();
        for i in 0..10 {
            let cache_clone = Arc::clone(&cache);
            let handle = tokio::spawn(async move {
                cache_clone
                    .set(format!("key{}", i), serde_json::json!(i))
                    .await
                    .unwrap();
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap();
        }

        // Verify all keys were set
        for i in 0..10 {
            let result = cache.get(&format!("key{}", i)).await;
            assert_eq!(result, Some(serde_json::json!(i)));
        }
    }

    #[tokio::test]
    async fn test_cache_remove_expired() {
        let cache = ResourceCacheService::new(Duration::from_millis(50));

        cache.set("key1", serde_json::json!("value1")).await.unwrap();
        cache.set("key2", serde_json::json!("value2")).await.unwrap();

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(100)).await;

        let expired = cache.remove_expired().await;
        assert_eq!(expired.len(), 2);
        assert!(expired.contains(&"key1".to_string()));
        assert!(expired.contains(&"key2".to_string()));
    }
}
