// Tauri command handlers for PodPico application
// These functions are callable from the frontend via Tauri's IPC bridge

use crate::database::DatabaseManager;
use crate::file_manager::FileManager;
use crate::rss_manager::RssManager;
use crate::usb_manager::UsbManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
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

/// User Story #11: Sync episode status between device and database
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceSyncReport {
    pub processed_files: usize,
    pub updated_episodes: usize,
    pub sync_duration_ms: u128,
    pub is_consistent: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceEpisodeInfo {
    pub filename: String,
    pub podcast_name: String,
    pub file_size: u64,
    pub last_modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceStatusConsistencyReport {
    pub files_found: usize,
    pub database_episodes: usize,
    pub is_consistent: bool,
    pub missing_from_device: Vec<String>,
    pub missing_from_database: Vec<String>,
}

#[derive(Clone, serde::Serialize)]
pub struct PodcastCleanupInfo {
    pub podcast_name: String,
    pub total_episodes_count: usize,
    pub downloaded_episodes_count: usize,
    pub usb_episodes_count: usize,
    pub downloaded_episode_files: Vec<String>,
    pub usb_episodes: Vec<String>,
    pub removal_successful: bool,
}

// Global instances (to be initialized in lib.rs)
static DATABASE: Mutex<Option<Arc<DatabaseManager>>> = Mutex::const_new(None);
static RSS_MANAGER: Mutex<Option<Arc<RssManager>>> = Mutex::const_new(None);
static FILE_MANAGER: Mutex<Option<Arc<FileManager>>> = Mutex::const_new(None);
static USB_MANAGER: Mutex<Option<Arc<UsbManager>>> = Mutex::const_new(None);

pub async fn initialize_managers(
    db: DatabaseManager,
    rss: RssManager,
    file: FileManager,
    usb: UsbManager,
) {
    let mut db_lock = DATABASE.lock().await;
    *db_lock = Some(Arc::new(db));

    let mut rss_lock = RSS_MANAGER.lock().await;
    *rss_lock = Some(Arc::new(rss));

    let mut file_lock = FILE_MANAGER.lock().await;
    *file_lock = Some(Arc::new(file));

    let mut usb_lock = USB_MANAGER.lock().await;
    *usb_lock = Some(Arc::new(usb));
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

    // Step 1 & 2: Fetch and validate RSS feed in one operation (with 5-second timeout)
    let channel = rss_manager
        .validate_and_fetch_feed(&rss_url)
        .await
        .map_err(|e| format!("RSS validation/fetch failed: {}", e))?;

    let (title, description, artwork_url) = rss_manager
        .extract_podcast_info(&channel)
        .await
        .map_err(|e| format!("Failed to extract podcast info: {}", e))?;

    let website_url = rss_manager.extract_website_url(&channel);

    // Step 3: Save to database
    let podcast = db
        .add_podcast(
            &title,
            &rss_url,
            description.as_deref(),
            artwork_url.as_deref(),
            website_url.as_deref(),
        )
        .await
        .map_err(|e| format!("Failed to save podcast: {}", e))?;

    // Step 4: Extract and save episodes
    let episodes = rss_manager
        .extract_episodes(&channel)
        .await
        .map_err(|e| format!("Failed to extract episodes: {}", e))?;

    let episode_count = episodes.len();

    for item in &episodes {
        // Extract episode information
        let episode_title = item.title().unwrap_or("Untitled Episode").to_string();
        let episode_description = item.description().map(|s| s.to_string());

        // Get the audio file URL from enclosure (not link)
        let episode_url = item
            .enclosure()
            .map(|enc| enc.url().to_string())
            .unwrap_or_else(|| item.link().unwrap_or("").to_string());

        if !episode_url.is_empty() {
            // Try to parse duration from iTunes extension
            let duration = item
                .itunes_ext()
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
                        }
                        3 => {
                            // hours:minutes:seconds
                            let hours = parts[0].parse::<i32>().ok()?;
                            let minutes = parts[1].parse::<i32>().ok()?;
                            let seconds = parts[2].parse::<i32>().ok()?;
                            Some(hours * 3600 + minutes * 60 + seconds)
                        }
                        _ => None,
                    }
                });

            // Parse published date
            let published_date = item.pub_date().map(|s| s.to_string());

            // Save episode to database
            let _episode_id = db
                .add_episode(
                    podcast.id,
                    &episode_title,
                    episode_description.as_deref(),
                    &episode_url,
                    published_date.as_deref(),
                    duration,
                    None, // file_size - will be determined during download
                )
                .await
                .map_err(|e| {
                    log::warn!("Failed to save episode '{}': {}", episode_title, e);
                    e
                })?;
        }
    }

    log::info!(
        "Successfully added podcast: {} with {} episodes",
        title,
        episode_count
    );
    Ok(podcast)
}

