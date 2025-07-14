use tauri::{Emitter, AppHandle};
use crate::dns::{DNS_SERVERS, test_single_dns_server, test_url_with_dns, UrlTestResult, test_url_with_all_dns_servers, BulkTestResult, test_download_speed_with_dns, DownloadSpeedResult};
use crate::docker::{test_docker_registry_download_speed, validate_docker_image_name, docker_config_path, read_docker_registries_file, download_docker_config_file, DOCKER_CONFIG_URL};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::task::JoinHandle;
use std::sync::atomic::{AtomicU64, Ordering};

// Global storage for active DNS test tasks
lazy_static::lazy_static! {
    static ref ACTIVE_TASKS: Arc<Mutex<HashMap<String, Vec<JoinHandle<()>>>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref CURRENT_SESSION: AtomicU64 = AtomicU64::new(1);
}

#[tauri::command]
pub async fn test_dns_servers(domain: String, app_handle: AppHandle) -> Result<(), String> {
    let domain = domain.trim().to_string();
    
    if domain.is_empty() {
        return Err("Please enter a valid domain name".to_string());
    }

    // Get new session ID and increment counter
    let session_id = CURRENT_SESSION.fetch_add(1, Ordering::SeqCst);

    // Cancel any existing tests
    {
        let mut active_tasks = ACTIVE_TASKS.lock().unwrap();
        if let Some(handles) = active_tasks.remove(&domain) {
            for handle in handles {
                handle.abort();
            }
        }
    }

    let mut handles = Vec::new();

    for &dns_server in DNS_SERVERS {
        let domain_clone = domain.clone();
        let dns_server_string = dns_server.to_string();
        let app_handle_clone = app_handle.clone();
        
        let handle = tokio::spawn(async move {
            let result = test_single_dns_server(domain_clone, dns_server_string, session_id).await;
            
            if let Err(e) = app_handle_clone.emit("dns-test-result", &result) {
                eprintln!("Failed to emit DNS test result: {}", e);
            }
        });
        
        handles.push(handle);
    }

    // Store handles for potential cancellation
    {
        let mut active_tasks = ACTIVE_TASKS.lock().unwrap();
        active_tasks.insert(domain.clone(), handles);
    }

    // Wait for all tasks to complete by removing them from storage
    let stored_handles = {
        let mut active_tasks = ACTIVE_TASKS.lock().unwrap();
        active_tasks.remove(&domain).unwrap_or_default()
    };

    for handle in stored_handles {
        let _ = handle.await;
    }

    if let Err(e) = app_handle.emit("dns-test-complete", ()) {
        eprintln!("Failed to emit completion event: {}", e);
    }
    Ok(())
}

#[tauri::command]
pub async fn get_current_session() -> Result<u64, String> {
    Ok(CURRENT_SESSION.load(Ordering::SeqCst))
}

#[tauri::command]
pub async fn cancel_dns_tests() -> Result<(), String> {
    let mut active_tasks = ACTIVE_TASKS.lock().unwrap();
    
    // Only increment session if there are actually tasks to cancel
    if !active_tasks.is_empty() {
        let new_session = CURRENT_SESSION.fetch_add(1, Ordering::SeqCst);
        println!("Cancelling DNS tests, new session: {}", new_session);
        
        for (domain, handles) in active_tasks.drain() {
            println!("Cancelling DNS tests for domain: {}", domain);
            for handle in handles {
                handle.abort();
            }
        }
    } else {
        println!("No DNS tests to cancel");
    }
    
    Ok(())
}

#[tauri::command]
pub async fn cancel_download_tests() -> Result<(), String> {
    {
        let mut active_tasks = ACTIVE_TASKS.lock().unwrap();
        
        // Only increment session if there are actually tasks to cancel
        if !active_tasks.is_empty() {
            let new_session = CURRENT_SESSION.fetch_add(1, Ordering::SeqCst);
            println!("Cancelling download tests, new session: {}", new_session);
            
            // Cancel all active download tasks
            for (identifier, handles) in active_tasks.drain() {
                println!("Cancelling download tests for: {}", identifier);
                for handle in handles {
                    handle.abort();
                }
            }
        } else {
            println!("No download tests to cancel");
        }
    } // MutexGuard is dropped here
    
    // Give a small delay to allow tasks to properly cancel
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    
    Ok(())
}

