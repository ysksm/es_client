// Models module
// This module contains data structures and types

use serde::{Deserialize, Serialize};

/// Authentication type for Elasticsearch connection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    Basic,
    ApiKey,
}

/// Connection profile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub name: String,
    pub host: String,
    pub username: Option<String>,
    /// Encrypted password as HEX string
    pub password_encrypted: Option<String>,
    /// Encrypted API key as HEX string
    pub api_key_encrypted: Option<String>,
    pub auth_type: AuthType,
    pub use_ssl: bool,
    pub verify_certificate: bool,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ProfileConfig {
    /// Create a new profile with default values
    pub fn new(name: String, host: String, auth_type: AuthType) -> Self {
        let now = chrono::Utc::now();
        Self {
            name,
            host,
            username: None,
            password_encrypted: None,
            api_key_encrypted: None,
            auth_type,
            use_ssl: true,
            verify_certificate: true,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub default_profile: Option<String>,
    pub log_level: String,
    pub database_path: String,
    pub max_memory: String,
    pub theme: String,
    pub recent_profiles: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_profile: None,
            log_level: "info".to_string(),
            database_path: "~/.es_client/data.duckdb".to_string(),
            max_memory: "2GB".to_string(),
            theme: "dark".to_string(),
            recent_profiles: Vec::new(),
        }
    }
}

/// Cluster information from Elasticsearch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterInfo {
    pub name: String,
    pub cluster_name: String,
    pub cluster_uuid: String,
    pub version: VersionInfo,
}

/// Elasticsearch version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub number: String,
    pub build_flavor: String,
    pub build_type: String,
}

/// Sample index configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleIndexConfig {
    pub name: String,
    pub description: String,
    pub index_name: String,
    pub mappings: serde_json::Value,
    pub settings: serde_json::Value,
}

impl SampleIndexConfig {
    /// E-commerce products sample index
    pub fn ecommerce_products() -> Self {
        Self {
            name: "E-commerce Products".to_string(),
            description: "Sample e-commerce product catalog".to_string(),
            index_name: "sample_products".to_string(),
            mappings: serde_json::json!({
                "properties": {
                    "product_id": { "type": "keyword" },
                    "name": { "type": "text" },
                    "category": { "type": "keyword" },
                    "price": { "type": "float" },
                    "stock": { "type": "integer" },
                    "description": { "type": "text" },
                    "tags": { "type": "keyword" },
                    "created_at": { "type": "date" },
                    "updated_at": { "type": "date" }
                }
            }),
            settings: serde_json::json!({
                "number_of_shards": 1,
                "number_of_replicas": 0
            }),
        }
    }

    /// Application logs sample index
    pub fn application_logs() -> Self {
        Self {
            name: "Application Logs".to_string(),
            description: "Sample application log entries".to_string(),
            index_name: "sample_logs".to_string(),
            mappings: serde_json::json!({
                "properties": {
                    "timestamp": { "type": "date" },
                    "level": { "type": "keyword" },
                    "message": { "type": "text" },
                    "service": { "type": "keyword" },
                    "host": { "type": "keyword" },
                    "user_id": { "type": "keyword" },
                    "request_id": { "type": "keyword" },
                    "duration_ms": { "type": "integer" }
                }
            }),
            settings: serde_json::json!({
                "number_of_shards": 1,
                "number_of_replicas": 0
            }),
        }
    }

    /// User analytics sample index
    pub fn user_analytics() -> Self {
        Self {
            name: "User Analytics".to_string(),
            description: "Sample user behavior analytics".to_string(),
            index_name: "sample_analytics".to_string(),
            mappings: serde_json::json!({
                "properties": {
                    "user_id": { "type": "keyword" },
                    "session_id": { "type": "keyword" },
                    "event_type": { "type": "keyword" },
                    "page_url": { "type": "keyword" },
                    "referrer": { "type": "keyword" },
                    "device": { "type": "keyword" },
                    "browser": { "type": "keyword" },
                    "country": { "type": "keyword" },
                    "city": { "type": "keyword" },
                    "timestamp": { "type": "date" }
                }
            }),
            settings: serde_json::json!({
                "number_of_shards": 1,
                "number_of_replicas": 0
            }),
        }
    }

    /// Get all available sample index configs
    pub fn all() -> Vec<Self> {
        vec![
            Self::ecommerce_products(),
            Self::application_logs(),
            Self::user_analytics(),
        ]
    }
}

/// Data extraction job record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionJob {
    pub id: i64,
    pub profile_name: String,
    pub index_name: String,
    pub query: String,
    pub table_name: String,
    pub record_count: i64,
    pub created_at: String,
    pub status: String,
}

impl ExtractionJob {
    pub fn new(
        profile_name: String,
        index_name: String,
        query: String,
        table_name: String,
    ) -> Self {
        Self {
            id: 0,
            profile_name,
            index_name,
            query,
            table_name,
            record_count: 0,
            created_at: chrono::Utc::now().to_rfc3339(),
            status: "pending".to_string(),
        }
    }
}
