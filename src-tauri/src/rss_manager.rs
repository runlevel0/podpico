// RSS feed management module for PodPico
// Handles RSS feed parsing, validation, and episode extraction
// User Story #1: Add new podcast subscription via RSS URL

use crate::error::PodPicoError;
use reqwest;
use rss::Channel;
use std::time::Duration;
use tokio::time::timeout;

pub struct RssManager {
    client: reqwest::Client,
}

impl RssManager {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    pub async fn validate_feed(&self, rss_url: &str) -> Result<bool, PodPicoError> {
        log::info!("Validating RSS feed: {} (User Story #1)", rss_url);
        
        // User Story #1 Acceptance Criteria: Validate feed within 5 seconds
        let validation_timeout = Duration::from_secs(5);
        
        let validation_result = timeout(validation_timeout, self.fetch_and_validate_internal(rss_url)).await;
        
        match validation_result {
            Ok(Ok(_)) => {
                log::info!("RSS feed validation successful for: {}", rss_url);
                Ok(true)
            },
            Ok(Err(e)) => {
                log::warn!("RSS feed validation failed for {}: {}", rss_url, e);
                Err(e)
            },
            Err(_) => {
                log::error!("RSS feed validation timed out after 5 seconds for: {}", rss_url);
                Err(PodPicoError::InvalidRssUrl("Feed validation timed out after 5 seconds".to_string()))
            }
        }
    }

    async fn fetch_and_validate_internal(&self, rss_url: &str) -> Result<Channel, PodPicoError> {
        // Basic URL validation
        if rss_url.trim().is_empty() {
            return Err(PodPicoError::InvalidRssUrl("Empty URL provided".to_string()));
        }

        if !rss_url.starts_with("http://") && !rss_url.starts_with("https://") {
            return Err(PodPicoError::InvalidRssUrl("URL must start with http:// or https://".to_string()));
        }

        // Fetch the RSS feed
        let response = self.client.get(rss_url).send().await
            .map_err(|e| PodPicoError::NetworkError(format!("Failed to fetch RSS feed: {}", e)))?;

        if !response.status().is_success() {
            return Err(PodPicoError::NetworkError(format!("HTTP error {}: {}", response.status(), response.status().canonical_reason().unwrap_or("Unknown error"))));
        }

        let content = response.text().await
            .map_err(|e| PodPicoError::NetworkError(format!("Failed to read response: {}", e)))?;

        // Parse RSS content
        let channel = Channel::read_from(content.as_bytes())
            .map_err(|e| PodPicoError::InvalidRssUrl(format!("Invalid RSS format: {}", e)))?;

        // Basic validation - must have title and at least be parseable
        if channel.title().trim().is_empty() {
            return Err(PodPicoError::InvalidRssUrl("RSS feed has no title".to_string()));
        }

        Ok(channel)
    }

    pub async fn fetch_feed(&self, rss_url: &str) -> Result<Channel, PodPicoError> {
        log::info!("Fetching RSS feed: {}", rss_url);
        
        // Use the same validation logic but without the 5-second timeout for actual fetching
        self.fetch_and_validate_internal(rss_url).await
    }

    pub async fn extract_podcast_info(&self, channel: &Channel) -> Result<(String, Option<String>, Option<String>), PodPicoError> {
        log::info!("Extracting podcast information from RSS feed");
        
        let title = channel.title().to_string();
        let description = if channel.description().trim().is_empty() {
            None
        } else {
            Some(channel.description().to_string())
        };
        
        // Try to extract artwork URL from iTunes extension or image
        let artwork_url = channel.itunes_ext()
            .and_then(|itunes| itunes.image())
            .map(|s| s.to_string())
            .or_else(|| {
                channel.image().map(|img| img.url().to_string())
            });
        
        Ok((title, description, artwork_url))
    }

    pub async fn extract_episodes(&self, channel: &Channel) -> Result<Vec<rss::Item>, PodPicoError> {
        log::info!("Extracting {} episodes from RSS feed", channel.items().len());
        
        // Return clone of all items from the channel
        Ok(channel.items().to_vec())
    }

    /// Extract website URL from RSS channel
    pub fn extract_website_url(&self, channel: &Channel) -> Option<String> {
        if !channel.link().trim().is_empty() {
            Some(channel.link().to_string())
        } else {
            None
        }
    }
} 