use std::net::IpAddr;
use std::time::Duration;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts, NameServerConfig, Protocol};
use trust_dns_resolver::TokioAsyncResolver;
use serde::{Deserialize, Serialize};


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


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnsTestResult {
    pub dns_server: String,
    pub status: bool,
    pub response_time: Option<u64>, 
    pub error_message: Option<String>,
    pub session_id: u64,
}


pub async fn test_single_dns_server(domain: String, dns_server: String, session_id: u64) -> DnsTestResult {
    let start_time = std::time::Instant::now();
    
    
    let dns_ip: IpAddr = match dns_server.parse() {
        Ok(ip) => ip,
        Err(e) => {
            return DnsTestResult {
                dns_server,
                status: false,
                response_time: None,
                error_message: Some(format!("Invalid DNS server IP: {}", e)),
                session_id,
            };
        }
    };

    
    let resolver_config = create_resolver_config(dns_ip);
    let resolver_opts = create_resolver_opts();

    
    let resolver = TokioAsyncResolver::tokio(resolver_config, resolver_opts);

    
    match resolver.lookup_ip(&domain).await {
        Ok(lookup_result) => {
            let response_time = start_time.elapsed().as_millis() as u64;
            let has_ips = lookup_result.iter().count() > 0;
            
            DnsTestResult {
                dns_server,
                status: has_ips,
                response_time: Some(response_time),
                error_message: if has_ips { 
                    None 
                } else { 
                    Some("No IP addresses found".to_string()) 
                },
                session_id,
            }
        }
        Err(e) => {
            let response_time = start_time.elapsed().as_millis() as u64;
            DnsTestResult {
                dns_server,
                status: false,
                response_time: Some(response_time),
                error_message: Some(format!("DNS lookup failed: {}", e)),
                session_id,
            }
        }
    }
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