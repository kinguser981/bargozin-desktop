use std::fs::File;
use std::io::Write;
use std::net::IpAddr;
use std::time::Duration;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts, NameServerConfig, Protocol};
use trust_dns_resolver::TokioAsyncResolver;
use serde::{Deserialize, Serialize};
use tauri::{Emitter, AppHandle};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnsTestResult {
    pub dns_server: String,
    pub status: bool,
    pub response_time: Option<u64>, // in milliseconds
    pub error_message: Option<String>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_file(path: &str) -> String {
    let mut file = File::create(path).unwrap();
    file.write_all(b"Hello, world!").unwrap();
    "File created".to_string()
}

#[tauri::command]
async fn test_dns_servers(domain: String, app_handle: AppHandle) -> Result<(), String> {
    let dns_servers = vec![
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
    ];

    let mut handles = Vec::new();

    // Test each DNS server concurrently
    for dns_server in dns_servers {
        let domain_clone = domain.clone();
        let dns_server_clone = dns_server.to_string();
        let app_handle_clone = app_handle.clone();
        
        let handle = tokio::spawn(async move {
            let result = test_single_dns_server(domain_clone, dns_server_clone).await;
            
            // Emit the result immediately when it's ready
            if let Err(e) = app_handle_clone.emit("dns-test-result", &result) {
                eprintln!("Failed to emit DNS test result: {}", e);
            }
            
            result
        });
        
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Task join error: {}", e);
        }
    }

    // Emit completion event
    if let Err(e) = app_handle.emit("dns-test-complete", ()) {
        eprintln!("Failed to emit completion event: {}", e);
    }

    Ok(())
}

async fn test_single_dns_server(domain: String, dns_server: String) -> DnsTestResult {
    let start_time = std::time::Instant::now();
    
    // Parse the DNS server IP
    let dns_ip: IpAddr = match dns_server.parse() {
        Ok(ip) => ip,
        Err(e) => {
            return DnsTestResult {
                dns_server,
                status: false,
                response_time: None,
                error_message: Some(format!("Invalid DNS server IP: {}", e)),
            };
        }
    };

    // Create a custom resolver configuration
    let name_server = NameServerConfig {
        socket_addr: (dns_ip, 53).into(),
        protocol: Protocol::Udp,
        tls_dns_name: None,
        trust_negative_responses: false,
        bind_addr: None,
    };

    let mut config = ResolverConfig::new();
    config.add_name_server(name_server);

    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(5); // 5 second timeout
    opts.attempts = 2;

    // Create resolver
    let resolver = TokioAsyncResolver::tokio(config, opts);

    // Perform DNS lookup
    match resolver.lookup_ip(&domain).await {
        Ok(lookup_result) => {
            let response_time = start_time.elapsed().as_millis() as u64;
            let has_ips = lookup_result.iter().count() > 0;
            
            DnsTestResult {
                dns_server,
                status: has_ips,
                response_time: Some(response_time),
                error_message: if has_ips { None } else { Some("No IP addresses found".to_string()) },
            }
        }
        Err(e) => {
            let response_time = start_time.elapsed().as_millis() as u64;
            DnsTestResult {
                dns_server,
                status: false,
                response_time: Some(response_time),
                error_message: Some(format!("DNS lookup failed: {}", e)),
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, create_file, test_dns_servers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
