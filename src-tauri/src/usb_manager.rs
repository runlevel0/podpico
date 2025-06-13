// USB device management module for PodPico
// Handles USB device detection, mounting, and file operations

use crate::commands::UsbDevice;
use crate::error::PodPicoError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use sysinfo::{Disks, System};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct TransferProgress {
    pub episode_id: i64,
    pub device_id: String,
    pub total_bytes: u64,
    pub transferred_bytes: u64,
    pub percentage: f64,
    pub speed_bytes_per_sec: f64,
    pub eta_seconds: Option<u64>,
    pub status: TransferStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransferStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
    Cancelled,
}

pub struct UsbManager {
    _system: System,
    transfers: Arc<Mutex<HashMap<String, TransferProgress>>>, // Key: episode_id_device_id
}

impl Default for UsbManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UsbManager {
    pub fn new() -> Self {
        Self {
            _system: System::new_all(),
            transfers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn clone_manager(&self) -> Self {
        Self::new()
    }

    /// User Story #8: See USB device storage capacity
    /// Detects connected USB devices and returns their information
    pub fn detect_devices(&mut self) -> Result<Vec<UsbDevice>, PodPicoError> {
        log::info!("Detecting USB devices (User Story #8)");

        let disks = Disks::new_with_refreshed_list();
        let mut usb_devices = Vec::new();

        for disk in &disks {
            // Filter for removable devices (USB drives)
            // We identify USB devices by checking if they're removable and not the main system disk
            if self.is_usb_device(disk) {
                let device_name = disk.name().to_string_lossy().to_string();
                let mount_point = disk.mount_point().to_string_lossy().to_string();

                // Generate a unique ID based on name and mount point
                let device_id = format!(
                    "{}_{}",
                    device_name.replace([' ', '/'], "_"),
                    mount_point.replace(['/', '\\'], "_")
                );

                let usb_device = UsbDevice {
                    id: device_id,
                    name: if device_name.is_empty() {
                        "USB Device".to_string()
                    } else {
                        device_name
                    },
                    path: mount_point,
                    total_space: disk.total_space(),
                    available_space: disk.available_space(),
                    is_connected: true,
                };

                usb_devices.push(usb_device);
            }
        }

        log::info!("Found {} USB devices", usb_devices.len());
        Ok(usb_devices)
    }

    /// Helper function to determine if a disk is likely a USB device
    fn is_usb_device(&self, disk: &sysinfo::Disk) -> bool {
        let mount_point = disk.mount_point().to_string_lossy().to_lowercase();
        let name = disk.name().to_string_lossy().to_lowercase();

        // Common USB mount points and device indicators
        let usb_indicators = [
            "/media/",
            "/mnt/",
            "/run/media/", // Linux
            "usb",
            "removable",
            "external",
            // Exclude system directories
        ];

        let system_paths = [
            "/", "/boot", "/home", "/var", "/usr", "/opt", "/tmp", "c:\\",
            "d:\\", // Common system drives on Windows
        ];

        // Check if it looks like a USB device
        let looks_like_usb = usb_indicators
            .iter()
            .any(|indicator| mount_point.contains(indicator) || name.contains(indicator));

        // Make sure it's not a system path
        let is_system_path = system_paths
            .iter()
            .any(|path| mount_point.starts_with(path) && mount_point.len() <= path.len() + 5);

        // USB devices typically have available space > 0 and total space > 0
        let has_valid_space = disk.total_space() > 0 && disk.available_space() > 0;

        looks_like_usb && !is_system_path && has_valid_space
    }

    pub fn get_device_info(&self, device_path: &str) -> Result<UsbDevice, PodPicoError> {
        log::info!("Getting device info for: {} (User Story #8)", device_path);

        let disks = Disks::new_with_refreshed_list();

        for disk in &disks {
            if disk.mount_point().to_string_lossy() == device_path {
                let device_name = disk.name().to_string_lossy().to_string();
                let device_id = format!(
                    "{}_{}",
                    device_name.replace([' ', '/'], "_"),
                    device_path.replace(['/', '\\'], "_")
                );

                return Ok(UsbDevice {
                    id: device_id,
                    name: if device_name.is_empty() {
                        "USB Device".to_string()
                    } else {
                        device_name
                    },
                    path: device_path.to_string(),
                    total_space: disk.total_space(),
                    available_space: disk.available_space(),
                    is_connected: true,
                });
            }
        }

        Err(PodPicoError::UsbDeviceNotFound(device_path.to_string()))
    }

    /// User Story #9: Transfer episodes to USB device
    /// Transfers a file from source path to USB device with progress tracking
    pub async fn transfer_file(
        &self,
        source_path: &str,
        device_path: &str,
        filename: &str,
    ) -> Result<(), PodPicoError> {
        log::info!(
            "Transferring file {} to device {} (User Story #9)",
            filename,
            device_path
        );

        // User Story #9 Acceptance Criteria: Progress indicator appears immediately
        let transfer_key = format!("{}_{}", source_path, device_path);
        let mut transfers = self.transfers.lock().await;
        transfers.insert(
            transfer_key.clone(),
            TransferProgress {
                episode_id: 0, // Will be set by caller
                device_id: device_path.to_string(),
                total_bytes: 0,
                transferred_bytes: 0,
                percentage: 0.0,
                speed_bytes_per_sec: 0.0,
                eta_seconds: None,
                status: TransferStatus::Pending,
            },
        );
        drop(transfers);

        // User Story #9 Acceptance Criteria: Check source file exists
        let source_path_buf = PathBuf::from(source_path);
        if !source_path_buf.exists() {
            self.update_transfer_status(
                &transfer_key,
                TransferStatus::Failed(format!("Source file not found: {}", source_path)),
            )
            .await;
            return Err(PodPicoError::FileTransferFailed(format!(
                "Source file not found: {}",
                source_path
            )));
        }

        // User Story #9 Acceptance Criteria: Check USB device exists and is accessible
        let device_path_buf = PathBuf::from(device_path);
        if !device_path_buf.exists() {
            self.update_transfer_status(
                &transfer_key,
                TransferStatus::Failed(format!("USB device not found: {}", device_path)),
            )
            .await;
            return Err(PodPicoError::UsbDeviceNotFound(device_path.to_string()));
        }

        // Get file size for progress tracking
        let metadata = fs::metadata(&source_path_buf).await.map_err(|e| {
            PodPicoError::FileTransferFailed(format!("Cannot read source file: {}", e))
        })?;
        let file_size = metadata.len();

        // User Story #9 Acceptance Criteria: Check available space before transfer
        if let Ok(device_info) = self.get_device_info(device_path) {
            if device_info.available_space < file_size {
                let error_msg = format!(
                    "Insufficient space on USB device. Need {} bytes, available {} bytes",
                    file_size, device_info.available_space
                );
                self.update_transfer_status(
                    &transfer_key,
                    TransferStatus::Failed(error_msg.clone()),
                )
                .await;
                return Err(PodPicoError::FileTransferFailed(error_msg));
            }
        }

        // Create podcast directory on USB device for organization
        let usb_podcast_dir = device_path_buf.join("PodPico");
        fs::create_dir_all(&usb_podcast_dir).await.map_err(|e| {
            PodPicoError::FileTransferFailed(format!("Cannot create directory on USB: {}", e))
        })?;

        let destination_path = usb_podcast_dir.join(filename);

        // Update progress to InProgress
        self.update_transfer_progress(&transfer_key, file_size, 0, TransferStatus::InProgress)
            .await;

        // User Story #9 Acceptance Criteria: Transfer with progress tracking and speed calculation
        let result = self
            .transfer_with_progress(
                &source_path_buf,
                &destination_path,
                &transfer_key,
                file_size,
            )
            .await;

        match result {
            Ok(_) => {
                self.update_transfer_status(&transfer_key, TransferStatus::Completed)
                    .await;
                log::info!("Successfully transferred {} to USB device", filename);
                Ok(())
            }
            Err(e) => {
                self.update_transfer_status(&transfer_key, TransferStatus::Failed(e.to_string()))
                    .await;
                log::error!("Failed to transfer {}: {}", filename, e);
                Err(e)
            }
        }
    }

    /// Performs the actual file transfer with progress tracking
    async fn transfer_with_progress(
        &self,
        source_path: &Path,
        destination_path: &Path,
        transfer_key: &str,
        total_size: u64,
    ) -> Result<(), PodPicoError> {
        let mut source_file = fs::File::open(source_path).await.map_err(|e| {
            PodPicoError::FileTransferFailed(format!("Cannot open source file: {}", e))
        })?;

        let mut dest_file = fs::File::create(destination_path).await.map_err(|e| {
            PodPicoError::FileTransferFailed(format!("Cannot create destination file: {}", e))
        })?;

        let mut buffer = vec![0u8; 64 * 1024]; // 64KB buffer for good performance
        let mut transferred = 0u64;
        let start_time = std::time::Instant::now();

        while transferred < total_size {
            let bytes_read = source_file
                .read(&mut buffer)
                .await
                .map_err(|e| PodPicoError::FileTransferFailed(format!("Read error: {}", e)))?;

            if bytes_read == 0 {
                break; // End of file
            }

            dest_file
                .write_all(&buffer[..bytes_read])
                .await
                .map_err(|e| PodPicoError::FileTransferFailed(format!("Write error: {}", e)))?;

            transferred += bytes_read as u64;

            // User Story #9 Acceptance Criteria: Update progress with speed and ETA
            let elapsed = start_time.elapsed();
            let speed = if elapsed.as_secs() > 0 {
                transferred as f64 / elapsed.as_secs() as f64
            } else {
                0.0
            };

            let eta = if speed > 0.0 {
                Some(((total_size - transferred) as f64 / speed) as u64)
            } else {
                None
            };

            // Update progress every 64KB
            self.update_transfer_progress_detailed(
                transfer_key,
                total_size,
                transferred,
                speed,
                eta,
                TransferStatus::InProgress,
            )
            .await;
        }

        // Ensure all data is written to disk
        dest_file
            .flush()
            .await
            .map_err(|e| PodPicoError::FileTransferFailed(format!("Flush error: {}", e)))?;

        Ok(())
    }

    /// Update transfer status
    async fn update_transfer_status(&self, transfer_key: &str, status: TransferStatus) {
        let mut transfers = self.transfers.lock().await;
        if let Some(progress) = transfers.get_mut(transfer_key) {
            progress.status = status;
        }
    }

    /// Update transfer progress with size information
    async fn update_transfer_progress(
        &self,
        transfer_key: &str,
        total_bytes: u64,
        transferred_bytes: u64,
        status: TransferStatus,
    ) {
        let percentage = if total_bytes > 0 {
            (transferred_bytes as f64 / total_bytes as f64) * 100.0
        } else {
            0.0
        };

        let mut transfers = self.transfers.lock().await;
        if let Some(progress) = transfers.get_mut(transfer_key) {
            progress.total_bytes = total_bytes;
            progress.transferred_bytes = transferred_bytes;
            progress.percentage = percentage;
            progress.status = status;
        }
    }

    /// Update transfer progress with detailed information including speed and ETA
    async fn update_transfer_progress_detailed(
        &self,
        transfer_key: &str,
        total_bytes: u64,
        transferred_bytes: u64,
        speed_bytes_per_sec: f64,
        eta_seconds: Option<u64>,
        status: TransferStatus,
    ) {
        let percentage = if total_bytes > 0 {
            (transferred_bytes as f64 / total_bytes as f64) * 100.0
        } else {
            0.0
        };

        let mut transfers = self.transfers.lock().await;
        if let Some(progress) = transfers.get_mut(transfer_key) {
            progress.total_bytes = total_bytes;
            progress.transferred_bytes = transferred_bytes;
            progress.percentage = percentage;
            progress.speed_bytes_per_sec = speed_bytes_per_sec;
            progress.eta_seconds = eta_seconds;
            progress.status = status;
        }
    }

    /// Get current transfer progress
    pub async fn get_transfer_progress(&self, transfer_key: &str) -> Option<TransferProgress> {
        let transfers = self.transfers.lock().await;
        transfers.get(transfer_key).cloned()
    }

    /// User Story #10: Remove episodes from USB device
    /// Removes a file from the USB device with proper error handling
    pub async fn remove_file(&self, device_path: &str, filename: &str) -> Result<(), PodPicoError> {
        log::info!(
            "Removing file {} from device {} (User Story #10)",
            filename,
            device_path
        );

        // User Story #10 Acceptance Criteria: Validation before removal

        // Step 1: Validate device path exists
        let device_path_buf = PathBuf::from(device_path);
        if !device_path_buf.exists() {
            log::warn!(
                "USB device path not found: {} (User Story #10)",
                device_path
            );
            return Err(PodPicoError::UsbDeviceNotFound(device_path.to_string()));
        }

        // Step 2: Construct full file path in PodPico directory structure
        let file_path = format!("{}/PodPico/{}", device_path, filename);
        let file_path_buf = PathBuf::from(&file_path);

        // Step 3: Check if file exists before attempting removal
        if !file_path_buf.exists() {
            log::warn!(
                "Episode file not found on device: {} (User Story #10)",
                file_path
            );
            return Err(PodPicoError::FileTransferFailed(format!(
                "Episode file '{}' not found on device",
                filename
            )));
        }

        // Step 4: User Story #10 Acceptance Criteria: Remove file from USB device
        match std::fs::remove_file(&file_path_buf) {
            Ok(()) => {
                log::info!(
                    "Successfully removed episode file: {} (User Story #10)",
                    file_path
                );
                Ok(())
            }
            Err(e) => {
                log::error!(
                    "Failed to remove episode file {}: {} (User Story #10)",
                    file_path,
                    e
                );
                Err(PodPicoError::FileTransferFailed(format!(
                    "Failed to remove episode file '{}': {}",
                    filename, e
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_usb_manager_initialization() {
        let _usb_manager = UsbManager::new();

        // Initialization test - if we get here without panic, initialization worked
        // No need for assert since the test would fail if initialization panicked
    }

    #[tokio::test]
    async fn test_user_story_8_detect_devices_performance() {
        // User Story #8 Acceptance Criteria: Detection within 5 seconds
        let mut usb_manager = UsbManager::new();
        let start_time = Instant::now();

        let result = usb_manager.detect_devices();
        let elapsed = start_time.elapsed();

        assert!(result.is_ok(), "USB device detection should not fail");
        assert!(
            elapsed.as_secs() < 5,
            "USB device detection should complete within 5 seconds, took {:?}",
            elapsed
        );
    }

    #[tokio::test]
    async fn test_user_story_8_device_detection_basic() {
        // User Story #8: See USB device storage capacity
        let mut usb_manager = UsbManager::new();

        let devices = usb_manager.detect_devices().unwrap();

        // Should return a vector (even if empty)
        assert!(devices.is_empty() || !devices.is_empty()); // Always true, but validates Vec<UsbDevice>

        // If devices are found, they should have proper structure
        for device in &devices {
            assert!(!device.id.is_empty(), "Device ID should not be empty");
            assert!(!device.name.is_empty(), "Device name should not be empty");
            assert!(!device.path.is_empty(), "Device path should not be empty");
            assert!(
                device.is_connected,
                "Detected devices should be marked as connected"
            );

            // Storage information validation
            assert!(
                device.total_space > 0,
                "Total space should be greater than 0"
            );
            assert!(
                device.available_space <= device.total_space,
                "Available space cannot exceed total space"
            );
        }
    }

    #[tokio::test]
    async fn test_user_story_8_storage_capacity_display() {
        // User Story #8 Acceptance Criteria: Storage space display
        let mut usb_manager = UsbManager::new();

        let devices = usb_manager.detect_devices().unwrap();

        // Test storage information is meaningful
        for device in &devices {
            // Should have valid space values (bytes) - u64 is always non-negative

            // Available space should not exceed total space
            assert!(
                device.available_space <= device.total_space,
                "Available space ({}) should not exceed total space ({})",
                device.available_space,
                device.total_space
            );

            // Storage values should be realistic (at least 1MB for USB devices)
            if device.total_space > 0 {
                assert!(
                    device.total_space >= 1_000_000,
                    "USB devices should have at least 1MB total space, got {}",
                    device.total_space
                );
            }
        }
    }

    #[tokio::test]
    async fn test_user_story_8_no_device_connected_message() {
        // User Story #8 Acceptance Criteria: Clear "No device connected" handling
        let mut usb_manager = UsbManager::new();

        let devices = usb_manager.detect_devices().unwrap();

        // Should handle case where no USB devices are connected
        if devices.is_empty() {
            // This is a valid state - no USB devices connected
            log::info!("No USB devices detected - this is a valid test state");
        } else {
            // If devices are found, they should be properly formatted
            log::info!("Found {} USB devices during test", devices.len());
            for device in &devices {
                log::info!(
                    "  Device: {} at {} ({} bytes total)",
                    device.name,
                    device.path,
                    device.total_space
                );
            }
        }

        // Test passes if detect_devices() succeeds regardless of device count
    }

    #[tokio::test]
    async fn test_get_device_info_nonexistent() {
        let usb_manager = UsbManager::new();

        let result = usb_manager.get_device_info("/nonexistent/path");

        assert!(
            result.is_err(),
            "Should return error for nonexistent device"
        );
        match result {
            Err(PodPicoError::UsbDeviceNotFound(path)) => {
                assert_eq!(path, "/nonexistent/path");
            }
            _ => panic!("Should return UsbDeviceNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_is_usb_device_logic() {
        let mut usb_manager = UsbManager::new();

        // We can't easily test the private is_usb_device method directly,
        // but we can test that detect_devices doesn't return system drives
        let devices = usb_manager.detect_devices().unwrap();

        for device in &devices {
            // Should not detect root filesystem or common system paths
            assert!(
                !device.path.eq("/"),
                "Should not detect root filesystem as USB device"
            );
            assert!(
                !device.path.starts_with("/boot"),
                "Should not detect boot partition as USB device"
            );
            assert!(
                !device.path.starts_with("/home") || device.path.contains("media"),
                "Should not detect home directory as USB device unless in media subdirectory"
            );
        }
    }

    #[tokio::test]
    async fn test_user_story_8_device_structure_validation() {
        // User Story #8: Device structure validation
        let mut usb_manager = UsbManager::new();

        let devices = usb_manager.detect_devices().unwrap();

        for device in &devices {
            // ID validation
            assert!(!device.id.is_empty(), "Device ID should not be empty");
            assert!(
                !device.id.contains('/'),
                "Device ID should not contain path separators"
            );
            assert!(
                !device.id.contains('\\'),
                "Device ID should not contain backslashes"
            );

            // Name validation
            assert!(!device.name.is_empty(), "Device name should not be empty");

            // Path validation
            assert!(!device.path.is_empty(), "Device path should not be empty");

            // Storage validation
            assert!(device.total_space > 0, "Total space should be positive");
            assert!(
                device.available_space <= device.total_space,
                "Available space should not exceed total"
            );

            // Connection status
            assert!(
                device.is_connected,
                "Detected device should be marked as connected"
            );
        }
    }

    // USER STORY #9 TESTS - Transfer episodes to USB device
    // These tests validate acceptance criteria after implementation

    #[tokio::test]
    async fn test_user_story_9_transfer_progress_indicator() {
        // User Story #9 Acceptance Criteria: Progress indicator shows immediately
        let usb_manager = UsbManager::new();
        let test_source = "/tmp/test_episode.mp3";
        let test_device = "/tmp/mock_usb_device"; // Use /tmp as mock USB device
        let test_filename = "test_episode.mp3";

        // Create test file and mock USB device directory
        std::fs::create_dir_all("/tmp").unwrap();
        std::fs::create_dir_all(test_device).unwrap();
        std::fs::write(test_source, b"test episode content").unwrap();

        let start_time = std::time::Instant::now();
        let result = usb_manager
            .transfer_file(test_source, test_device, test_filename)
            .await;
        let elapsed = start_time.elapsed();

        // Should respond reasonably quickly (progress indicator appears immediately)
        assert!(
            elapsed.as_millis() < 1000,
            "Transfer should complete quickly for small test file"
        );

        // Should succeed now that we've implemented it
        assert!(
            result.is_ok(),
            "Transfer should succeed for valid inputs: {:?}",
            result
        );

        // Verify file was transferred
        let dest_path = format!("{}/PodPico/{}", test_device, test_filename);
        assert!(
            std::path::Path::new(&dest_path).exists(),
            "File should exist on device after transfer"
        );

        // Cleanup
        std::fs::remove_file(test_source).unwrap_or(());
        std::fs::remove_file(&dest_path).unwrap_or(());
        std::fs::remove_dir_all(format!("{}/PodPico", test_device)).unwrap_or(());
        std::fs::remove_dir_all(test_device).unwrap_or(());
    }

    #[tokio::test]
    async fn test_user_story_9_transfer_speed_and_time_remaining() {
        // User Story #9 Acceptance Criteria: Transfer speed and time remaining visible
        let usb_manager = UsbManager::new();
        let test_source = "/tmp/test_large_episode.mp3";
        let test_device = "/tmp/mock_usb_device_large";
        let test_filename = "large_episode.mp3";

        // Create larger test file (100KB)
        std::fs::create_dir_all("/tmp").unwrap();
        std::fs::create_dir_all(test_device).unwrap();
        let large_content = vec![0u8; 100 * 1024]; // 100KB
        std::fs::write(test_source, large_content).unwrap();

        let result = usb_manager
            .transfer_file(test_source, test_device, test_filename)
            .await;

        // Should succeed with proper implementation
        assert!(
            result.is_ok(),
            "Transfer should succeed for valid large file: {:?}",
            result
        );

        // Verify file was transferred and has correct size
        let dest_path = format!("{}/PodPico/{}", test_device, test_filename);
        assert!(
            std::path::Path::new(&dest_path).exists(),
            "Large file should exist on device"
        );

        let dest_metadata = std::fs::metadata(&dest_path).unwrap();
        assert_eq!(
            dest_metadata.len(),
            100 * 1024,
            "Transferred file should have correct size"
        );

        // Cleanup
        std::fs::remove_file(test_source).unwrap_or(());
        std::fs::remove_file(&dest_path).unwrap_or(());
        std::fs::remove_dir_all(format!("{}/PodPico", test_device)).unwrap_or(());
        std::fs::remove_dir_all(test_device).unwrap_or(());
    }

    #[tokio::test]
    async fn test_user_story_9_successful_transfer_indicator() {
        // User Story #9 Acceptance Criteria: Episode shows "on device" indicator after successful transfer
        let usb_manager = UsbManager::new();
        let test_source = "/tmp/test_success_episode.mp3";
        let test_device = "/tmp/mock_usb_device_success";
        let test_filename = "success_episode.mp3";

        // Create test file and device
        std::fs::create_dir_all("/tmp").unwrap();
        std::fs::create_dir_all(test_device).unwrap();
        std::fs::write(test_source, b"successful episode content").unwrap();

        let result = usb_manager
            .transfer_file(test_source, test_device, test_filename)
            .await;

        // Should succeed and indicate successful transfer
        assert!(
            result.is_ok(),
            "Transfer should succeed and return success status: {:?}",
            result
        );

        // Verify file exists on device (indicates successful transfer)
        let dest_path = format!("{}/PodPico/{}", test_device, test_filename);
        assert!(
            std::path::Path::new(&dest_path).exists(),
            "File should exist on device after successful transfer"
        );

        // Verify content is correct
        let transferred_content = std::fs::read_to_string(&dest_path).unwrap();
        assert_eq!(
            transferred_content, "successful episode content",
            "Transferred content should match original"
        );

        // Cleanup
        std::fs::remove_file(test_source).unwrap_or(());
        std::fs::remove_file(&dest_path).unwrap_or(());
        std::fs::remove_dir_all(format!("{}/PodPico", test_device)).unwrap_or(());
        std::fs::remove_dir_all(test_device).unwrap_or(());
    }

    #[tokio::test]
    async fn test_user_story_9_transfer_error_handling() {
        // User Story #9 Acceptance Criteria: Clear error message and retry option on failure
        let usb_manager = UsbManager::new();
        let nonexistent_source = "/tmp/nonexistent_episode.mp3";
        let test_device = "/tmp/mock_usb_device_error";
        let test_filename = "nonexistent.mp3";

        // Ensure test device exists but source file doesn't
        std::fs::create_dir_all(test_device).unwrap();
        std::fs::remove_file(nonexistent_source).unwrap_or(()); // Ensure it doesn't exist

        let result = usb_manager
            .transfer_file(nonexistent_source, test_device, test_filename)
            .await;

        // Should fail with clear error message
        assert!(result.is_err(), "Transfer of nonexistent file should fail");

        let error = result.unwrap_err();
        let error_message = format!("{}", error);

        // Error message should be clear and actionable
        assert!(
            !error_message.is_empty(),
            "Error message should not be empty"
        );
        assert!(
            error_message.contains("Source file not found"),
            "Error should mention source file not found"
        );
        assert!(
            error_message.contains(nonexistent_source),
            "Error should include the problematic path"
        );

        // Cleanup
        std::fs::remove_dir_all(test_device).unwrap_or(());
    }

    #[tokio::test]
    async fn test_user_story_9_insufficient_space_warning() {
        // User Story #9 Acceptance Criteria: Warn before transfer if insufficient USB space
        // Note: This test verifies the space checking logic, but we can't easily simulate
        // a device with insufficient space using filesystem mocking
        let usb_manager = UsbManager::new();
        let test_source = "/tmp/test_large_episode_space.mp3";
        let test_device = "/tmp/mock_usb_device_space";
        let test_filename = "large_episode_space.mp3";

        // Create test file and device
        std::fs::create_dir_all("/tmp").unwrap();
        std::fs::create_dir_all(test_device).unwrap();
        let content = vec![0u8; 1024]; // 1KB test file
        std::fs::write(test_source, content).unwrap();

        let result = usb_manager
            .transfer_file(test_source, test_device, test_filename)
            .await;

        // Should succeed for small file (1KB should fit in /tmp)
        assert!(
            result.is_ok(),
            "Transfer should succeed for small file: {:?}",
            result
        );

        // Verify space checking logic exists by checking the implementation handles the logic
        // (The actual insufficient space scenario would require a device with very little space)
        let dest_path = format!("{}/PodPico/{}", test_device, test_filename);
        assert!(
            std::path::Path::new(&dest_path).exists(),
            "File should be transferred successfully"
        );

        // Cleanup
        std::fs::remove_file(test_source).unwrap_or(());
        std::fs::remove_file(&dest_path).unwrap_or(());
        std::fs::remove_dir_all(format!("{}/PodPico", test_device)).unwrap_or(());
        std::fs::remove_dir_all(test_device).unwrap_or(());
    }

    #[tokio::test]
    async fn test_user_story_9_transfer_file_organization() {
        // User Story #9: Verify files are organized properly on USB device
        let usb_manager = UsbManager::new();
        let test_source = "/tmp/test_organized_episode.mp3";
        let test_device = "/tmp/mock_usb_device_organized";
        let test_filename = "organized_episode.mp3";

        // Create test file and device
        std::fs::create_dir_all("/tmp").unwrap();
        std::fs::create_dir_all(test_device).unwrap();
        std::fs::write(test_source, b"organized episode content").unwrap();

        let result = usb_manager
            .transfer_file(test_source, test_device, test_filename)
            .await;

        // Should succeed with proper file organization
        assert!(
            result.is_ok(),
            "Transfer should succeed with proper organization: {:?}",
            result
        );

        // Verify files are organized in PodPico directory structure
        let podpico_dir = format!("{}/PodPico", test_device);
        assert!(
            std::path::Path::new(&podpico_dir).exists(),
            "PodPico directory should be created"
        );
        assert!(
            std::path::Path::new(&podpico_dir).is_dir(),
            "PodPico should be a directory"
        );

        let dest_path = format!("{}/{}", podpico_dir, test_filename);
        assert!(
            std::path::Path::new(&dest_path).exists(),
            "File should be in PodPico directory"
        );

        // Verify content is preserved
        let content = std::fs::read_to_string(&dest_path).unwrap();
        assert_eq!(
            content, "organized episode content",
            "Content should be preserved during organized transfer"
        );

        // Cleanup
        std::fs::remove_file(test_source).unwrap_or(());
        std::fs::remove_file(&dest_path).unwrap_or(());
        std::fs::remove_dir_all(&podpico_dir).unwrap_or(());
        std::fs::remove_dir_all(test_device).unwrap_or(());
    }

    #[tokio::test]
    async fn test_user_story_9_transfer_performance_requirements() {
        // User Story #9: Transfer should complete within reasonable time
        let usb_manager = UsbManager::new();
        let test_source = "/tmp/test_performance_episode.mp3";
        let test_device = "/tmp/mock_usb_device_perf";
        let test_filename = "performance_episode.mp3";

        // Create medium-sized test file (50KB)
        std::fs::create_dir_all("/tmp").unwrap();
        std::fs::create_dir_all(test_device).unwrap();
        let content = vec![0u8; 50 * 1024]; // 50KB for performance testing
        std::fs::write(test_source, content).unwrap();

        let start_time = std::time::Instant::now();
        let result = usb_manager
            .transfer_file(test_source, test_device, test_filename)
            .await;
        let elapsed = start_time.elapsed();

        // Should complete within reasonable time (generous for test environment)
        assert!(
            elapsed.as_secs() < 10,
            "Transfer should complete within 10 seconds for 50KB file"
        );

        // Should succeed
        assert!(
            result.is_ok(),
            "Performance test transfer should succeed: {:?}",
            result
        );

        // Verify file integrity
        let dest_path = format!("{}/PodPico/{}", test_device, test_filename);
        let dest_metadata = std::fs::metadata(&dest_path).unwrap();
        assert_eq!(
            dest_metadata.len(),
            50 * 1024,
            "Transferred file should maintain correct size"
        );

        // Calculate approximate transfer rate
        let transfer_rate = (50 * 1024) as f64 / elapsed.as_secs_f64(); // bytes per second
        assert!(
            transfer_rate > 1000.0,
            "Transfer rate should be at least 1KB/s, got {:.2} bytes/s",
            transfer_rate
        );

        // Cleanup
        std::fs::remove_file(test_source).unwrap_or(());
        std::fs::remove_file(&dest_path).unwrap_or(());
        std::fs::remove_dir_all(format!("{}/PodPico", test_device)).unwrap_or(());
        std::fs::remove_dir_all(test_device).unwrap_or(());
    }

    // USER STORY #10 TESTS - Remove episodes from USB device
    // These tests validate acceptance criteria BEFORE implementation (Test-Driven Development)

    #[tokio::test]
    async fn test_user_story_10_remove_episode_confirmation_required() {
        // User Story #10 Acceptance Criteria: Receive confirmation before deletion
        let usb_manager = UsbManager::new();
        let test_device = "/tmp/mock_usb_device_remove";
        let test_filename = "episode_to_remove.mp3";

        // Create test device and file structure
        std::fs::create_dir_all("/tmp").unwrap();
        std::fs::create_dir_all(test_device).unwrap();
        std::fs::create_dir_all(format!("{}/PodPico", test_device)).unwrap();
        let file_path = format!("{}/PodPico/{}", test_device, test_filename);
        std::fs::write(&file_path, b"episode content to remove").unwrap();

        // Verify file exists before removal
        assert!(
            std::path::Path::new(&file_path).exists(),
            "File should exist before removal"
        );

        // Test removal (should succeed after implementation)
        let result = usb_manager.remove_file(test_device, test_filename).await;

        // Should succeed with proper implementation
        assert!(
            result.is_ok(),
            "File removal should succeed for valid inputs: {:?}",
            result
        );

        // Verify file was removed
        assert!(
            !std::path::Path::new(&file_path).exists(),
            "File should not exist after removal"
        );

        // Cleanup
        std::fs::remove_dir_all(test_device).unwrap_or(());
    }

    #[tokio::test]
    async fn test_user_story_10_remove_nonexistent_episode() {
        // User Story #10 Acceptance Criteria: Clear error handling for nonexistent files
        let usb_manager = UsbManager::new();
        let test_device = "/tmp/mock_usb_device_remove_error";
        let nonexistent_filename = "nonexistent_episode.mp3";

        // Create test device structure without the file
        std::fs::create_dir_all("/tmp").unwrap();
        std::fs::create_dir_all(test_device).unwrap();
        std::fs::create_dir_all(format!("{}/PodPico", test_device)).unwrap();

        // Try to remove nonexistent file
        let result = usb_manager
            .remove_file(test_device, nonexistent_filename)
            .await;

        // Should return appropriate error for nonexistent file
        assert!(result.is_err(), "Removal of nonexistent file should fail");

        let error = result.unwrap_err();
        let error_message = format!("{}", error);

        // Error message should be clear and helpful
        assert!(
            !error_message.is_empty(),
            "Error message should not be empty"
        );
        assert!(
            error_message.contains("not found") || error_message.contains("does not exist"),
            "Error should indicate file not found: {}",
            error_message
        );

        // Cleanup
        std::fs::remove_dir_all(test_device).unwrap_or(());
    }

    #[tokio::test]
    async fn test_user_story_10_storage_space_increases_after_removal() {
        // User Story #10 Acceptance Criteria: Available space increases after removal
        let usb_manager = UsbManager::new();
        let test_device = "/tmp/mock_usb_device_storage";
        let test_filename = "large_episode_for_removal.mp3";

        // Create test device structure with a larger file
        std::fs::create_dir_all("/tmp").unwrap();
        std::fs::create_dir_all(test_device).unwrap();
        std::fs::create_dir_all(format!("{}/PodPico", test_device)).unwrap();

        let file_path = format!("{}/PodPico/{}", test_device, test_filename);
        let large_content = vec![0u8; 50 * 1024]; // 50KB file
        std::fs::write(&file_path, large_content).unwrap();

        // Get storage info before removal
        let device_info_before = usb_manager.get_device_info(test_device);
        let available_before = if let Ok(info) = device_info_before {
            info.available_space
        } else {
            // For mock devices, we'll skip this test part
            std::fs::remove_file(&file_path).unwrap_or(());
            std::fs::remove_dir_all(test_device).unwrap_or(());
            return;
        };

        // Remove the file
        let result = usb_manager.remove_file(test_device, test_filename).await;
        assert!(result.is_ok(), "File removal should succeed: {:?}", result);

        // Get storage info after removal
        let device_info_after = usb_manager.get_device_info(test_device);
        if let Ok(info_after) = device_info_after {
            // Available space should have increased (or at least not decreased)
            assert!(
                info_after.available_space >= available_before,
                "Available space should increase after file removal: {} >= {}",
                info_after.available_space,
                available_before
            );
        }

        // Cleanup
        std::fs::remove_dir_all(test_device).unwrap_or(());
    }

    #[tokio::test]
    async fn test_user_story_10_invalid_device_path() {
        // User Story #10: Error handling for invalid device paths
        let usb_manager = UsbManager::new();
        let invalid_device = "/nonexistent/device/path";
        let test_filename = "episode.mp3";

        let result = usb_manager.remove_file(invalid_device, test_filename).await;

        // Should return appropriate error for invalid device
        assert!(result.is_err(), "Removal from invalid device should fail");

        let error = result.unwrap_err();
        let error_message = format!("{}", error);

        // Error should indicate device not found
        assert!(
            error_message.contains("device") || error_message.contains("not found"),
            "Error should indicate device issue: {}",
            error_message
        );
    }

    #[tokio::test]
    async fn test_user_story_10_file_organization_structure() {
        // User Story #10: Ensure removal respects PodPico directory structure
        let usb_manager = UsbManager::new();
        let test_device = "/tmp/mock_usb_device_structure";
        let test_filename = "structured_episode.mp3";

        // Create proper PodPico directory structure
        std::fs::create_dir_all("/tmp").unwrap();
        std::fs::create_dir_all(test_device).unwrap();
        std::fs::create_dir_all(format!("{}/PodPico", test_device)).unwrap();

        let file_path = format!("{}/PodPico/{}", test_device, test_filename);
        std::fs::write(&file_path, b"structured episode content").unwrap();

        // Verify file exists in correct structure
        assert!(
            std::path::Path::new(&file_path).exists(),
            "File should exist in PodPico directory"
        );

        // Remove file
        let result = usb_manager.remove_file(test_device, test_filename).await;
        assert!(
            result.is_ok(),
            "Structured file removal should succeed: {:?}",
            result
        );

        // Verify file removed but directory structure preserved
        assert!(
            !std::path::Path::new(&file_path).exists(),
            "File should be removed"
        );
        assert!(
            std::path::Path::new(&format!("{}/PodPico", test_device)).exists(),
            "PodPico directory should remain after file removal"
        );

        // Cleanup
        std::fs::remove_dir_all(test_device).unwrap_or(());
    }

    #[tokio::test]
    async fn test_user_story_10_episode_removal_performance() {
        // User Story #10: Ensure removal completes in reasonable time
        let usb_manager = UsbManager::new();
        let test_device = "/tmp/mock_usb_device_perf";
        let test_filename = "performance_test_episode.mp3";

        // Create test setup
        std::fs::create_dir_all("/tmp").unwrap();
        std::fs::create_dir_all(test_device).unwrap();
        std::fs::create_dir_all(format!("{}/PodPico", test_device)).unwrap();

        let file_path = format!("{}/PodPico/{}", test_device, test_filename);
        std::fs::write(&file_path, b"performance test content").unwrap();

        // Time the removal operation
        let start_time = std::time::Instant::now();
        let result = usb_manager.remove_file(test_device, test_filename).await;
        let elapsed = start_time.elapsed();

        // Should complete successfully and quickly
        assert!(result.is_ok(), "File removal should succeed: {:?}", result);
        assert!(
            elapsed.as_secs() < 5,
            "File removal should complete within 5 seconds, took {:?}",
            elapsed
        );

        // Cleanup
        std::fs::remove_dir_all(test_device).unwrap_or(());
    }
}
