// Database management module for PodPico
// Handles SQLite database operations for podcasts, episodes, and related data
// User Stories #1-11: Podcast and Episode Management

use sqlx::{SqlitePool, Row};
use crate::error::PodPicoError;
use crate::commands::{Podcast, Episode};

pub struct DatabaseManager {
    pool: SqlitePool,
}

impl DatabaseManager {
    pub async fn new(database_url: &str) -> Result<Self, PodPicoError> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn initialize(&self) -> Result<(), PodPicoError> {
        log::info!("Creating database tables for podcast management user stories");
        
        // User Story #1: Add podcast subscription
        // User Story #4: Remove podcast subscription
        sqlx::query(r#"
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
        "#)
        .execute(&self.pool)
        .await?;
        
        // User Story #2: View episodes of specific podcast
        // User Story #3: Download episodes
        // User Story #5: Mark episodes as listened
        sqlx::query(r#"
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
        "#)
        .execute(&self.pool)
        .await?;
        
        // User Story #8-11: USB device integration
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS usb_devices (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                device_name TEXT NOT NULL,
                device_path TEXT NOT NULL,
                last_connected DATETIME DEFAULT CURRENT_TIMESTAMP
            )
        "#)
        .execute(&self.pool)
        .await?;
        
        // User Story #9-11: Track episode transfers to USB devices
        sqlx::query(r#"
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
        "#)
        .execute(&self.pool)
        .await?;
        
        log::info!("Database tables created successfully");
        Ok(())
    }

    pub async fn add_podcast(&self, name: &str, rss_url: &str, description: Option<&str>, artwork_url: Option<&str>, website_url: Option<&str>) -> Result<Podcast, PodPicoError> {
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
        
        let podcasts = rows.into_iter().map(|row| {
            Podcast {
                id: row.get("id"),
                name: row.get("name"),
                rss_url: row.get("rss_url"),
                description: row.get("description"),
                artwork_url: row.get("artwork_url"),
                website_url: row.get("website_url"),
                last_updated: row.get("last_updated"),
                episode_count: row.get("episode_count"),
                new_episode_count: row.get("new_episode_count"),
            }
        }).collect();
        
        Ok(podcasts)
    }

    pub async fn remove_podcast(&self, podcast_id: i64) -> Result<(), PodPicoError> {
        log::info!("Removing podcast from database: {} (User Story #4)", podcast_id);
        
        sqlx::query("DELETE FROM podcasts WHERE id = ?")
            .bind(podcast_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }

    pub async fn get_episodes(&self, podcast_id: Option<i64>) -> Result<Vec<Episode>, PodPicoError> {
        log::info!("Retrieving episodes from database for podcast: {:?} (User Story #2, #7)", podcast_id);
        
        let rows = if let Some(podcast_id) = podcast_id {
            // User Story #2: Get episodes for specific podcast
            sqlx::query(r#"
                SELECT e.id, e.podcast_id, p.name as podcast_name, e.title, e.description, 
                       e.episode_url, e.published_date, e.duration, e.file_size, 
                       e.local_file_path, e.status, e.downloaded, e.on_device
                FROM episodes e
                JOIN podcasts p ON e.podcast_id = p.id
                WHERE e.podcast_id = ?
                ORDER BY e.published_date DESC
            "#)
            .bind(podcast_id)
            .fetch_all(&self.pool)
            .await?
        } else {
            // User Story #7: Get all new episodes across all podcasts (Combined Inbox)
            sqlx::query(r#"
                SELECT e.id, e.podcast_id, p.name as podcast_name, e.title, e.description, 
                       e.episode_url, e.published_date, e.duration, e.file_size, 
                       e.local_file_path, e.status, e.downloaded, e.on_device
                FROM episodes e
                JOIN podcasts p ON e.podcast_id = p.id
                WHERE e.status = 'new'
                ORDER BY e.published_date DESC
            "#)
            .fetch_all(&self.pool)
            .await?
        };
        
        let episodes = rows.into_iter().map(|row| {
            Episode {
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
            }
        }).collect();
        
        Ok(episodes)
    }

    pub async fn update_episode_status(&self, episode_id: i64, status: &str) -> Result<(), PodPicoError> {
        log::info!("Updating episode {} status to: {} (User Story #5, #6)", episode_id, status);
        
        sqlx::query(r#"
            UPDATE episodes 
            SET status = ?, updated_at = CURRENT_TIMESTAMP 
            WHERE id = ?
        "#)
        .bind(status)
        .bind(episode_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn add_episode(&self, podcast_id: i64, title: &str, description: Option<&str>, episode_url: &str, published_date: Option<&str>, duration: Option<i32>, file_size: Option<i64>) -> Result<i64, PodPicoError> {
        log::info!("Adding episode to database for podcast {}: {}", podcast_id, title);
        
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