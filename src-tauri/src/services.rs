// Services module
// This module contains business logic services

use crate::models::{AppConfig, AuthType, ClusterInfo, ExtractionJob, ProfileConfig};
use crate::utils::Encryptor;
use base64::{Engine as _, engine::general_purpose};
use duckdb::{Connection, params};
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
        let status = response.status();
        if status.is_success() {
            Ok(true)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("HTTP {}: {}", status.as_u16(), error_text).into())
        }
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

    /// Bulk insert documents into an index
    pub async fn bulk_insert(
        &self,
        index_name: &str,
        documents: Vec<serde_json::Value>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let url = format!("{}/_bulk", self.base_url);

        // Build NDJSON bulk request body
        let mut bulk_body = String::new();
        for doc in documents {
            // Action line
            let action = serde_json::json!({
                "index": {
                    "_index": index_name
                }
            });
            bulk_body.push_str(&serde_json::to_string(&action)?);
            bulk_body.push('\n');

            // Document line
            bulk_body.push_str(&serde_json::to_string(&doc)?);
            bulk_body.push('\n');
        }

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/x-ndjson")
            .body(bulk_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Bulk insert failed: {}", error_text).into());
        }

        let result: serde_json::Value = response.json().await?;

        // Count successful inserts
        let items = result["items"].as_array()
            .ok_or("Invalid bulk response")?;

        let success_count = items.iter()
            .filter(|item| {
                item["index"]["status"].as_u64()
                    .map(|status| status >= 200 && status < 300)
                    .unwrap_or(false)
            })
            .count();

        Ok(success_count)
    }

    /// Search documents in an index
    pub async fn search(
        &self,
        index_name: &str,
        query: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}{}/_search", self.base_url,
            if index_name.is_empty() { "".to_string() } else { format!("/{}", index_name) });

        let response = self.client
            .post(&url)
            .json(&query)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Search failed: {}", error_text).into());
        }

        let result: serde_json::Value = response.json().await?;
        Ok(result)
    }

    /// Get document count in an index
    pub async fn count(&self, index_name: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let url = format!("{}{}/_count", self.base_url,
            if index_name.is_empty() { "".to_string() } else { format!("/{}", index_name) });

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Count failed: {}", error_text).into());
        }

        let result: serde_json::Value = response.json().await?;
        let count = result["count"].as_u64()
            .ok_or("Invalid count response")?;

        Ok(count)
    }
}

/// DuckDB service for local data storage
pub struct DuckDBService {
    db_path: PathBuf,
}

impl DuckDBService {
    /// Create a new DuckDBService
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let home_dir = dirs::home_dir()
            .ok_or("Failed to get home directory")?;
        let db_path = home_dir.join(".es_client").join("data.duckdb");

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let service = Self { db_path };

        // Initialize database tables
        service.init_tables()?;

