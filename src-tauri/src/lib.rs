// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// PodPico - Desktop podcast management application
// Backend entry point and module declarations

// Module declarations
pub mod commands;
pub mod database;
pub mod rss_manager;
pub mod usb_manager;
pub mod file_manager;
pub mod episode_manager;
pub mod config;
pub mod error;

// Re-exports
pub use commands::*;
pub use error::PodPicoError;

// Tauri application entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    env_logger::init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
