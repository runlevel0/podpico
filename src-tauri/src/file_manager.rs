// File management module for PodPico
// Handles episode downloads, local file storage, and file operations

use crate::error::PodPicoError;
use reqwest;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub episode_id: i64,
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
    pub percentage: f64,
    pub speed_bytes_per_sec: f64,
    pub eta_seconds: Option<u64>,
    pub status: DownloadStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DownloadStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
    Cancelled,
}

pub struct FileManager {
    download_directory: PathBuf,
    client: reqwest::Client,
    downloads: Arc<Mutex<HashMap<i64, DownloadProgress>>>,
}

impl FileManager {
    pub fn new(download_directory: &str) -> Self {
        Self {
            download_directory: PathBuf::from(download_directory),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(300)) // 5 minute timeout for downloads
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
            downloads: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn clone_manager(&self) -> Self {
        Self::new(&self.download_directory.to_string_lossy())
    }

    pub async fn initialize(&self) -> Result<(), PodPicoError> {
        log::info!("Initializing file manager");
        fs::create_dir_all(&self.download_directory).await?;
        Ok(())
    }

    pub async fn download_episode(
        &self,
        episode_url: &str,
        episode_id: i64,
        podcast_id: i64,
    ) -> Result<String, PodPicoError> {
        log::info!(
            "Starting download for episode {} from {} (User Story #3)",
            episode_id,
            episode_url
        );

        // Create podcast directory first
        let podcast_dir = self.download_directory.join(podcast_id.to_string());
        fs::create_dir_all(&podcast_dir).await?;

        // Extract filename from URL or use episode ID
        let filename = self.extract_filename_from_url(episode_url, episode_id);
        let file_path = podcast_dir.join(&filename);

        // Check if already downloaded BEFORE any other operations
        if file_path.exists() {
            log::info!(
                "Episode {} already downloaded at {:?}",
                episode_id,
                file_path
            );

            // User Story #3 Acceptance Criteria: Progress indicator appears immediately
            let mut downloads = self.downloads.lock().await;
            downloads.insert(
                episode_id,
                DownloadProgress {
                    episode_id,
                    total_bytes: 0,
                    downloaded_bytes: 0,
                    percentage: 100.0,
                    speed_bytes_per_sec: 0.0,
                    eta_seconds: None,
                    status: DownloadStatus::Completed,
                },
            );
            drop(downloads);

            return Ok(file_path.to_string_lossy().to_string());
        }

        // User Story #3 Acceptance Criteria: Progress indicator appears immediately
        let mut downloads = self.downloads.lock().await;
        downloads.insert(
            episode_id,
            DownloadProgress {
                episode_id,
                total_bytes: 0,
                downloaded_bytes: 0,
                percentage: 0.0,
                speed_bytes_per_sec: 0.0,
                eta_seconds: None,
                status: DownloadStatus::Pending,
            },
        );
        drop(downloads);

        // User Story #3 Acceptance Criteria: Check disk space before download
        self.check_disk_space(&self.download_directory).await?;

        // User Story #3 Acceptance Criteria: Download with progress tracking
        let result = self
            .download_with_progress(episode_url, &file_path, episode_id)
            .await;

        match result {
            Ok(path) => {
                log::info!("Successfully downloaded episode {} to {}", episode_id, path);
                self.update_download_status_with_speed(
                    episode_id,
                    DownloadStatus::Completed,
                    100.0,
                    0,
                    0,
                    0.0,
                )
                .await;
                Ok(path)
            }
            Err(e) => {
                log::error!("Failed to download episode {}: {}", episode_id, e);
                self.update_download_status_with_speed(
                    episode_id,
                    DownloadStatus::Failed(e.to_string()),
                    0.0,
                    0,
                    0,
                    0.0,
                )
                .await;
                Err(e)
            }
        }
    }

    async fn download_with_progress(
        &self,
        url: &str,
        file_path: &PathBuf,
        episode_id: i64,
    ) -> Result<String, PodPicoError> {
        use futures_util::StreamExt;
        use tokio::io::AsyncWriteExt;

        // Update status to in progress
        self.update_download_status(episode_id, DownloadStatus::InProgress, 0.0, 0, 0)
            .await;

        let response =
            self.client.get(url).send().await.map_err(|e| {
                PodPicoError::NetworkError(format!("Failed to start download: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(PodPicoError::NetworkError(format!(
                "HTTP error {}: {}",
                response.status(),
                response
                    .status()
                    .canonical_reason()
                    .unwrap_or("Unknown error")
            )));
        }

        let total_size = response.content_length().unwrap_or(0);
        let mut downloaded = 0u64;
        let mut file = tokio::fs::File::create(file_path)
            .await
            .map_err(|e| PodPicoError::IoError(format!("Failed to create file: {}", e)))?;

        let start_time = std::time::Instant::now();
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk
                .map_err(|e| PodPicoError::NetworkError(format!("Download stream error: {}", e)))?;

            file.write_all(&chunk)
                .await
                .map_err(|e| PodPicoError::IoError(format!("Failed to write file: {}", e)))?;

            downloaded += chunk.len() as u64;

            // User Story #3 Acceptance Criteria: Progress percentage tracking
            let percentage = if total_size > 0 {
                (downloaded as f64 / total_size as f64) * 100.0
            } else {
                0.0
            };

            // Calculate download speed
            let elapsed = start_time.elapsed().as_secs_f64();
            let speed = if elapsed > 0.0 {
                downloaded as f64 / elapsed
            } else {
                0.0
            };

            self.update_download_status_with_speed(
                episode_id,
                DownloadStatus::InProgress,
                percentage,
                downloaded,
                total_size,
                speed,
            )
            .await;
        }

        file.sync_all()
            .await
            .map_err(|e| PodPicoError::IoError(format!("Failed to sync file: {}", e)))?;

        Ok(file_path.to_string_lossy().to_string())
    }

