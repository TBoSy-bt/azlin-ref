//! azlin-core: Core types, configuration, models, and error handling.
//!
//! This crate provides the foundational types and utilities used across azlin:
//! - Configuration management with validation
//! - Domain models for Azure resources
//! - Error types with rich context
//! - Security utilities (sanitization, validation)
//! - High-performance caching ⚡
//! - Parallel processing utilities ⚡
//! - Optimized configuration with caching ⚡

pub mod cache;
pub mod config;
pub mod config_optimized;
pub mod error;
pub mod models;
pub mod parallel;
pub mod sanitizer;
pub mod validation;

pub use cache::{
    create_bastion_cache, create_rg_cache, create_vm_cache, log_cache_stats, AzureCache,
    BastionCache, CacheStats, ResourceGroupCache, VmInfoCache,
};
pub use config::{AzlinConfig, RestoreMode, SshSyncMethod};
pub use config_optimized::{
    get_config_cache, init_config_cache, quick_validate_region, AzlinConfigExt, ConfigCache,
    ConfigCacheStats,
};
pub use error::{AzlinError, Result};
pub use models::{
    CommandResult, CostSummary, CreateVmParams, OsType, PowerState, ProvisioningState,
    ProvisioningStateExt, TmuxSession, VmCost, VmImage, VmInfo,
};
pub use parallel::{
    batch_process, fast_filter, fast_map, par_batch_async, par_cpu_process, par_group_by,
    par_process_with_limit, par_reduce,
};
pub use sanitizer::{contains_sensitive_data, sanitize};
pub use validation::{validate_azure_region, validate_vm_name, validate_vm_size};

/// Версия crates
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Инициализация ядра (кэши, пулы и т.д.)
pub fn init() {
    init_config_cache();
    tracing::debug!("azlin-core v{} initialized", VERSION);
}

/// Быстрая инициализация для production
pub fn init_production() {
    init();
    tracing::info!("azlin-core v{} production mode enabled", VERSION);
}
