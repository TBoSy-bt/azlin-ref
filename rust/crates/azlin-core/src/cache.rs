//! 🔥 Высокопроизводительное кэширование для azlin
//! 
//! Использует Moka cache с автоматической инвалидацией,
//! параллельным обновлением и метриками производительности.

use moka::future::Cache;
use std::time::Duration;
use tracing::{debug, info, warn};

/// Кэш для Azure API ответов
/// TTL: 15 минут по умолчанию
pub struct AzureCache<T> {
    cache: Cache<String, T>,
    hits: std::sync::Arc<std::sync::atomic::AtomicU64>,
    misses: std::sync::Arc<std::sync::atomic::AtomicU64>,
}

impl<T> AzureCache<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// Создать новый кэш с TTL
    pub fn new(ttl_seconds: u64, max_capacity: u64) -> Self {
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(ttl_seconds))
            .time_to_idle(Duration::from_secs(ttl_seconds / 2))
            .max_capacity(max_capacity)
            .initial_capacity(max_capacity / 10)
            .record_access_history()
            .build();

        Self {
            cache,
            hits: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0)),
            misses: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    /// Получить значение из кэша или вычислить
    pub async fn get_or_insert<F, Fut, E>(&self, key: &str, compute: F) -> Result<T, E>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        // Пробуем получить из кэша
        if let Some(value) = self.cache.get(key) {
            self.hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            debug!(key, "cache hit");
            return Ok(value);
        }

        // Кэш-мисс — вычисляем
        self.misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        debug!(key, "cache miss, computing");
        
        let value = compute().await?;
        self.cache.insert(key.to_string(), value.clone()).await;
        
        Ok(value)
    }

    /// Вставить значение в кэш
    pub async fn insert(&self, key: String, value: T) {
        self.cache.insert(key, value).await;
    }

    /// Удалить значение из кэша
    pub fn remove(&self, key: &str) {
        self.cache.invalidate(key);
    }

    /// Очистить весь кэш
    pub fn clear(&self) {
        self.cache.invalidate_all();
    }

    /// Получить статистику кэша
    pub fn stats(&self) -> CacheStats {
        let hits = self.hits.load(std::sync::atomic::Ordering::Relaxed);
        let misses = self.misses.load(std::sync::atomic::Ordering::Relaxed);
        let total = hits + misses;
        
        CacheStats {
            hits,
            misses,
            hit_rate: if total > 0 { 
                (hits as f64 / total as f64) * 100.0 
            } else { 
                0.0 
            },
            size: self.cache.entry_count(),
        }
    }

    /// Принудительное обновление значения
    pub async fn refresh<F, Fut, E>(&self, key: &str, compute: F) -> Result<T, E>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        // Удаляем старое значение
        self.cache.invalidate(key);
        
        // Вычисляем новое
        let value = compute().await?;
        self.cache.insert(key.to_string(), value.clone()).await;
        
        Ok(value)
    }
}

/// Статистика кэша
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub size: u64,
}

impl std::fmt::Display for CacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cache: hits={}, misses={}, hit_rate={:.1}%, size={}",
            self.hits, self.misses, self.hit_rate, self.size
        )
    }
}

/// Глобальный кэш для resource groups
/// TTL: 15 минут, максимум 1000 записей
pub type ResourceGroupCache = AzureCache<Vec<String>>;

/// Глобальный кэш для VM информации
/// TTL: 5 минут, максимум 500 записей
pub type VmInfoCache = AzureCache<crate::models::VmInfo>;

/// Глобальный кэш для bastion информации
/// TTL: 30 минут, максимум 100 записей
pub type BastionCache = AzureCache<String>;

/// Создать кэш resource groups по умолчанию
pub fn create_rg_cache() -> ResourceGroupCache {
    AzureCache::new(900, 1000) // 15 минут TTL
}

/// Создать кэш VM информации по умолчанию
pub fn create_vm_cache() -> VmInfoCache {
    AzureCache::new(300, 500) // 5 минут TTL
}

/// Создать кэш bastion по умолчанию
pub fn create_bastion_cache() -> BastionCache {
    AzureCache::new(1800, 100) // 30 минут TTL
}

/// Логирование статистики кэша
pub fn log_cache_stats<T>(cache: &AzureCache<T>, name: &str) {
    let stats = cache.stats();
    info!(name, "cache stats", hit_rate = stats.hit_rate, size = stats.size);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_hits_and_misses() {
        let cache = AzureCache::new(60, 100);
        
        // Первый запрос — мисс
        let value = cache
            .get_or_insert("key1", || async { Ok::<_, ()>("value1".to_string()) })
            .await
            .unwrap();
        assert_eq!(value, "value1");
        
        // Второй запрос — хит
        let value = cache
            .get_or_insert("key1", || async { Ok::<_, ()>("value2".to_string()) })
            .await
            .unwrap();
        assert_eq!(value, "value1"); // Старое значение из кэша
        
        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert!(stats.hit_rate > 40.0);
    }

    #[tokio::test]
    async fn test_cache_invalidation() {
        let cache = AzureCache::new(60, 100);
        
        cache
            .get_or_insert("key1", || async { Ok::<_, ()>("value1".to_string()) })
            .await
            .unwrap();
        
        cache.remove("key1");
        
        let value = cache
            .get_or_insert("key1", || async { Ok::<_, ()>("value2".to_string()) })
            .await
            .unwrap();
        
        assert_eq!(value, "value2"); // Новое значение
    }

    #[tokio::test]
    async fn test_cache_refresh() {
        let cache = AzureCache::new(60, 100);
        
        cache
            .get_or_insert("key1", || async { Ok::<_, ()>("value1".to_string()) })
            .await
            .unwrap();
        
        let value = cache
            .refresh("key1", || async { Ok::<_, ()>("value2".to_string()) })
            .await
            .unwrap();
        
        assert_eq!(value, "value2");
        
        let stats = cache.stats();
        assert_eq!(stats.misses, 2); // refresh считается как мисс
    }
}