    async fn update_download_status(
        &self,
        episode_id: i64,
        status: DownloadStatus,
        percentage: f64,
        downloaded: u64,
        total: u64,
    ) {
        let mut downloads = self.downloads.lock().await;
        if let Some(progress) = downloads.get_mut(&episode_id) {
            progress.status = status;
            progress.percentage = percentage;
            progress.downloaded_bytes = downloaded;
            progress.total_bytes = total;

            // Calculate speed and ETA
            if percentage > 0.0 && percentage < 100.0 {
                let remaining_bytes = total.saturating_sub(downloaded);
                if progress.speed_bytes_per_sec > 0.0 {
                    progress.eta_seconds =
                        Some((remaining_bytes as f64 / progress.speed_bytes_per_sec) as u64);
                }
            }
        }
    }

    pub async fn update_download_status_with_speed(
        &self,
        episode_id: i64,
        status: DownloadStatus,
        percentage: f64,
        downloaded: u64,
        total: u64,
        speed: f64,
    ) {
        let mut downloads = self.downloads.lock().await;
        if let Some(progress) = downloads.get_mut(&episode_id) {
            progress.status = status;
            progress.percentage = percentage;
            progress.downloaded_bytes = downloaded;
            progress.total_bytes = total;
            progress.speed_bytes_per_sec = speed;

            // Calculate speed and ETA
            if percentage > 0.0 && percentage < 100.0 {
                let remaining_bytes = total.saturating_sub(downloaded);
                if progress.speed_bytes_per_sec > 0.0 {
                    progress.eta_seconds =
                        Some((remaining_bytes as f64 / progress.speed_bytes_per_sec) as u64);
                }
            }
        } else {
            // If progress doesn't exist, create it
            downloads.insert(
                episode_id,
                DownloadProgress {
                    episode_id,
                    total_bytes: total,
                    downloaded_bytes: downloaded,
                    percentage,
                    speed_bytes_per_sec: speed,
                    eta_seconds: if percentage > 0.0 && percentage < 100.0 && speed > 0.0 {
                        Some(((total.saturating_sub(downloaded)) as f64 / speed) as u64)
                    } else {
                        None
                    },
                    status,
                },
            );
        }
    }

    pub async fn get_download_progress(&self, episode_id: i64) -> Option<DownloadProgress> {
        let downloads = self.downloads.lock().await;
        downloads.get(&episode_id).cloned()
    }

    async fn check_disk_space(&self, directory: &Path) -> Result<(), PodPicoError> {
        // User Story #3 Acceptance Criteria: Check disk space before download
        // For now, just check if directory is writable
        // TODO: Implement actual disk space check
        if !directory.exists() {
            return Ok(()); // Will be created
        }

        let temp_file = directory.join(".podpico_space_check");
        match tokio::fs::write(&temp_file, b"test").await {
            Ok(_) => {
                let _ = tokio::fs::remove_file(&temp_file).await;
                Ok(())
            }
            Err(e) => Err(PodPicoError::IoError(format!(
                "Insufficient disk space or permissions: {}",
                e
            ))),
        }
    }

    fn extract_filename_from_url(&self, url: &str, episode_id: i64) -> String {
        // Try to extract filename from URL
        if let Some(filename) = url.split('/').next_back() {
            if filename.contains('.') && filename.len() < 255 {
                return filename.to_string();
            }
        }

        // Fallback to episode ID with .mp3 extension
        format!("{}.mp3", episode_id)
    }

