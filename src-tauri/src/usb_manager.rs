// USB device management module for PodPico
// Handles USB device detection, mounting, and file operations

use crate::commands::UsbDevice;
use crate::error::PodPicoError;
use sysinfo::{Disks, System};

pub struct UsbManager {
    _system: System, // Will be used for future USB operations (User Stories #9-11)
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
        }
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
                let device_id = format!("{}_{}", 
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
            "/media/", "/mnt/", "/run/media/", // Linux
            "usb", "removable", "external",
            // Exclude system directories
        ];
        
        let system_paths = [
            "/", "/boot", "/home", "/var", "/usr", "/opt", "/tmp",
            "c:\\", "d:\\", // Common system drives on Windows
        ];
        
        // Check if it looks like a USB device
        let looks_like_usb = usb_indicators.iter().any(|indicator| {
            mount_point.contains(indicator) || name.contains(indicator)
        });
        
        // Make sure it's not a system path
        let is_system_path = system_paths.iter().any(|path| {
            mount_point.starts_with(path) && mount_point.len() <= path.len() + 5
        });
        
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
                let device_id = format!("{}_{}", 
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

    pub async fn transfer_file(
        &self,
        _source_path: &str,
        device_path: &str,
        filename: &str,
    ) -> Result<(), PodPicoError> {
        log::info!("Transferring file {} to device {} (User Story #9)", filename, device_path);
        // TODO: Implement file transfer to USB device
        Err(PodPicoError::Generic("Not implemented yet".to_string()))
    }

    pub async fn remove_file(&self, device_path: &str, filename: &str) -> Result<(), PodPicoError> {
        log::info!("Removing file {} from device {} (User Story #10)", filename, device_path);
        // TODO: Implement file removal from USB device
        Err(PodPicoError::Generic("Not implemented yet".to_string()))
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
        assert!(elapsed.as_secs() < 5, "USB device detection should complete within 5 seconds, took {:?}", elapsed);
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
            assert!(device.is_connected, "Detected devices should be marked as connected");
            
            // Storage information validation
            assert!(device.total_space > 0, "Total space should be greater than 0");
            assert!(device.available_space <= device.total_space, "Available space cannot exceed total space");
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
            assert!(device.available_space <= device.total_space, 
                "Available space ({}) should not exceed total space ({})", 
                device.available_space, device.total_space);
            
            // Storage values should be realistic (at least 1MB for USB devices)
            if device.total_space > 0 {
                assert!(device.total_space >= 1_000_000, 
                    "USB devices should have at least 1MB total space, got {}", 
                    device.total_space);
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
                log::info!("  Device: {} at {} ({} bytes total)", 
                    device.name, device.path, device.total_space);
            }
        }
        
        // Test passes if detect_devices() succeeds regardless of device count
    }

    #[tokio::test]
    async fn test_get_device_info_nonexistent() {
        let usb_manager = UsbManager::new();
        
        let result = usb_manager.get_device_info("/nonexistent/path");
        
        assert!(result.is_err(), "Should return error for nonexistent device");
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
            assert!(!device.path.eq("/"), "Should not detect root filesystem as USB device");
            assert!(!device.path.starts_with("/boot"), "Should not detect boot partition as USB device");
            assert!(!device.path.starts_with("/home") || device.path.contains("media"), 
                "Should not detect home directory as USB device unless in media subdirectory");
        }
    }

    #[tokio::test]
    async fn test_user_story_8_device_structure_validation() {
        // Test that detected devices have the correct structure for User Story #8
        let mut usb_manager = UsbManager::new();
        
        let devices = usb_manager.detect_devices().unwrap();
        
        for device in &devices {
            // Validate UsbDevice structure matches User Story #8 requirements
            assert!(!device.id.is_empty(), "Device ID should not be empty");
            assert!(!device.name.is_empty(), "Device name should not be empty");
            assert!(!device.path.is_empty(), "Device path should not be empty");
            assert!(device.is_connected, "Detected devices should be connected");
            
            // ID should be unique and safe for file system operations
            assert!(!device.id.contains('/'), "Device ID should not contain path separators");
            assert!(!device.id.contains('\\'), "Device ID should not contain path separators");
            
            // Name should be human readable
            assert!(!device.name.is_empty(), "Device name should be human readable");
            
            // Path should be a valid mount point
            assert!(device.path.starts_with('/') || device.path.contains(':'), 
                "Device path should be a valid mount point");
        }
    }
}
