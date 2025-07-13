use std::net::IpAddr;
use std::time::{Duration, Instant};
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts, NameServerConfig, Protocol};
use trust_dns_resolver::TokioAsyncResolver;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use tokio::task;
use std::path::PathBuf;
use std::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use url::Url;
use reqwest::dns::{Resolve, Resolving, Name, Addrs};
use futures_util::StreamExt;

// Original DNS servers constants
pub const DNS_SERVERS: &[&str] = &[
    "178.22.122.100", 
    "185.51.200.2",   
    "192.104.158.78",
    "194.104.158.48", 
    "172.29.0.100",   
    "172.29.2.100",   
    "10.202.10.202",  
    "10.202.10.102",  
    "185.55.226.26",  
    "185.55.225.25",  
    "10.202.10.10",   
    "10.202.10.11",   
    "37.27.41.228",   
    "87.107.52.11",   
    "87.107.52.13",   
    "5.202.100.100",
    "5.202.100.101",  
    "94.103.125.157", 
    "94.103.125.158", 
    "8.8.8.8",
    "8.8.4.4",
    "1.1.1.1",
    "1.0.0.1",
    "9.9.9.9",
    "149.112.112.112",
    "149.112.112.112",
];

pub const DNS_TIMEOUT_SECONDS: u64 = 5;
pub const DNS_ATTEMPTS: usize = 2;

// New constants for bulk testing
pub const DNS_CONFIG_URL: &str = "https://raw.githubusercontent.com/403unlocker/403Unlocker-cli/refs/heads/main/config/dns.yml";

// Original structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HttpStatus {
    Success,
    Forbidden403,
    Other(u16),
    Failed(String),
    NotTested,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnsTestResult {
    pub dns_server: String,
    pub status: bool,
    pub response_time: Option<u64>, 
    pub error_message: Option<String>,
    pub session_id: u64,
    pub http_status: HttpStatus,
    pub test_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UrlTestResult {
    pub url: String,
    pub dns_server: String,
    pub success: bool,
    pub response_time_ms: u64,
    pub status_code: Option<u16>,
    pub status_text: String,
    pub error_message: Option<String>,
    pub dns_lookup_time_ms: Option<u64>,
    pub http_request_time_ms: Option<u64>,
}

// New structures for bulk testing
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BulkTestResult {
    pub url: String,
    pub total_servers: usize,
    pub successful_servers: Vec<DnsServerResult>,
    pub failed_servers: Vec<DnsServerResult>,
    pub test_duration_ms: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnsServerResult {
    pub dns_server: String,
    pub status_code: Option<u16>,
    pub status_text: String,
    pub response_time_ms: u64,
    pub success: bool,
    pub error_message: Option<String>,
}

// Download speed testing structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadSpeedResult {
    pub dns_server: String,
    pub url: String,
    pub success: bool,
    pub download_speed_mbps: f64,
    pub downloaded_bytes: u64,
    pub test_duration_seconds: f64,
    pub error_message: Option<String>,
    pub resolution_time_ms: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BulkDownloadTestResult {
    pub url: String,
    pub timeout_seconds: u64,
    pub total_servers: usize,
    pub successful_tests: Vec<DownloadSpeedResult>,
    pub failed_tests: Vec<DownloadSpeedResult>,
    pub test_duration_ms: u64,
}

// Utility functions for config management
pub fn dns_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.join("bargozin").join("dns.yml")
}

pub async fn read_dns_file(path: &PathBuf) -> Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&content)?;
    let list = yaml["dnsServers"]
        .as_sequence()
        .ok_or_else(|| anyhow::anyhow!("dnsServers key missing"))?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();
    Ok(list)
}

pub async fn download_config_file(url: &str, path: &PathBuf) -> Result<()> {
    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let mut file = File::create(path).await?;
    file.write_all(&content).await?;
    Ok(())
}