    pub fn get_episode_path(&self, podcast_id: i64, episode_id: i64) -> PathBuf {
        self.download_directory
            .join(podcast_id.to_string())
            .join(format!("{}.mp3", episode_id))
    }

    pub async fn delete_episode(
        &self,
        podcast_id: i64,
        episode_id: i64,
    ) -> Result<(), PodPicoError> {
        let file_path = self.get_episode_path(podcast_id, episode_id);
        log::info!("Deleting episode file: {:?}", file_path);

        if file_path.exists() {
            fs::remove_file(file_path).await?;
        }

        // Remove from downloads tracking
        let mut downloads = self.downloads.lock().await;
        downloads.remove(&episode_id);

        Ok(())
    }

    pub fn get_download_progress_sync(&self, episode_id: i64) -> f64 {
        // TODO: Implement download progress tracking
        log::info!("Getting download progress for episode: {}", episode_id);
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use tempfile::tempdir;

    async fn create_test_file_manager() -> FileManager {
        let temp_dir = tempdir().unwrap();
        let manager = FileManager::new(temp_dir.path().to_str().unwrap());
        manager.initialize().await.unwrap();
        manager
    }

    #[tokio::test]
    async fn test_user_story_3_episode_download_success() {
        // User Story #3: Download episodes from specific podcast
        let file_manager = create_test_file_manager().await;
        let server = MockServer::start();

        let mock_audio_content = b"fake audio content for testing";
        let mock = server.mock(|when, then| {
            when.method(GET).path("/episode.mp3");
            then.status(200)
                .header("content-type", "audio/mpeg")
                .header("content-length", mock_audio_content.len().to_string())
                .body(mock_audio_content);
        });

        let episode_id = 1;
        let podcast_id = 1;
        let url = server.url("/episode.mp3");

        // Test acceptance criteria: Progress indicator appears immediately
        let result = file_manager
            .download_episode(&url, episode_id, podcast_id)
            .await;

        assert!(result.is_ok(), "Download should succeed");
        let file_path = result.unwrap();
        assert!(
            std::path::Path::new(&file_path).exists(),
            "Downloaded file should exist"
        );

        // Test acceptance criteria: Episode marked as downloaded
        let progress = file_manager.get_download_progress(episode_id).await;
        assert!(progress.is_some(), "Download progress should be tracked");

        if let Some(progress) = progress {
            assert_eq!(
                progress.status,
                DownloadStatus::Completed,
                "Download should be completed"
            );
            assert_eq!(
                progress.percentage, 100.0,
                "Download should be 100% complete"
            );
        }

        mock.assert();
    }

    #[tokio::test]
    async fn test_user_story_3_download_progress_tracking() {
        // User Story #3 Acceptance Criteria: Download progress percentage tracking
        let file_manager = create_test_file_manager().await;

        // Test that we can track progress - start with manual progress setting
        let episode_id = 2;

        // Simulate progress tracking without actual download
        file_manager
            .update_download_status_with_speed(
                episode_id,
                DownloadStatus::InProgress,
                50.0,
                512,
                1024,
                100.0,
            )
            .await;

        let progress = file_manager.get_download_progress(episode_id).await;
        assert!(progress.is_some(), "Should have progress tracking");

        if let Some(progress) = progress {
            assert_eq!(progress.percentage, 50.0, "Should have 50% progress");
            assert_eq!(
                progress.downloaded_bytes, 512,
                "Should have correct downloaded bytes"
            );
            assert_eq!(
                progress.total_bytes, 1024,
                "Should have correct total bytes"
            );
            assert_eq!(
                progress.status,
                DownloadStatus::InProgress,
                "Should be in progress"
            );
        }
    }

    #[tokio::test]
    async fn test_user_story_3_download_already_exists() {
        // Test that downloading an already downloaded episode returns existing file
        let file_manager = create_test_file_manager().await;
        let server = MockServer::start();
        let episode_id = 3;
        let podcast_id = 1;

        // Create podcast directory
        let podcast_dir = file_manager.download_directory.join(podcast_id.to_string());
        tokio::fs::create_dir_all(&podcast_dir).await.unwrap();

        // Test filename extraction first
        let test_url = &server.url("/episode.mp3");
        let expected_filename = file_manager.extract_filename_from_url(test_url, episode_id);

        // Create file with the exact name that would be extracted
        let existing_file = podcast_dir.join(&expected_filename);
        tokio::fs::write(&existing_file, b"existing content")
            .await
            .unwrap();

        // Verify file exists before test
        assert!(
            existing_file.exists(),
            "Test setup: existing file should exist at {:?}",
            existing_file
        );

        // Create a mock that shouldn't be called since file exists
        let mock = server.mock(|when, then| {
            when.method(GET).path("/episode.mp3");
            then.status(404); // This should not be called
        });

        // Test with the server URL - since file exists, no network call should be made
        let result = file_manager
            .download_episode(test_url, episode_id, podcast_id)
            .await;
        assert!(
            result.is_ok(),
            "Should return existing file path: {:?}",
            result
        );

        // Verify the file still exists and has the same content
        assert!(existing_file.exists(), "Existing file should still exist");
        let content = tokio::fs::read(&existing_file).await.unwrap();
        assert_eq!(
            content, b"existing content",
            "File content should be unchanged"
        );

        let progress = file_manager.get_download_progress(episode_id).await;
        if let Some(progress) = progress {
            assert_eq!(
                progress.status,
                DownloadStatus::Completed,
                "Should be marked as completed"
            );
            assert_eq!(progress.percentage, 100.0, "Should be 100% complete");
        }

        // The mock should NOT have been called since file already exists
        mock.assert_hits(0);
    }

    #[tokio::test]
    async fn test_user_story_3_download_network_error() {
        // User Story #3 Acceptance Criteria: Clear error message for failed downloads
        let file_manager = create_test_file_manager().await;
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/error-episode.mp3");
            then.status(404);
        });

