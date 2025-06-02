// Tauri commands for PodPico application
// These are the functions that can be called from the frontend

use crate::error::PodPicoError;
use serde::{Deserialize, Serialize};

// Data structures for communication between frontend and backend
#[derive(Debug, Serialize, Deserialize)]
pub struct Podcast {
    pub id: i64,
    pub name: String,
    pub rss_url: String,
    pub description: Option<String>,
    pub artwork_url: Option<String>,
    pub website_url: Option<String>,
    pub last_updated: Option<String>,
    pub episode_count: i64,
    pub new_episode_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub id: i64,
    pub podcast_id: i64,
    pub podcast_name: String,
    pub title: String,
    pub description: Option<String>,
    pub episode_url: String,
    pub published_date: Option<String>,
    pub duration: Option<i32>,
    pub file_size: Option<i64>,
    pub local_file_path: Option<String>,
    pub status: String,
    pub downloaded: bool,
    pub on_device: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsbDevice {
    pub id: String,
    pub name: String,
    pub path: String,
    pub total_space: u64,
    pub available_space: u64,
    pub is_connected: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub download_directory: String,
    pub max_concurrent_downloads: i32,
    pub auto_download_new_episodes: bool,
    pub check_for_updates_interval: i32,
    pub default_episode_status: String,
}

// Development/demo command
#[tauri::command]
pub async fn greet(name: &str) -> Result<String, String> {
    Ok(format!("Hello, {}! Welcome to PodPico!", name))
}

// Core podcast management commands
#[tauri::command]
pub async fn add_podcast(rss_url: String) -> Result<Podcast, String> {
    log::info!("Adding podcast: {}", rss_url);
    // TODO: Implement RSS feed validation and podcast creation
    Err("Not implemented yet".to_string())
}

#[tauri::command]
pub async fn remove_podcast(podcast_id: i64) -> Result<(), String> {
    log::info!("Removing podcast: {}", podcast_id);
    // TODO: Implement podcast removal
    Err("Not implemented yet".to_string())
}

#[tauri::command]
pub async fn get_podcasts() -> Result<Vec<Podcast>, String> {
    log::info!("Getting all podcasts");
    // TODO: Implement podcast retrieval from database
    Ok(vec![]) // Return empty list for now
}

#[tauri::command]
pub async fn get_episodes(podcast_id: Option<i64>) -> Result<Vec<Episode>, String> {
    log::info!("Getting episodes for podcast: {:?}", podcast_id);
    // TODO: Implement episode retrieval from database
    Ok(vec![]) // Return empty list for now
}

#[tauri::command]
pub async fn update_episode_status(episode_id: i64, status: String) -> Result<(), String> {
    log::info!("Updating episode {} status to: {}", episode_id, status);
    // TODO: Implement episode status update
    Err("Not implemented yet".to_string())
}

// Download management commands
#[tauri::command]
pub async fn download_episode(episode_id: i64) -> Result<(), String> {
    log::info!("Starting download for episode: {}", episode_id);
    // TODO: Implement episode download
    Err("Not implemented yet".to_string())
}

#[tauri::command]
pub async fn get_download_progress(episode_id: i64) -> Result<f64, String> {
    log::info!("Getting download progress for episode: {}", episode_id);
    // TODO: Implement download progress tracking
    Ok(0.0) // Return 0% progress for now
}

// USB device management commands
#[tauri::command]
pub async fn get_usb_devices() -> Result<Vec<UsbDevice>, String> {
    log::info!("Getting USB devices");
    // TODO: Implement USB device detection
    Ok(vec![]) // Return empty list for now
}

#[tauri::command]
pub async fn transfer_episode_to_device(episode_id: i64, device_id: String) -> Result<(), String> {
    log::info!("Transferring episode {} to device: {}", episode_id, device_id);
    // TODO: Implement file transfer to USB device
    Err("Not implemented yet".to_string())
}

#[tauri::command]
pub async fn remove_episode_from_device(episode_id: i64, device_id: String) -> Result<(), String> {
    log::info!("Removing episode {} from device: {}", episode_id, device_id);
    // TODO: Implement file removal from USB device
    Err("Not implemented yet".to_string())
}

// Configuration commands
#[tauri::command]
pub async fn get_app_config() -> Result<AppConfig, String> {
    log::info!("Getting app configuration");
    // TODO: Implement configuration loading
    Ok(AppConfig {
        download_directory: "./episodes".to_string(),
        max_concurrent_downloads: 3,
        auto_download_new_episodes: false,
        check_for_updates_interval: 3600,
        default_episode_status: "new".to_string(),
    })
}

#[tauri::command]
pub async fn update_app_config(config: AppConfig) -> Result<(), String> {
    log::info!("Updating app configuration: {:?}", config);
    // TODO: Implement configuration saving
    Err("Not implemented yet".to_string())
} 