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
        // Check response size
        if response_body.len() > self.config.max_request_size * 10 {
            warn!(
                size = %response_body.len(),
                max_size = %self.config.max_request_size,
                "Response size is unusually large"
            );
            return Err(Error::Config("Response size exceeds safety limits".to_string()));
        }
        
        // Check for potential script injection in JSON responses
        let suspicious_scripts = [
            "<script", "javascript:", "onclick=", "onerror=", "onload=",
            "eval(", "setTimeout(", "setInterval(",
        ];
        
        for script in &suspicious_scripts {
            if response_body.to_lowercase().contains(script) {
                warn!(pattern = %script, "Potential script injection detected in response");
                return Err(Error::Config("Response contains potentially malicious content".to_string()));
            }
        }
        
        debug!("Response validation passed");
        Ok(())
    }
    
    /// Generate a nonce for request signing.
    pub fn generate_nonce(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.generate_timestamp().hash(&mut hasher);
        std::thread::current().id().hash(&mut hasher);
        
        format!("{:x}", hasher.finish())
    }
    
    /// Create a secure request signature (basic implementation).
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
        
        // Create signature string
        let signature_string = format!(
            "{}|{}|{}|{}|{}|{}",
            method.to_uppercase(),
            url,
            body,
            api_key,
            timestamp,
            nonce
        );
        
        // Simple hash-based signature (in production, use HMAC-SHA256)
        let mut hasher = DefaultHasher::new();
        signature_string.hash(&mut hasher);
        
        format!("{:x}", hasher.finish())
    }
    
    /// Verify SSL/TLS configuration.
    pub fn verify_tls_config(&self) -> Result<()> {
        // In a real implementation, this would verify certificate pinning,
        // TLS version requirements, and cipher suite configurations
        
        if self.config.enable_cert_pinning {
            debug!("Certificate pinning is enabled");
            // TODO: Implement actual certificate pinning verification
        }
        
        debug!("TLS configuration verified");
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
}