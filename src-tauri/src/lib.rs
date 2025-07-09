// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// PodPico - Desktop podcast management application
// Backend entry point and module declarations
// User Story Foundation Setup

// Module declarations
pub mod commands;
pub mod config;
pub mod database;
pub mod episode_manager;
pub mod error;
pub mod file_manager;
pub mod rss_manager;
pub mod usb_manager;

// Re-exports
pub use commands::*;
pub use error::PodPicoError;

use database::DatabaseManager;
use file_manager::FileManager;
use rss_manager::RssManager;
use std::fs;
use usb_manager::UsbManager;

// Tauri application entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            // Initialize database and managers
            tauri::async_runtime::spawn(async move {
                if let Err(e) = initialize_app().await {
                    log::error!("Failed to initialize application: {}", e);
                    std::process::exit(1);
                }
                log::info!("Application initialized successfully");
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Podcast management commands
            commands::add_podcast,
            commands::remove_podcast,
            commands::get_podcasts,
            commands::get_episodes,
            commands::search_episodes,
            commands::update_episode_status,
            // Download management commands
            commands::download_episode,
            commands::get_download_progress,
            // Episode file management commands
            commands::delete_downloaded_episode,
            // USB device management commands
            commands::get_usb_devices,
            commands::transfer_episode_to_device,
            commands::remove_episode_from_device,
            // User Story #11: Episode device status management commands
            commands::sync_episode_device_status,
            commands::get_device_episodes_by_podcast,
            commands::get_device_status_indicators,
            commands::verify_episode_status_consistency,
            // Configuration commands
            commands::get_app_config,
            commands::update_app_config,
            // Development/demo command
            commands::greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn initialize_app() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log::info!("Initializing PodPico application...");

    // Create data directory OUTSIDE src-tauri to avoid file watcher conflicts
    let data_dir = std::path::PathBuf::from("../data");
    fs::create_dir_all(&data_dir)?;
    log::info!("Created data directory: {}", data_dir.display());

    // Create downloads directory for episodes OUTSIDE src-tauri to avoid file watcher conflicts
    let downloads_dir = std::path::PathBuf::from("../episodes");
    fs::create_dir_all(&downloads_dir)?;
    log::info!("Created downloads directory: {}", downloads_dir.display());

    // Initialize database OUTSIDE src-tauri to avoid file watcher conflicts
    let db_path = data_dir.join("podcasts.db");

    // Create the database file if it doesn't exist
    if !db_path.exists() {
        std::fs::File::create(&db_path)?;
        log::info!("Created new database file: {}", db_path.display());
    }

    let database_url = format!("sqlite:{}", db_path.display());
    log::info!("Connecting to database: {}", database_url);

    let db = DatabaseManager::new(&database_url).await?;

    // Create database tables (User Stories #1-11 foundation)
    log::info!("Creating database schema for podcast management...");
    db.initialize().await?;

    // Initialize RSS manager
    let rss_manager = RssManager::new();

    // Initialize file manager (User Story #3, #9)
    let downloads_dir_str = downloads_dir.to_string_lossy().to_string();
    let file_manager = FileManager::new(&downloads_dir_str);
    file_manager.initialize().await?;
    log::info!(
        "File manager initialized with downloads directory: {}",
        downloads_dir_str
    );

    // Initialize USB manager (User Stories #8, #9, #10, #11)
    let usb_manager = UsbManager::new();
    log::info!("USB manager initialized for device operations");

    // Initialize managers globally
    commands::initialize_managers(db, rss_manager, file_manager, usb_manager).await;

    log::info!("All managers initialized successfully");
    Ok(())
}