        Ok(service)
    }

    /// Get a connection to the database
    fn get_connection(&self) -> Result<Connection, Box<dyn std::error::Error>> {
        let conn = Connection::open(&self.db_path)?;
        Ok(conn)
    }

    /// Initialize database tables
    fn init_tables(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.get_connection()?;

        // Check if table exists
        let table_exists: bool = {
            let mut stmt = conn.prepare(
                "SELECT COUNT(*) FROM information_schema.tables
                 WHERE table_schema = 'main' AND table_name = 'extraction_history'"
            )?;
            let count: i64 = stmt.query_row([], |row| row.get(0))?;
            count > 0
        };

        if !table_exists {
            // Create sequence for auto-increment
            conn.execute("CREATE SEQUENCE IF NOT EXISTS extraction_history_seq START 1", [])?;

            // Create extraction_history table with auto-increment
            conn.execute(
                "CREATE TABLE extraction_history (
                    id INTEGER PRIMARY KEY DEFAULT nextval('extraction_history_seq'),
                    profile_name TEXT NOT NULL,
                    index_name TEXT NOT NULL,
                    query TEXT NOT NULL,
                    table_name TEXT NOT NULL,
                    record_count INTEGER NOT NULL,
                    created_at TEXT NOT NULL,
                    status TEXT NOT NULL
                )",
                [],
            )?;
        }

        Ok(())
    }

    /// Save extraction job to history
    pub fn save_extraction_job(&self, job: &ExtractionJob) -> Result<i64, Box<dyn std::error::Error>> {
        let conn = self.get_connection()?;

        // Use RETURNING to get the inserted ID
        let mut stmt = conn.prepare(
            "INSERT INTO extraction_history (profile_name, index_name, query, table_name, record_count, created_at, status)
             VALUES (?, ?, ?, ?, ?, ?, ?)
             RETURNING id"
        )?;

        let id: i64 = stmt.query_row(
            params![
                &job.profile_name,
                &job.index_name,
                &job.query,
                &job.table_name,
                &job.record_count,
                &job.created_at,
                &job.status,
            ],
            |row| row.get(0),
        )?;

        Ok(id)
    }

    /// Get extraction job history
    pub fn get_extraction_history(&self) -> Result<Vec<ExtractionJob>, Box<dyn std::error::Error>> {
        let conn = self.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, profile_name, index_name, query, table_name, record_count, created_at, status
             FROM extraction_history
             ORDER BY created_at DESC
             LIMIT 100"
        )?;

        let jobs = stmt.query_map([], |row| {
            Ok(ExtractionJob {
                id: row.get(0)?,
                profile_name: row.get(1)?,
                index_name: row.get(2)?,
                query: row.get(3)?,
                table_name: row.get(4)?,
                record_count: row.get(5)?,
                created_at: row.get(6)?,
                status: row.get(7)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(jobs)
    }

    /// Sanitize column name for DuckDB
    fn sanitize_column_name(name: &str) -> String {
        // Replace special characters with underscore
        let sanitized: String = name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();

        // Ensure it doesn't start with a number
        if sanitized.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
            format!("col_{}", sanitized)
        } else if sanitized.is_empty() {
            "col_unnamed".to_string()
        } else {
            sanitized
        }
    }

    /// Create a table for extracted data
    pub fn create_data_table(
        &self,
        table_name: &str,
        documents: &[serde_json::Value],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if documents.is_empty() {
            return Err("No documents to create table schema".into());
        }

        let conn = self.get_connection()?;

        // Sanitize table name
        let safe_table_name = Self::sanitize_column_name(table_name);

        // Drop table if exists
        conn.execute(&format!("DROP TABLE IF EXISTS \"{}\"", safe_table_name), [])?;

        // Analyze first document to determine schema
        let first_doc = &documents[0];
        let mut columns = Vec::new();

        if let Some(obj) = first_doc.as_object() {
            for (key, value) in obj {
                let safe_col_name = Self::sanitize_column_name(key);
                let col_type = match value {
                    serde_json::Value::Number(_) => {
                        if value.is_f64() {
                            "DOUBLE"
                        } else {
                            "BIGINT"
                        }
                    }
                    serde_json::Value::Bool(_) => "BOOLEAN",
                    serde_json::Value::Array(_) => "TEXT", // Store as JSON string
                    _ => "TEXT",
                };
                columns.push(format!("\"{}\" {}", safe_col_name, col_type));
            }
        }

        let create_sql = format!(
            "CREATE TABLE \"{}\" ({})",
            safe_table_name,
            columns.join(", ")
        );

        conn.execute(&create_sql, [])?;

        Ok(())
    }

    /// Insert extracted data into table
    pub fn insert_data(
        &self,
        table_name: &str,
        documents: Vec<serde_json::Value>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        if documents.is_empty() {
            return Ok(0);
        }

        let conn = self.get_connection()?;

        // Sanitize table name
        let safe_table_name = Self::sanitize_column_name(table_name);

        // Get column names from first document (original -> sanitized mapping)
        let first_doc = &documents[0];
        let column_mapping: Vec<(String, String)> = if let Some(obj) = first_doc.as_object() {
            obj.keys()
                .map(|k| (k.clone(), Self::sanitize_column_name(k)))
                .collect()
        } else {
            return Err("Invalid document format".into());
        };

        let safe_columns: Vec<&str> = column_mapping.iter().map(|(_, s)| s.as_str()).collect();

        let placeholders = (0..column_mapping.len())
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");

        let insert_sql = format!(
            "INSERT INTO \"{}\" ({}) VALUES ({})",
            safe_table_name,
            safe_columns.iter().map(|c| format!("\"{}\"", c)).collect::<Vec<_>>().join(", "),
            placeholders
        );

        let mut inserted = 0;
        for doc in documents {
            if let Some(obj) = doc.as_object() {
                let values: Vec<duckdb::types::Value> = column_mapping
                    .iter()
                    .map(|(original_col, _)| {
                        let val = obj.get(original_col).unwrap_or(&serde_json::Value::Null);
                        match val {
                            serde_json::Value::Number(n) => {
                                if let Some(i) = n.as_i64() {
                                    duckdb::types::Value::BigInt(i)
                                } else if let Some(f) = n.as_f64() {
                                    duckdb::types::Value::Double(f)
                                } else {
                                    duckdb::types::Value::Null
                                }
                            }
                            serde_json::Value::Bool(b) => duckdb::types::Value::Boolean(*b),
                            serde_json::Value::String(s) => duckdb::types::Value::Text(s.clone()),
                            serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                                duckdb::types::Value::Text(val.to_string())
                            }
                            serde_json::Value::Null => duckdb::types::Value::Null,
                        }
                    })
                    .collect();

                conn.execute(&insert_sql, duckdb::params_from_iter(values))?;
                inserted += 1;
            }
        }

        Ok(inserted)
    }

    /// Query data from a table
    pub fn query_table(
        &self,
        table_name: &str,
        limit: Option<usize>,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let conn = self.get_connection()?;

        // Sanitize table name for safety
        let safe_table_name = Self::sanitize_column_name(table_name);

        let sql = if let Some(l) = limit {
            format!("SELECT * FROM \"{}\" LIMIT {}", safe_table_name, l)
        } else {
            format!("SELECT * FROM \"{}\"", safe_table_name)
        };

        let mut stmt = conn.prepare(&sql)?;
        let column_count = stmt.column_count();

        let column_names: Vec<String> = (0..column_count)
            .map(|i| {
                stmt.column_name(i)
                    .map(|name| name.to_string())
                    .unwrap_or_else(|_| "unknown".to_string())
            })
            .collect();

        let rows = stmt.query_map([], |row| {
            let mut obj = serde_json::Map::new();

            for (i, col_name) in column_names.iter().enumerate() {
                let value: Result<duckdb::types::Value, _> = row.get(i);
                if let Ok(val) = value {
                    let json_val = match val {
                        duckdb::types::Value::BigInt(n) => serde_json::Value::Number(n.into()),
                        duckdb::types::Value::Double(f) => {
                            serde_json::Number::from_f64(f)
                                .map(serde_json::Value::Number)
                                .unwrap_or(serde_json::Value::Null)
                        }
                        duckdb::types::Value::Text(s) => serde_json::Value::String(s),
                        duckdb::types::Value::Boolean(b) => serde_json::Value::Bool(b),
                        _ => serde_json::Value::Null,
                    };
                    obj.insert(col_name.clone(), json_val);
                }
            }

            Ok(serde_json::Value::Object(obj))
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(rows)
    }

    /// List all tables in the database
    pub fn list_tables(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let conn = self.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'main'"
        )?;

        let tables = stmt.query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tables)
    }

    /// Execute arbitrary SQL query
    pub fn execute_sql(&self, sql: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let conn = self.get_connection()?;

        // Get column names using DESCRIBE or by wrapping in a subquery
        let column_names: Vec<String> = {
            let describe_sql = format!("DESCRIBE ({})", sql);
            let mut desc_stmt = conn.prepare(&describe_sql)?;
            let names: Vec<String> = desc_stmt
                .query_map([], |row| row.get::<_, String>(0))?
                .filter_map(|r| r.ok())
                .collect();
            if names.is_empty() {
                // Fallback: generate generic column names
                (0..100).map(|i| format!("col_{}", i)).collect()
            } else {
                names
            }
        };

        // Execute the actual query
        let mut stmt = conn.prepare(sql)?;
        let rows: Vec<Vec<(usize, duckdb::types::Value)>> = stmt
            .query_map([], |row| {
                let mut values = Vec::new();
                let mut idx = 0;
                loop {
                    match row.get::<_, duckdb::types::Value>(idx) {
                        Ok(val) => {
                            values.push((idx, val));
                            idx += 1;
                        }
                        Err(_) => break,
                    }
                }
                Ok(values)
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Convert to JSON
        let results: Vec<serde_json::Value> = rows
            .into_iter()
            .map(|row_values| {
                let mut obj = serde_json::Map::new();
                for (i, val) in row_values {
                    let col_name = column_names.get(i).cloned().unwrap_or_else(|| format!("col_{}", i));
                    let json_val = match val {
                        duckdb::types::Value::BigInt(n) => serde_json::Value::Number(n.into()),
                        duckdb::types::Value::Int(n) => serde_json::Value::Number(n.into()),
                        duckdb::types::Value::Double(f) => {
                            serde_json::Number::from_f64(f)
                                .map(serde_json::Value::Number)
                                .unwrap_or(serde_json::Value::Null)
                        }
                        duckdb::types::Value::Text(s) => serde_json::Value::String(s),
                        duckdb::types::Value::Boolean(b) => serde_json::Value::Bool(b),
                        _ => serde_json::Value::Null,
                    };
                    obj.insert(col_name, json_val);
                }
                serde_json::Value::Object(obj)
            })
            .collect();

        Ok(results)
    }

    /// Export query result to Parquet file
    pub fn export_to_parquet(
        &self,
        sql_or_table: &str,
        output_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let conn = self.get_connection()?;

        // Determine the full output path
        let full_path = if output_path.starts_with('/') || output_path.starts_with('~') {
            output_path.to_string()
        } else {
            // Use home directory as default location
            let home = dirs::home_dir().ok_or("Failed to get home directory")?;
            home.join(output_path).to_string_lossy().to_string()
        };

        // Check if input is a SELECT query or a table name
        let export_sql = if sql_or_table.trim().to_uppercase().starts_with("SELECT") {
            // Export query result
            format!("COPY ({}) TO '{}' (FORMAT PARQUET)", sql_or_table, full_path)
        } else {
            // Export table
            let safe_table_name = Self::sanitize_column_name(sql_or_table);
            format!("COPY \"{}\" TO '{}' (FORMAT PARQUET)", safe_table_name, full_path)
        };

        conn.execute(&export_sql, [])?;

        Ok(format!("Exported to {}", full_path))
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
