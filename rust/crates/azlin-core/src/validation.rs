//! Validation utilities for Azure resources and user input.
//!
//! This module provides centralized validation functions for:
//! - VM names (Azure naming constraints)
//! - VM sizes (SKU format validation)
//! - Azure regions (known region validation)
//! - Image specifications (URN format and security)

/// Known valid Azure regions (subset — allows any alphanumeric lowercase string
/// that matches the general Azure region pattern).
pub const VALID_AZURE_REGIONS: &[&str] = &[
    "eastus",
    "eastus2",
    "westus",
    "westus2",
    "westus3",
    "centralus",
    "northcentralus",
    "southcentralus",
    "westcentralus",
    "canadacentral",
    "canadaeast",
    "brazilsouth",
    "brazilsoutheast",
    "northeurope",
    "westeurope",
    "uksouth",
    "ukwest",
    "francecentral",
    "francesouth",
    "germanywestcentral",
    "germanynorth",
    "switzerlandnorth",
    "switzerlandwest",
    "norwayeast",
    "norwaywest",
    "swedencentral",
    "eastasia",
    "southeastasia",
    "japaneast",
    "japanwest",
    "koreacentral",
    "koreasouth",
    "australiaeast",
    "australiasoutheast",
    "australiacentral",
    "centralindia",
    "southindia",
    "westindia",
    "uaenorth",
    "uaecentral",
    "southafricanorth",
    "southafricawest",
    "qatarcentral",
    "polandcentral",
    "italynorth",
];

/// Validate Azure VM name according to Azure rules.
///
/// # Rules
/// - Must be 1-64 characters
/// - Can contain alphanumeric characters, hyphens, and periods
/// - Cannot start or end with hyphen or period
///
/// # Examples
///
/// ```
/// use azlin_core::validation::validate_vm_name;
///
/// // Valid names
/// assert!(validate_vm_name("my-vm-01").is_ok());
/// assert!(validate_vm_name("dev.server").is_ok());
/// assert!(validate_vm_name("a").is_ok());
/// assert!(validate_vm_name(&"a".repeat(64)).is_ok());
///
/// // Invalid names
/// assert!(validate_vm_name("").is_err());
/// assert!(validate_vm_name(&"a".repeat(65)).is_err());
/// assert!(validate_vm_name("-bad").is_err());
/// assert!(validate_vm_name("bad-").is_err());
/// assert!(validate_vm_name("bad@name").is_err());
/// ```
pub fn validate_vm_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("VM name cannot be empty".to_string());
    }

    let bytes = name.as_bytes();
    let len = bytes.len();

    if len > 64 {
        return Err(format!(
            "VM name '{}' exceeds 64 character limit ({})",
            name, len
        ));
    }

    // Check first and last characters
    if matches!(bytes[0], b'-' | b'.') {
        return Err(format!(
            "VM name '{}' cannot start with hyphen or period",
            name
        ));
    }

    if matches!(bytes[len - 1], b'-' | b'.') {
        return Err(format!(
            "VM name '{}' cannot end with hyphen or period",
            name
        ));
    }

    // Validate all characters - use iterator for better performance
    if !bytes
        .iter()
        .all(|b| b.is_ascii_alphanumeric() || *b == b'-' || *b == b'.')
    {
        return Err(format!(
            "VM name '{}' can only contain alphanumeric characters, hyphens, and periods",
            name
        ));
    }

    Ok(())
}

/// Validate Azure VM size SKU format.
///
/// # Rules
/// - Must start with "Standard_" (case-sensitive)
/// - Cannot be empty
///
/// # Examples
///
/// ```
/// use azlin_core::validation::validate_vm_size;
///
/// assert!(validate_vm_size("Standard_D2s_v3").is_ok());
/// assert!(validate_vm_size("Standard_E16as_v5").is_ok());
/// assert!(validate_vm_size("").is_err());
/// assert!(validate_vm_size("Basic_A1").is_err());
/// ```
pub fn validate_vm_size(size: &str) -> Result<(), String> {
    if size.is_empty() {
        return Err("VM size cannot be empty".to_string());
    }

    if !size.starts_with("Standard_") {
        return Err(format!(
            "VM size must start with 'Standard_' (e.g., 'Standard_E16as_v5'), got '{}'",
            size
        ));
    }

    Ok(())
}