pub fn ensure_https_url(input: &str) -> Option<Url> {
    let clean = input.trim().replace("http://", "").replace("https://", "");
    Url::parse(&format!("https://{}/", clean)).ok()
}

// Custom DNS resolver that uses a specific DNS server
struct CustomDnsResolver {
    resolver: TokioAsyncResolver,
}

impl CustomDnsResolver {
    fn new(dns_ip: &str) -> Option<Self> {
        let socket_addr = format!("{}:53", dns_ip).parse::<SocketAddr>().ok()?;
        
        let resolver_config = ResolverConfig::from_parts(
            None,
            vec![],
            vec![NameServerConfig {
                socket_addr,
                protocol: Protocol::Udp,
                tls_dns_name: None,
                trust_negative_responses: true,
                bind_addr: None,
            }],
        );

        let resolver = TokioAsyncResolver::tokio(resolver_config, ResolverOpts::default());
        Some(Self { resolver })
    }
}

impl Resolve for CustomDnsResolver {
    fn resolve(&self, name: Name) -> Resolving {
        let resolver = self.resolver.clone();
        Box::pin(async move {
            let response = resolver.lookup_ip(name.as_str()).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            
            let addrs: Vec<SocketAddr> = response
                .iter()
                .map(|ip| SocketAddr::new(ip, 443))  // Always use 443 for HTTPS URLs
                .collect();
            
            let addrs: Addrs = Box::new(addrs.into_iter());
            Ok(addrs)
        })
    }
}

pub async fn check_url_with_custom_dns(url: &Url, dns_ip: &str) -> Option<(u16, String)> {
    let resolver = CustomDnsResolver::new(dns_ip)?;
    
    let client = Client::builder()
        .dns_resolver(Arc::new(resolver))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (compatible; Bargozin-DNS-Tester)")
        .build()
        .ok()?;

    match client.get(url.as_str()).send().await {
        Ok(res) => {
            let code = res.status().as_u16();
            let msg = res.status().canonical_reason().unwrap_or("Unknown").to_string();
            Some((code, msg))
        }
        Err(_) => None
    }
}

pub async fn test_url_with_all_dns_servers(url: String) -> Result<BulkTestResult> {
    let start_time = std::time::Instant::now();
    
    // Ensure URL has HTTPS protocol
    let parsed_url = ensure_https_url(&url)
        .ok_or_else(|| anyhow::anyhow!("Invalid URL format"))?;

    // Get DNS servers list
    let dns_file_path = dns_config_path();
    let dns_list = match read_dns_file(&dns_file_path).await {
        Ok(list) => list,
        Err(_) => {
            // Download config file if it doesn't exist
            download_config_file(DNS_CONFIG_URL, &dns_file_path).await?;
            read_dns_file(&dns_file_path).await?
        }
    };

    // Test all DNS servers concurrently
    let tasks: Vec<_> = dns_list
        .into_iter()
        .map(|dns| {
            let url = parsed_url.clone();
            tokio::spawn(async move {
                let test_start = std::time::Instant::now();
                
                match check_url_with_custom_dns(&url, &dns).await {
                    Some((code, msg)) => {
                        let response_time = test_start.elapsed().as_millis() as u64;
                        DnsServerResult {
                            dns_server: dns,
                            status_code: Some(code),
                            status_text: msg,
                            response_time_ms: response_time,
                            success: code >= 200 && code < 400,
                            error_message: None,
                        }
                    }
                    None => {
                        let response_time = test_start.elapsed().as_millis() as u64;
                        DnsServerResult {
                            dns_server: dns,
                            status_code: None,
                            status_text: "Error".to_string(),
                            response_time_ms: response_time,
                            success: false,
                            error_message: Some("Connection failed or DNS resolution failed".to_string()),
                        }
                    }
                }
            })
        })
        .collect();

    // Wait for all tasks to complete
    let results = futures::future::join_all(tasks).await;
    let mut all_results: Vec<DnsServerResult> = Vec::new();
    
    for result in results {
        if let Ok(dns_result) = result {
            all_results.push(dns_result);
        }
    }

    // Separate successful and failed results
    let mut successful_servers = Vec::new();
    let mut failed_servers = Vec::new();

    for result in all_results {
        if result.success {
            successful_servers.push(result);
        } else {
            failed_servers.push(result);
        }
    }

    let total_servers = successful_servers.len() + failed_servers.len();
    let test_duration = start_time.elapsed().as_millis() as u64;

    Ok(BulkTestResult {
        url,
        total_servers,
        successful_servers,
        failed_servers,
        test_duration_ms: test_duration,
    })
}

