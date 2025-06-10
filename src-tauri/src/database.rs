// Database management module for PodPico
// Handles SQLite database operations for podcasts, episodes, and related data
// User Stories #1-11: Podcast and Episode Management

use crate::commands::{Episode, Podcast};
use crate::error::PodPicoError;
use sqlx::{Row, SqlitePool};

pub struct DatabaseManager {
    pool: SqlitePool,
}

impl DatabaseManager {
    pub async fn new(database_url: &str) -> Result<Self, PodPicoError> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub fn clone_manager(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }

    pub async fn initialize(&self) -> Result<(), PodPicoError> {
        log::info!("Creating database tables for podcast management user stories");

        // User Story #1: Add podcast subscription
        // User Story #4: Remove podcast subscription
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS podcasts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                rss_url TEXT UNIQUE NOT NULL,
                description TEXT,
                artwork_url TEXT,
                website_url TEXT,
                last_updated DATETIME,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        // User Story #2: View episodes of specific podcast
        // User Story #3: Download episodes
        // User Story #5: Mark episodes as listened
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS episodes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                podcast_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                episode_url TEXT NOT NULL,
                published_date DATETIME,
                duration INTEGER,
                file_size INTEGER,
                local_file_path TEXT,
                status TEXT CHECK(status IN ('new', 'unlistened', 'listened')) DEFAULT 'new',
                downloaded BOOLEAN DEFAULT FALSE,
                on_device BOOLEAN DEFAULT FALSE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (podcast_id) REFERENCES podcasts (id) ON DELETE CASCADE
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        // User Story #8-11: USB device integration
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS usb_devices (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                device_name TEXT NOT NULL,
                device_path TEXT NOT NULL,
                last_connected DATETIME DEFAULT CURRENT_TIMESTAMP
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        // User Story #9-11: Track episode transfers to USB devices
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS episode_transfers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                episode_id INTEGER NOT NULL,
                device_id INTEGER NOT NULL,
                transferred_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                file_path_on_device TEXT,
                FOREIGN KEY (episode_id) REFERENCES episodes (id) ON DELETE CASCADE,
                FOREIGN KEY (device_id) REFERENCES usb_devices (id) ON DELETE CASCADE,
                UNIQUE(episode_id, device_id)
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        log::info!("Database tables created successfully");
        Ok(())
    }

    pub async fn add_podcast(
        &self,
        name: &str,
        rss_url: &str,
        description: Option<&str>,
        artwork_url: Option<&str>,
        website_url: Option<&str>,
    ) -> Result<Podcast, PodPicoError> {
        log::info!("Adding podcast to database: {} (User Story #1)", rss_url);

        let result = sqlx::query(r#"
            INSERT INTO podcasts (name, rss_url, description, artwork_url, website_url, last_updated)
            VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
        "#)
        .bind(name)
        .bind(rss_url)
        .bind(description)
        .bind(artwork_url)
        .bind(website_url)
        .execute(&self.pool)
        .await?;

        let podcast_id = result.last_insert_rowid();

        // Return the created podcast
        self.get_podcast_by_id(podcast_id).await
    }

    pub async fn get_podcast_by_id(&self, podcast_id: i64) -> Result<Podcast, PodPicoError> {
        let row = sqlx::query(r#"
            SELECT p.id, p.name, p.rss_url, p.description, p.artwork_url, p.website_url, p.last_updated,
                   COUNT(e.id) as episode_count,
                   COUNT(CASE WHEN e.status = 'new' THEN 1 END) as new_episode_count
            FROM podcasts p
            LEFT JOIN episodes e ON p.id = e.podcast_id
            WHERE p.id = ?
            GROUP BY p.id
        "#)
        .bind(podcast_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Podcast {
            id: row.get("id"),
            name: row.get("name"),
            rss_url: row.get("rss_url"),
            description: row.get("description"),
            artwork_url: row.get("artwork_url"),
            website_url: row.get("website_url"),
            last_updated: row.get("last_updated"),
            episode_count: row.get("episode_count"),
            new_episode_count: row.get("new_episode_count"),
        })
    }

    pub async fn get_podcasts(&self) -> Result<Vec<Podcast>, PodPicoError> {
        log::info!("Retrieving podcasts from database (User Story #2, #7)");

        let rows = sqlx::query(r#"
            SELECT p.id, p.name, p.rss_url, p.description, p.artwork_url, p.website_url, p.last_updated,
                   COUNT(e.id) as episode_count,
                   COUNT(CASE WHEN e.status = 'new' THEN 1 END) as new_episode_count
            FROM podcasts p
            LEFT JOIN episodes e ON p.id = e.podcast_id
            GROUP BY p.id
            ORDER BY p.name
        "#)
        .fetch_all(&self.pool)
        .await?;

        let podcasts = rows
            .into_iter()
            .map(|row| Podcast {
                id: row.get("id"),
                name: row.get("name"),
                rss_url: row.get("rss_url"),
                description: row.get("description"),
                artwork_url: row.get("artwork_url"),
                website_url: row.get("website_url"),
                last_updated: row.get("last_updated"),
                episode_count: row.get("episode_count"),
                new_episode_count: row.get("new_episode_count"),
            })
            .collect();

        Ok(podcasts)
    }

    pub async fn remove_podcast(&self, podcast_id: i64) -> Result<(), PodPicoError> {
        log::info!(
            "Removing podcast from database: {} (User Story #4)",
            podcast_id
        );

        sqlx::query("DELETE FROM podcasts WHERE id = ?")
            .bind(podcast_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_episodes(
        &self,
        podcast_id: Option<i64>,
    ) -> Result<Vec<Episode>, PodPicoError> {
        log::info!(
            "Retrieving episodes from database for podcast: {:?} (User Story #2, #7)",
            podcast_id
        );

        let rows = if let Some(podcast_id) = podcast_id {
            // User Story #2: Get episodes for specific podcast
            sqlx::query(
                r#"
                SELECT e.id, e.podcast_id, p.name as podcast_name, e.title, e.description, 
                       e.episode_url, e.published_date, e.duration, e.file_size, 
                       e.local_file_path, e.status, e.downloaded, e.on_device
                FROM episodes e
                JOIN podcasts p ON e.podcast_id = p.id
                WHERE e.podcast_id = ?
                ORDER BY e.published_date DESC
            "#,
            )
            .bind(podcast_id)
            .fetch_all(&self.pool)
            .await?
        } else {
            // User Story #7: Get all new episodes across all podcasts (Combined Inbox)
            sqlx::query(
                r#"
                SELECT e.id, e.podcast_id, p.name as podcast_name, e.title, e.description, 
                       e.episode_url, e.published_date, e.duration, e.file_size, 
                       e.local_file_path, e.status, e.downloaded, e.on_device
                FROM episodes e
                JOIN podcasts p ON e.podcast_id = p.id
                WHERE e.status = 'new'
                ORDER BY e.published_date DESC
            "#,
            )
            .fetch_all(&self.pool)
            .await?
        };

        let episodes = rows
            .into_iter()
            .map(|row| Episode {
                id: row.get("id"),
                podcast_id: row.get("podcast_id"),
                podcast_name: row.get("podcast_name"),
                title: row.get("title"),
                description: row.get("description"),
                episode_url: row.get("episode_url"),
                published_date: row.get("published_date"),
                duration: row.get("duration"),
                file_size: row.get("file_size"),
                local_file_path: row.get("local_file_path"),
                status: row.get("status"),
                downloaded: row.get("downloaded"),
                on_device: row.get("on_device"),
            })
            .collect();

        Ok(episodes)
    }

    pub async fn update_episode_status(
        &self,
        episode_id: i64,
        status: &str,
    ) -> Result<(), PodPicoError> {
        log::info!(
            "Updating episode {} status to: {} (User Story #5, #6)",
            episode_id,
            status
        );

        sqlx::query(
            r#"
            UPDATE episodes 
            SET status = ?, updated_at = CURRENT_TIMESTAMP 
            WHERE id = ?
        "#,
        )
        .bind(status)
        .bind(episode_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_episode_downloaded_status(
        &self,
        episode_id: i64,
        downloaded: bool,
        local_file_path: Option<&str>,
    ) -> Result<(), PodPicoError> {
        log::info!(
            "Updating episode {} downloaded status to: {} (User Story #3)",
            episode_id,
            downloaded
        );

        sqlx::query(
            r#"
            UPDATE episodes 
            SET downloaded = ?, local_file_path = ?, updated_at = CURRENT_TIMESTAMP 
            WHERE id = ?
        "#,
        )
        .bind(downloaded)
        .bind(local_file_path)
        .bind(episode_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_episode_on_device_status(
        &self,
        episode_id: i64,
        on_device: bool,
    ) -> Result<(), PodPicoError> {
        log::info!(
            "Updating episode {} on_device status to: {} (User Story #9)",
            episode_id,
            on_device
        );

        sqlx::query(
            r#"
            UPDATE episodes 
            SET on_device = ?, updated_at = CURRENT_TIMESTAMP 
            WHERE id = ?
        "#,
        )
        .bind(on_device)
        .bind(episode_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn add_episode(
        &self,
        podcast_id: i64,
        title: &str,
        description: Option<&str>,
        episode_url: &str,
        published_date: Option<&str>,
        duration: Option<i32>,
        file_size: Option<i64>,
    ) -> Result<i64, PodPicoError> {
        log::info!(
            "Adding episode to database for podcast {}: {}",
            podcast_id,
            title
        );

        let result = sqlx::query(r#"
            INSERT INTO episodes (podcast_id, title, description, episode_url, published_date, duration, file_size)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(podcast_id)
        .bind(title)
        .bind(description)
        .bind(episode_url)
        .bind(published_date)
        .bind(duration)
        .bind(file_size)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn create_test_db() -> DatabaseManager {
        // Use in-memory SQLite database for testing
        let db_url = "sqlite::memory:";

        let db = DatabaseManager::new(db_url).await.unwrap();
        db.initialize().await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_database_initialization() {
        // Test that database tables are created correctly
        let db = create_test_db().await;

        // Verify tables exist by attempting basic operations
        let podcasts = db.get_podcasts().await.unwrap();
        assert_eq!(podcasts.len(), 0);
    }

    #[tokio::test]
    async fn test_user_story_1_add_podcast() {
        // User Story #1: Add new podcast subscription via RSS URL
        let db = create_test_db().await;

        let podcast = db
            .add_podcast(
                "Test Podcast",
                "https://example.com/feed.xml",
                Some("A test podcast"),
                Some("https://example.com/art.jpg"),
                Some("https://example.com"),
            )
            .await
            .unwrap();

        assert_eq!(podcast.name, "Test Podcast");
        assert_eq!(podcast.rss_url, "https://example.com/feed.xml");
        assert_eq!(podcast.description, Some("A test podcast".to_string()));
        assert_eq!(
            podcast.artwork_url,
            Some("https://example.com/art.jpg".to_string())
        );
        assert_eq!(podcast.website_url, Some("https://example.com".to_string()));
        assert_eq!(podcast.episode_count, 0);
        assert_eq!(podcast.new_episode_count, 0);
        assert!(podcast.id > 0);
    }

    #[tokio::test]
    async fn test_add_podcast_duplicate_url() {
        // Test that duplicate RSS URLs are rejected
        let db = create_test_db().await;

        // Add first podcast
        db.add_podcast(
            "Test Podcast 1",
            "https://example.com/feed.xml",
            None,
            None,
            None,
        )
        .await
        .unwrap();

        // Try to add duplicate URL
        let result = db
            .add_podcast(
                "Test Podcast 2",
                "https://example.com/feed.xml",
                None,
                None,
                None,
            )
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_podcasts() {
        // User Story #2, #7: Get all podcasts with episode counts
        let db = create_test_db().await;

        // Add multiple podcasts
        let _podcast1 = db
            .add_podcast(
                "Podcast 1",
                "https://example1.com/feed.xml",
                None,
                None,
                None,
            )
            .await
            .unwrap();
        let _podcast2 = db
            .add_podcast(
                "Podcast 2",
                "https://example2.com/feed.xml",
                None,
                None,
                None,
            )
            .await
            .unwrap();

        let podcasts = db.get_podcasts().await.unwrap();
        assert_eq!(podcasts.len(), 2);

        // Should be sorted by name
        assert_eq!(podcasts[0].name, "Podcast 1");
        assert_eq!(podcasts[1].name, "Podcast 2");
    }

    #[tokio::test]
    async fn test_user_story_4_remove_podcast() {
        // User Story #4: Remove podcast subscriptions
        let db = create_test_db().await;

        let podcast = db
            .add_podcast(
                "Test Podcast",
                "https://example.com/feed.xml",
                None,
                None,
                None,
            )
            .await
            .unwrap();

        // Add an episode to test cascade deletion
        let _episode_id = db
            .add_episode(
                podcast.id,
                "Test Episode",
                Some("Description"),
                "https://example.com/episode.mp3",
                Some("2023-01-01T00:00:00Z"),
                Some(1800),
                Some(25000000),
            )
            .await
            .unwrap();

        // Remove podcast
        db.remove_podcast(podcast.id).await.unwrap();

        // Verify podcast is removed
        let podcasts = db.get_podcasts().await.unwrap();
        assert_eq!(podcasts.len(), 0);

        // Verify episodes are cascade deleted
        let episodes = db.get_episodes(Some(podcast.id)).await.unwrap();
        assert_eq!(episodes.len(), 0);
    }

    #[tokio::test]
    async fn test_remove_nonexistent_podcast() {
        let db = create_test_db().await;

        // Removing non-existent podcast should not error (returns 0 affected rows)
        let result = db.remove_podcast(999).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_episode() {
        let db = create_test_db().await;

        let podcast = db
            .add_podcast(
                "Test Podcast",
                "https://example.com/feed.xml",
                None,
                None,
                None,
            )
            .await
            .unwrap();

        let episode_id = db
            .add_episode(
                podcast.id,
                "Test Episode",
                Some("Episode description"),
                "https://example.com/episode.mp3",
                Some("2023-01-01T00:00:00Z"),
                Some(1800),     // 30 minutes
                Some(25000000), // ~25MB
            )
            .await
            .unwrap();

        assert!(episode_id > 0);

        // Verify episode was added
        let episodes = db.get_episodes(Some(podcast.id)).await.unwrap();
        assert_eq!(episodes.len(), 1);

        let episode = &episodes[0];
        assert_eq!(episode.title, "Test Episode");
        assert_eq!(episode.description, Some("Episode description".to_string()));
        assert_eq!(episode.episode_url, "https://example.com/episode.mp3");
        assert_eq!(episode.duration, Some(1800));
        assert_eq!(episode.file_size, Some(25000000));
        assert_eq!(episode.status, "new");
        assert!(!episode.downloaded);
        assert!(!episode.on_device);
    }

    #[tokio::test]
    async fn test_user_story_2_get_episodes_by_podcast() {
        // User Story #2: View all episodes of specific podcast
        let db = create_test_db().await;

        let podcast1 = db
            .add_podcast(
                "Podcast 1",
                "https://example1.com/feed.xml",
                None,
                None,
                None,
            )
            .await
            .unwrap();
        let podcast2 = db
            .add_podcast(
                "Podcast 2",
                "https://example2.com/feed.xml",
                None,
                None,
                None,
            )
            .await
            .unwrap();

        // Add episodes to podcast 1
        db.add_episode(
            podcast1.id,
            "Episode 1-1",
            None,
            "https://example.com/ep1.mp3",
            Some("2023-01-01T00:00:00Z"),
            None,
            None,
        )
        .await
        .unwrap();
        db.add_episode(
            podcast1.id,
            "Episode 1-2",
            None,
            "https://example.com/ep2.mp3",
            Some("2023-01-02T00:00:00Z"),
            None,
            None,
        )
        .await
        .unwrap();

        // Add episode to podcast 2
        db.add_episode(
            podcast2.id,
            "Episode 2-1",
            None,
            "https://example.com/ep3.mp3",
            Some("2023-01-03T00:00:00Z"),
            None,
            None,
        )
        .await
        .unwrap();

        // Get episodes for podcast 1 only
        let episodes = db.get_episodes(Some(podcast1.id)).await.unwrap();
        assert_eq!(episodes.len(), 2);

        // Should be ordered by published date DESC
        assert_eq!(episodes[0].title, "Episode 1-2");
        assert_eq!(episodes[1].title, "Episode 1-1");

        // All episodes should be from podcast 1
        for episode in &episodes {
            assert_eq!(episode.podcast_id, podcast1.id);
            assert_eq!(episode.podcast_name, "Podcast 1");
        }
    }

    #[tokio::test]
    async fn test_user_story_7_get_all_new_episodes() {
        // User Story #7: View all new episodes across podcasts (Combined Inbox)
        let db = create_test_db().await;

        let podcast1 = db
            .add_podcast(
                "Podcast 1",
                "https://example1.com/feed.xml",
                None,
                None,
                None,
            )
            .await
            .unwrap();
        let podcast2 = db
            .add_podcast(
                "Podcast 2",
                "https://example2.com/feed.xml",
                None,
                None,
                None,
            )
            .await
            .unwrap();

        // Add episodes with different statuses
        db.add_episode(
            podcast1.id,
            "New Episode 1",
            None,
            "https://example.com/ep1.mp3",
            Some("2023-01-03T00:00:00Z"),
            None,
            None,
        )
        .await
        .unwrap();
        let _episode2_id = db
            .add_episode(
                podcast2.id,
                "New Episode 2",
                None,
                "https://example.com/ep2.mp3",
                Some("2023-01-02T00:00:00Z"),
                None,
                None,
            )
            .await
            .unwrap();
        let episode3_id = db
            .add_episode(
                podcast1.id,
                "Old Episode",
                None,
                "https://example.com/ep3.mp3",
                Some("2023-01-01T00:00:00Z"),
                None,
                None,
            )
            .await
            .unwrap();

        // Mark one episode as listened
        db.update_episode_status(episode3_id, "listened")
            .await
            .unwrap();

        // Get all new episodes (no podcast filter)
        let new_episodes = db.get_episodes(None).await.unwrap();
        assert_eq!(new_episodes.len(), 2); // Only episodes with "new" status

        // Should be ordered by published date DESC
        assert_eq!(new_episodes[0].title, "New Episode 1");
        assert_eq!(new_episodes[1].title, "New Episode 2");

        // Episodes should have correct podcast names
        assert_eq!(new_episodes[0].podcast_name, "Podcast 1");
        assert_eq!(new_episodes[1].podcast_name, "Podcast 2");

        // All should have "new" status
        for episode in &new_episodes {
            assert_eq!(episode.status, "new");
        }
    }

    #[tokio::test]
    async fn test_user_story_5_update_episode_status() {
        // User Story #5: Mark episodes as "listened"
        let db = create_test_db().await;

        let podcast = db
            .add_podcast(
                "Test Podcast",
                "https://example.com/feed.xml",
                None,
                None,
                None,
            )
            .await
            .unwrap();
        let episode_id = db
            .add_episode(
                podcast.id,
                "Test Episode",
                None,
                "https://example.com/ep.mp3",
                None,
                None,
                None,
            )
            .await
            .unwrap();

        // Initially should be "new"
        let episodes = db.get_episodes(Some(podcast.id)).await.unwrap();
        assert_eq!(episodes[0].status, "new");

        // Update to "listened"
        db.update_episode_status(episode_id, "listened")
            .await
            .unwrap();

        // Verify status changed
        let episodes = db.get_episodes(Some(podcast.id)).await.unwrap();
        assert_eq!(episodes[0].status, "listened");

        // Update to "unlistened"
        db.update_episode_status(episode_id, "unlistened")
            .await
            .unwrap();

        // Verify status changed again
        let episodes = db.get_episodes(Some(podcast.id)).await.unwrap();
        assert_eq!(episodes[0].status, "unlistened");
    }

    #[tokio::test]
    async fn test_update_episode_status_invalid() {
        let db = create_test_db().await;

        let podcast = db
            .add_podcast(
                "Test Podcast",
                "https://example.com/feed.xml",
                None,
                None,
                None,
            )
            .await
            .unwrap();
        let episode_id = db
            .add_episode(
                podcast.id,
                "Test Episode",
                None,
                "https://example.com/ep.mp3",
                None,
                None,
                None,
            )
            .await
            .unwrap();

        // Try to set invalid status (should be rejected by CHECK constraint)
        let result = db.update_episode_status(episode_id, "invalid_status").await;
        assert!(result.is_err());

        // Status should remain unchanged
        let episodes = db.get_episodes(Some(podcast.id)).await.unwrap();
        assert_eq!(episodes[0].status, "new");
    }

    #[tokio::test]
    async fn test_update_nonexistent_episode_status() {
        let db = create_test_db().await;

        // Try to update non-existent episode
        let result = db.update_episode_status(999, "listened").await;
        assert!(result.is_ok()); // SQLx returns Ok even if no rows affected
    }

    #[tokio::test]
    async fn test_podcast_episode_counts() {
        // Test that episode counts are calculated correctly
        let db = create_test_db().await;

        let podcast = db
            .add_podcast(
                "Test Podcast",
                "https://example.com/feed.xml",
                None,
                None,
                None,
            )
            .await
            .unwrap();

        // Add episodes with different statuses
        let _ep1_id = db
            .add_episode(
                podcast.id,
                "Episode 1",
                None,
                "https://example.com/ep1.mp3",
                None,
                None,
                None,
            )
            .await
            .unwrap();
        let ep2_id = db
            .add_episode(
                podcast.id,
                "Episode 2",
                None,
                "https://example.com/ep2.mp3",
                None,
                None,
                None,
            )
            .await
            .unwrap();
        let ep3_id = db
            .add_episode(
                podcast.id,
                "Episode 3",
                None,
                "https://example.com/ep3.mp3",
                None,
                None,
                None,
            )
            .await
            .unwrap();

        // Mark one as listened
        db.update_episode_status(ep2_id, "listened").await.unwrap();

        // Get podcast with updated counts
        let podcast = db.get_podcast_by_id(podcast.id).await.unwrap();
        assert_eq!(podcast.episode_count, 3);
        assert_eq!(podcast.new_episode_count, 2); // ep1 and ep3 are still "new"

        // Mark another as unlistened
        db.update_episode_status(ep3_id, "unlistened")
            .await
            .unwrap();

        let podcast = db.get_podcast_by_id(podcast.id).await.unwrap();
        assert_eq!(podcast.episode_count, 3);
        assert_eq!(podcast.new_episode_count, 1); // only ep1 is "new"
    }

    #[tokio::test]
    async fn test_get_podcast_by_id_not_found() {
        let db = create_test_db().await;

        let result = db.get_podcast_by_id(999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_episode_invalid_podcast() {
        let db = create_test_db().await;

        // Try to add episode to non-existent podcast
        let result = db
            .add_episode(
                999,
                "Test Episode",
                None,
                "https://example.com/ep.mp3",
                None,
                None,
                None,
            )
            .await;

        assert!(result.is_err()); // Should fail due to foreign key constraint
    }
}