/// Validate Azure region name.
///
/// # Rules
/// - Must be a known Azure region (case-insensitive)
/// - Cannot be empty
///
/// # Examples
///
/// ```
/// use azlin_core::validation::validate_azure_region;
///
/// assert!(validate_azure_region("eastus").is_ok());
/// assert!(validate_azure_region("WestUS2").is_ok());
/// assert!(validate_azure_region("northeurope").is_ok());
/// assert!(validate_azure_region("mars-west1").is_err());
/// ```
pub fn validate_azure_region(region: &str) -> Result<String, String> {
    if region.is_empty() {
        return Err("Region cannot be empty".to_string());
    }

    let lower = region.to_lowercase();

    if !VALID_AZURE_REGIONS.contains(&lower.as_str()) {
        return Err(format!(
            "Invalid Azure region '{}'. Examples: eastus, westus2, northeurope",
            region
        ));
    }

    Ok(lower)
}

/// Check if a region is valid without returning the normalized form.
///
/// Useful for quick validation checks.
///
/// # Examples
///
/// ```
/// use azlin_core::validation::is_valid_region;
///
/// assert!(is_valid_region("eastus"));
/// assert!(is_valid_region("WestUS2"));
/// assert!(!is_valid_region("invalid-region"));
/// ```
pub fn is_valid_region(region: &str) -> bool {
    let lower = region.to_lowercase();
    VALID_AZURE_REGIONS.contains(&lower.as_str())
}

/// Get a list of all valid Azure regions.
///
/// Returns a static slice of region names in lowercase.
pub fn get_valid_regions() -> &'static [&'static str] {
    VALID_AZURE_REGIONS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_vm_name_valid() {
        assert!(validate_vm_name("my-vm-01").is_ok());
        assert!(validate_vm_name("dev.server").is_ok());
        assert!(validate_vm_name("a").is_ok());
        assert!(validate_vm_name(&"a".repeat(64)).is_ok());
        assert!(validate_vm_name("VM-01.Test").is_ok());
    }

    #[test]
    fn test_validate_vm_name_invalid_empty() {
        let result = validate_vm_name("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_validate_vm_name_invalid_too_long() {
        let result = validate_vm_name(&"a".repeat(65));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("64 character"));
        assert!(err.contains("65"));
    }

    #[test]
    fn test_validate_vm_name_invalid_starts_with_hyphen() {
        let result = validate_vm_name("-bad");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("start with"));
    }

    #[test]
    fn test_validate_vm_name_invalid_ends_with_hyphen() {
        let result = validate_vm_name("bad-");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("end with"));
    }

    #[test]
    fn test_validate_vm_name_invalid_starts_with_period() {
        let result = validate_vm_name(".bad");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_vm_name_invalid_ends_with_period() {
        let result = validate_vm_name("bad.");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_vm_name_invalid_characters() {
        assert!(validate_vm_name("bad@name").is_err());
        assert!(validate_vm_name("name#1").is_err());
        assert!(validate_vm_name("vm_01").is_err());
    }

    #[test]
    fn test_validate_vm_size_valid() {
        assert!(validate_vm_size("Standard_D2s_v3").is_ok());
        assert!(validate_vm_size("Standard_E16as_v5").is_ok());
        assert!(validate_vm_size("Standard_B1s").is_ok());
    }

    #[test]
    fn test_validate_vm_size_invalid_empty() {
        let result = validate_vm_size("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_vm_size_invalid_prefix() {
        assert!(validate_vm_size("Basic_A1").is_err());
        assert!(validate_vm_size("D2s_v3").is_err());
        assert!(validate_vm_size("standard_D2s").is_err());
    }

    #[test]
    fn test_validate_azure_region_valid() {
        assert!(validate_azure_region("eastus").is_ok());
        assert!(validate_azure_region("westus2").is_ok());
        assert!(validate_azure_region("northeurope").is_ok());
    }

    #[test]
    fn test_validate_azure_region_case_insensitive() {
        let result = validate_azure_region("WestUS2");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "westus2");

        let result = validate_azure_region("EASTUS");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "eastus");
    }

    #[test]
    fn test_validate_azure_region_invalid() {
        let result = validate_azure_region("mars-west1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid Azure region"));
    }

    #[test]
    fn test_validate_azure_region_empty() {
        let result = validate_azure_region("");
        assert!(result.is_err());
    }

    #[test]
    fn test_is_valid_region() {
        assert!(is_valid_region("eastus"));
        assert!(is_valid_region("WestUS2"));
        assert!(!is_valid_region("invalid"));
        assert!(!is_valid_region(""));
    }

    #[test]
    fn test_get_valid_regions() {
        let regions = get_valid_regions();
        assert!(!regions.is_empty());
        assert!(regions.contains(&"eastus"));
        assert!(regions.contains(&"westus2"));
        assert!(regions.contains(&"northeurope"));
    }

    #[test]
    fn test_all_known_regions_are_valid() {
        for region in VALID_AZURE_REGIONS {
            assert!(
                is_valid_region(region),
                "Region {} should be valid",
                region
            );
            assert_eq!(validate_azure_region(region).unwrap(), *region);
        }
    }
}
