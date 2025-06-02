// Configuration management module for PodPico
// Handles loading and saving application configuration

use crate::error::PodPicoError;
use crate::commands::AppConfig;
use std::path::PathBuf;
use tokio::fs;

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new(config_path: &str) -> Self {
        Self {
            config_path: PathBuf::from(config_path),
        }
    }

    pub async fn load_config(&self) -> Result<AppConfig, PodPicoError> {
        log::info!("Loading configuration from: {:?}", self.config_path);
        // TODO: Load configuration from file or return defaults
        Ok(AppConfig {
            download_directory: "./episodes".to_string(),
            max_concurrent_downloads: 3,
            auto_download_new_episodes: false,
            check_for_updates_interval: 3600,
            default_episode_status: "new".to_string(),
        })
    }

    pub async fn save_config(&self, config: &AppConfig) -> Result<(), PodPicoError> {
        log::info!("Saving configuration to: {:?}", self.config_path);
        // TODO: Save configuration to file
        Err(PodPicoError::Generic("Not implemented yet".to_string()))
    }

    pub async fn initialize(&self) -> Result<(), PodPicoError> {
        log::info!("Initializing configuration manager");
        // TODO: Create config directory if it doesn't exist
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        Ok(())
    }
} 