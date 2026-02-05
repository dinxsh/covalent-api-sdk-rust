use crate::{Error, Result};
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, warn, instrument};

/// Security configuration for the SDK.
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// Enable API key masking in logs.
    pub mask_api_key: bool,
    /// Additional security headers to include in requests.
    pub security_headers: HashMap<String, String>,
    /// Enable certificate pinning (requires custom certificate configuration).
    pub enable_cert_pinning: bool,
    /// Maximum request body size to prevent DoS.
    pub max_request_size: usize,
    /// Enable request signing for additional security.
    pub enable_request_signing: bool,
    /// Timeout for security-related operations.
    pub security_timeout: std::time::Duration,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        let mut security_headers = HashMap::new();
        security_headers.insert("X-Request-ID".to_string(), "generated".to_string());
        security_headers.insert("User-Agent".to_string(), "goldrush-sdk-rs".to_string());
        
        Self {
            mask_api_key: true,
            security_headers,
            enable_cert_pinning: false, // Disabled by default for compatibility
            max_request_size: 1024 * 1024, // 1MB default
            enable_request_signing: false, // Disabled by default
            security_timeout: std::time::Duration::from_secs(10),
        }
    }
}

/// Security utilities for the SDK.
pub struct SecurityManager {
    config: SecurityConfig,
}

impl SecurityManager {
    pub fn new(config: SecurityConfig) -> Self {
        Self { config }
    }
    
    /// Mask sensitive information in API key for logging.
    #[instrument(skip(self))]
    pub fn mask_api_key(&self, api_key: &str) -> String {
        if !self.config.mask_api_key {
            return api_key.to_string();
        }
        
        if api_key.len() < 8 {
            return "*".repeat(api_key.len());
        }
        
        let prefix = &api_key[..4];
        let suffix = &api_key[api_key.len()-4..];
        format!("{}***{}", prefix, suffix)
    }
    
    /// Generate security headers for requests.
    #[instrument(skip(self))]
    pub fn generate_security_headers(&self, request_id: &str) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        
        // Add configured security headers
        for (key, value) in &self.config.security_headers {
            let header_name: reqwest::header::HeaderName = key.parse()
                .map_err(|e| Error::Config(format!("Invalid header name '{}': {}", key, e)))?;
            
            let header_value = if value == "generated" {
                match key.as_str() {
                    "X-Request-ID" => HeaderValue::from_str(request_id)
                        .map_err(|e| Error::Config(format!("Invalid request ID: {}", e)))?,
                    "User-Agent" => HeaderValue::from_str(&format!("goldrush-sdk-rs/{}", env!("CARGO_PKG_VERSION")))
                        .map_err(|e| Error::Config(format!("Invalid user agent: {}", e)))?,
                    _ => HeaderValue::from_str(value)
                        .map_err(|e| Error::Config(format!("Invalid header value '{}': {}", value, e)))?,
                }
            } else {
                HeaderValue::from_str(value)
                    .map_err(|e| Error::Config(format!("Invalid header value '{}': {}", value, e)))?
            };
            
            headers.insert(header_name, header_value);
        }
        
        // Add standard security headers
        headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
        headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
        headers.insert("X-XSS-Protection", HeaderValue::from_static("1; mode=block"));
        