// Original functions (keeping existing functionality)
pub async fn test_single_dns_server(domain: String, dns_server: String, session_id: u64) -> DnsTestResult {
    let start_time = std::time::Instant::now();
    
    // Ensure HTTPS URL like in CLI code
    let url_string = ensure_https(&domain);
    let parsed_url = match ensure_https_url(&domain) {
        Some(url) => url,
        None => {
            return DnsTestResult {
                dns_server,
                status: false,
                response_time: Some(start_time.elapsed().as_millis() as u64),
                error_message: Some("Invalid domain format".to_string()),
                session_id,
                http_status: HttpStatus::Failed("Invalid domain".to_string()),
                test_url: Some(url_string),
            };
        }
    };
    
    // Use custom DNS resolver like in CLI
    match check_url_with_custom_dns(&parsed_url, &dns_server).await {
        Some((status_code, status_msg)) => {
            let response_time = start_time.elapsed().as_millis() as u64;
            
            let http_status = match status_code {
                200..=299 => HttpStatus::Success,
                403 => HttpStatus::Forbidden403,
                _ => HttpStatus::Other(status_code),
            };
            
            // Consider 200-299 as usable (like CLI)
            let is_usable = status_code >= 200 && status_code < 300;
            
            DnsTestResult {
                dns_server,
                status: is_usable,
                response_time: Some(response_time),
                error_message: if is_usable { 
                    None 
                } else { 
                    Some(format!("HTTP {} - {}", status_code, status_msg)) 
                },
                session_id,
                http_status,
                test_url: Some(url_string),
            }
        }
        None => {
            let response_time = start_time.elapsed().as_millis() as u64;
            DnsTestResult {
                dns_server,
                status: false,
                response_time: Some(response_time),
                error_message: Some("DNS resolution or HTTP request failed".to_string()),
                session_id,
                http_status: HttpStatus::Failed("Connection failed".to_string()),
                test_url: Some(url_string),
            }
        }
    }
}

fn ensure_https(domain: &str) -> String {
    let mut url = domain.to_string();
    
    // Remove existing protocol if present
    if url.starts_with("http://") {
        url = url.strip_prefix("http://").unwrap().to_string();
    }
    if url.starts_with("https://") {
        url = url.strip_prefix("https://").unwrap().to_string();
    }
    
    // Add https:// and ensure it ends with /
    let mut result = format!("https://{}", url);
    
    // Parse to extract just the host part (like Go code)
    if let Ok(parsed) = url::Url::parse(&result) {
        if let Some(host) = parsed.host_str() {
            result = format!("https://{}/", host);
        }
    }
    
    result
}

fn create_resolver_config(dns_ip: IpAddr) -> ResolverConfig {
    let name_server = NameServerConfig {
        socket_addr: (dns_ip, 53).into(),
        protocol: Protocol::Udp,
        tls_dns_name: None,
        trust_negative_responses: false,
        bind_addr: None,
    };

    let mut config = ResolverConfig::new();
    config.add_name_server(name_server);
    config
}

fn create_resolver_opts() -> ResolverOpts {
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(DNS_TIMEOUT_SECONDS);
    opts.attempts = DNS_ATTEMPTS;
    opts
}

