// Tauri commands for PodPico application
// These are the functions that can be called from the frontend
// User Story Driven Implementation

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
    log::info!(
        "Transferring episode {} to device: {} (User Story #9)",
        episode_id,
        device_id
    );
    // TODO: Implement file transfer to USB device
    Err("Not implemented yet".to_string())
}

#[tauri::command]
pub async fn remove_episode_from_device(episode_id: i64, device_id: String) -> Result<(), String> {
    log::info!(
        "Removing episode {} from device: {} (User Story #10)",
        episode_id,
        device_id
    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::DatabaseManager;
    use crate::rss_manager::RssManager;
    use httpmock::prelude::*;
    use serial_test::serial;
    use std::time::Instant;

    async fn setup_test_environment() -> (DatabaseManager, RssManager) {
        // Use in-memory SQLite database for testing
        let db_url = "sqlite::memory:";

        let db = DatabaseManager::new(db_url).await.unwrap();
        db.initialize().await.unwrap();

        let rss_manager = RssManager::new();

        // Initialize global managers for command testing
        initialize_managers(db.clone_manager(), rss_manager.clone_manager()).await;

        (db, rss_manager)
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
        let (_db, _rss) = setup_test_environment().await;
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
        let (_db, _rss) = setup_test_environment().await;

        let result = add_podcast("invalid-url".to_string()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must start with http://"));
    }

    #[tokio::test]
    #[serial]
    async fn test_user_story_4_remove_podcast_command() {
        // User Story #4: Remove podcast subscriptions
        let (_db, _rss) = setup_test_environment().await;
        let server = MockServer::start();

        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
        <channel>
            <title>Podcast to Remove</title>
            <description>This podcast will be removed</description>
        </channel>
        </rss>"#;

        let mock = server.mock(|when, then| {
            when.method(GET).path("/remove-me.xml");
            then.status(200)
                .header("content-type", "application/rss+xml")
                .body(mock_feed);
        });

        let url = server.url("/remove-me.xml");

        // Add podcast first
        let podcast = add_podcast(url).await.unwrap();

        // Verify it exists
        let podcasts = get_podcasts().await.unwrap();
        assert_eq!(podcasts.len(), 1);
        assert_eq!(podcasts[0].id, podcast.id);

        // Remove podcast
        let result = remove_podcast(podcast.id).await;
        assert!(result.is_ok());

        // Verify it's gone
        let podcasts = get_podcasts().await.unwrap();
        assert_eq!(podcasts.len(), 0);

        mock.assert();
    }

    #[tokio::test]
    #[serial]
    async fn test_user_story_2_get_episodes_command() {
        // User Story #2: View all episodes of specific podcast
        let (_db, _rss) = setup_test_environment().await;
        let server = MockServer::start();

        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
        <channel>
            <title>Episode Test Podcast</title>
            <description>For testing episode retrieval</description>
            <item>
                <title>Latest Episode</title>
                <description>The newest episode</description>
                <enclosure url="https://example.com/latest.mp3" type="audio/mpeg" length="1000000"/>
                <pubDate>Wed, 03 Jan 2023 00:00:00 +0000</pubDate>
            </item>
            <item>
                <title>Older Episode</title>
                <description>An older episode</description>
                <enclosure url="https://example.com/older.mp3" type="audio/mpeg" length="1500000"/>
                <pubDate>Mon, 01 Jan 2023 00:00:00 +0000</pubDate>
            </item>
        </channel>
        </rss>"#;

        let mock = server.mock(|when, then| {
            when.method(GET).path("/episodes.xml");
            then.status(200)
                .header("content-type", "application/rss+xml")
                .body(mock_feed);
        });

        let url = server.url("/episodes.xml");
        let podcast = add_podcast(url).await.unwrap();

        // Test User Story #2: Get episodes for specific podcast
        let start_time = Instant::now();
        let episodes = get_episodes(Some(podcast.id)).await.unwrap();
        let elapsed = start_time.elapsed();

        assert!(elapsed.as_secs() < 3); // User Story #2 acceptance criteria: within 3 seconds
        assert_eq!(episodes.len(), 2);

        // Episodes should be ordered by published date DESC
        assert_eq!(episodes[0].title, "Latest Episode");
        assert_eq!(episodes[1].title, "Older Episode");

        // All episodes should belong to the correct podcast
        for episode in &episodes {
            assert_eq!(episode.podcast_id, podcast.id);
            assert_eq!(episode.podcast_name, "Episode Test Podcast");
            assert_eq!(episode.status, "new"); // Default status
        }

        mock.assert();
    }

    #[tokio::test]
    #[serial]
    async fn test_user_story_7_combined_inbox_command() {
        // User Story #7: View all new episodes across podcasts (Combined Inbox)
        let (_db, _rss) = setup_test_environment().await;
        let server = MockServer::start();

        let feed1 = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
        <channel>
            <title>Podcast One</title>
            <description>First podcast</description>
            <item>
                <title>Podcast 1 Episode</title>
                <enclosure url="https://example.com/p1e1.mp3" type="audio/mpeg" length="1000000"/>
                <pubDate>Wed, 03 Jan 2023 00:00:00 +0000</pubDate>
            </item>
        </channel>
        </rss>"#;

        let feed2 = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
        <channel>
            <title>Podcast Two</title>
            <description>Second podcast</description>
            <item>
                <title>Podcast 2 Episode</title>
                <enclosure url="https://example.com/p2e1.mp3" type="audio/mpeg" length="1500000"/>
                <pubDate>Tue, 02 Jan 2023 00:00:00 +0000</pubDate>
            </item>
        </channel>
        </rss>"#;

        let mock1 = server.mock(|when, then| {
            when.method(GET).path("/feed1.xml");
            then.status(200).body(feed1);
        });

        let mock2 = server.mock(|when, then| {
            when.method(GET).path("/feed2.xml");
            then.status(200).body(feed2);
        });

        // Add both podcasts
        let _podcast1 = add_podcast(server.url("/feed1.xml")).await.unwrap();
        let _podcast2 = add_podcast(server.url("/feed2.xml")).await.unwrap();

        // Test User Story #7: Get all new episodes (Combined Inbox)
        let new_episodes = get_episodes(None).await.unwrap();
        assert_eq!(new_episodes.len(), 2);

        // Should be ordered by published date DESC
        assert_eq!(new_episodes[0].title, "Podcast 1 Episode");
        assert_eq!(new_episodes[0].podcast_name, "Podcast One");
        assert_eq!(new_episodes[1].title, "Podcast 2 Episode");
        assert_eq!(new_episodes[1].podcast_name, "Podcast Two");

        // All should have "new" status
        for episode in &new_episodes {
            assert_eq!(episode.status, "new");
        }

        mock1.assert();
        mock2.assert();
    }

    #[tokio::test]
    #[serial]
    async fn test_user_story_5_update_episode_status_command() {
        // User Story #5: Mark episodes as "listened"
        let (_db, _rss) = setup_test_environment().await;
        let server = MockServer::start();

        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
        <channel>
            <title>Status Test Podcast</title>
            <description>For testing status updates</description>
            <item>
                <title>Status Test Episode</title>
                <enclosure url="https://example.com/status.mp3" type="audio/mpeg" length="1000000"/>
                <pubDate>Mon, 01 Jan 2023 00:00:00 +0000</pubDate>
            </item>
        </channel>
        </rss>"#;

        let mock = server.mock(|when, then| {
            when.method(GET).path("/status.xml");
            then.status(200).body(mock_feed);
        });

        let url = server.url("/status.xml");
        let podcast = add_podcast(url).await.unwrap();
        let episodes = get_episodes(Some(podcast.id)).await.unwrap();
        let episode_id = episodes[0].id;

        // Initially should be "new"
        assert_eq!(episodes[0].status, "new");

        // Update to "listened"
        let result = update_episode_status(episode_id, "listened".to_string()).await;
        assert!(result.is_ok());

        // Verify status changed and persists
        let episodes = get_episodes(Some(podcast.id)).await.unwrap();
        assert_eq!(episodes[0].status, "listened");

        // Update to "unlistened"
        let result = update_episode_status(episode_id, "unlistened".to_string()).await;
        assert!(result.is_ok());

        let episodes = get_episodes(Some(podcast.id)).await.unwrap();
        assert_eq!(episodes[0].status, "unlistened");

        mock.assert();
    }

    #[tokio::test]
    #[serial]
    async fn test_update_episode_status_invalid() {
        let (_db, _rss) = setup_test_environment().await;

        let result = update_episode_status(999, "invalid_status".to_string()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid status"));
    }

    #[tokio::test]
    #[serial]
    async fn test_get_podcasts_empty() {
        let (_db, _rss) = setup_test_environment().await;

        let result = get_podcasts().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_episodes_empty() {
        let (_db, _rss) = setup_test_environment().await;

        let result = get_episodes(Some(1)).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    #[serial]
    async fn test_user_story_1_complete_workflow() {
        // Complete User Story #1 workflow test
        let (_db, _rss) = setup_test_environment().await;
        let server = MockServer::start();

        let complete_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd">
        <channel>
            <title>Complete Workflow Test</title>
            <description>Testing complete User Story #1 workflow</description>
            <link>https://example.com/complete</link>
            <itunes:image href="https://example.com/complete-art.jpg"/>
            <item>
                <title>Complete Episode</title>
                <description>Complete episode metadata</description>
                <enclosure url="https://example.com/complete.mp3" type="audio/mpeg" length="25000000"/>
                <pubDate>Mon, 01 Jan 2023 12:00:00 +0000</pubDate>
                <itunes:duration>1:23:45</itunes:duration>
            </item>
        </channel>
        </rss>"#;

        let mock = server.mock(|when, then| {
            when.method(GET).path("/complete.xml");
            then.status(200)
                .header("content-type", "application/rss+xml")
                .body(complete_feed);
        });

        let url = server.url("/complete.xml");

        // Step 1: Add podcast (User Story #1)
        let start_time = Instant::now();
        let podcast = add_podcast(url).await.unwrap();
        let add_time = start_time.elapsed();

        // Verify acceptance criteria
        assert!(add_time.as_secs() < 5); // Within 5 seconds
        assert_eq!(podcast.name, "Complete Workflow Test");
        assert_eq!(
            podcast.description,
            Some("Testing complete User Story #1 workflow".to_string())
        );
        assert_eq!(
            podcast.artwork_url,
            Some("https://example.com/complete-art.jpg".to_string())
        );
        assert_eq!(
            podcast.website_url,
            Some("https://example.com/complete".to_string())
        );

        // Step 2: Verify podcast is in list
        let podcasts = get_podcasts().await.unwrap();
        assert_eq!(podcasts.len(), 1);
        assert_eq!(podcasts[0].id, podcast.id);
        assert_eq!(podcasts[0].episode_count, 1);
        assert_eq!(podcasts[0].new_episode_count, 1);

        // Step 3: Verify episodes were extracted and saved
        let episodes = get_episodes(Some(podcast.id)).await.unwrap();
        assert_eq!(episodes.len(), 1);

        let episode = &episodes[0];
        assert_eq!(episode.title, "Complete Episode");
        assert_eq!(
            episode.description,
            Some("Complete episode metadata".to_string())
        );
        assert_eq!(episode.episode_url, "https://example.com/complete.mp3");
        assert_eq!(episode.duration, Some(5025)); // 1:23:45 = 5025 seconds
        assert_eq!(episode.status, "new");
        assert_eq!(episode.podcast_name, "Complete Workflow Test");

        mock.assert();
    }
}
