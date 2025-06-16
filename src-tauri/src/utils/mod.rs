pub fn validate_domain(domain: &str) -> bool {
    let domain = domain.trim();
    
    // Reject empty strings
    if domain.is_empty() {
        return false;
    }
    
    // Reject URLs (protocols)
    if domain.starts_with("http://") 
        || domain.starts_with("https://") 
        || domain.starts_with("ftp://") 
        || domain.starts_with("://") {
        return false;
    }
    
    // Reject if contains URL patterns
    if domain.contains("://") || domain.contains("?") || domain.contains("#") {
        return false;
    }
    
    // Reject if contains path separators (except at end which we'll handle)
    if domain.contains('/') {
        return false;
    }
    
    // Must contain at least one dot
    if !domain.contains('.') {
        return false;
    }
    
    // Cannot start or end with dot
    if domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }
    
    // Cannot start or end with hyphen
    if domain.starts_with('-') || domain.ends_with('-') {
        return false;
    }
    
    // Check each label (part separated by dots)
    let labels: Vec<&str> = domain.split('.').collect();
    
    // Must have at least 2 labels (e.g., "example.com")
    if labels.len() < 2 {
        return false;
    }
    
    // Validate each label
    for label in &labels {
        if !validate_domain_label(label) {
            return false;
        }
    }
    
    // TLD (last label) must be at least 2 characters and only letters
    if let Some(tld) = &labels.last() {
        if tld.len() < 2 || !tld.chars().all(|c| c.is_ascii_alphabetic()) {
            return false;
        }
    }
    
    true
}

fn validate_domain_label(label: &str) -> bool {
    // Label cannot be empty
    if label.is_empty() {
        return false;
    }
    
    // Label cannot be longer than 63 characters
    if label.len() > 63 {
        return false;
    }
    
    // Label cannot start or end with hyphen
    if label.starts_with('-') || label.ends_with('-') {
        return false;
    }
    
    // Label can only contain letters, numbers, and hyphens
    label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

pub fn format_response_time(time_ms: u64) -> String {
    if time_ms < 1000 {
        format!("{}ms", time_ms)
    } else {
        format!("{:.1}s", time_ms as f64 / 1000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_domain() {
        // Valid domains
        assert!(validate_domain("example.com"));
        assert!(validate_domain("subdomain.example.com"));
        assert!(validate_domain("google.com"));
        assert!(validate_domain("dl2.soft98.ir"));
        assert!(validate_domain("test-site.org"));
        
        // Invalid: empty or malformed
        assert!(!validate_domain(""));
        assert!(!validate_domain("   "));
        assert!(!validate_domain(".com"));
        assert!(!validate_domain("example."));
        assert!(!validate_domain("invalid"));
        assert!(!validate_domain("no-tld"));
        
        // Invalid: URLs (should be rejected)
        assert!(!validate_domain("https://dl2.soft98.ir/soft/i/Internet.Download.Manager.6.42.41.zip?1750036701"));
        assert!(!validate_domain("http://example.com"));
        assert!(!validate_domain("https://google.com"));
        assert!(!validate_domain("ftp://files.example.com"));
        
        // Invalid: contains URL patterns
        assert!(!validate_domain("example.com/path"));
        assert!(!validate_domain("example.com?query=1"));
        assert!(!validate_domain("example.com#fragment"));
        assert!(!validate_domain("example.com:8080"));
        
        // Invalid: malformed domains
        assert!(!validate_domain("-example.com"));
        assert!(!validate_domain("example-.com"));
        assert!(!validate_domain("example.c"));  // TLD too short
        assert!(!validate_domain("example.123")); // TLD must be letters
    }

    #[test]
    fn test_format_response_time() {
        assert_eq!(format_response_time(500), "500ms");
        assert_eq!(format_response_time(1500), "1.5s");
        assert_eq!(format_response_time(2000), "2.0s");
    }
} 