#[tauri::command]
pub async fn remove_podcast(podcast_id: i64) -> Result<(), String> {
    log::info!("Removing podcast: {} (User Story #4)", podcast_id);

    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;

    db.remove_podcast(podcast_id)
        .await
        .map_err(|e| format!("Failed to remove podcast: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn remove_podcast_with_cleanup_info(
    podcast_id: i64,
) -> Result<PodcastCleanupInfo, String> {
    log::info!(
        "Removing podcast with cleanup info: {} (User Story #4)",
        podcast_id
    );

    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;

    // User Story #4 Acceptance Criteria Implementation:
    // "Given I unsubscribe from a podcast, when the action completes, then I'm offered the option to delete downloaded episodes"
    // "Given episodes from an unsubscribed podcast are on USB, when I unsubscribe, then I'm notified about USB episodes"

    // Step 1: Get podcast information before removal
    let podcasts = db
        .get_podcasts()
        .await
        .map_err(|e| format!("Failed to get podcasts: {}", e))?;
    let podcast = podcasts
        .into_iter()
        .find(|p| p.id == podcast_id)
        .ok_or(format!("Podcast {} not found", podcast_id))?;

    // Step 2: Get all episodes for this podcast BEFORE removal (since they'll be cascade deleted)
    let episodes = db
        .get_episodes(Some(podcast_id))
        .await
        .map_err(|e| format!("Failed to get episodes: {}", e))?;

    // Step 3: Analyze downloaded episodes (User Story #4 Acceptance Criteria)
    let mut downloaded_episode_files = Vec::new();
    let mut downloaded_count = 0;

    for episode in &episodes {
        if episode.downloaded && episode.local_file_path.is_some() {
            downloaded_count += 1;
            if let Some(file_path) = &episode.local_file_path {
                downloaded_episode_files.push(file_path.clone());
            }
        }
    }

    // Step 4: Analyze USB episodes (User Story #4 Acceptance Criteria)
    let mut usb_episodes = Vec::new();
    let mut usb_count = 0;

    for episode in &episodes {
        if episode.on_device {
            usb_count += 1;
            usb_episodes.push(format!("{} (Episode ID: {})", episode.title, episode.id));
        }
    }

    // Step 5: Remove podcast from database (cascade deletes episodes)
    let removal_result = db.remove_podcast(podcast_id).await;
    let removal_successful = removal_result.is_ok();

    if let Err(e) = removal_result {
        log::error!(
            "Failed to remove podcast {}: {} (User Story #4)",
            podcast_id,
            e
        );
        return Err(format!("Failed to remove podcast: {}", e));
    }

    // Step 6: Create comprehensive cleanup information
    let cleanup_info = PodcastCleanupInfo {
        podcast_name: podcast.name,
        total_episodes_count: episodes.len(),
        downloaded_episodes_count: downloaded_count,
        usb_episodes_count: usb_count,
        downloaded_episode_files,
        usb_episodes,
        removal_successful,
    };

    log::info!(
        "User Story #4 completed successfully: Podcast removed with {} downloaded episodes and {} USB episodes",
        cleanup_info.downloaded_episodes_count,
        cleanup_info.usb_episodes_count
    );

    Ok(cleanup_info)
}

#[tauri::command]
pub async fn get_podcasts() -> Result<Vec<Podcast>, String> {
    log::info!("Getting all podcasts (User Story #2, #7)");

    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;

    db.get_podcasts()
        .await
        .map_err(|e| format!("Failed to get podcasts: {}", e))
}

#[tauri::command]
pub async fn get_episodes(podcast_id: Option<i64>) -> Result<Vec<Episode>, String> {
    log::info!(
        "Getting episodes for podcast: {:?} (User Story #2, #7)",
        podcast_id
    );

    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;

    db.get_episodes(podcast_id)
        .await
        .map_err(|e| format!("Failed to get episodes: {}", e))
}

#[tauri::command]
pub async fn update_episode_status(episode_id: i64, status: String) -> Result<(), String> {
    log::info!(
        "Updating episode {} status to: {} (User Story #5, #6)",
        episode_id,
        status
    );

    // Validate status
    if !["new", "unlistened", "listened"].contains(&status.as_str()) {
        return Err("Invalid status. Must be 'new', 'unlistened', or 'listened'".to_string());
    }

    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;

    db.update_episode_status(episode_id, &status)
        .await
        .map_err(|e| format!("Failed to update episode status: {}", e))
}

// Download management commands
#[tauri::command]
pub async fn download_episode(episode_id: i64) -> Result<(), String> {
    log::info!(
        "Starting download for episode: {} (User Story #3)",
        episode_id
    );

    // Get managers
    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;

    // Get episode information
    let episodes = db
        .get_episodes(None)
        .await
        .map_err(|e| format!("Failed to get episode: {}", e))?;

    let episode = episodes
        .iter()
        .find(|e| e.id == episode_id)
        .ok_or("Episode not found")?;

    // User Story #3 Acceptance Criteria: Check if already downloaded
    if episode.downloaded {
        log::info!("Episode {} already downloaded", episode_id);
        return Ok(());
    }

    // Initialize FileManager for this download
    // TODO: Get download directory from config
    let file_manager = crate::file_manager::FileManager::new("./episodes");
    file_manager
        .initialize()
        .await
        .map_err(|e| format!("Failed to initialize file manager: {}", e))?;

    // User Story #3 Acceptance Criteria: Download with progress tracking
    let result = file_manager
        .download_episode(&episode.episode_url, episode_id, episode.podcast_id)
        .await;

    match result {
        Ok(file_path) => {
            log::info!(
                "Successfully downloaded episode {} to {}",
                episode_id,
                file_path
            );

            // Update database to mark episode as downloaded
            db.update_episode_downloaded_status(episode_id, true, Some(&file_path))
                .await
                .map_err(|e| format!("Failed to update episode status: {}", e))?;

            Ok(())
        }
        Err(e) => {
            log::error!("Failed to download episode {}: {}", episode_id, e);
            Err(format!("Download failed: {}", e))
        }
    }
}

#[tauri::command]
pub async fn get_download_progress(episode_id: i64) -> Result<f64, String> {
    log::info!("Getting download progress for episode: {}", episode_id);

    // User Story #3 Acceptance Criteria: Return download progress percentage
    // TODO: Get from FileManager singleton when properly implemented
    // For now, return a basic implementation

    let file_manager = crate::file_manager::FileManager::new("./episodes");

    if let Some(progress) = file_manager.get_download_progress(episode_id).await {
        Ok(progress.percentage)
    } else {
        Ok(0.0) // No download in progress
    }
}

// USB device management commands
#[tauri::command]
pub async fn get_usb_devices() -> Result<Vec<UsbDevice>, String> {
    log::info!("Getting USB devices (User Story #8)");

    let mut usb_manager = crate::usb_manager::UsbManager::new();
    usb_manager
        .detect_devices()
        .map_err(|e| format!("Failed to detect USB devices: {}", e))
}

#[tauri::command]
pub async fn transfer_episode_to_device(episode_id: i64, device_id: String) -> Result<(), String> {
    log::info!(
        "Transferring episode {} to device: {} (User Story #9)",
        episode_id,
        device_id
    );

    // Get managers
    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;

    let usb_lock = USB_MANAGER.lock().await;
    let usb_manager = usb_lock.as_ref().ok_or("USB manager not initialized")?;

    // User Story #9: Transfer episodes to USB device
    // Acceptance Criteria: Progress indicator, transfer speed, success indication, error handling

    // Step 1: Get episode information from database
    let episodes = db
        .get_episodes(Some(episode_id))
        .await
        .map_err(|e| format!("Failed to get episode: {}", e))?;

    let episode = episodes
        .into_iter()
        .find(|ep| ep.id == episode_id)
        .ok_or(format!("Episode {} not found", episode_id))?;

    // Step 2: Verify episode is downloaded
    if !episode.downloaded || episode.local_file_path.is_none() {
        return Err(format!(
            "Episode {} is not downloaded yet. Please download it first.",
            episode_id
        ));
    }

    let local_file_path = episode.local_file_path.unwrap();

    // Step 3: Find USB device by device_id
    let mut usb_manager_mut = UsbManager::new(); // Create a mutable instance for device detection
    let devices = usb_manager_mut
        .detect_devices()
        .map_err(|e| format!("Failed to detect USB devices: {}", e))?;

    let device = devices
        .into_iter()
        .find(|dev| dev.id == device_id)
        .ok_or(format!(
            "USB device {} not found or not connected",
            device_id
        ))?;

    // Step 4: Generate filename from episode title (sanitized for filesystem)
    let filename = format!(
        "{}.mp3",
        episode
            .title
            .chars()
            .map(
                |c| if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            )
            .collect::<String>()
            .trim()
    );

    // Step 5: Transfer file with progress tracking
    usb_manager
        .transfer_file(&local_file_path, &device.path, &filename)
        .await
        .map_err(|e| format!("Transfer failed: {}", e))?;

    // Step 6: Update episode transfer status in database
    // TODO: Add database field for tracking which episodes are on which devices
    // For now, we'll update the on_device flag
    db.update_episode_on_device_status(episode_id, true)
        .await
        .map_err(|e| format!("Failed to update episode transfer status: {}", e))?;

    log::info!(
        "Successfully transferred episode {} to device {}",
        episode_id,
        device_id
    );
    Ok(())
}

#[tauri::command]
pub async fn remove_episode_from_device(episode_id: i64, device_id: String) -> Result<(), String> {
    log::info!(
        "Removing episode {} from device: {} (User Story #10)",
        episode_id,
        device_id
    );

    // Get managers
    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;

    let usb_lock = USB_MANAGER.lock().await;
    let usb_manager = usb_lock.as_ref().ok_or("USB manager not initialized")?;

    // User Story #10: Remove episodes from USB device
    // Acceptance Criteria: Confirmation before deletion, update indicators, increase storage space

    // Step 1: Get episode information from database
    let episodes = db
        .get_episodes(Some(episode_id))
        .await
        .map_err(|e| format!("Failed to get episode: {}", e))?;

    let episode = episodes
        .into_iter()
        .find(|ep| ep.id == episode_id)
        .ok_or(format!("Episode {} not found", episode_id))?;

    // Step 2: Verify episode is currently on device
    if !episode.on_device {
        return Err(format!(
            "Episode {} is not currently on any USB device",
            episode_id
        ));
    }

    // Step 3: Find USB device by device_id
    let mut usb_manager_mut = UsbManager::new(); // Create a mutable instance for device detection
    let devices = usb_manager_mut
        .detect_devices()
        .map_err(|e| format!("Failed to detect USB devices: {}", e))?;

    let device = devices
        .into_iter()
        .find(|dev| dev.id == device_id)
        .ok_or(format!(
            "USB device {} not found or not connected",
            device_id
        ))?;

    // Step 4: Generate filename from episode title (same as transfer logic)
    let filename = format!(
        "{}.mp3",
        episode
            .title
            .chars()
            .map(
                |c| if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            )
            .collect::<String>()
            .trim()
    );

    // Step 5: User Story #10 Acceptance Criteria: Remove file from USB device
    usb_manager
        .remove_file(&device.path, &filename)
        .await
        .map_err(|e| format!("File removal failed: {}", e))?;

    // Step 6: User Story #10 Acceptance Criteria: Update episode status (no longer shows "on device" indicator)
    db.update_episode_on_device_status(episode_id, false)
        .await
        .map_err(|e| format!("Failed to update episode device status: {}", e))?;

    log::info!(
        "Successfully removed episode {} from device {} (User Story #10)",
        episode_id,
        device_id
    );
    Ok(())
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

/// User Story #11: Sync episode device status command
#[tauri::command]
pub async fn sync_episode_device_status(device_path: String) -> Result<DeviceSyncReport, String> {
    log::info!(
        "Syncing episode device status for: {} (User Story #11)",
        device_path
    );

    let start_time = std::time::Instant::now();

    // Get managers
    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;

    let usb_manager = UsbManager::new();

    // Get episodes that should be on device according to database
    let episodes_marked_on_device = db
        .get_episodes_on_device()
        .await
        .map_err(|e| format!("Failed to get episodes marked as on device: {}", e))?;

    let expected_filenames = db
        .get_on_device_episode_filenames()
        .await
        .map_err(|e| format!("Failed to get expected filenames: {}", e))?;

    // Get actual device status
    let device_status_indicators = usb_manager
        .get_device_status_indicators(&device_path)
        .await
        .map_err(|e| format!("Failed to get device status: {}", e))?;

    // Sync logic: Update database based on actual device contents
    let mut updated_episodes = 0;
    let mut missing_from_device = Vec::new();

    // Check episodes that should be on device but aren't
    for expected_filename in &expected_filenames {
        if !device_status_indicators.contains_key(expected_filename) {
            missing_from_device.push(expected_filename.clone());
            // Find episode by filename and update its on_device status
            for episode in &episodes_marked_on_device {
                if let Some(ref local_path) = episode.local_file_path {
                    if let Some(filename) = std::path::Path::new(local_path).file_name() {
                        if filename.to_string_lossy() == *expected_filename {
                            db.update_episode_on_device_status(episode.id, false)
                                .await
                                .map_err(|e| format!("Failed to update episode status: {}", e))?;
                            updated_episodes += 1;
                            log::info!(
                                "Updated episode {} on_device status to false (User Story #11)",
                                episode.id
                            );
                            break;
                        }
                    }
                }
            }
        }
    }

    let sync_duration_ms = start_time.elapsed().as_millis();
    let is_consistent = missing_from_device.is_empty();

    log::info!(
        "User Story #11 sync completed: {} files processed, {} episodes updated, consistent: {}",
        device_status_indicators.len(),
        updated_episodes,
        is_consistent
    );

    Ok(DeviceSyncReport {
        processed_files: device_status_indicators.len(),
        updated_episodes,
        sync_duration_ms,
        is_consistent,
    })
}

/// User Story #11: Get episodes organized by podcast on device
#[tauri::command]
pub async fn get_device_episodes_by_podcast(
    device_path: String,
) -> Result<HashMap<String, Vec<DeviceEpisodeInfo>>, String> {
    log::info!(
        "Getting device episodes by podcast: {} (User Story #11)",
        device_path
    );

    let usb_manager = UsbManager::new();
    let episodes_by_podcast = usb_manager
        .get_device_episodes_by_podcast(&device_path)
        .await
        .map_err(|e| format!("Failed to get device episodes: {}", e))?;

    // Convert internal format to serializable format
    let mut result = HashMap::new();
    for (podcast_name, episodes) in episodes_by_podcast {
        let converted_episodes: Vec<DeviceEpisodeInfo> = episodes
            .into_iter()
            .map(|episode| DeviceEpisodeInfo {
                filename: episode.filename,
                podcast_name: episode.podcast_name,
                file_size: episode.file_size,
                last_modified: format!("{:?}", episode.last_modified),
            })
            .collect();
        result.insert(podcast_name, converted_episodes);
    }

    Ok(result)
}

/// User Story #11: Get device status indicators for episodes
#[tauri::command]
pub async fn get_device_status_indicators(
    device_path: String,
) -> Result<HashMap<String, bool>, String> {
    log::info!(
        "Getting device status indicators: {} (User Story #11)",
        device_path
    );

    let usb_manager = UsbManager::new();
    usb_manager
        .get_device_status_indicators(&device_path)
        .await
        .map_err(|e| format!("Failed to get device status indicators: {}", e))
}

/// User Story #11: Verify episode status consistency
#[tauri::command]
pub async fn verify_episode_status_consistency(
    device_path: String,
) -> Result<DeviceStatusConsistencyReport, String> {
    log::info!(
        "Verifying episode status consistency: {} (User Story #11)",
        device_path
    );

    // Get managers
    let db_lock = DATABASE.lock().await;
    let db = db_lock.as_ref().ok_or("Database not initialized")?;

    let usb_manager = UsbManager::new();

    // Get expected filenames from database
    let expected_filenames = db
        .get_on_device_episode_filenames()
        .await
        .map_err(|e| format!("Failed to get expected filenames: {}", e))?;

    // Get actual device files
    let device_status_indicators = usb_manager
        .get_device_status_indicators(&device_path)
        .await
        .map_err(|e| format!("Failed to get device status: {}", e))?;

    let mut missing_from_device = Vec::new();
    let mut missing_from_database = Vec::new();

    // Check which database episodes are missing from device
    for expected_filename in &expected_filenames {
        if !device_status_indicators.contains_key(expected_filename) {
            missing_from_device.push(expected_filename.clone());
        }
    }

    // Check which device files are not in database
    for device_filename in device_status_indicators.keys() {
        if !expected_filenames.contains(device_filename) {
            missing_from_database.push(device_filename.clone());
        }
    }

    let is_consistent = missing_from_device.is_empty() && missing_from_database.is_empty();

    log::info!(
        "User Story #11 consistency check: {} files on device, {} in database, consistent: {}",
        device_status_indicators.len(),
        expected_filenames.len(),
        is_consistent
    );

    Ok(DeviceStatusConsistencyReport {
        files_found: device_status_indicators.len(),
        database_episodes: expected_filenames.len(),
        is_consistent,
        missing_from_device,
        missing_from_database,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::{Method::GET, MockServer};
    use serial_test::serial;
    use std::time::Instant;

    #[allow(dead_code)]
    async fn create_test_db() -> DatabaseManager {
        let db_url = "sqlite::memory:"; // Use in-memory database for tests
        let db_manager = DatabaseManager::new(db_url).await.unwrap();
        db_manager.initialize().await.unwrap();
        db_manager
    }

    async fn setup_test_environment() -> (DatabaseManager, RssManager, FileManager, UsbManager) {
        // Use in-memory SQLite database for testing
        let db_url = "sqlite::memory:";

        let db = DatabaseManager::new(db_url).await.unwrap();
        db.initialize().await.unwrap();

        let rss_manager = RssManager::new();
        let file_manager = FileManager::new("./test_episodes");
        file_manager.initialize().await.unwrap();
        let usb_manager = UsbManager::new();

        // Initialize global managers for command testing
        initialize_managers(
            db.clone_manager(),
            rss_manager.clone_manager(),
            file_manager.clone_manager(),
            usb_manager.clone_manager(),
        )
        .await;

        (db, rss_manager, file_manager, usb_manager)
    }

    impl Clone for DatabaseManager {
        fn clone(&self) -> Self {
            self.clone_manager()
        }
    }

    impl Clone for RssManager {
        fn clone(&self) -> Self {
            self.clone_manager()
        }
    }

    impl Clone for FileManager {
        fn clone(&self) -> Self {
            FileManager::new("./test_episodes")
        }
    }

    impl Clone for UsbManager {
        fn clone(&self) -> Self {
            UsbManager::new()
        }
    }

    #[tokio::test]
    async fn test_greet_command() {
        let result = greet("Test User").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, Test User! Welcome to PodPico!");
    }

    #[tokio::test]
    #[serial]
    async fn test_user_story_1_add_podcast_command() {
        // User Story #1: Add new podcast subscription via RSS URL
        let (_db, _rss, _file, _usb) = setup_test_environment().await;
        let server = MockServer::start();

        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd">
        <channel>
            <title>Test Podcast Command</title>
            <description>A test podcast for command testing</description>
            <link>https://example.com/podcast</link>
            <itunes:image href="https://example.com/artwork.jpg"/>
            <item>
                <title>Test Episode 1</title>
                <description>First test episode</description>
                <enclosure url="https://example.com/ep1.mp3" type="audio/mpeg" length="1000000"/>
                <pubDate>Mon, 01 Jan 2023 00:00:00 +0000</pubDate>
                <itunes:duration>30:00</itunes:duration>
            </item>
            <item>
                <title>Test Episode 2</title>
                <description>Second test episode</description>
                <enclosure url="https://example.com/ep2.mp3" type="audio/mpeg" length="2000000"/>
                <pubDate>Tue, 02 Jan 2023 00:00:00 +0000</pubDate>
                <itunes:duration>45:30</itunes:duration>
            </item>
        </channel>
        </rss>"#;

        let mock = server.mock(|when, then| {
            when.method(GET).path("/feed.xml");
            then.status(200)
                .header("content-type", "application/rss+xml")
                .body(mock_feed);
        });

        let url = server.url("/feed.xml");
        let start_time = Instant::now();

        let result = add_podcast(url).await;
        let elapsed = start_time.elapsed();

        assert!(result.is_ok());
        assert!(elapsed.as_secs() < 5); // User Story #1 acceptance criteria

        let podcast = result.unwrap();
        assert_eq!(podcast.name, "Test Podcast Command");
        assert_eq!(
            podcast.description,
            Some("A test podcast for command testing".to_string())
        );
        assert_eq!(
            podcast.artwork_url,
            Some("https://example.com/artwork.jpg".to_string())
        );
        assert_eq!(
            podcast.website_url,
            Some("https://example.com/podcast".to_string())
        );
        assert!(podcast.id > 0);

        mock.assert();

        // Verify episodes were saved
        let episodes = get_episodes(Some(podcast.id)).await.unwrap();
        assert_eq!(episodes.len(), 2);
        assert_eq!(episodes[0].title, "Test Episode 2"); // Should be ordered by date DESC
        assert_eq!(episodes[1].title, "Test Episode 1");
    }

    #[tokio::test]
    #[serial]
    async fn test_add_podcast_invalid_url() {
        let (_db, _rss, _file, _usb) = setup_test_environment().await;

        let result = add_podcast("invalid-url".to_string()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must start with http://"));
    }

    #[tokio::test]
    #[serial]
    async fn test_user_story_4_remove_podcast_with_episode_cleanup_options() {
        // User Story #4 Acceptance Criteria: "Given I unsubscribe from a podcast, when the action completes, then I'm offered the option to delete downloaded episodes"
        let (_db, _rss, _file, _usb) = setup_test_environment().await;
        let server = MockServer::start();

        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
        <channel>
            <title>Podcast with Downloaded Episodes</title>
            <description>This podcast has downloaded episodes</description>
            <item>
                <title>Downloaded Episode 1</title>
                <description>This episode is downloaded</description>
                <enclosure url="https://example.com/downloaded1.mp3" type="audio/mpeg" length="1000000"/>
                <pubDate>Wed, 01 Feb 2023 00:00:00 +0000</pubDate>
            </item>
            <item>
                <title>Downloaded Episode 2</title>
                <description>This episode is also downloaded</description>
                <enclosure url="https://example.com/downloaded2.mp3" type="audio/mpeg" length="1500000"/>
                <pubDate>Thu, 02 Feb 2023 00:00:00 +0000</pubDate>
            </item>
        </channel>
        </rss>"#;

        let mock = server.mock(|when, then| {
            when.method(GET).path("/cleanup-test.xml");
            then.status(200).body(mock_feed);
        });

        let url = server.url("/cleanup-test.xml");
        let podcast = add_podcast(url).await.unwrap();

        // Simulate downloaded episodes by marking them as downloaded in database
        let episodes = get_episodes(Some(podcast.id)).await.unwrap();
        assert_eq!(episodes.len(), 2);

        let db_lock = DATABASE.lock().await;
        let db = db_lock.as_ref().unwrap();

        // Mark episodes as downloaded with mock file paths
        db.update_episode_downloaded_status(
            episodes[0].id,
            true,
            Some("./test_episodes/1/episode1.mp3"),
        )
        .await
        .unwrap();
        db.update_episode_downloaded_status(
            episodes[1].id,
            true,
            Some("./test_episodes/1/episode2.mp3"),
        )
        .await
        .unwrap();

        // Test enhanced remove_podcast that returns cleanup information
        let result = remove_podcast_with_cleanup_info(podcast.id).await;

        // Should succeed and return cleanup information
        assert!(result.is_ok(), "Enhanced remove_podcast should succeed");
        let cleanup_info = result.unwrap();

        // Acceptance Criteria: Should indicate downloaded episodes that can be cleaned up
        assert_eq!(
            cleanup_info.downloaded_episodes_count, 2,
            "Should report 2 downloaded episodes"
        );
        assert!(
            !cleanup_info.downloaded_episode_files.is_empty(),
            "Should list downloaded episode files"
        );
        assert_eq!(
            cleanup_info.usb_episodes_count, 0,
            "Should report 0 USB episodes initially"
        );

        // Verify podcast is removed from database
        let podcasts = get_podcasts().await.unwrap();
        assert_eq!(podcasts.len(), 0, "Podcast should be removed from database");

        mock.assert();
    }

    #[tokio::test]
    #[serial]
    async fn test_user_story_4_remove_podcast_with_usb_episodes_notification() {
        // User Story #4 Acceptance Criteria: "Given episodes from an unsubscribed podcast are on USB, when I unsubscribe, then I'm notified about USB episodes"
        let (_db, _rss, _file, _usb) = setup_test_environment().await;
        let server = MockServer::start();

        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
        <channel>
            <title>Podcast with USB Episodes</title>
            <description>This podcast has episodes on USB</description>
            <item>
                <title>USB Episode 1</title>
                <description>This episode is on USB device</description>
                <enclosure url="https://example.com/usb1.mp3" type="audio/mpeg" length="2000000"/>
                <pubDate>Fri, 03 Feb 2023 00:00:00 +0000</pubDate>
            </item>
        </channel>
        </rss>"#;

        let mock = server.mock(|when, then| {
            when.method(GET).path("/usb-test.xml");
            then.status(200).body(mock_feed);
        });

        let url = server.url("/usb-test.xml");
        let podcast = add_podcast(url).await.unwrap();

        let episodes = get_episodes(Some(podcast.id)).await.unwrap();
        assert_eq!(episodes.len(), 1);

        // Mark episode as on USB device
        let db_lock = DATABASE.lock().await;
        let db = db_lock.as_ref().unwrap();
        db.update_episode_on_device_status(episodes[0].id, true)
            .await
            .unwrap();

        // Test enhanced remove_podcast with USB episode notification
        let result = remove_podcast_with_cleanup_info(podcast.id).await;

        assert!(result.is_ok(), "Enhanced remove_podcast should succeed");
        let cleanup_info = result.unwrap();

        // Acceptance Criteria: Should notify about USB episodes
        assert_eq!(
            cleanup_info.usb_episodes_count, 1,
            "Should report 1 USB episode"
        );
        assert!(
            !cleanup_info.usb_episodes.is_empty(),
            "Should list USB episodes"
        );
        assert_eq!(
            cleanup_info.downloaded_episodes_count, 0,
            "Should report 0 downloaded episodes"
        );

        // Verify podcast is removed
        let podcasts = get_podcasts().await.unwrap();
        assert_eq!(podcasts.len(), 0);

        mock.assert();
    }

    #[tokio::test]
    #[serial]
    async fn test_user_story_4_remove_podcast_complete_cleanup_workflow() {
        // User Story #4 Complete Acceptance Criteria Test: Downloaded episodes + USB episodes + cleanup options
        let (_db, _rss, _file, _usb) = setup_test_environment().await;
        let server = MockServer::start();

        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
        <channel>
            <title>Complete Cleanup Test Podcast</title>
            <description>Podcast with both downloaded and USB episodes</description>
            <item>
                <title>Complete Test Episode 1</title>
                <description>Downloaded but not on USB</description>
                <enclosure url="https://example.com/complete1.mp3" type="audio/mpeg" length="1000000"/>
                <pubDate>Sat, 04 Feb 2023 00:00:00 +0000</pubDate>
            </item>
            <item>
                <title>Complete Test Episode 2</title>
                <description>Downloaded and on USB</description>
                <enclosure url="https://example.com/complete2.mp3" type="audio/mpeg" length="1500000"/>
                <pubDate>Sun, 05 Feb 2023 00:00:00 +0000</pubDate>
            </item>
            <item>
                <title>Complete Test Episode 3</title>
                <description>Not downloaded, just in database</description>
                <enclosure url="https://example.com/complete3.mp3" type="audio/mpeg" length="2000000"/>
                <pubDate>Mon, 06 Feb 2023 00:00:00 +0000</pubDate>
            </item>
        </channel>
        </rss>"#;

        let mock = server.mock(|when, then| {
            when.method(GET).path("/complete-cleanup.xml");
            then.status(200).body(mock_feed);
        });

        let url = server.url("/complete-cleanup.xml");
        let podcast = add_podcast(url).await.unwrap();

        let episodes = get_episodes(Some(podcast.id)).await.unwrap();
        assert_eq!(episodes.len(), 3);

        let db_lock = DATABASE.lock().await;
        let db = db_lock.as_ref().unwrap();

        // Set up complex scenario:
        // Episode 1: Downloaded only
        db.update_episode_downloaded_status(
            episodes[0].id,
            true,
            Some("./test_episodes/complete1.mp3"),
        )
        .await
        .unwrap();

        // Episode 2: Downloaded AND on USB
        db.update_episode_downloaded_status(
            episodes[1].id,
            true,
            Some("./test_episodes/complete2.mp3"),
        )
        .await
        .unwrap();
        db.update_episode_on_device_status(episodes[1].id, true)
            .await
            .unwrap();

        // Episode 3: Neither downloaded nor on USB (just in database)

        // Test complete cleanup workflow
        let result = remove_podcast_with_cleanup_info(podcast.id).await;

        assert!(result.is_ok(), "Complete cleanup workflow should succeed");
        let cleanup_info = result.unwrap();

        // Comprehensive acceptance criteria validation
        assert_eq!(
            cleanup_info.downloaded_episodes_count, 2,
            "Should report 2 downloaded episodes"
        );
        assert_eq!(
            cleanup_info.usb_episodes_count, 1,
            "Should report 1 USB episode"
        );
        assert_eq!(
            cleanup_info.total_episodes_count, 3,
            "Should report 3 total episodes"
        );

        // Verify cleanup information is comprehensive
        assert!(
            cleanup_info.downloaded_episode_files.len() >= 2,
            "Should list downloaded episode files"
        );
        assert!(
            cleanup_info.usb_episodes.len() == 1,
            "Should list USB episodes"
        );

        // Verify podcast removal
        let podcasts = get_podcasts().await.unwrap();
        assert_eq!(podcasts.len(), 0, "Podcast should be removed");

        mock.assert();
    }

    #[tokio::test]
    #[serial]
    async fn test_user_story_4_remove_podcast_performance_requirements() {
        // User Story #4 Performance Test: Removal should complete quickly even with many episodes
        let (_db, _rss, _file, _usb) = setup_test_environment().await;
        let server = MockServer::start();

        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
        <channel>
            <title>Performance Test Podcast</title>
            <description>Podcast for performance testing</description>
            <item>
                <title>Performance Episode</title>
                <enclosure url="https://example.com/perf.mp3" type="audio/mpeg" length="500000"/>
                <pubDate>Tue, 07 Feb 2023 00:00:00 +0000</pubDate>
            </item>
        </channel>
        </rss>"#;

        let mock = server.mock(|when, then| {
            when.method(GET).path("/performance.xml");
            then.status(200).body(mock_feed);
        });

        let url = server.url("/performance.xml");
        let podcast = add_podcast(url).await.unwrap();

        // Time the removal operation
        let start_time = std::time::Instant::now();
        let result = remove_podcast_with_cleanup_info(podcast.id).await;
        let elapsed = start_time.elapsed();

        // Performance requirement: Should complete within reasonable time (< 5 seconds)
        assert!(result.is_ok(), "Performance test should succeed");
        assert!(
            elapsed.as_secs() < 5,
            "User Story #4: Removal should complete within 5 seconds, took {:?}",
            elapsed
        );

        // Verify removal worked
        let podcasts = get_podcasts().await.unwrap();
        assert_eq!(podcasts.len(), 0);

        mock.assert();
    }

    // ... existing code ...
}
