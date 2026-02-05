use crate::{Error, Result};
use std::collections::HashSet;
use tracing::{debug, instrument};

/// Validation utilities for blockchain data.
pub struct Validator;

impl Validator {
    /// Validate an Ethereum-style address (42 characters, starts with 0x).
    #[instrument(fields(address = %address))]
    pub fn validate_address(address: &str) -> Result<()> {
        let address = address.trim();
        
        if address.is_empty() {
            return Err(Error::Config("Address cannot be empty".to_string()));
        }
        
        if !address.starts_with("0x") {
            return Err(Error::Config("Address must start with '0x'".to_string()));
        }
        
        if address.len() != 42 {
            return Err(Error::Config(format!(
                "Address must be 42 characters long, got {}", address.len()
            )));
        }
        
        // Check if the rest are valid hex characters
        for char in address.chars().skip(2) {
            if !char.is_ascii_hexdigit() {
                return Err(Error::Config(format!(
                    "Address contains invalid character: '{}'", char
                )));
            }
        }
        
        debug!("Address validation passed");
        Ok(())
    }
    
    /// Validate a transaction hash (66 characters, starts with 0x).
    #[instrument(fields(tx_hash = %tx_hash))]
    pub fn validate_tx_hash(tx_hash: &str) -> Result<()> {
        let tx_hash = tx_hash.trim();
        
        if tx_hash.is_empty() {
            return Err(Error::Config("Transaction hash cannot be empty".to_string()));
        }
        
        if !tx_hash.starts_with("0x") {
            return Err(Error::Config("Transaction hash must start with '0x'".to_string()));
        }
        
        if tx_hash.len() != 66 {
            return Err(Error::Config(format!(
                "Transaction hash must be 66 characters long, got {}", tx_hash.len()
            )));
        }
        
        // Check if the rest are valid hex characters
        for char in tx_hash.chars().skip(2) {
            if !char.is_ascii_hexdigit() {
                return Err(Error::Config(format!(
                    "Transaction hash contains invalid character: '{}'", char
                )));
            }
        }
        
        debug!("Transaction hash validation passed");
        Ok(())
    }
    
    /// Validate a chain name/identifier.
    #[instrument(fields(chain_name = %chain_name))]
    pub fn validate_chain_name(chain_name: &str) -> Result<()> {
        let chain_name = chain_name.trim();
        
        if chain_name.is_empty() {
            return Err(Error::Config("Chain name cannot be empty".to_string()));
        }
        
        // Check for valid characters (alphanumeric, hyphens, underscores)
        for char in chain_name.chars() {
            if !char.is_alphanumeric() && char != '-' && char != '_' {
                return Err(Error::Config(format!(
                    "Chain name contains invalid character: '{}'. Only alphanumeric, hyphens, and underscores are allowed", char
                )));
            }
        }
        
        debug!("Chain name validation passed");
        Ok(())
    }
    
    /// Validate page size parameter.
    #[instrument(fields(page_size = %page_size))]
    pub fn validate_page_size(page_size: u32) -> Result<()> {
        const MIN_PAGE_SIZE: u32 = 1;
        const MAX_PAGE_SIZE: u32 = 1000;
        
        if page_size < MIN_PAGE_SIZE {
            return Err(Error::Config(format!(
                "Page size must be at least {}, got {}", MIN_PAGE_SIZE, page_size
            )));
        }
        
        if page_size > MAX_PAGE_SIZE {
            return Err(Error::Config(format!(
                "Page size cannot exceed {}, got {}", MAX_PAGE_SIZE, page_size
            )));
        }
        
        debug!("Page size validation passed");
        Ok(())
    }
    
    /// Validate an API key format.
    #[instrument(fields(api_key_prefix = %api_key.chars().take(8).collect::<String>()))]
    pub fn validate_api_key(api_key: &str) -> Result<()> {
        let api_key = api_key.trim();
        
        if api_key.is_empty() {
            return Err(Error::Config("API key cannot be empty".to_string()));
        }
        
        if api_key.len() < 10 {
            return Err(Error::Config("API key appears to be too short".to_string()));
        }
        
        // Check for potential placeholder values
        let invalid_keys = ["your-api-key", "test", "demo", "placeholder", "xxx"];
        let lower_key = api_key.to_lowercase();
        
        for invalid in &invalid_keys {
            if lower_key.contains(invalid) {
                return Err(Error::Config("API key appears to be a placeholder value".to_string()));
            }
        }
        
        debug!("API key validation passed");
        Ok(())
    }
    