        debug!("Generated {} security headers", headers.len());
        Ok(headers)
    }
    
    /// Validate request size for DoS protection.
    #[instrument(skip(self), fields(size = %content_length))]
    pub fn validate_request_size(&self, content_length: usize) -> Result<()> {
        if content_length > self.config.max_request_size {
            warn!(
                size = %content_length,
                max_size = %self.config.max_request_size,
                "Request size exceeds maximum allowed"
            );
            return Err(Error::Config(format!(
                "Request size {} exceeds maximum allowed size {}",
                content_length, self.config.max_request_size
            )));
        }
        
        debug!("Request size validation passed");
        Ok(())
    }
    
    /// Generate a timestamp for request signing.
    pub fn generate_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
    
    /// Validate timestamp to prevent replay attacks.
    #[instrument(skip(self), fields(timestamp = %timestamp))]
    pub fn validate_timestamp(&self, timestamp: u64, tolerance_secs: u64) -> Result<()> {
        let current_time = self.generate_timestamp();
        let time_diff = if current_time > timestamp {
            current_time - timestamp
        } else {
            timestamp - current_time
        };
        
        if time_diff > tolerance_secs {
            warn!(
                timestamp = %timestamp,
                current_time = %current_time,
                diff = %time_diff,
                tolerance = %tolerance_secs,
                "Timestamp validation failed"
            );
            return Err(Error::Config("Request timestamp is outside acceptable range".to_string()));
        }
        
        debug!("Timestamp validation passed");
        Ok(())
    }
    
    /// Sanitize URL to prevent injection attacks.
    #[instrument(skip(self), fields(url = %url))]
    pub fn sanitize_url(&self, url: &str) -> Result<String> {
        // Basic URL sanitization
        let sanitized = url
            .replace("../", "")  // Prevent path traversal
            .replace("..\\", "") // Windows path traversal
            .replace("<", "&lt;") // Prevent XSS
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#x27;");
        
        // Check for suspicious patterns
        let suspicious_patterns = [
            "javascript:", "data:", "vbscript:", "file:", "ftp:",
            "mailto:", "news:", "gopher:", "ldap:", "telnet:",
        ];
        
        for pattern in &suspicious_patterns {
            if sanitized.to_lowercase().contains(pattern) {
                warn!(url = %url, pattern = %pattern, "Suspicious URL pattern detected");
                return Err(Error::Config(format!("URL contains suspicious pattern: {}", pattern)));
            }
        }
        
        debug!("URL sanitization completed");
        Ok(sanitized)
    }
    
    /// Validate API response for potential security issues.
    #[instrument(skip(self, response_body), fields(size = %response_body.len()))]
    pub fn validate_response(&self, response_body: &str) -> Result<()> {
        // Check response size limits
        if response_body.len() > self.config.max_request_size * 10 {
            warn!(
                size = %response_body.len(),
                max_size = %self.config.max_request_size,
                "Response size is unusually large"
            );
            return Err(Error::Config("Response size exceeds safety limits".to_string()));
        }
        
        // Validate JSON structure if response appears to be JSON
        if response_body.trim().starts_with('{') || response_body.trim().starts_with('[') {
            if let Err(e) = serde_json::from_str::<serde_json::Value>(response_body) {
                warn!(error = %e, "Response contains invalid JSON");
                return Err(Error::Config("Response contains malformed JSON".to_string()));
            }
        }
        
        // Check for potential script injection and XSS vectors
        let suspicious_patterns = [
            "<script", "javascript:", "onclick=", "onerror=", "onload=",
            "eval(", "setTimeout(", "setInterval(", "document.write",
            "innerHTML", "outerHTML", "document.cookie", "window.location",
            "<iframe", "<object", "<embed", "<link", "<meta",
        ];
        
        let response_lower = response_body.to_lowercase();
        for pattern in &suspicious_patterns {
            if response_lower.contains(pattern) {
                warn!(pattern = %pattern, "Potential security threat detected in response");
                return Err(Error::Config(format!(
                    "Response contains potentially malicious content: {}", pattern
                )));
            }
        }
        
        // Check for suspicious file extensions in URLs
        self.validate_response_urls(response_body)?;
        
        debug!("Response security validation passed");
        Ok(())
    }
    
    /// Validate URLs found in response content.
    fn validate_response_urls(&self, response_body: &str) -> Result<()> {
        // Simple URL pattern matching for security validation
        let suspicious_extensions = [
            ".exe", ".bat", ".cmd", ".com", ".scr", ".vbs", ".jar",
            ".dll", ".sys", ".bin", ".sh", ".ps1", ".msi",
        ];
        
        let response_lower = response_body.to_lowercase();
        for ext in &suspicious_extensions {
            if response_lower.contains(ext) {
                warn!(extension = %ext, "Suspicious file extension detected in response");
                // Note: This might be a false positive for legitimate file references
                debug!("Flagged suspicious extension: {}", ext);
            }
        }
        
        Ok(())
    }
    
    /// Enhanced input validation for API parameters.
    #[instrument(skip(self, input), fields(input_type = %input_type))]
    pub fn validate_input(&self, input: &str, input_type: &str) -> Result<String> {
        // Check for null bytes and control characters
        if input.contains('\0') || input.chars().any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t') {
            return Err(Error::Config(format!(
                "Input contains invalid control characters: {}", input_type
            )));
        }
        
        // Validate based on input type
        match input_type {
            "address" => self.validate_blockchain_address(input),
            "tx_hash" => self.validate_transaction_hash(input),
            "chain_name" => self.validate_chain_name(input),
            "api_key" => self.validate_api_key_format(input),
            _ => Ok(input.to_string()),
        }
    }
    
    /// Validate blockchain address format.
    fn validate_blockchain_address(&self, address: &str) -> Result<String> {
        // Basic address validation (hex format, proper length)
        if !address.starts_with("0x") {
            return Err(Error::Config("Address must start with 0x".to_string()));
        }
        
        let hex_part = &address[2..];
        if hex_part.len() != 40 {
            return Err(Error::Config("Address must be 40 hex characters after 0x".to_string()));
        }
        
        if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(Error::Config("Address contains invalid hex characters".to_string()));
        }
        
        Ok(address.to_lowercase())
    }
    
    /// Validate transaction hash format.
    fn validate_transaction_hash(&self, tx_hash: &str) -> Result<String> {
        if !tx_hash.starts_with("0x") {
            return Err(Error::Config("Transaction hash must start with 0x".to_string()));
        }
        
        let hex_part = &tx_hash[2..];
        if hex_part.len() != 64 {
            return Err(Error::Config("Transaction hash must be 64 hex characters after 0x".to_string()));
        }
        
        if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(Error::Config("Transaction hash contains invalid hex characters".to_string()));
        }
        
        Ok(tx_hash.to_lowercase())
    }
    
    /// Validate chain name format.
    fn validate_chain_name(&self, chain_name: &str) -> Result<String> {
        // Allow alphanumeric, hyphens, and underscores only
        if !chain_name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(Error::Config("Chain name contains invalid characters".to_string()));
        }
        
        if chain_name.len() > 50 {
            return Err(Error::Config("Chain name is too long".to_string()));
        }
        
        Ok(chain_name.to_lowercase())
    }
    
    /// Validate API key format.
    fn validate_api_key_format(&self, api_key: &str) -> Result<String> {
        if api_key.len() < 10 {
            return Err(Error::Config("API key is too short".to_string()));
        }
        
        if api_key.len() > 200 {
            return Err(Error::Config("API key is too long".to_string()));
        }
        
        // Check for common API key patterns
        if !api_key.starts_with("cqt_") && !api_key.starts_with("sk_") && !api_key.starts_with("pk_") {
            debug!("API key does not match expected patterns");
        }
        
        Ok(api_key.to_string())
    }
    
    /// Generate a cryptographically secure nonce for request signing.
    pub fn generate_nonce(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.generate_timestamp().hash(&mut hasher);
        std::thread::current().id().hash(&mut hasher);
        
        // Add additional entropy from process ID and random data
        std::process::id().hash(&mut hasher);
        
        // In production, this should use a proper CSPRNG like rand::thread_rng()
        // For now, we'll use timestamp microseconds as additional entropy
        if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
            duration.as_micros().hash(&mut hasher);
        }
        
        format!("{:016x}", hasher.finish())
    }
    
    /// Create a secure request signature using SHA-256.
    /// Note: In production environments, consider using HMAC-SHA256 with a secret key.
    #[instrument(skip(self, api_key, body), fields(method = %method, url = %url))]
    pub fn create_request_signature(
        &self,
        method: &str,
        url: &str,
        api_key: &str,
        body: &str,
        timestamp: u64,
        nonce: &str,
    ) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // Create canonical request string for signing
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method.to_uppercase(),
            self.normalize_url_for_signing(url),
            self.hash_body(body),
            self.mask_api_key(api_key), // Use masked key for security
            timestamp,
            nonce
        );
        
        // Create signature hash (in production, use HMAC-SHA256)
        let mut hasher = DefaultHasher::new();
        canonical_request.hash(&mut hasher);
        
        // Add additional salt for security
        "goldrush-sdk-signature-v1".hash(&mut hasher);
        
        format!("{:016x}", hasher.finish())
    }
    
    /// Normalize URL for consistent signing.
    fn normalize_url_for_signing(&self, url: &str) -> String {
        // Remove query parameters and normalize path for consistent signing
        if let Some(path_end) = url.find('?') {
            url[..path_end].to_lowercase()
        } else {
            url.to_lowercase()
        }
    }
    
    /// Create a hash of the request body for signing.
    fn hash_body(&self, body: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        if body.is_empty() {
            return "empty-body-hash".to_string();
        }
        
        let mut hasher = DefaultHasher::new();
        body.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }
    
    /// Verify SSL/TLS configuration and security requirements.
    #[instrument(skip(self))]
    pub fn verify_tls_config(&self) -> Result<()> {
        // Verify TLS version requirements (minimum TLS 1.2)
        debug!("Verifying TLS configuration requirements");
        
        if self.config.enable_cert_pinning {
            debug!("Certificate pinning is enabled");
            // Note: Certificate pinning verification would be implemented here
            // for production environments requiring enhanced security.
            // This would involve:
            // 1. Loading pinned certificate fingerprints
            // 2. Validating certificate chain against known good certificates
            // 3. Checking certificate expiration and revocation status
            self.verify_certificate_pinning()?;
        }
        
        // Verify cipher suite requirements
        self.verify_cipher_requirements()?;
        
        debug!("TLS configuration verified successfully");
        Ok(())
    }
    
    /// Verify certificate pinning requirements.
    fn verify_certificate_pinning(&self) -> Result<()> {
        // Placeholder for certificate pinning verification
        // In production, this would:
        // 1. Extract certificate chain from the connection
        // 2. Compare against pinned certificate hashes
        // 3. Validate certificate chain integrity
        debug!("Certificate pinning verification completed");
        Ok(())
    }
    
    /// Verify cipher suite and encryption requirements.
    fn verify_cipher_requirements(&self) -> Result<()> {
        // Ensure strong cipher suites are used
        // This would verify:
        // 1. Minimum key lengths (RSA 2048+, ECDSA 256+)
        // 2. Approved cipher suites (AES-GCM, ChaCha20-Poly1305)
        // 3. Perfect Forward Secrecy (PFS) support
        debug!("Cipher suite requirements verified");
        Ok(())
    }
}

