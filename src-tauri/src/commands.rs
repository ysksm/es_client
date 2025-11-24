// Commands module
// This module contains Tauri command handlers

use crate::models::{AppConfig, ClusterInfo, ProfileConfig};
use crate::services::{ConfigService, ESClient};
use std::sync::Mutex;
use tauri::State;

/// Shared state for ConfigService
pub struct AppState {
    pub config_service: Mutex<ConfigService>,
}

impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            config_service: Mutex::new(ConfigService::new()?),
        })
    }
}

// === Profile Management Commands ===

#[tauri::command]
pub async fn list_profiles(state: State<'_, AppState>) -> Result<Vec<ProfileConfig>, String> {
    let config_service = state
        .config_service
        .lock()
        .map_err(|e| format!("Failed to lock config service: {}", e))?;

    config_service
        .load_profiles()
        .map_err(|e| format!("Failed to load profiles: {}", e))
}

#[tauri::command]
pub async fn get_profile(
    state: State<'_, AppState>,
    name: String,
) -> Result<ProfileConfig, String> {
    let config_service = state
        .config_service
        .lock()
        .map_err(|e| format!("Failed to lock config service: {}", e))?;

    config_service
        .get_profile(&name)
        .map_err(|e| format!("Failed to get profile: {}", e))
}

#[tauri::command]
pub async fn save_profile(
    state: State<'_, AppState>,
    profile: ProfileConfig,
) -> Result<(), String> {
    let config_service = state
        .config_service
        .lock()
        .map_err(|e| format!("Failed to lock config service: {}", e))?;

    config_service
        .save_profile(&profile)
        .map_err(|e| format!("Failed to save profile: {}", e))
}

#[tauri::command]
pub async fn delete_profile(state: State<'_, AppState>, name: String) -> Result<(), String> {
    let config_service = state
        .config_service
        .lock()
        .map_err(|e| format!("Failed to lock config service: {}", e))?;

    config_service
        .delete_profile(&name)
        .map_err(|e| format!("Failed to delete profile: {}", e))
}

#[tauri::command]
pub async fn encrypt_password(state: State<'_, AppState>, password: String) -> Result<String, String> {
    let config_service = state
        .config_service
        .lock()
        .map_err(|e| format!("Failed to lock config service: {}", e))?;

    config_service
        .encrypt_password(&password)
        .map_err(|e| format!("Failed to encrypt password: {}", e))
}

// === App Configuration Commands ===

#[tauri::command]
pub async fn load_app_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let config_service = state
        .config_service
        .lock()
        .map_err(|e| format!("Failed to lock config service: {}", e))?;

    config_service
        .load_app_config()
        .map_err(|e| format!("Failed to load app config: {}", e))
}

#[tauri::command]
pub async fn save_app_config(
    state: State<'_, AppState>,
    config: AppConfig,
) -> Result<(), String> {
    let config_service = state
        .config_service
        .lock()
        .map_err(|e| format!("Failed to lock config service: {}", e))?;

    config_service
        .save_app_config(&config)
        .map_err(|e| format!("Failed to save app config: {}", e))
}

// === Connection Commands ===

#[tauri::command]
pub async fn test_connection(
    state: State<'_, AppState>,
    profile_name: String,
) -> Result<bool, String> {
    let profile = {
        let config_service = state
            .config_service
            .lock()
            .map_err(|e| format!("Failed to lock config service: {}", e))?;

        config_service
            .get_profile(&profile_name)
            .map_err(|e| format!("Failed to get profile: {}", e))?
    };

    let encryptor = crate::utils::Encryptor::new()
        .map_err(|e| format!("Failed to create encryptor: {}", e))?;

    let client = ESClient::new(&profile, &encryptor)
        .map_err(|e| format!("Failed to create ES client: {}", e))?;

    client
        .test_connection()
        .await
        .map_err(|e| format!("Connection test failed: {}", e))
}

