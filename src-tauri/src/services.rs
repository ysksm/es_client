// Services module
// This module contains business logic services

use crate::models::{AppConfig, AuthType, ClusterInfo, ProfileConfig};
use crate::utils::Encryptor;
use base64::{Engine as _, engine::general_purpose};
use reqwest::{self, header, Client};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

/// Configuration service for managing profiles and app settings
pub struct ConfigService {
    config_dir: PathBuf,
    encryptor: Encryptor,
}

impl ConfigService {
    /// Create a new ConfigService
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;

            // Set directory permissions to 700 (owner only) on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&config_dir)?.permissions();
                perms.set_mode(0o700);
                fs::set_permissions(&config_dir, perms)?;
            }
        }

        let encryptor = Encryptor::new()?;

        Ok(Self {
            config_dir,
            encryptor,
        })
    }

    /// Get the configuration directory path
    fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let home_dir = dirs::home_dir()
            .ok_or("Failed to get home directory")?;
        Ok(home_dir.join(".es_client"))
    }

    /// Get path to profiles.toml
    fn get_profiles_path(&self) -> PathBuf {
        self.config_dir.join("profiles.toml")
    }

    /// Get path to config.toml
    fn get_config_path(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    // === Profile Management ===

    /// Load all profiles from profiles.toml
    pub fn load_profiles(&self) -> Result<Vec<ProfileConfig>, Box<dyn std::error::Error>> {
        let path = self.get_profiles_path();

        if !path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&path)?;

        #[derive(serde::Deserialize)]
        struct ProfilesFile {
            profiles: Vec<ProfileConfig>,
        }

        let profiles_file: ProfilesFile = toml::from_str(&content)?;
        Ok(profiles_file.profiles)
    }

    /// Get a specific profile by name
    pub fn get_profile(&self, name: &str) -> Result<ProfileConfig, Box<dyn std::error::Error>> {
        let profiles = self.load_profiles()?;
        profiles
            .into_iter()
            .find(|p| p.name == name)
            .ok_or_else(|| format!("Profile '{}' not found", name).into())
    }

    /// Save a profile (create or update)
    pub fn save_profile(&self, profile: &ProfileConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut profiles = self.load_profiles()?;

        // Remove existing profile with the same name
        profiles.retain(|p| p.name != profile.name);

        // Add the new/updated profile
        profiles.push(profile.clone());

        self.write_profiles(&profiles)?;
        Ok(())
    }

    /// Delete a profile by name
    pub fn delete_profile(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut profiles = self.load_profiles()?;
        let original_len = profiles.len();

        profiles.retain(|p| p.name != name);

        if profiles.len() == original_len {
            return Err(format!("Profile '{}' not found", name).into());
        }

        self.write_profiles(&profiles)?;
        Ok(())
    }

    /// Write profiles to profiles.toml
    fn write_profiles(&self, profiles: &[ProfileConfig]) -> Result<(), Box<dyn std::error::Error>> {
        #[derive(serde::Serialize)]
        struct ProfilesFile {
            profiles: Vec<ProfileConfig>,
        }

        let profiles_file = ProfilesFile {
            profiles: profiles.to_vec(),
        };

        let content = toml::to_string_pretty(&profiles_file)?;
        let path = self.get_profiles_path();

        fs::write(&path, content)?;

        // Set file permissions to 600 (owner read/write only) on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&path, perms)?;
        }

        Ok(())
    }

    // === Encryption Helpers ===

    /// Encrypt a password for storage
    pub fn encrypt_password(&self, password: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.encryptor.encrypt(password)
    }

    /// Decrypt a password from storage
    pub fn decrypt_password(&self, encrypted: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.encryptor.decrypt(encrypted)
    }

    // === App Configuration ===

    /// Load application configuration
    pub fn load_app_config(&self) -> Result<AppConfig, Box<dyn std::error::Error>> {
        let path = self.get_config_path();

        if !path.exists() {
            // Return default config if file doesn't exist
            return Ok(AppConfig::default());
        }

        let content = fs::read_to_string(&path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save application configuration
    pub fn save_app_config(&self, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(config)?;
        let path = self.get_config_path();

        fs::write(&path, content)?;
        Ok(())
    }
}

/// Elasticsearch client for connecting and managing indices
pub struct ESClient {
    client: Client,
    base_url: String,
}

impl ESClient {
    /// Create a new ESClient from a profile
    pub fn new(profile: &ProfileConfig, encryptor: &Encryptor) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        // Setup authentication
        match profile.auth_type {
            AuthType::Basic => {
                if let (Some(username), Some(password_encrypted)) = (&profile.username, &profile.password_encrypted) {
                    let password = encryptor.decrypt(password_encrypted)?;
                    let auth_value = format!("{}:{}", username, password);
                    let encoded = general_purpose::STANDARD.encode(auth_value.as_bytes());
                    headers.insert(
                        header::AUTHORIZATION,
                        header::HeaderValue::from_str(&format!("Basic {}", encoded))?,
                    );
                } else {
                    return Err("Username and password required for Basic auth".into());
                }
            }
            AuthType::ApiKey => {
                if let Some(api_key_encrypted) = &profile.api_key_encrypted {
                    let api_key = encryptor.decrypt(api_key_encrypted)?;
                    headers.insert(
                        header::AUTHORIZATION,
                        header::HeaderValue::from_str(&format!("ApiKey {}", api_key))?,
                    );
                } else {
                    return Err("API key required for ApiKey auth".into());
                }
            }
        }

        // Build HTTP client
        let mut client_builder = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30));

        // Handle SSL settings
        if profile.use_ssl && !profile.verify_certificate {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }

        let client = client_builder.build()?;
        let base_url = profile.host.trim_end_matches('/').to_string();

        Ok(Self { client, base_url })
    }

    /// Test connection to Elasticsearch
    pub async fn test_connection(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let url = format!("{}/", self.base_url);
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }

    /// Get cluster information
    pub async fn get_cluster_info(&self) -> Result<ClusterInfo, Box<dyn std::error::Error>> {
        let url = format!("{}/", self.base_url);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(format!("Failed to get cluster info: {}", response.status()).into());
        }

        let info: ClusterInfo = response.json().await?;
        Ok(info)
    }

    /// List all indices
    pub async fn list_indices(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!("{}/_cat/indices?format=json", self.base_url);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(format!("Failed to list indices: {}", response.status()).into());
        }

        #[derive(serde::Deserialize)]
        struct IndexInfo {
            index: String,
        }

        let indices: Vec<IndexInfo> = response.json().await?;
        Ok(indices.into_iter().map(|i| i.index).collect())
    }

    /// Create an index with optional settings and mappings
    pub async fn create_index(
        &self,
        index_name: &str,
        settings: Option<serde_json::Value>,
        mappings: Option<serde_json::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/{}", self.base_url, index_name);

        let mut body = serde_json::json!({});
        if let Some(s) = settings {
            body["settings"] = s;
        }
        if let Some(m) = mappings {
            body["mappings"] = m;
        }

        let response = self.client.put(&url).json(&body).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Failed to create index: {}", error_text).into());
        }

        Ok(())
    }

    /// Delete an index
    pub async fn delete_index(&self, index_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/{}", self.base_url, index_name);
        let response = self.client.delete(&url).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Failed to delete index: {}", error_text).into());
        }

        Ok(())
    }

    /// Check if an index exists
    pub async fn index_exists(&self, index_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let url = format!("{}/{}", self.base_url, index_name);
        let response = self.client.head(&url).send().await?;
        Ok(response.status().is_success())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_service_creation() {
        let service = ConfigService::new();
        assert!(service.is_ok());
    }

    #[test]
    fn test_encrypt_decrypt() {
        let service = ConfigService::new().unwrap();
        let password = "my_secret_password";

        let encrypted = service.encrypt_password(password).unwrap();
        assert_ne!(encrypted, password);

        let decrypted = service.decrypt_password(&encrypted).unwrap();
        assert_eq!(decrypted, password);
    }

    #[test]
    fn test_profile_crud() {
        let service = ConfigService::new().unwrap();

        // Create a test profile
        let mut profile = ProfileConfig::new(
            "test_profile".to_string(),
            "https://localhost:9200".to_string(),
            AuthType::Basic,
        );
        profile.username = Some("elastic".to_string());

        // Save profile
        service.save_profile(&profile).unwrap();

        // Load profile
        let loaded = service.get_profile("test_profile").unwrap();
        assert_eq!(loaded.name, "test_profile");
        assert_eq!(loaded.host, "https://localhost:9200");

        // Delete profile
        service.delete_profile("test_profile").unwrap();

        // Verify deletion
        let result = service.get_profile("test_profile");
        assert!(result.is_err());
    }
}