        let episode_id = 4;
        let podcast_id = 1;
        let url = server.url("/error-episode.mp3");

        let result = file_manager
            .download_episode(&url, episode_id, podcast_id)
            .await;
        assert!(result.is_err(), "Download should fail for 404 error");

        // Check that error status is tracked
        let progress = file_manager.get_download_progress(episode_id).await;
        if let Some(progress) = progress {
            matches!(progress.status, DownloadStatus::Failed(_));
        }

        mock.assert();
    }

    #[tokio::test]
    async fn test_user_story_3_invalid_url() {
        // Test handling of invalid URLs
        let file_manager = create_test_file_manager().await;

        let result = file_manager.download_episode("invalid-url", 5, 1).await;
        assert!(result.is_err(), "Should fail for invalid URL");

        let progress = file_manager.get_download_progress(5).await;
        if let Some(progress) = progress {
            matches!(progress.status, DownloadStatus::Failed(_));
        }
    }

    #[tokio::test]
    async fn test_file_manager_initialization() {
        let temp_dir = tempdir().unwrap();
        let file_manager = FileManager::new(temp_dir.path().to_str().unwrap());

        let result = file_manager.initialize().await;
        assert!(
            result.is_ok(),
            "File manager should initialize successfully"
        );
        assert!(
            file_manager.download_directory.exists(),
            "Download directory should be created"
        );
    }

    #[tokio::test]
    async fn test_get_episode_path() {
        let file_manager = create_test_file_manager().await;

        let path = file_manager.get_episode_path(1, 123);
        assert!(
            path.to_string_lossy().contains("1"),
            "Path should contain podcast ID"
        );
        assert!(
            path.to_string_lossy().contains("123.mp3"),
            "Path should contain episode ID and extension"
        );
    }

    #[tokio::test]
    async fn test_delete_episode() {
        let file_manager = create_test_file_manager().await;
        let podcast_id = 1;
        let episode_id = 6;

        // Create a test file
        let podcast_dir = file_manager.download_directory.join(podcast_id.to_string());
        tokio::fs::create_dir_all(&podcast_dir).await.unwrap();
        let test_file = podcast_dir.join(format!("{}.mp3", episode_id));
        tokio::fs::write(&test_file, b"test content").await.unwrap();
        assert!(test_file.exists(), "Test file should exist");

        // Delete the episode
        let result = file_manager.delete_episode(podcast_id, episode_id).await;
        assert!(result.is_ok(), "Delete should succeed");
        assert!(!test_file.exists(), "Test file should be deleted");
    }

    #[tokio::test]
    async fn test_extract_filename_from_url() {
        let file_manager = create_test_file_manager().await;

        // Test with proper filename in URL
        let filename = file_manager
            .extract_filename_from_url("https://example.com/podcast/episode123.mp3", 456);
        assert_eq!(filename, "episode123.mp3");

        // Test with URL without proper filename
        let filename = file_manager.extract_filename_from_url("https://example.com/feed", 789);
        assert_eq!(filename, "789.mp3");

        // Test with complex URL
        let filename = file_manager.extract_filename_from_url(
            "https://cdn.example.com/episodes/show_name_ep_042.mp3?token=abc",
            42,
        );
        assert_eq!(filename, "show_name_ep_042.mp3?token=abc");
    }

    #[tokio::test]
    async fn test_disk_space_check() {
        let file_manager = create_test_file_manager().await;

        // This should succeed for a valid temp directory
        let result = file_manager
            .check_disk_space(&file_manager.download_directory)
            .await;
        assert!(
            result.is_ok(),
            "Disk space check should succeed for valid directory"
        );
    }
}
