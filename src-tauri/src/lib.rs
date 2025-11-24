// Modules
mod commands;
mod models;
mod services;
mod utils;

use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize app state
    let app_state = AppState::new().expect("Failed to initialize app state");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Profile management
            commands::list_profiles,
            commands::get_profile,
            commands::save_profile,
            commands::delete_profile,
            commands::encrypt_password,
            // App configuration
            commands::load_app_config,
            commands::save_app_config,
            // Connection
            commands::test_connection,
            commands::get_cluster_info,
            // Index management
            commands::list_indices,
            commands::create_index,
            commands::delete_index,
            commands::index_exists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
