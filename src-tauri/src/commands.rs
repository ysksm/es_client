// Commands module
// This module contains Tauri command handlers

use crate::models::{AppConfig, ClusterInfo, ExtractionJob, ProfileConfig, SampleIndexConfig};
use crate::sample_data;
use crate::services::{ConfigService, DuckDBService, ESClient};
use std::sync::Mutex;
use tauri::State;

/// Shared state for services
pub struct AppState {
    pub config_service: Mutex<ConfigService>,
    pub duckdb_service: Mutex<DuckDBService>,
}

impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            config_service: Mutex::new(ConfigService::new()?),
            duckdb_service: Mutex::new(DuckDBService::new()?),
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

// === Data Extraction Commands ===

#[tauri::command]
pub async fn extract_and_store_data(
    state: State<'_, AppState>,
    profile_name: String,
    index_name: String,
    query: serde_json::Value,
    table_name: String,
) -> Result<String, String> {
    // Get profile and create ES client
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

    // Search documents from Elasticsearch
    let search_result = client
        .search(&index_name, query.clone())
        .await
        .map_err(|e| format!("Search failed: {}", e))?;

    // Extract hits from search result
    let hits = search_result["hits"]["hits"]
        .as_array()
        .ok_or("Invalid search response")?;

    let documents: Vec<serde_json::Value> = hits
        .iter()
        .filter_map(|hit| hit["_source"].clone().as_object().map(|obj| serde_json::Value::Object(obj.clone())))
        .collect();

    if documents.is_empty() {
        return Err("No documents found".to_string());
    }

    // Create extraction job
    let mut job = ExtractionJob::new(
        profile_name.clone(),
        index_name.clone(),
        serde_json::to_string(&query).unwrap_or_default(),
        table_name.clone(),
    );
    job.record_count = documents.len() as i64;

    // Store in DuckDB
    {
        let duckdb_service = state
            .duckdb_service
            .lock()
            .map_err(|e| format!("Failed to lock duckdb service: {}", e))?;

        // Create table with schema from documents
        duckdb_service
            .create_data_table(&table_name, &documents)
            .map_err(|e| format!("Failed to create table: {}", e))?;

        // Insert data
        let inserted = duckdb_service
            .insert_data(&table_name, documents)
            .map_err(|e| format!("Failed to insert data: {}", e))?;

        job.record_count = inserted as i64;
        job.status = "completed".to_string();

        // Save extraction job to history
        duckdb_service
            .save_extraction_job(&job)
            .map_err(|e| format!("Failed to save extraction job: {}", e))?;
    }

    Ok(format!(
        "Successfully extracted {} documents to table '{}'",
        job.record_count, table_name
    ))
}

#[tauri::command]
pub async fn get_extraction_history(state: State<'_, AppState>) -> Result<Vec<ExtractionJob>, String> {
    let duckdb_service = state
        .duckdb_service
        .lock()
        .map_err(|e| format!("Failed to lock duckdb service: {}", e))?;

    duckdb_service
        .get_extraction_history()
        .map_err(|e| format!("Failed to get extraction history: {}", e))
}

#[tauri::command]
pub async fn query_extracted_data(
    state: State<'_, AppState>,
    table_name: String,
    limit: Option<usize>,
) -> Result<Vec<serde_json::Value>, String> {
    let duckdb_service = state
        .duckdb_service
        .lock()
        .map_err(|e| format!("Failed to lock duckdb service: {}", e))?;

    duckdb_service
        .query_table(&table_name, limit)
        .map_err(|e| format!("Failed to query table: {}", e))
}

#[tauri::command]
pub async fn list_duckdb_tables(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let duckdb_service = state
        .duckdb_service
        .lock()
        .map_err(|e| format!("Failed to lock duckdb service: {}", e))?;

    duckdb_service
        .list_tables()
        .map_err(|e| format!("Failed to list tables: {}", e))
}
