use tauri::{Emitter, AppHandle};
use crate::dns::{DNS_SERVERS, test_single_dns_server};
use crate::utils::validate_domain;

#[tauri::command]
pub async fn test_dns_servers(domain: String, app_handle: AppHandle) -> Result<(), String> {
    let domain = domain.trim();
    
    if !validate_domain(domain) {
        return Err("Please enter a valid domain name".to_string());
    }

    let mut handles = Vec::new();

    
    for &dns_server in DNS_SERVERS {
        let domain_clone = domain.to_string();
        let dns_server_string = dns_server.to_string();
        let app_handle_clone = app_handle.clone();
        
        let handle = tokio::spawn(async move {
            let result = test_single_dns_server(domain_clone, dns_server_string).await;
            
            if let Err(e) = app_handle_clone.emit("dns-test-result", &result) {
                eprintln!("Failed to emit DNS test result: {}", e);
            }
            
            result
        });
        
        handles.push(handle);
    }

    
    // let mut results = Vec::new();
    // for handle in handles {
    //     match handle.await {
    //         Ok(result) => results.push(result),
    //         Err(e) => eprintln!("Task join error: {}", e),
    //     }
    // }

    
    if let Err(e) = app_handle.emit("dns-test-complete", ()) {
        eprintln!("Failed to emit completion event: {}", e);
    }

    // println!("DNS test completed for domain '{}'. Total results: {}", domain, results.len());
    Ok(())
} 