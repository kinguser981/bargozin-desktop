use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::sync::Arc;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use anyhow::Result;
use regex::Regex;
use futures_util::StreamExt;
use std::sync::atomic::{AtomicU64, Ordering};

pub const DOCKER_CONFIG_URL: &str = "https://raw.githubusercontent.com/403unlocker/403Unlocker-cli/refs/heads/main/config/dockerRegistry.yml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DockerRegistryTestResult {
    pub registry: String,
    pub image_name: String,
    pub success: bool,
    pub download_speed_mbps: f64,
    pub downloaded_bytes: u64,
    pub test_duration_seconds: f64,
    pub error_message: Option<String>,
    pub session_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DockerRegistryBulkTestResult {
    pub image_name: String,
    pub total_registries: usize,
    pub successful_tests: Vec<DockerRegistryTestResult>,
    pub failed_tests: Vec<DockerRegistryTestResult>,
    pub test_duration_ms: u64,
    pub best_registry: Option<String>,
    pub best_speed_mbps: f64,
}

// Config management functions
pub fn docker_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.join("bargozin").join("dockerRegistry.yml")
}

pub async fn read_docker_registries_file(path: &PathBuf) -> Result<Vec<String>> {
    let content = tokio::fs::read_to_string(path).await?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&content)?;
    let list = yaml["registryList"]
        .as_sequence()
        .ok_or_else(|| anyhow::anyhow!("registryList key missing"))?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();
    Ok(list)
}

pub async fn download_docker_config_file(url: &str, path: &PathBuf) -> Result<()> {
    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let mut file = File::create(path).await?;
    file.write_all(&content).await?;
    Ok(())
}

// Docker image name validation
pub fn validate_docker_image_name(image_name: &str) -> bool {
    // Pattern similar to Go code
    let pattern = r"^(?:[a-zA-Z0-9\-._]+(?::[0-9]+)?/)?(?:[a-z0-9\-._]+/)?[a-z0-9\-._]+(?::[a-zA-Z0-9\-._]+)?(?:@[a-zA-Z0-9\-._:]+)?$";
    let regex = Regex::new(pattern).unwrap();
    regex.is_match(image_name) && !image_name.contains("@@")
}

// Custom HTTP client with byte counting
#[derive(Debug, Clone)]
pub struct ByteCountingClient {
    client: Client,
    bytes_downloaded: Arc<AtomicU64>,
}

impl ByteCountingClient {
    pub fn new(timeout: Duration) -> Self {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent("Bargozin-Docker-Registry-Tester/1.0")
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        
        Self {
            client,
            bytes_downloaded: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn get_bytes_downloaded(&self) -> u64 {
        self.bytes_downloaded.load(Ordering::SeqCst)
    }

    pub async fn download_stream(&self, url: &str, max_duration: Duration) -> Result<u64> {
        let start_time = Instant::now();
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("HTTP error: {}", response.status()));
        }

        let mut stream = response.bytes_stream();
        let mut total_bytes = 0u64;

        while let Some(chunk_result) = stream.next().await {
            // Check timeout
            if start_time.elapsed() >= max_duration {
                break;
            }

            let chunk = chunk_result?;
            let chunk_size = chunk.len() as u64;
            total_bytes += chunk_size;
            self.bytes_downloaded.fetch_add(chunk_size, Ordering::SeqCst);

            // Yield control periodically
            if total_bytes % (1024 * 1024) == 0 {
                tokio::task::yield_now().await;
            }
        }

        Ok(total_bytes)
    }
}

pub async fn test_docker_registry_download_speed(
    registry: &str,
    image_name: &str,
    timeout_seconds: u64,
    session_id: u64,
) -> DockerRegistryTestResult {
    let start_time = Instant::now();
    
    // Validate image name
    if !validate_docker_image_name(image_name) {
        return DockerRegistryTestResult {
            registry: registry.to_string(),
            image_name: image_name.to_string(),
            success: false,
            download_speed_mbps: 0.0,
            downloaded_bytes: 0,
            test_duration_seconds: 0.0,
            error_message: Some("Invalid Docker image name format".to_string()),
            session_id,
        };
    }

    // Build manifest URL
    let manifest_url = if registry.contains("://") {
        format!("{}/v2/{}/manifests/latest", registry, image_name)
    } else {
        format!("https://{}/v2/{}/manifests/latest", registry, image_name)
    };

    let client = ByteCountingClient::new(Duration::from_secs(timeout_seconds + 5));
    let download_duration = Duration::from_secs(timeout_seconds);

    match client.download_stream(&manifest_url, download_duration).await {
        Ok(downloaded_bytes) => {
            let elapsed = start_time.elapsed().as_secs_f64();
            let speed_mbps = if elapsed > 0.0 {
                (downloaded_bytes as f64 * 8.0) / (elapsed * 1_000_000.0)
            } else {
                0.0
            };

            DockerRegistryTestResult {
                registry: registry.to_string(),
                image_name: image_name.to_string(),
                success: true,
                download_speed_mbps: speed_mbps,
                downloaded_bytes,
                test_duration_seconds: elapsed,
                error_message: None,
                session_id,
            }
        }
        Err(e) => {
            let elapsed = start_time.elapsed().as_secs_f64();
            DockerRegistryTestResult {
                registry: registry.to_string(),
                image_name: image_name.to_string(),
                success: false,
                download_speed_mbps: 0.0,
                downloaded_bytes: client.get_bytes_downloaded(),
                test_duration_seconds: elapsed,
                error_message: Some(e.to_string()),
                session_id,
            }
        }
    }
}