#[tauri::command]
pub async fn test_url_with_dns_server(url: String, dns_server: String) -> Result<UrlTestResult, String> {
    let url = url.trim();
    let dns_server = dns_server.trim();
    
    if url.is_empty() {
        return Err("Please enter a valid URL".to_string());
    }
    
    if dns_server.is_empty() {
        return Err("Please enter a valid DNS server".to_string());
    }
    
    // Validate URL format
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("URL must start with http:// or https://".to_string());
    }
    
    // Call the DNS testing function
    let result = test_url_with_dns(url.to_string(), dns_server.to_string()).await;
    Ok(result)
}

#[tauri::command]
pub async fn bulk_test_url_with_all_dns_servers(url: String) -> Result<BulkTestResult, String> {
    let url = url.trim();
    
    if url.is_empty() {
        return Err("Please enter a valid URL".to_string());
    }
    
    // Validate URL format
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("URL must start with http:// or https://".to_string());
    }
    
    // Call the bulk testing function
    match test_url_with_all_dns_servers(url.to_string()).await {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Bulk test failed: {}", e)),
    }
}

#[tauri::command]
pub async fn test_download_speed_single_dns(url: String, dns_server: String, timeout_seconds: u64) -> Result<DownloadSpeedResult, String> {
    let url = url.trim();
    let dns_server = dns_server.trim();
    
    if url.is_empty() {
        return Err("Please enter a valid URL".to_string());
    }
    
    if dns_server.is_empty() {
        return Err("Please enter a valid DNS server".to_string());
    }
    
    // Validate URL format
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("URL must start with http:// or https://".to_string());
    }
    
    // Call the download speed testing function
    let result = test_download_speed_with_dns(url.to_string(), dns_server.to_string(), timeout_seconds).await;
    Ok(result)
}

#[tauri::command]
pub async fn test_download_speed_all_dns(url: String, timeout_seconds: u64, app_handle: AppHandle) -> Result<(), String> {
    let url = url.trim().to_string();
    
    if url.is_empty() {
        return Err("Please enter a valid URL".to_string());
    }
    
    // Validate URL format
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("URL must start with http:// or https://".to_string());
    }

    // Use current session ID, increment if it's 0 (first run)
    let session_id = {
        let current = CURRENT_SESSION.load(Ordering::SeqCst);
        if current == 0 {
            CURRENT_SESSION.fetch_add(1, Ordering::SeqCst) + 1
        } else {
            current
        }
    };

    // Cancel any existing tests
    {
        let mut active_tasks = ACTIVE_TASKS.lock().unwrap();
        if let Some(handles) = active_tasks.remove(&url) {
            for handle in handles {
                handle.abort();
            }
        }
    }

    // Clone URL for task storage
    let url_for_storage = url.clone();
    
    // Create the main testing task
    let main_task = tokio::spawn(async move {
        println!("Starting download tests for session: {}", session_id);
        
        // Test DNS servers sequentially
        for (index, &dns_server) in DNS_SERVERS.iter().enumerate() {
            // Check if the current session is still valid (not cancelled)
            let current_session = CURRENT_SESSION.load(Ordering::SeqCst);
            if current_session != session_id {
                println!("Download test cancelled, session changed: {} -> {}", session_id, current_session);
                return;
            }

            println!("Testing DNS server {} ({}/{}): {}", dns_server, index + 1, DNS_SERVERS.len(), dns_server);
            
            let url_clone = url.clone();
            let dns_server_string = dns_server.to_string();
            let app_handle_clone = app_handle.clone();
            
            let result = test_download_speed_with_dns(url_clone, dns_server_string, timeout_seconds).await;
            
            println!("Download test result for {}: success={}, speed={:.3} Mbps", 
                     result.dns_server, result.success, result.download_speed_mbps);
            
            // Check session again before emitting result
            let current_session = CURRENT_SESSION.load(Ordering::SeqCst);
            if current_session != session_id {
                println!("Download test cancelled before emit, session changed: {} -> {}", session_id, current_session);
                return;
            }
            
            if let Err(e) = app_handle_clone.emit("download-test-result", &result) {
                eprintln!("Failed to emit download test result: {}", e);
            } else {
                println!("Successfully emitted result for {}", result.dns_server);
            }
        }

        // Check session one final time before completion
        let current_session = CURRENT_SESSION.load(Ordering::SeqCst);
        if current_session == session_id {
            println!("All download tests completed for session: {}", session_id);
            if let Err(e) = app_handle.emit("download-test-complete", ()) {
                eprintln!("Failed to emit completion event: {}", e);
            } else {
                println!("Successfully emitted completion event");
            }
        }
    });

    // Store the main task for potential cancellation
    {
        let mut active_tasks = ACTIVE_TASKS.lock().unwrap();
        active_tasks.insert(url_for_storage.clone(), vec![main_task]);
    }

    // Wait for the main task to complete
    let stored_handles = {
        let mut active_tasks = ACTIVE_TASKS.lock().unwrap();
        active_tasks.remove(&url_for_storage).unwrap_or_default()
    };

    for handle in stored_handles {
        let _ = handle.await;
    }
    
    Ok(())
}