#[tauri::command]
pub async fn get_cluster_info(
    state: State<'_, AppState>,
    profile_name: String,
) -> Result<ClusterInfo, String> {
    let profile = {
        let config_service = state
            .config_service
            .lock()
            .map_err(|e| format!("Failed to lock config service: {}", e))?;

        config_service
            .get_profile(&profile_name)
            .map_err(|e| format!("Failed to get profile: {}", e))?
    };

    let encryptor = crate::utils::Encryptor::new()
        .map_err(|e| format!("Failed to create encryptor: {}", e))?;

    let client = ESClient::new(&profile, &encryptor)
        .map_err(|e| format!("Failed to create ES client: {}", e))?;

    client
        .get_cluster_info()
        .await
        .map_err(|e| format!("Failed to get cluster info: {}", e))
}

// === Index Management Commands ===

#[tauri::command]
pub async fn list_indices(
    state: State<'_, AppState>,
    profile_name: String,
) -> Result<Vec<String>, String> {
    let profile = {
        let config_service = state
            .config_service
            .lock()
            .map_err(|e| format!("Failed to lock config service: {}", e))?;

        config_service
            .get_profile(&profile_name)
            .map_err(|e| format!("Failed to get profile: {}", e))?
    };

    let encryptor = crate::utils::Encryptor::new()
        .map_err(|e| format!("Failed to create encryptor: {}", e))?;

    let client = ESClient::new(&profile, &encryptor)
        .map_err(|e| format!("Failed to create ES client: {}", e))?;

    client
        .list_indices()
        .await
        .map_err(|e| format!("Failed to list indices: {}", e))
}

#[tauri::command]
pub async fn create_index(
    state: State<'_, AppState>,
    profile_name: String,
    index_name: String,
    settings: Option<serde_json::Value>,
    mappings: Option<serde_json::Value>,
) -> Result<(), String> {
    let profile = {
        let config_service = state
            .config_service
            .lock()
            .map_err(|e| format!("Failed to lock config service: {}", e))?;

        config_service
            .get_profile(&profile_name)
            .map_err(|e| format!("Failed to get profile: {}", e))?
    };

    let encryptor = crate::utils::Encryptor::new()
        .map_err(|e| format!("Failed to create encryptor: {}", e))?;

    let client = ESClient::new(&profile, &encryptor)
        .map_err(|e| format!("Failed to create ES client: {}", e))?;

    client
        .create_index(&index_name, settings, mappings)
        .await
        .map_err(|e| format!("Failed to create index: {}", e))
}

#[tauri::command]
pub async fn delete_index(
    state: State<'_, AppState>,
    profile_name: String,
    index_name: String,
) -> Result<(), String> {
    let profile = {
        let config_service = state
            .config_service
            .lock()
            .map_err(|e| format!("Failed to lock config service: {}", e))?;

        config_service
            .get_profile(&profile_name)
            .map_err(|e| format!("Failed to get profile: {}", e))?
    };

    let encryptor = crate::utils::Encryptor::new()
        .map_err(|e| format!("Failed to create encryptor: {}", e))?;

    let client = ESClient::new(&profile, &encryptor)
        .map_err(|e| format!("Failed to create ES client: {}", e))?;

    client
        .delete_index(&index_name)
        .await
        .map_err(|e| format!("Failed to delete index: {}", e))
}

#[tauri::command]
pub async fn index_exists(
    state: State<'_, AppState>,
    profile_name: String,
    index_name: String,
) -> Result<bool, String> {
    let profile = {
        let config_service = state
            .config_service
            .lock()
            .map_err(|e| format!("Failed to lock config service: {}", e))?;

        config_service
            .get_profile(&profile_name)
            .map_err(|e| format!("Failed to get profile: {}", e))?
    };

    let encryptor = crate::utils::Encryptor::new()
        .map_err(|e| format!("Failed to create encryptor: {}", e))?;

    let client = ESClient::new(&profile, &encryptor)
        .map_err(|e| format!("Failed to create ES client: {}", e))?;

    client
        .index_exists(&index_name)
        .await
        .map_err(|e| format!("Failed to check index existence: {}", e))
}
