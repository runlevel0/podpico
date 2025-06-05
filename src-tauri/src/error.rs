// Error types for PodPico application

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PodPicoError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("RSS parsing error: {0}")]
    RssParsing(#[from] rss::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("UUID parsing error: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Invalid RSS feed URL: {0}")]
    InvalidRssUrl(String),

    #[error("Episode not found: {0}")]
    EpisodeNotFound(i64),

    #[error("Podcast not found: {0}")]
    PodcastNotFound(i64),

    #[error("USB device not found: {0}")]
    UsbDeviceNotFound(String),

    #[error("File transfer failed: {0}")]
    FileTransferFailed(String),

    #[error("Download in progress")]
    DownloadInProgress,

    #[error("Generic error: {0}")]
    Generic(String),
}

// Convert PodPicoError to String for Tauri commands
impl From<PodPicoError> for String {
    fn from(error: PodPicoError) -> Self {
        error.to_string()
    }
}
