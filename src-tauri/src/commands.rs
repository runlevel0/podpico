// Tauri commands for PodPico application
// These are the functions that can be called from the frontend
// User Story Driven Implementation

use crate::error::PodPicoError;
use crate::database::DatabaseManager;
use crate::rss_manager::RssManager;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

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

// Global instances (to be initialized in lib.rs)
static DATABASE: Mutex<Option<Arc<DatabaseManager>>> = Mutex::const_new(None);
static RSS_MANAGER: Mutex<Option<Arc<RssManager>>> = Mutex::const_new(None);

pub async fn initialize_managers(db: DatabaseManager, rss: RssManager) {
    let mut db_lock = DATABASE.lock().await;
    *db_lock = Some(Arc::new(db));
    
    let mut rss_lock = RSS_MANAGER.lock().await;
    *rss_lock = Some(Arc::new(rss));
}

// Development/demo command
#[tauri::command]
pub async fn greet(name: &str) -> Result<String, String> {
    Ok(format!("Hello, {}! Welcome to PodPico!", name))
}

// Core podcast management commands
#[tauri::command]
pub async fn add_podcast(rss_url: String) -> Result<Podcast, String> {
    log::info!("Adding podcast: {} (User Story #1)", rss_url);
    
    // Get managers
    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;
    
    let rss_lock = RSS_MANAGER.lock().await;
    let rss_manager = rss_lock.as_ref().ok_or("RSS manager not initialized")?;
    
    // User Story #1: Add new podcast subscription via RSS URL
    // Acceptance Criteria: Given a valid RSS feed URL, when I paste it in the add podcast dialog, 
    // then the app validates the feed within 5 seconds
    
    // Step 1: Validate RSS feed (with 5-second timeout)
    rss_manager.validate_feed(&rss_url).await
        .map_err(|e| format!("RSS validation failed: {}", e))?;
    
    // Step 2: Fetch and extract podcast information
    let channel = rss_manager.fetch_feed(&rss_url).await
        .map_err(|e| format!("Failed to fetch RSS feed: {}", e))?;
    
    let (title, description, artwork_url) = rss_manager.extract_podcast_info(&channel).await
        .map_err(|e| format!("Failed to extract podcast info: {}", e))?;
    
    let website_url = rss_manager.extract_website_url(&channel);
    
    // Step 3: Save to database
    let podcast = db.add_podcast(
        &title,
        &rss_url,
        description.as_deref(),
        artwork_url.as_deref(),
        website_url.as_deref()
    ).await
    .map_err(|e| format!("Failed to save podcast: {}", e))?;
    
    // Step 4: Extract and save episodes
    let episodes = rss_manager.extract_episodes(&channel).await
        .map_err(|e| format!("Failed to extract episodes: {}", e))?;
    
    let episode_count = episodes.len();
    
    for item in &episodes {
        // Extract episode information
        let episode_title = item.title().unwrap_or("Untitled Episode").to_string();
        let episode_description = item.description().map(|s| s.to_string());
        let episode_url = item.link().unwrap_or("").to_string();
        
        if !episode_url.is_empty() {
            // Try to parse duration from iTunes extension
            let duration = item.itunes_ext()
                .and_then(|itunes| itunes.duration())
                .and_then(|d| {
                    // Parse duration string (e.g., "1:23:45" or "23:45" or "45")
                    let parts: Vec<&str> = d.split(':').collect();
                    match parts.len() {
                        1 => parts[0].parse::<i32>().ok(), // seconds
                        2 => {
                            // minutes:seconds
                            let minutes = parts[0].parse::<i32>().ok()?;
                            let seconds = parts[1].parse::<i32>().ok()?;
                            Some(minutes * 60 + seconds)
                        },
                        3 => {
                            // hours:minutes:seconds
                            let hours = parts[0].parse::<i32>().ok()?;
                            let minutes = parts[1].parse::<i32>().ok()?;
                            let seconds = parts[2].parse::<i32>().ok()?;
                            Some(hours * 3600 + minutes * 60 + seconds)
                        },
                        _ => None
                    }
                });
            
            // Parse published date
            let published_date = item.pub_date().map(|s| s.to_string());
            
            // Save episode to database
            let _episode_id = db.add_episode(
                podcast.id,
                &episode_title,
                episode_description.as_deref(),
                &episode_url,
                published_date.as_deref(),
                duration,
                None // file_size - will be determined during download
            ).await
            .map_err(|e| {
                log::warn!("Failed to save episode '{}': {}", episode_title, e);
                e
            })?;
        }
    }
    
    log::info!("Successfully added podcast: {} with {} episodes", title, episode_count);
    Ok(podcast)
}

#[tauri::command]
pub async fn remove_podcast(podcast_id: i64) -> Result<(), String> {
    log::info!("Removing podcast: {} (User Story #4)", podcast_id);
    
    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;
    
    db.remove_podcast(podcast_id).await
        .map_err(|e| format!("Failed to remove podcast: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_podcasts() -> Result<Vec<Podcast>, String> {
    log::info!("Getting all podcasts (User Story #2, #7)");
    
    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;
    
    db.get_podcasts().await
        .map_err(|e| format!("Failed to get podcasts: {}", e))
}

#[tauri::command]
pub async fn get_episodes(podcast_id: Option<i64>) -> Result<Vec<Episode>, String> {
    log::info!("Getting episodes for podcast: {:?} (User Story #2, #7)", podcast_id);
    
    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;
    
    db.get_episodes(podcast_id).await
        .map_err(|e| format!("Failed to get episodes: {}", e))
}

#[tauri::command]
pub async fn update_episode_status(episode_id: i64, status: String) -> Result<(), String> {
    log::info!("Updating episode {} status to: {} (User Story #5, #6)", episode_id, status);
    
    // Validate status
    if !["new", "unlistened", "listened"].contains(&status.as_str()) {
        return Err("Invalid status. Must be 'new', 'unlistened', or 'listened'".to_string());
    }
    
    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;
    
    db.update_episode_status(episode_id, &status).await
        .map_err(|e| format!("Failed to update episode status: {}", e))
}

// Download management commands
#[tauri::command]
pub async fn download_episode(episode_id: i64) -> Result<(), String> {
    log::info!("Starting download for episode: {} (User Story #3)", episode_id);
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
    log::info!("Getting USB devices (User Story #8)");
    // TODO: Implement USB device detection
    Ok(vec![]) // Return empty list for now
}

#[tauri::command]
pub async fn transfer_episode_to_device(episode_id: i64, device_id: String) -> Result<(), String> {
    log::info!("Transferring episode {} to device: {} (User Story #9)", episode_id, device_id);
    // TODO: Implement file transfer to USB device
    Err("Not implemented yet".to_string())
}

#[tauri::command]
pub async fn remove_episode_from_device(episode_id: i64, device_id: String) -> Result<(), String> {
    log::info!("Removing episode {} from device: {} (User Story #10)", episode_id, device_id);
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