//! azlin-core: Core types, configuration, models, and error handling.
//!
//! This crate provides the foundational types and utilities used across azlin:
//! - Configuration management with validation
//! - Domain models for Azure resources
//! - Error types with rich context
//! - Security utilities (sanitization, validation)

pub mod config;
pub mod error;
pub mod models;
pub mod sanitizer;
pub mod validation;

pub use config::{AzlinConfig, RestoreMode, SshSyncMethod};
pub use error::{AzlinError, Result};
pub use models::{
    CommandResult, CostSummary, CreateVmParams, OsType, PowerState, ProvisioningState,
    ProvisioningStateExt, TmuxSession, VmCost, VmImage, VmInfo,
};
pub use sanitizer::{contains_sensitive_data, sanitize};
pub use validation::{validate_azure_region, validate_vm_name, validate_vm_size};
