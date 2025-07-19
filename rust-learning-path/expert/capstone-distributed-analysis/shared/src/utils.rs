use anyhow::Result;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, timeout};
use tracing::{error, warn};

/// Utility functions for the distributed analysis system

/// Calculate CRC32 checksum for data integrity
pub fn calculate_checksum(data: &[u8]) -> u32 {
    // Simple CRC32 implementation for demonstration
    // In production, use a proper CRC32 library
    let mut crc = 0xFFFFFFFF_u32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

/// Get current timestamp in milliseconds since Unix epoch
pub fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Retry an async operation with exponential backoff
pub async fn retry_with_backoff<F, Fut, T, E>(
    mut operation: F,
    max_retries: u32,
    initial_delay: Duration,
    max_delay: Duration,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut delay = initial_delay;
    let mut last_error = None;

    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if attempt == max_retries {
                    error!("Operation failed after {} attempts: {}", max_retries + 1, error);
                    return Err(error);
                }

                warn!("Operation failed (attempt {}), retrying in {:?}: {}", 
                      attempt + 1, delay, error);
                
                last_error = Some(error);
                sleep(delay).await;
                
                // Exponential backoff with jitter
                delay = std::cmp::min(delay * 2, max_delay);
                let jitter = Duration::from_millis(fastrand::u64(0..=delay.as_millis() as u64 / 10));
                delay += jitter;
            }
        }
    }

    // This should never be reached due to the loop logic above
    Err(last_error.unwrap())
}

/// Execute an operation with a timeout
pub async fn with_timeout<F, T>(
    operation: F,
    timeout_duration: Duration,
) -> Result<T>
where
    F: std::future::Future<Output = T>,
{
    match timeout(timeout_duration, operation).await {
        Ok(result) => Ok(result),
        Err(_) => anyhow::bail!("Operation timed out after {:?}", timeout_duration),
    }
}

/// Format file size in human-readable format
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: f64 = 1024.0;

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Format duration in human-readable format
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    
    if total_seconds < 60 {
        format!("{}s", total_seconds)
    } else if total_seconds < 3600 {
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{}m {}s", minutes, seconds)
    } else {
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        format!("{}h {}m {}s", hours, minutes, seconds)
    }
}

/// Validate project path and check if it's a valid Rust project
pub fn validate_rust_project(path: &str) -> Result<()> {
    let project_path = std::path::Path::new(path);
    
    if !project_path.exists() {
        anyhow::bail!("Project path does not exist: {}", path);
    }
    
    if !project_path.is_dir() {
        anyhow::bail!("Project path is not a directory: {}", path);
    }
    
    // Check for Cargo.toml
    let cargo_toml = project_path.join("Cargo.toml");
    if !cargo_toml.exists() {
        anyhow::bail!("No Cargo.toml found in project directory: {}", path);
    }
    
    // Check for src directory
    let src_dir = project_path.join("src");
    if !src_dir.exists() {
        anyhow::bail!("No src directory found in project: {}", path);
    }
    
    Ok(())
}

/// Generate a unique worker ID based on hostname and process info
pub fn generate_worker_id() -> String {
    let hostname = hostname::get()
        .unwrap_or_else(|_| "unknown".into())
        .to_string_lossy()
        .to_string();
    
    let process_id = std::process::id();
    let timestamp = current_timestamp_ms();
    
    format!("worker-{}-{}-{}", hostname, process_id, timestamp)
}

/// Calculate system load average (simplified version)
pub fn get_system_load() -> f32 {
    // This is a simplified implementation
    // In production, you'd want to use proper system APIs
    #[cfg(unix)]
    {
        if let Ok(loadavg) = sys_info::loadavg() {
            return loadavg.one as f32;
        }
    }
    
    // Fallback: estimate based on CPU usage
    0.5 // Placeholder value
}

/// Compress data using the specified compression type
pub fn compress_data(data: &[u8], compression: crate::protocol::CompressionType) -> Result<Vec<u8>> {
    match compression {
        crate::protocol::CompressionType::None => Ok(data.to_vec()),
        crate::protocol::CompressionType::Gzip => {
            use std::io::Write;
            let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
            encoder.write_all(data)?;
            Ok(encoder.finish()?)
        }
        crate::protocol::CompressionType::Lz4 => {
            // For demonstration - in production use proper LZ4 library
            Ok(data.to_vec())
        }
    }
}

/// Decompress data using the specified compression type
pub fn decompress_data(data: &[u8], compression: crate::protocol::CompressionType) -> Result<Vec<u8>> {
    match compression {
        crate::protocol::CompressionType::None => Ok(data.to_vec()),
        crate::protocol::CompressionType::Gzip => {
            use std::io::Read;
            let mut decoder = flate2::read::GzDecoder::new(data);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)?;
            Ok(decompressed)
        }
        crate::protocol::CompressionType::Lz4 => {
            // For demonstration - in production use proper LZ4 library
            Ok(data.to_vec())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_calculation() {
        let data = b"hello world";
        let checksum1 = calculate_checksum(data);
        let checksum2 = calculate_checksum(data);
        assert_eq!(checksum1, checksum2);
        
        let different_data = b"hello world!";
        let checksum3 = calculate_checksum(different_data);
        assert_ne!(checksum1, checksum3);
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1024 * 1024), "1.0 MB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
    }

    #[tokio::test]
    async fn test_retry_with_backoff() {
        let mut attempts = 0;
        let result = retry_with_backoff(
            || {
                attempts += 1;
                async move {
                    if attempts < 3 {
                        Err("temporary error")
                    } else {
                        Ok("success")
                    }
                }
            },
            5,
            Duration::from_millis(10),
            Duration::from_millis(100),
        ).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(attempts, 3);
    }
}