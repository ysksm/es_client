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
