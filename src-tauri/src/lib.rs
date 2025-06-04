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
use rss_manager::RssManager;
use std::fs;

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
            commands::update_episode_status,
            // Download management commands
            commands::download_episode,
            commands::get_download_progress,
            // USB device management commands
            commands::get_usb_devices,
            commands::transfer_episode_to_device,
            commands::remove_episode_from_device,
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

    // Create data directory in the current working directory for now
    let data_dir = std::path::PathBuf::from("./data");
    fs::create_dir_all(&data_dir)?;
    log::info!("Created data directory: {}", data_dir.display());

    // Initialize database - create the file first to ensure permissions work
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

    // Initialize managers globally
    commands::initialize_managers(db, rss_manager).await;

    log::info!("Database and RSS manager initialized successfully");
    Ok(())
}