/// Test a URL using a specific DNS server in a separate thread
/// Returns detailed timing and response information
pub async fn test_url_with_dns(url: String, dns_server: String) -> UrlTestResult {
    let url_clone = url.clone();
    let dns_server_clone = dns_server.clone();
    
    // Spawn the test in a separate task (thread)
    let handle = task::spawn(async move {
        test_url_with_dns_impl(url_clone, dns_server_clone).await
    });
    
    // Wait for the result
    match handle.await {
        Ok(result) => result,
        Err(e) => UrlTestResult {
            url,
            dns_server,
            success: false,
            response_time_ms: 0,
            status_code: None,
            status_text: "Thread Error".to_string(),
            error_message: Some(format!("Failed to execute in thread: {}", e)),
            dns_lookup_time_ms: None,
            http_request_time_ms: None,
        },
    }
}

async fn test_url_with_dns_impl(url: String, dns_server: String) -> UrlTestResult {
    let start_time = std::time::Instant::now();
    
    // Parse DNS server IP
    let dns_ip: IpAddr = match dns_server.parse() {
        Ok(ip) => ip,
        Err(e) => {
            return UrlTestResult {
                url,
                dns_server,
                success: false,
                response_time_ms: start_time.elapsed().as_millis() as u64,
                status_code: None,
                status_text: "Invalid DNS Server".to_string(),
                error_message: Some(format!("Invalid DNS server IP: {}", e)),
                dns_lookup_time_ms: None,
                http_request_time_ms: None,
            };
        }
    };

    // Parse URL to get domain
    let parsed_url = match url::Url::parse(&url) {
        Ok(u) => u,
        Err(e) => {
            return UrlTestResult {
                url,
                dns_server,
                success: false,
                response_time_ms: start_time.elapsed().as_millis() as u64,
                status_code: None,
                status_text: "Invalid URL".to_string(),
                error_message: Some(format!("Invalid URL: {}", e)),
                dns_lookup_time_ms: None,
                http_request_time_ms: None,
            };
        }
    };

    let domain = match parsed_url.host_str() {
        Some(host) => host,
        None => {
            return UrlTestResult {
                url,
                dns_server,
                success: false,
                response_time_ms: start_time.elapsed().as_millis() as u64,
                status_code: None,
                status_text: "No Host in URL".to_string(),
                error_message: Some("No host found in URL".to_string()),
                dns_lookup_time_ms: None,
                http_request_time_ms: None,
            };
        }
    };

    // Step 1: DNS Lookup
    let dns_start = std::time::Instant::now();
    let resolver_config = create_resolver_config(dns_ip);
    let resolver_opts = create_resolver_opts();
    let resolver = TokioAsyncResolver::tokio(resolver_config, resolver_opts);

    let resolved_ips = match resolver.lookup_ip(domain).await {
        Ok(lookup_result) => {
            let ips: Vec<IpAddr> = lookup_result.iter().collect();
            if ips.is_empty() {
                return UrlTestResult {
                    url,
                    dns_server,
                    success: false,
                    response_time_ms: start_time.elapsed().as_millis() as u64,
                    status_code: None,
                    status_text: "DNS Resolution Failed".to_string(),
                    error_message: Some("No IP addresses found for domain".to_string()),
                    dns_lookup_time_ms: Some(dns_start.elapsed().as_millis() as u64),
                    http_request_time_ms: None,
                };
            }
            ips
        }
        Err(e) => {
            return UrlTestResult {
                url,
                dns_server,
                success: false,
                response_time_ms: start_time.elapsed().as_millis() as u64,
                status_code: None,
                status_text: "DNS Lookup Failed".to_string(),
                error_message: Some(format!("DNS lookup failed: {}", e)),
                dns_lookup_time_ms: Some(dns_start.elapsed().as_millis() as u64),
                http_request_time_ms: None,
            };
        }
    };

    let dns_lookup_time = dns_start.elapsed().as_millis() as u64;

    // Step 2: HTTP Request using resolved IP
    let http_start = std::time::Instant::now();
    
    let client = Client::builder()
        .timeout(Duration::from_secs(DNS_TIMEOUT_SECONDS))
        .build();

    let client = match client {
        Ok(c) => c,
        Err(e) => {
            return UrlTestResult {
                url,
                dns_server,
                success: false,
                response_time_ms: start_time.elapsed().as_millis() as u64,
                status_code: None,
                status_text: "HTTP Client Error".to_string(),
                error_message: Some(format!("Failed to create HTTP client: {}", e)),
                dns_lookup_time_ms: Some(dns_lookup_time),
                http_request_time_ms: None,
            };
        }
    };

    // Use the first resolved IP to make the request
    let target_ip = resolved_ips[0];
    let port = parsed_url.port_or_known_default().unwrap_or(80);
    let scheme = parsed_url.scheme();
    let path = parsed_url.path();
    let query = parsed_url.query().map(|q| format!("?{}", q)).unwrap_or_default();
    
    let request_url = format!("{}://{}:{}{}{}", scheme, target_ip, port, path, query);

    match client.get(&request_url)
        .header("Host", domain)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .await
    {
        Ok(response) => {
            let http_request_time = http_start.elapsed().as_millis() as u64;
            let total_time = start_time.elapsed().as_millis() as u64;
            let status_code = response.status().as_u16();
            
            let (success, status_text) = match status_code {
                200..=299 => (true, "Success".to_string()),
                403 => (false, "Forbidden (403)".to_string()),
                404 => (false, "Not Found (404)".to_string()),
                500..=599 => (false, format!("Server Error ({})", status_code)),
                _ => (false, format!("HTTP {}", status_code)),
            };

            UrlTestResult {
                url,
                dns_server,
                success,
                response_time_ms: total_time,
                status_code: Some(status_code),
                status_text,
                error_message: if success { None } else { Some(format!("HTTP {}", status_code)) },
                dns_lookup_time_ms: Some(dns_lookup_time),
                http_request_time_ms: Some(http_request_time),
            }
        }
        Err(e) => {
            let http_request_time = http_start.elapsed().as_millis() as u64;
            let total_time = start_time.elapsed().as_millis() as u64;
            
            UrlTestResult {
                url,
                dns_server,
                success: false,
                response_time_ms: total_time,
                status_code: None,
                status_text: "Request Failed".to_string(),
                error_message: Some(format!("HTTP request failed: {}", e)),
                dns_lookup_time_ms: Some(dns_lookup_time),
                http_request_time_ms: Some(http_request_time),
            }
        }
    }
}