// Docker Registry Testing Commands - SIMPLIFIED

#[tauri::command]
pub async fn test_docker_registries(image_name: String, timeout_seconds: u64, app_handle: AppHandle) -> Result<(), String> {
    let image_name = image_name.trim().to_string();
    
    if image_name.is_empty() {
        return Err("Please enter a valid Docker image name".to_string());
    }

    // Validate image name
    if !validate_docker_image_name(&image_name) {
        return Err("Invalid Docker image name format".to_string());
    }

    // Get new session ID and increment counter
    let session_id = CURRENT_SESSION.fetch_add(1, Ordering::SeqCst);
    println!("Starting Docker registry tests for image: {} (session: {})", image_name, session_id);

    // Get registries list
    let docker_file_path = docker_config_path();
    let registries = match read_docker_registries_file(&docker_file_path).await {
        Ok(list) => list,
        Err(_) => {
            // Download config file if it doesn't exist
            if let Err(e) = download_docker_config_file(DOCKER_CONFIG_URL, &docker_file_path).await {
                return Err(format!("Failed to download Docker registry config: {}", e));
            }
            match read_docker_registries_file(&docker_file_path).await {
                Ok(list) => list,
                Err(e) => {
                    return Err(format!("Failed to read Docker registry config: {}", e));
                }
            }
        }
    };

    println!("Testing {} registries sequentially with {}s timeout", registries.len(), timeout_seconds);

    // Test registries one by one
    for (index, registry) in registries.iter().enumerate() {
        println!("Testing registry {}/{}: {}", index + 1, registries.len(), registry);
        
        let mut result = test_docker_registry_download_speed(registry, &image_name, timeout_seconds).await;
        
        // Set the correct session ID
        result.session_id = session_id;
        
        println!("Registry {} test completed: success={}, speed={:.2} Mbps", 
                 registry, result.success, result.download_speed_mbps);
        
        // Emit result immediately
        if let Err(e) = app_handle.emit("docker-registry-test-result", &result) {
            eprintln!("Failed to emit Docker registry test result: {}", e);
        } else {
            println!("Successfully emitted result for {}", registry);
        }
    }

    // All tests completed
    println!("All Docker registry tests completed");
    if let Err(e) = app_handle.emit("docker-registry-test-complete", ()) {
        eprintln!("Failed to emit Docker registry test completion event: {}", e);
    } else {
        println!("Successfully emitted completion event");
    }

    Ok(())
}

#[tauri::command]
pub async fn cancel_docker_registry_tests() -> Result<(), String> {
    println!("Docker registry test cancellation not needed with simplified approach");
    Ok(())
}

#[tauri::command]
pub async fn validate_docker_image(image_name: String) -> Result<bool, String> {
    Ok(validate_docker_image_name(&image_name))
}
