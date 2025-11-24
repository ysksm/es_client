// Commands module
// This module contains Tauri command handlers

use crate::models::{AppConfig, ClusterInfo, ProfileConfig, SampleIndexConfig};
use crate::sample_data;
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

// === Sample Index Commands ===

#[tauri::command]
pub async fn list_sample_index_templates() -> Result<Vec<SampleIndexConfig>, String> {
    Ok(SampleIndexConfig::all())
}

#[tauri::command]
pub async fn create_sample_index(
    state: State<'_, AppState>,
    profile_name: String,
    template_name: String,
    document_count: usize,
) -> Result<String, String> {
    // Get the sample index config
    let template = match template_name.as_str() {
        "ecommerce_products" => SampleIndexConfig::ecommerce_products(),
        "application_logs" => SampleIndexConfig::application_logs(),
        "user_analytics" => SampleIndexConfig::user_analytics(),
        _ => return Err("Invalid template name".to_string()),
    };

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

    // Check if index already exists
    let exists = client
        .index_exists(&template.index_name)
        .await
        .map_err(|e| format!("Failed to check index existence: {}", e))?;

    if exists {
        return Err(format!("Index '{}' already exists", template.index_name));
    }

    // Create the index
    client
        .create_index(
            &template.index_name,
            Some(template.settings),
            Some(template.mappings),
        )
        .await
        .map_err(|e| format!("Failed to create index: {}", e))?;

    // Generate and insert sample data
    let documents = match template_name.as_str() {
        "ecommerce_products" => sample_data::generate_products(document_count),
        "application_logs" => sample_data::generate_logs(document_count),
        "user_analytics" => sample_data::generate_analytics(document_count),
        _ => return Err("Invalid template name".to_string()),
    };

    let inserted_count = client
        .bulk_insert(&template.index_name, documents)
        .await
        .map_err(|e| format!("Failed to bulk insert: {}", e))?;

    Ok(format!(
        "Successfully created index '{}' with {} documents",
        template.index_name, inserted_count
    ))
}

#[tauri::command]
pub async fn search_documents(
    state: State<'_, AppState>,
    profile_name: String,
    index_name: String,
    query: serde_json::Value,
) -> Result<serde_json::Value, String> {
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
        .search(&index_name, query)
        .await
        .map_err(|e| format!("Search failed: {}", e))
}

#[tauri::command]
pub async fn count_documents(
    state: State<'_, AppState>,
    profile_name: String,
    index_name: String,
) -> Result<u64, String> {
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
        .count(&index_name)
        .await
        .map_err(|e| format!("Count failed: {}", e))
}
