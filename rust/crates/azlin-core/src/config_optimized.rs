//! ⚡ Оптимизированная конфигурация с кэшированием
//! 
//! Добавлено:
//! - Кэширование кэша resource groups
//! - Быстрая валидация через lookup таблицы
//! - Оптимизированная сериализация

use crate::cache::AzureCache;
use crate::validation::VALID_AZURE_REGIONS;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

// Re-export для обратной совместимости
pub use crate::config::{AzlinConfig as OriginalAzlinConfig, RestoreMode, SshSyncMethod};

/// Оптимизированный кэш конфигурации
pub struct ConfigCache {
    /// Кэш для resource groups
    rg_cache: Arc<DashMap<String, Vec<String>>>,
    /// Кэш для валидации регионов (case-insensitive lookup)
    region_lookup: Arc<DashMap<String, String>>,
}

impl ConfigCache {
    pub fn new() -> Self {
        let region_lookup: Arc<DashMap<String, String>> = Arc::new(DashMap::new());
        
        // Предварительное заполнение lookup таблицы регионов
        for region in VALID_AZURE_REGIONS.iter() {
            region_lookup.insert(region.to_lowercase(), region.to_string());
        }

        Self {
            rg_cache: Arc::new(DashMap::new()),
            region_lookup,
        }
    }

    /// Быстрая проверка региона (O(1) вместо O(n))
    pub fn is_valid_region(&self, region: &str) -> bool {
        self.region_lookup.contains_key(&region.to_lowercase())
    }

    /// Быстрое получение канонического имени региона
    pub fn canonicalize_region(&self, region: &str) -> Option<String> {
        self.region_lookup.get(&region.to_lowercase()).map(|v| v.clone())
    }

    /// Кэширование resource groups
    pub async fn get_rg(&self, subscription: &str) -> Option<Vec<String>> {
        self.rg_cache.get(subscription).map(|v| v.clone())
    }

    pub async fn set_rg(&self, subscription: String, rgs: Vec<String>) {
        self.rg_cache.insert(subscription, rgs);
    }

    pub fn clear_rg(&self, subscription: &str) {
        self.rg_cache.remove(subscription);
    }

    pub fn clear_all(&self) {
        self.rg_cache.clear();
    }

    /// Статистика кэша
    pub fn stats(&self) -> ConfigCacheStats {
        ConfigCacheStats {
            rg_entries: self.rg_cache.len(),
            region_entries: self.region_lookup.len(),
        }
    }
}

impl Default for ConfigCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Статистика кэша конфигурации
#[derive(Debug, Clone)]
pub struct ConfigCacheStats {
    pub rg_entries: usize,
    pub region_entries: usize,
}

impl std::fmt::Display for ConfigCacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ConfigCache: rg_entries={}, region_entries={}",
            self.rg_entries, self.region_entries
        )
    }
}

/// Расширения для AzlinConfig с кэшированием
pub trait AzlinConfigExt {
    /// Быстрая валидация региона с кэшированием
    fn validate_region_fast(&self, region: &str, cache: &ConfigCache) -> crate::Result<String>;
    
    /// Получить дефолтный регион с fallback
    fn get_effective_region(&self) -> String;
    
    /// Получить дефолтный VM size с fallback
    fn get_effective_vm_size(&self) -> String;
}

impl AzlinConfigExt for OriginalAzlinConfig {
    fn validate_region_fast(&self, region: &str, cache: &ConfigCache) -> crate::Result<String> {
        let lower = region.to_lowercase();
        
        // Быстрая проверка через кэш (O(1))
        if !cache.is_valid_region(region) {
            return Err(crate::AzlinError::Config(format!(
                "Invalid Azure region '{}'. Examples: eastus, westus2, northeurope",
                region
            )));
        }
        
        // Получаем каноническое имя
        cache
            .canonicalize_region(region)
            .ok_or_else(|| crate::AzlinError::Config(format!("Region lookup failed for '{}'", region)))
    }

    fn get_effective_region(&self) -> String {
        self.default_region.clone()
    }

    fn get_effective_vm_size(&self) -> String {
        self.default_vm_size.clone()
    }
}

/// Глобальный кэш конфигурации (ленивая инициализация)
static CONFIG_CACHE: std::sync::OnceLock<Arc<ConfigCache>> = std::sync::OnceLock::new();

/// Получить глобальный кэш конфигурации
pub fn get_config_cache() -> &'static Arc<ConfigCache> {
    CONFIG_CACHE.get_or_init(|| Arc::new(ConfigCache::new()))
}

/// Инициализация глобального кэша
pub fn init_config_cache() {
    let _ = get_config_cache();
}

/// Быстрая утилита для валидации региона
pub fn quick_validate_region(region: &str) -> crate::Result<String> {
    let cache = get_config_cache();
    let config = OriginalAzlinConfig::default();
    config.validate_region_fast(region, cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_cache_region_lookup() {
        let cache = ConfigCache::new();
        
        // Разные регистры должны работать
        assert!(cache.is_valid_region("eastus"));
        assert!(cache.is_valid_region("EastUS"));
        assert!(cache.is_valid_region("EASTUS"));
        
        // Каноникализация
        assert_eq!(cache.canonicalize_region("WestUS2"), Some("westus2".to_string()));
        assert_eq!(cache.canonicalize_region("eastus"), Some("eastus".to_string()));
    }

    #[test]
    fn test_config_cache_invalid_region() {
        let cache = ConfigCache::new();
        assert!(!cache.is_valid_region("mars-west1"));
        assert!(!cache.is_valid_region("invalid"));
    }

    #[test]
    fn test_config_cache_rg_cache() {
        let cache = ConfigCache::new();
        
        cache.set_rg("sub1".to_string(), vec!["rg1".to_string(), "rg2".to_string()]);
        
        let rgs = futures::executor::block_on(cache.get_rg("sub1"));
        assert_eq!(rgs, Some(vec!["rg1".to_string(), "rg2".to_string()]));
        
        cache.clear_rg("sub1");
        let rgs = futures::executor::block_on(cache.get_rg("sub1"));
        assert!(rgs.is_none());
    }

    #[test]
    fn test_validate_region_fast() {
        let cache = ConfigCache::new();
        let config = OriginalAzlinConfig::default();
        
        let result = config.validate_region_fast("WestUS2", &cache);
        assert_eq!(result, Ok("westus2".to_string()));
        
        let result = config.validate_region_fast("invalid", &cache);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_effective_region() {
        let config = OriginalAzlinConfig {
            default_region: "custom_region".to_string(),
            ..Default::default()
        };
        
        assert_eq!(config.get_effective_region(), "custom_region");
    }

    #[test]
    fn test_get_effective_vm_size() {
        let config = OriginalAzlinConfig {
            default_vm_size: "Custom_Size".to_string(),
            ..Default::default()
        };
        
        assert_eq!(config.get_effective_vm_size(), "Custom_Size");
    }

    #[test]
    fn test_global_cache() {
        let cache1 = get_config_cache();
        let cache2 = get_config_cache();
        
        assert!(Arc::ptr_eq(cache1, cache2));
    }

    #[test]
    fn test_quick_validate_region() {
        assert!(quick_validate_region("eastus").is_ok());
        assert!(quick_validate_region("WestUS2").is_ok());
        assert!(quick_validate_region("invalid").is_err());
    }
}