// Download speed testing functions
async fn resolve_host_with_dns(host: &str, dns_server: &str) -> anyhow::Result<IpAddr> {
    let socket_addr: SocketAddr = format!("{}:53", dns_server).parse()?;
    let nameserver = NameServerConfig {
        socket_addr,
        protocol: Protocol::Udp,
        tls_dns_name: None,
        bind_addr: None,
        trust_negative_responses: false,
    };

    let resolver_config = ResolverConfig::from_parts(None, vec![], vec![nameserver]);
    
    // Configure resolver options with a reasonable timeout for DNS resolution
    let mut resolver_opts = ResolverOpts::default();
    resolver_opts.timeout = Duration::from_secs(5); // 5 second timeout for DNS resolution
    resolver_opts.attempts = 2; // 2 attempts max
    
    let resolver = TokioAsyncResolver::tokio(resolver_config, resolver_opts);

    let response = resolver.lookup_ip(host).await?;
    response
        .iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No IP found for host"))
}

async fn download_with_custom_dns(url: &str, dns_ip: &str, timeout_seconds: u64) -> anyhow::Result<DownloadSpeedResult> {
    println!("Starting download test: {} with DNS: {}", url, dns_ip);
    
    // Start the overall timer from the beginning (includes DNS resolution + connection + download)
    let overall_start = Instant::now();
    let timeout_duration = std::time::Duration::from_secs(timeout_seconds);
    
    let resolution_start = Instant::now();
    
    let parsed_url = reqwest::Url::parse(url)?;
    let host = parsed_url.host_str().ok_or_else(|| anyhow::anyhow!("Invalid host"))?;
    
    println!("Parsed URL - host: {}, scheme: {}", host, parsed_url.scheme());

    // Resolve host with custom DNS with timeout
    println!("Resolving {} using DNS {}", host, dns_ip);
    
    // Apply timeout to DNS resolution
    let resolved_ip = tokio::time::timeout(
        timeout_duration,
        resolve_host_with_dns(host, dns_ip)
    ).await
    .map_err(|_| anyhow::anyhow!("DNS resolution timed out after {} seconds", timeout_seconds))?
    .map_err(|e| anyhow::anyhow!("DNS resolution failed: {}", e))?;
    
    let resolution_time_ms = resolution_start.elapsed().as_millis() as u64;
    println!("DNS resolution successful: {} -> {} ({}ms)", host, resolved_ip, resolution_time_ms);

    // Check if we still have time left after DNS resolution
    if overall_start.elapsed() >= timeout_duration {
        return Err(anyhow::anyhow!("Operation timed out during DNS resolution"));
    }

    // Determine port based on scheme
    let port = match parsed_url.scheme() {
        "https" => 443,
        "http" => 80,
        _ => return Err(anyhow::anyhow!("Unsupported scheme")),
    };

    let socket_addr = SocketAddr::new(resolved_ip, port);

    // Calculate remaining time for HTTP operations
    let remaining_time = timeout_duration.saturating_sub(overall_start.elapsed());
    if remaining_time.is_zero() {
        return Err(anyhow::anyhow!("Operation timed out before HTTP request"));
    }

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(remaining_time) // Use remaining time, not extra time
        .resolve(host, socket_addr)
        .build()?;

    let download_start = Instant::now();
    
    let response = client.get(url).send().await
        .map_err(|e| anyhow::anyhow!("HTTP request failed: {}", e))?;

    let mut downloaded_bytes = 0u64;
    let mut stream = response.bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        // Check if overall timeout has been reached
        if overall_start.elapsed() >= timeout_duration {
            break;
        }

        let chunk = chunk_result
            .map_err(|e| anyhow::anyhow!("Stream error: {}", e))?;
        downloaded_bytes += chunk.len() as u64;
        
        // Add periodic check for cancellation (every 1MB or every 1 second)
        if downloaded_bytes % (1024 * 1024) == 0 || download_start.elapsed().as_secs() > 1 {
            tokio::task::yield_now().await; // Allow other tasks to run and check for cancellation
        }
    }

    let elapsed = overall_start.elapsed().as_secs_f64(); // Use overall elapsed time
    let speed_mbps = (downloaded_bytes as f64 * 8.0) / (elapsed * 1_000_000.0);

    Ok(DownloadSpeedResult {
        dns_server: dns_ip.to_string(),
        url: url.to_string(),
        success: true,
        download_speed_mbps: speed_mbps,
        downloaded_bytes,
        test_duration_seconds: elapsed,
        error_message: None,
        resolution_time_ms: Some(resolution_time_ms),
    })
}

pub async fn test_download_speed_with_dns(url: String, dns_server: String, timeout_seconds: u64) -> DownloadSpeedResult {
    // Add a small delay to allow for cancellation check
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    
    match download_with_custom_dns(&url, &dns_server, timeout_seconds).await {
        Ok(result) => result,
        Err(e) => DownloadSpeedResult {
            dns_server,
            url,
            success: false,
            download_speed_mbps: 0.0,
            downloaded_bytes: 0,
            test_duration_seconds: 0.0,
            error_message: Some(e.to_string()),
            resolution_time_ms: None,
        },
    }
}