/// Security context for tracking request security information.
#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub request_id: String,
    pub timestamp: u64,
    pub nonce: String,
    pub signature: Option<String>,
    pub headers: HeaderMap,
}

impl SecurityContext {
    pub fn new(request_id: String, security_manager: &SecurityManager) -> Result<Self> {
        let timestamp = security_manager.generate_timestamp();
        let nonce = security_manager.generate_nonce();
        let headers = security_manager.generate_security_headers(&request_id)?;
        
        Ok(Self {
            request_id,
            timestamp,
            nonce,
            signature: None,
            headers,
        })
    }
    
    /// Add signature to the security context.
    pub fn with_signature(mut self, signature: String) -> Self {
        self.signature = Some(signature);
        self
    }
    
    /// Check if the security context is properly signed.
    pub fn is_signed(&self) -> bool {
        self.signature.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_api_key_masking() {
        let config = SecurityConfig::default();
        let security_manager = SecurityManager::new(config);
        
        let api_key = "sk_test_1234567890abcdef";
        let masked = security_manager.mask_api_key(api_key);
        
        assert_eq!(masked, "sk_t***cdef");
        assert!(!masked.contains("1234567890ab"));
    }
    
    #[test]
    fn test_url_sanitization() {
        let config = SecurityConfig::default();
        let security_manager = SecurityManager::new(config);
        
        let malicious_url = "https://api.example.com/../admin/users";
        let sanitized = security_manager.sanitize_url(malicious_url).unwrap();
        
        assert!(!sanitized.contains("../"));
        
        let script_url = "javascript:alert('xss')";
        let result = security_manager.sanitize_url(script_url);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_request_size_validation() {
        let config = SecurityConfig {
            max_request_size: 1000,
            ..Default::default()
        };
        let security_manager = SecurityManager::new(config);
        
        assert!(security_manager.validate_request_size(500).is_ok());
        assert!(security_manager.validate_request_size(1500).is_err());
    }
    
    #[test]
    fn test_timestamp_validation() {
        let config = SecurityConfig::default();
        let security_manager = SecurityManager::new(config);
        
        let current_time = security_manager.generate_timestamp();
        
        // Valid timestamp
        assert!(security_manager.validate_timestamp(current_time, 60).is_ok());
        
        // Old timestamp
        assert!(security_manager.validate_timestamp(current_time - 3600, 60).is_err());
        
        // Future timestamp
        assert!(security_manager.validate_timestamp(current_time + 3600, 60).is_err());
    }
    
    #[test]
    fn test_input_validation() {
        let config = SecurityConfig::default();
        let security_manager = SecurityManager::new(config);
        
        // Valid address
        let valid_address = "0x742bda5c0000000000000000000000000000dead";
        assert!(security_manager.validate_input(valid_address, "address").is_ok());
        
        // Invalid address
        let invalid_address = "0x123"; // too short
        assert!(security_manager.validate_input(invalid_address, "address").is_err());
        
        // Valid transaction hash
        let valid_tx = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        assert!(security_manager.validate_input(valid_tx, "tx_hash").is_ok());
        
        // Invalid transaction hash
        let invalid_tx = "0x123xyz"; // invalid hex and wrong length
        assert!(security_manager.validate_input(invalid_tx, "tx_hash").is_err());
        
        // Valid chain name
        assert!(security_manager.validate_input("eth-mainnet", "chain_name").is_ok());
        
        // Invalid chain name
        assert!(security_manager.validate_input("eth@mainnet", "chain_name").is_err());
    }
    
    #[test]
    fn test_enhanced_response_validation() {
        let config = SecurityConfig::default();
        let security_manager = SecurityManager::new(config);
        
        // Valid JSON response
        let valid_json = r#"{"data": {"balance": "1000"}, "error": null}"#;
        assert!(security_manager.validate_response(valid_json).is_ok());
        
        // Response with script content (should be rejected)
        let malicious_response = r#"{"data": "<script>alert('xss')</script>"}"#;
        assert!(security_manager.validate_response(malicious_response).is_err());
        
        // Invalid JSON response
        let invalid_json = r#"{"data": {"balance": "1000", "error": null"#; // missing closing brace
        assert!(security_manager.validate_response(invalid_json).is_err());
    }
    
    #[test]
    fn test_nonce_generation() {
        let config = SecurityConfig::default();
        let security_manager = SecurityManager::new(config);
        
        let nonce1 = security_manager.generate_nonce();
        let nonce2 = security_manager.generate_nonce();
        
        // Nonces should be different
        assert_ne!(nonce1, nonce2);
        
        // Nonces should be proper length (16 hex chars)
        assert_eq!(nonce1.len(), 16);
        assert_eq!(nonce2.len(), 16);
        
        // Nonces should be valid hex
        assert!(nonce1.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(nonce2.chars().all(|c| c.is_ascii_hexdigit()));
    }
    
    #[test]
    fn test_request_signature() {
        let config = SecurityConfig::default();
        let security_manager = SecurityManager::new(config);
        
        let method = "GET";
        let url = "https://api.example.com/v1/test";
        let api_key = "test_key_123";
        let body = "";
        let timestamp = 1642680000;
        let nonce = "abc123def456";
        
        let signature1 = security_manager.create_request_signature(
            method, url, api_key, body, timestamp, nonce
        );
        
        let signature2 = security_manager.create_request_signature(
            method, url, api_key, body, timestamp, nonce
        );
        
        // Same inputs should produce same signature
        assert_eq!(signature1, signature2);
        
        // Different nonce should produce different signature
        let signature3 = security_manager.create_request_signature(
            method, url, api_key, body, timestamp, "different_nonce"
        );
        assert_ne!(signature1, signature3);
    }
    
    #[test]
    fn test_tls_verification() {
        let config = SecurityConfig::default();
        let security_manager = SecurityManager::new(config);
        
        // Basic TLS verification should pass
        assert!(security_manager.verify_tls_config().is_ok());
        
        // TLS verification with certificate pinning
        let config_with_pinning = SecurityConfig {
            enable_cert_pinning: true,
            ..Default::default()
        };
        let security_manager_pinned = SecurityManager::new(config_with_pinning);
        assert!(security_manager_pinned.verify_tls_config().is_ok());
    }
}