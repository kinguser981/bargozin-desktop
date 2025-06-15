pub fn validate_domain(domain: &str) -> bool {
    !domain.trim().is_empty() 
        && domain.contains('.') 
        && !domain.starts_with('.') 
        && !domain.ends_with('.')
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
        assert!(validate_domain("example.com"));
        assert!(validate_domain("subdomain.example.com"));
        assert!(!validate_domain(""));
        assert!(!validate_domain(".com"));
        assert!(!validate_domain("example."));
        assert!(!validate_domain("invalid"));
    }

    #[test]
    fn test_format_response_time() {
        assert_eq!(format_response_time(500), "500ms");
        assert_eq!(format_response_time(1500), "1.5s");
        assert_eq!(format_response_time(2000), "2.0s");
    }
} 