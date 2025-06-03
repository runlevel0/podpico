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

    pub fn clone_manager(&self) -> Self {
        Self {
            client: self.client.clone(),
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

    pub async fn validate_and_fetch_feed(&self, rss_url: &str) -> Result<Channel, PodPicoError> {
        log::info!("Validating and fetching RSS feed: {} (User Story #1)", rss_url);
        
        // User Story #1 Acceptance Criteria: Validate feed within 5 seconds
        let validation_timeout = Duration::from_secs(5);
        
        let validation_result = timeout(validation_timeout, self.fetch_and_validate_internal(rss_url)).await;
        
        match validation_result {
            Ok(Ok(channel)) => {
                log::info!("RSS feed validation and fetch successful for: {}", rss_url);
                Ok(channel)
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

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use tokio_test;
    use std::time::Instant;

    #[test]
    fn test_rss_manager_creation() {
        let rss_manager = RssManager::new();
        // Just ensure it creates without panic
        assert!(true);
    }

    #[tokio::test]
    async fn test_user_story_1_valid_rss_url_validation() {
        // User Story #1: Validate RSS feed within 5 seconds
        let server = MockServer::start();
        let rss_manager = RssManager::new();
        
        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd">
        <channel>
            <title>Test Podcast</title>
            <description>A test podcast feed</description>
            <link>https://example.com</link>
            <language>en-us</language>
            <itunes:image href="https://example.com/artwork.jpg"/>
            <item>
                <title>Test Episode</title>
                <description>A test episode</description>
                <enclosure url="https://example.com/episode.mp3" type="audio/mpeg" length="1000000"/>
                <pubDate>Mon, 01 Jan 2023 00:00:00 +0000</pubDate>
                <itunes:duration>30:00</itunes:duration>
            </item>
        </channel>
        </rss>"#;
        
        let mock = server.mock(|when, then| {
            when.method(GET).path("/feed.xml");
            then.status(200)
                .header("content-type", "application/rss+xml")
                .body(mock_feed);
        });
        
        let url = &server.url("/feed.xml");
        let start_time = Instant::now();
        
        let result = rss_manager.validate_feed(url).await;
        let elapsed = start_time.elapsed();
        
        assert!(result.is_ok());
        assert!(result.unwrap());
        assert!(elapsed.as_secs() < 5); // Acceptance criteria: within 5 seconds
        mock.assert();
    }

    #[tokio::test]
    async fn test_user_story_1_timeout_validation() {
        // User Story #1: Validate RSS feed times out after 5 seconds
        let server = MockServer::start();
        let rss_manager = RssManager::new();
        
        let mock = server.mock(|when, then| {
            when.method(GET).path("/slow-feed.xml");
            then.status(200)
                .delay(std::time::Duration::from_secs(6)) // Longer than 5 second timeout
                .body("delayed response");
        });
        
        let url = &server.url("/slow-feed.xml");
        let start_time = Instant::now();
        
        let result = rss_manager.validate_feed(url).await;
        let elapsed = start_time.elapsed();
        
        assert!(result.is_err());
        assert!(elapsed.as_secs() >= 5 && elapsed.as_secs() < 7); // Should timeout at 5 seconds
        
        if let Err(PodPicoError::InvalidRssUrl(msg)) = result {
            assert!(msg.contains("timed out"));
        } else {
            panic!("Expected InvalidRssUrl timeout error");
        }
    }

    #[tokio::test]
    async fn test_validate_feed_empty_url() {
        let rss_manager = RssManager::new();
        
        let result = rss_manager.validate_feed("").await;
        assert!(result.is_err());
        
        if let Err(PodPicoError::InvalidRssUrl(msg)) = result {
            assert!(msg.contains("Empty URL"));
        } else {
            panic!("Expected InvalidRssUrl error for empty URL");
        }
    }

    #[tokio::test]
    async fn test_validate_feed_invalid_protocol() {
        let rss_manager = RssManager::new();
        
        let result = rss_manager.validate_feed("ftp://example.com/feed.xml").await;
        assert!(result.is_err());
        
        if let Err(PodPicoError::InvalidRssUrl(msg)) = result {
            assert!(msg.contains("must start with http://"));
        } else {
            panic!("Expected InvalidRssUrl error for invalid protocol");
        }
    }

    #[tokio::test]
    async fn test_validate_feed_http_error() {
        let server = MockServer::start();
        let rss_manager = RssManager::new();
        
        let mock = server.mock(|when, then| {
            when.method(GET).path("/not-found.xml");
            then.status(404);
        });
        
        let url = &server.url("/not-found.xml");
        let result = rss_manager.validate_feed(url).await;
        
        assert!(result.is_err());
        if let Err(PodPicoError::NetworkError(msg)) = result {
            assert!(msg.contains("HTTP error 404"));
        } else {
            panic!("Expected NetworkError for 404 response");
        }
        mock.assert();
    }

    #[tokio::test]
    async fn test_validate_feed_invalid_xml() {
        let server = MockServer::start();
        let rss_manager = RssManager::new();
        
        let mock = server.mock(|when, then| {
            when.method(GET).path("/invalid.xml");
            then.status(200)
                .header("content-type", "text/html")
                .body("<html><body>Not RSS</body></html>");
        });
        
        let url = &server.url("/invalid.xml");
        let result = rss_manager.validate_feed(url).await;
        
        assert!(result.is_err());
        if let Err(PodPicoError::InvalidRssUrl(msg)) = result {
            assert!(msg.contains("Invalid RSS format"));
        } else {
            panic!("Expected InvalidRssUrl error for invalid XML");
        }
        mock.assert();
    }

    #[tokio::test]
    async fn test_validate_feed_no_title() {
        let server = MockServer::start();
        let rss_manager = RssManager::new();
        
        let feed_without_title = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
        <channel>
            <title></title>
            <description>No title</description>
        </channel>
        </rss>"#;
        
        let mock = server.mock(|when, then| {
            when.method(GET).path("/no-title.xml");
            then.status(200)
                .header("content-type", "application/rss+xml")
                .body(feed_without_title);
        });
        
        let url = &server.url("/no-title.xml");
        let result = rss_manager.validate_feed(url).await;
        
        assert!(result.is_err());
        if let Err(PodPicoError::InvalidRssUrl(msg)) = result {
            assert!(msg.contains("no title"));
        } else {
            panic!("Expected InvalidRssUrl error for missing title");
        }
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_feed_success() {
        let server = MockServer::start();
        let rss_manager = RssManager::new();
        
        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
        <channel>
            <title>Test Podcast</title>
            <description>A test podcast</description>
        </channel>
        </rss>"#;
        
        let mock = server.mock(|when, then| {
            when.method(GET).path("/feed.xml");
            then.status(200)
                .header("content-type", "application/rss+xml")
                .body(mock_feed);
        });
        
        let url = &server.url("/feed.xml");
        let result = rss_manager.fetch_feed(url).await;
        
        assert!(result.is_ok());
        let channel = result.unwrap();
        assert_eq!(channel.title(), "Test Podcast");
        assert_eq!(channel.description(), "A test podcast");
        mock.assert();
    }

    #[tokio::test]
    async fn test_extract_podcast_info_complete() {
        let rss_manager = RssManager::new();
        
        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd">
        <channel>
            <title>Test Podcast</title>
            <description>A comprehensive test podcast</description>
            <link>https://example.com</link>
            <itunes:image href="https://example.com/artwork.jpg"/>
        </channel>
        </rss>"#;
        
        let channel = Channel::read_from(mock_feed.as_bytes()).unwrap();
        let result = rss_manager.extract_podcast_info(&channel).await;
        
        assert!(result.is_ok());
        let (title, description, artwork_url) = result.unwrap();
        
        assert_eq!(title, "Test Podcast");
        assert_eq!(description, Some("A comprehensive test podcast".to_string()));
        assert_eq!(artwork_url, Some("https://example.com/artwork.jpg".to_string()));
    }

    #[tokio::test]
    async fn test_extract_podcast_info_minimal() {
        let rss_manager = RssManager::new();
        
        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
        <channel>
            <title>Minimal Podcast</title>
            <description></description>
        </channel>
        </rss>"#;
        
        let channel = Channel::read_from(mock_feed.as_bytes()).unwrap();
        let result = rss_manager.extract_podcast_info(&channel).await;
        
        assert!(result.is_ok());
        let (title, description, artwork_url) = result.unwrap();
        
        assert_eq!(title, "Minimal Podcast");
        assert_eq!(description, None); // Empty description should be None
        assert_eq!(artwork_url, None);
    }

    #[tokio::test]
    async fn test_extract_episodes() {
        let rss_manager = RssManager::new();
        
        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd">
        <channel>
            <title>Test Podcast</title>
            <description>Test</description>
            <item>
                <title>Episode 1</title>
                <description>First episode</description>
                <enclosure url="https://example.com/ep1.mp3" type="audio/mpeg" length="1000000"/>
                <pubDate>Mon, 01 Jan 2023 00:00:00 +0000</pubDate>
                <itunes:duration>30:00</itunes:duration>
            </item>
            <item>
                <title>Episode 2</title>
                <description>Second episode</description>
                <enclosure url="https://example.com/ep2.mp3" type="audio/mpeg" length="2000000"/>
                <pubDate>Tue, 02 Jan 2023 00:00:00 +0000</pubDate>
                <itunes:duration>45:30</itunes:duration>
            </item>
        </channel>
        </rss>"#;
        
        let channel = Channel::read_from(mock_feed.as_bytes()).unwrap();
        let result = rss_manager.extract_episodes(&channel).await;
        
        assert!(result.is_ok());
        let episodes = result.unwrap();
        
        assert_eq!(episodes.len(), 2);
        assert_eq!(episodes[0].title(), Some("Episode 1"));
        assert_eq!(episodes[1].title(), Some("Episode 2"));
    }

    #[tokio::test]
    async fn test_extract_website_url() {
        let rss_manager = RssManager::new();
        
        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
        <channel>
            <title>Test Podcast</title>
            <description>Test</description>
            <link>https://example.com/podcast</link>
        </channel>
        </rss>"#;
        
        let channel = Channel::read_from(mock_feed.as_bytes()).unwrap();
        let website_url = rss_manager.extract_website_url(&channel);
        
        assert_eq!(website_url, Some("https://example.com/podcast".to_string()));
    }

    #[tokio::test]
    async fn test_extract_website_url_empty() {
        let rss_manager = RssManager::new();
        
        let mock_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
        <channel>
            <title>Test Podcast</title>
            <description>Test</description>
            <link></link>
        </channel>
        </rss>"#;
        
        let channel = Channel::read_from(mock_feed.as_bytes()).unwrap();
        let website_url = rss_manager.extract_website_url(&channel);
        
        assert_eq!(website_url, None);
    }

    #[tokio::test]
    async fn test_user_story_1_acceptance_criteria_complete() {
        // Complete User Story #1 acceptance criteria test
        let server = MockServer::start();
        let rss_manager = RssManager::new();
        
        let complete_feed = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd">
        <channel>
            <title>Complete Test Podcast</title>
            <description>A complete test podcast with all metadata</description>
            <link>https://example.com/podcast</link>
            <language>en-us</language>
            <itunes:image href="https://example.com/artwork.jpg"/>
            <image>
                <url>https://example.com/fallback-art.jpg</url>
                <title>Complete Test Podcast</title>
                <link>https://example.com/podcast</link>
            </image>
            <item>
                <title>Complete Episode</title>
                <description>A complete episode with all metadata</description>
                <enclosure url="https://example.com/episode.mp3" type="audio/mpeg" length="25000000"/>
                <pubDate>Mon, 01 Jan 2023 12:00:00 +0000</pubDate>
                <itunes:duration>1:23:45</itunes:duration>
            </item>
        </channel>
        </rss>"#;
        
        let mock = server.mock(|when, then| {
            when.method(GET).path("/complete-feed.xml");
            then.status(200)
                .header("content-type", "application/rss+xml")
                .body(complete_feed);
        });
        
        let url = &server.url("/complete-feed.xml");
        
        // Test acceptance criteria: Validation and fetch within 5 seconds
        let start_time = Instant::now();
        let channel = rss_manager.validate_and_fetch_feed(url).await.unwrap();
        let validation_time = start_time.elapsed();
        
        assert!(validation_time.as_secs() < 5);
        
        // Test acceptance criteria: Complete podcast info extraction
        let (title, description, artwork_url) = rss_manager.extract_podcast_info(&channel).await.unwrap();
        let website_url = rss_manager.extract_website_url(&channel);
        let episodes = rss_manager.extract_episodes(&channel).await.unwrap();
        
        // Verify all metadata extracted correctly
        assert_eq!(title, "Complete Test Podcast");
        assert_eq!(description, Some("A complete test podcast with all metadata".to_string()));
        assert_eq!(artwork_url, Some("https://example.com/artwork.jpg".to_string()));
        assert_eq!(website_url, Some("https://example.com/podcast".to_string()));
        assert_eq!(episodes.len(), 1);
        assert_eq!(episodes[0].title(), Some("Complete Episode"));
        
        mock.assert();
    }
} 