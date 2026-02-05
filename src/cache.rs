use crate::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, instrument};

/// Cache entry with expiration time.
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    pub value: T,
    pub expires_at: Instant,
    pub created_at: Instant,
}

impl<T> CacheEntry<T> {
    pub fn new(value: T, ttl: Duration) -> Self {
        let now = Instant::now();
        Self {
            value,
            expires_at: now + ttl,
            created_at: now,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }
    
    pub fn age(&self) -> Duration {
        Instant::now().duration_since(self.created_at)
    }
}

/// In-memory cache with TTL and size limits.
#[derive(Debug)]
pub struct MemoryCache<T> {
    store: Arc<RwLock<HashMap<String, CacheEntry<T>>>>,
    default_ttl: Duration,
    max_entries: usize,
}

impl<T: Clone + Send + Sync + 'static> MemoryCache<T> {
    pub fn new(default_ttl: Duration, max_entries: usize) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            default_ttl,
            max_entries,
        }
    }

    /// Get a value from the cache.
    #[instrument(skip(self), fields(key = %key))]
    pub async fn get(&self, key: &str) -> Option<T> {
        let mut store = self.store.write().await;
        
        if let Some(entry) = store.get(key) {
            if entry.is_expired() {
                debug!("Cache entry expired, removing");
                store.remove(key);
                None
            } else {
                debug!(age_ms = %entry.age().as_millis(), "Cache hit");
                Some(entry.value.clone())
            }
        } else {
            debug!("Cache miss");
            None
        }
    }

    /// Set a value in the cache with default TTL.
    #[instrument(skip(self, value), fields(key = %key))]
    pub async fn set(&self, key: String, value: T) {
        self.set_with_ttl(key, value, self.default_ttl).await;
    }

    /// Set a value in the cache with custom TTL.
    #[instrument(skip(self, value), fields(key = %key, ttl_secs = %ttl.as_secs()))]
    pub async fn set_with_ttl(&self, key: String, value: T, ttl: Duration) {
        let mut store = self.store.write().await;
        
        // Evict expired entries if we're at capacity
        if store.len() >= self.max_entries {
            self.evict_expired_entries(&mut store).await;
            
            // If still at capacity, remove oldest entry
            if store.len() >= self.max_entries {
                if let Some(oldest_key) = self.find_oldest_entry(&store).await {
                    debug!(evicted_key = %oldest_key, "Evicting oldest cache entry");
                    store.remove(&oldest_key);
                }
            }
        }
        
        let entry = CacheEntry::new(value, ttl);
        store.insert(key, entry);
        debug!("Value cached successfully");
    }

    /// Remove a value from the cache.
    #[instrument(skip(self), fields(key = %key))]
    pub async fn remove(&self, key: &str) -> Option<T> {
        let mut store = self.store.write().await;
        store.remove(key).map(|entry| {
            debug!("Cache entry removed");
            entry.value
        })
    }

    /// Clear all entries from the cache.
    #[instrument(skip(self))]
    pub async fn clear(&self) {
        let mut store = self.store.write().await;
        let count = store.len();
        store.clear();
        debug!(cleared_entries = %count, "Cache cleared");
    }

    /// Get cache statistics.
    pub async fn stats(&self) -> CacheStats {
        let store = self.store.read().await;
        let total_entries = store.len();
        let expired_entries = store.values().filter(|entry| entry.is_expired()).count();
        
        CacheStats {
            total_entries,
            expired_entries,
            active_entries: total_entries - expired_entries,
            max_entries: self.max_entries,
        }
    }

    /// Remove all expired entries from the cache.
    async fn evict_expired_entries(&self, store: &mut HashMap<String, CacheEntry<T>>) {
        let expired_keys: Vec<String> = store
            .iter()
            .filter_map(|(key, entry)| {
                if entry.is_expired() {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .collect();
        
        let expired_count = expired_keys.len();
        
        for key in expired_keys {
            store.remove(&key);
        }
        
        if expired_count > 0 {
            debug!(expired_count = %expired_count, "Evicted expired cache entries");
        }
    }

    /// Find the oldest entry in the cache for LRU eviction.
    async fn find_oldest_entry(&self, store: &HashMap<String, CacheEntry<T>>) -> Option<String> {
        store
            .iter()
            .min_by_key(|(_, entry)| entry.created_at)
            .map(|(key, _)| key.clone())
    }
}

/// Cache statistics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub expired_entries: usize,
    pub active_entries: usize,
    pub max_entries: usize,
}

/// Cache configuration for different endpoint types.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// TTL for balance data (relatively static)
    pub balance_ttl: Duration,
    /// TTL for transaction data (immutable once confirmed)
    pub transaction_ttl: Duration,
    /// TTL for NFT metadata (mostly static)
    pub nft_metadata_ttl: Duration,
    /// TTL for NFT collections (mostly static)
    pub nft_collection_ttl: Duration,
    /// Maximum number of cached entries
    pub max_entries: usize,
    /// Enable caching
    pub enabled: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            balance_ttl: Duration::from_secs(30),      // 30 seconds for balances
            transaction_ttl: Duration::from_secs(300), // 5 minutes for transactions
            nft_metadata_ttl: Duration::from_secs(3600), // 1 hour for NFT metadata
            nft_collection_ttl: Duration::from_secs(3600), // 1 hour for NFT collections
            max_entries: 1000,
            enabled: true,
        }
    }
}

/// Generate cache keys for different types of requests.
pub fn cache_key_for_balances(chain_name: &str, address: &str, options: &str) -> String {
    format!("balances:{}:{}:{}", chain_name, address, options)
}

pub fn cache_key_for_transactions(chain_name: &str, address: &str, options: &str) -> String {
    format!("transactions:{}:{}:{}", chain_name, address, options)
}

pub fn cache_key_for_transaction(chain_name: &str, tx_hash: &str) -> String {
    format!("transaction:{}:{}", chain_name, tx_hash)
}

pub fn cache_key_for_nfts(chain_name: &str, address: &str, options: &str) -> String {
    format!("nfts:{}:{}:{}", chain_name, address, options)
}

pub fn cache_key_for_nft_metadata(chain_name: &str, address: &str, token_id: &str) -> String {
    format!("nft_metadata:{}:{}:{}", chain_name, address, token_id)
}