use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose};
use clap::Parser;
use reqwest::Client;
use rust_xlsxwriter::{Workbook, Format};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

/// Simple CLI tool to export Elasticsearch data to JSON and Excel
#[derive(Parser, Debug)]
#[command(name = "es-export")]
#[command(about = "Export Elasticsearch data to JSON and Excel")]
struct Args {
    /// Path to configuration file
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,

    /// Output format: json, excel, or both
    #[arg(short, long, default_value = "both")]
    format: String,

    /// Output file path (without extension)
    #[arg(short, long, default_value = "output")]
    output: String,
}

/// Configuration structure
#[derive(Debug, Deserialize)]
struct Config {
    elasticsearch: ElasticsearchConfig,
    query: QueryConfig,
}

#[derive(Debug, Deserialize)]
struct ElasticsearchConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    #[serde(default)]
    use_ssl: bool,
    #[serde(default = "default_true")]
    verify_certificate: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
struct QueryConfig {
    index: String,
    #[serde(default = "default_query")]
    query: serde_json::Value,
    #[serde(default = "default_size")]
    size: u32,
}

fn default_query() -> serde_json::Value {
    serde_json::json!({"match_all": {}})
}

fn default_size() -> u32 {
    1000
}

/// Elasticsearch client
struct ESClient {
    client: Client,
    base_url: String,
    auth_header: String,
}

impl ESClient {
    fn new(config: &ElasticsearchConfig) -> Result<Self> {
        let scheme = if config.use_ssl { "https" } else { "http" };
        let base_url = format!("{}://{}:{}", scheme, config.host, config.port);

        // Create Basic Auth header
        let auth_value = format!("{}:{}", config.username, config.password);
        let encoded = general_purpose::STANDARD.encode(auth_value.as_bytes());
        let auth_header = format!("Basic {}", encoded);

        // Build HTTP client
        let mut client_builder = Client::builder()
            .timeout(Duration::from_secs(60));

        if config.use_ssl && !config.verify_certificate {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }

        let client = client_builder.build()?;

        Ok(Self {
            client,
            base_url,
            auth_header,
        })
    }

    async fn search(&self, index: &str, query: &serde_json::Value, size: u32) -> Result<Vec<serde_json::Value>> {
        let url = format!("{}/{}/_search", self.base_url, index);

        let search_body = serde_json::json!({
            "query": query,
            "size": size
        });

        let response = self.client
            .post(&url)
            .header("Authorization", &self.auth_header)
            .header("Content-Type", "application/json")
            .json(&search_body)
            .send()
            .await
            .context("Failed to send search request")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Search failed with status {}: {}", status, error_text);
        }

        let result: serde_json::Value = response.json().await
            .context("Failed to parse search response")?;

        let hits = result["hits"]["hits"]
            .as_array()
            .context("Invalid search response format")?;

        let documents: Vec<serde_json::Value> = hits
            .iter()
            .filter_map(|hit| hit["_source"].clone().as_object().map(|obj| serde_json::Value::Object(obj.clone())))
            .collect();

        Ok(documents)
    }

    async fn test_connection(&self) -> Result<()> {
        let response = self.client
            .get(&self.base_url)
            .header("Authorization", &self.auth_header)
            .send()
            .await
            .context("Failed to connect to Elasticsearch")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Connection failed with status {}: {}", status, error_text);
        }

        Ok(())
    }
}

/// Export documents to JSON file
fn export_to_json(documents: &[serde_json::Value], output_path: &str) -> Result<()> {
    let json_path = format!("{}.json", output_path);
    let json_content = serde_json::to_string_pretty(documents)
        .context("Failed to serialize documents to JSON")?;
    fs::write(&json_path, json_content)
        .context("Failed to write JSON file")?;
    println!("Exported {} documents to {}", documents.len(), json_path);
    Ok(())
}

/// Export documents to Excel file
fn export_to_excel(documents: &[serde_json::Value], output_path: &str) -> Result<()> {
    if documents.is_empty() {
        anyhow::bail!("No documents to export");
    }

    let excel_path = format!("{}.xlsx", output_path);
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Get all unique keys from all documents
    let mut all_keys: Vec<String> = Vec::new();
    let mut key_set: std::collections::HashSet<String> = std::collections::HashSet::new();

    for doc in documents {
        if let Some(obj) = doc.as_object() {
            for key in obj.keys() {
                if !key_set.contains(key) {
                    key_set.insert(key.clone());
                    all_keys.push(key.clone());
                }
            }
        }
    }

    // Create header format
    let header_format = Format::new().set_bold();

    // Write headers
    for (col, key) in all_keys.iter().enumerate() {
        worksheet.write_string_with_format(0, col as u16, key, &header_format)?;
    }

    // Write data rows
    for (row_idx, doc) in documents.iter().enumerate() {
        let row = (row_idx + 1) as u32;
        if let Some(obj) = doc.as_object() {
            for (col, key) in all_keys.iter().enumerate() {
                let col = col as u16;
                if let Some(value) = obj.get(key) {
                    match value {
                        serde_json::Value::String(s) => {
                            worksheet.write_string(row, col, s)?;
                        }
                        serde_json::Value::Number(n) => {
                            if let Some(f) = n.as_f64() {
                                worksheet.write_number(row, col, f)?;
                            } else if let Some(i) = n.as_i64() {
                                worksheet.write_number(row, col, i as f64)?;
                            }
                        }
                        serde_json::Value::Bool(b) => {
                            worksheet.write_boolean(row, col, *b)?;
                        }
                        serde_json::Value::Null => {
                            worksheet.write_string(row, col, "")?;
                        }
                        _ => {
                            // For arrays and objects, serialize as JSON string
                            worksheet.write_string(row, col, &value.to_string())?;
                        }
                    }
                }
            }
        }
    }

    workbook.save(&excel_path)?;
    println!("Exported {} documents to {}", documents.len(), excel_path);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Load configuration
    let config_content = fs::read_to_string(&args.config)
        .with_context(|| format!("Failed to read config file: {:?}", args.config))?;
    let config: Config = toml::from_str(&config_content)
        .context("Failed to parse config file")?;

    println!("Connecting to Elasticsearch at {}:{}...",
             config.elasticsearch.host, config.elasticsearch.port);

    // Create client and test connection
    let client = ESClient::new(&config.elasticsearch)?;
    client.test_connection().await?;
    println!("Connection successful!");

    // Execute search
    println!("Searching index '{}' with size {}...",
             config.query.index, config.query.size);
    let documents = client.search(
        &config.query.index,
        &config.query.query,
        config.query.size
    ).await?;

    println!("Found {} documents", documents.len());

    if documents.is_empty() {
        println!("No documents found. Nothing to export.");
        return Ok(());
    }

    // Export based on format
    let format = args.format.to_lowercase();
    match format.as_str() {
        "json" => {
            export_to_json(&documents, &args.output)?;
        }
        "excel" | "xlsx" => {
            export_to_excel(&documents, &args.output)?;
        }
        "both" => {
            export_to_json(&documents, &args.output)?;
            export_to_excel(&documents, &args.output)?;
        }
        _ => {
            anyhow::bail!("Invalid format '{}'. Use 'json', 'excel', or 'both'", format);
        }
    }

    println!("Export completed successfully!");
    Ok(())
}