    /// Validate contract address and token ID combination.
    #[instrument(fields(contract_address = %contract_address, token_id = %token_id))]
    pub fn validate_nft_identifier(contract_address: &str, token_id: &str) -> Result<()> {
        Self::validate_address(contract_address)?;
        
        let token_id = token_id.trim();
        if token_id.is_empty() {
            return Err(Error::Config("Token ID cannot be empty".to_string()));
        }
        
        // Token ID should be a valid number (decimal or hex)
        if token_id.starts_with("0x") {
            // Hex token ID
            for char in token_id.chars().skip(2) {
                if !char.is_ascii_hexdigit() {
                    return Err(Error::Config(format!(
                        "Token ID contains invalid hex character: '{}'", char
                    )));
                }
            }
        } else {
            // Decimal token ID
            if token_id.parse::<u64>().is_err() {
                return Err(Error::Config("Token ID must be a valid number".to_string()));
            }
        }
        
        debug!("NFT identifier validation passed");
        Ok(())
    }
    
    /// Validate URL format.
    #[instrument(fields(url = %url))]
    pub fn validate_url(url: &str) -> Result<()> {
        let url = url.trim();
        
        if url.is_empty() {
            return Err(Error::Config("URL cannot be empty".to_string()));
        }
        
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(Error::Config("URL must start with http:// or https://".to_string()));
        }
        
        // Basic URL validation - check for valid characters
        let valid_chars: HashSet<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~:/?#[]@!$&'()*+,;=%"
            .chars()
            .collect();
        
        for char in url.chars() {
            if !valid_chars.contains(&char) {
                return Err(Error::Config(format!(
                    "URL contains invalid character: '{}'", char
                )));
            }
        }
        
        debug!("URL validation passed");
        Ok(())
    }
}

/// Sanitization utilities for user input.
pub struct Sanitizer;

impl Sanitizer {
    /// Sanitize an address by trimming and converting to lowercase.
    pub fn sanitize_address(address: &str) -> String {
        let trimmed = address.trim();
        if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
            format!("0x{}", &trimmed[2..].to_lowercase())
        } else {
            trimmed.to_lowercase()
        }
    }
    
    /// Sanitize a transaction hash by trimming and converting to lowercase.
    pub fn sanitize_tx_hash(tx_hash: &str) -> String {
        let trimmed = tx_hash.trim();
        if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
            format!("0x{}", &trimmed[2..].to_lowercase())
        } else {
            format!("0x{}", trimmed.to_lowercase())
        }
    }
    
    /// Sanitize chain name by trimming and converting to lowercase.
    pub fn sanitize_chain_name(chain_name: &str) -> String {
        chain_name.trim().to_lowercase()
    }
    
    /// Remove potentially dangerous characters from user input.
    pub fn sanitize_user_input(input: &str) -> String {
        input
            .trim()
            .chars()
            .filter(|c| c.is_alphanumeric() || "-_. ".contains(*c))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_address_validation() {
        // Valid address
        assert!(Validator::validate_address("0x742d35Cc6634C0532925a3b8D4fc24f3C4aD6a8b").is_ok());
        
        // Invalid cases
        assert!(Validator::validate_address("").is_err());
        assert!(Validator::validate_address("742d35Cc6634C0532925a3b8D4fc24f3C4aD6a8b").is_err());
        assert!(Validator::validate_address("0x742d35Cc6634").is_err());
        assert!(Validator::validate_address("0x742d35Cc6634C0532925a3b8D4fc24f3C4aD6a8bXX").is_err());
    }
    
    #[test]
    fn test_tx_hash_validation() {
        // Valid tx hash
        assert!(Validator::validate_tx_hash("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef").is_ok());
        
        // Invalid cases
        assert!(Validator::validate_tx_hash("").is_err());
        assert!(Validator::validate_tx_hash("1234567890abcdef").is_err());
        assert!(Validator::validate_tx_hash("0x1234").is_err());
    }
    
    #[test]
    fn test_sanitization() {
        assert_eq!(
            Sanitizer::sanitize_address("  0X742d35Cc6634C0532925a3b8D4fc24f3C4aD6a8b  "),
            "0x742d35cc6634c0532925a3b8d4fc24f3c4ad6a8b"
        );
        
        assert_eq!(
            Sanitizer::sanitize_chain_name("  ETH-MAINNET  "),
            "eth-mainnet"
        );
    }
}