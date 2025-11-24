use clap::{Parser, Subcommand};
use colored::*;
use es_client_lib::{
    models::{AuthType, ProfileConfig},
    services::{ConfigService, DuckDBService, ESClient},
    utils::Encryptor,
};
use std::process;

#[derive(Parser)]
#[command(name = "es-client")]
#[command(about = "Elasticsearch Index Management CLI", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage connection profiles
    #[command(subcommand)]
    Profile(ProfileCommands),

    /// Connect to Elasticsearch and create/update a profile
    Connect {
        /// Profile name
        #[arg(short, long)]
        name: String,

        /// Elasticsearch host URL
        #[arg(short = 'H', long)]
        host: String,

        /// Username for Basic auth
        #[arg(short, long)]
        user: Option<String>,

        /// Password for Basic auth
        #[arg(short, long)]
        password: Option<String>,

        /// API key for API key auth
        #[arg(short, long)]
        api_key: Option<String>,

        /// Disable SSL verification (not recommended)
        #[arg(long)]
        insecure: bool,
    },

    /// Manage Elasticsearch indices
    #[command(subcommand)]
    Index(IndexCommands),

    /// Extract data from Elasticsearch to DuckDB
    Extract {
        /// Profile name
        #[arg(long)]
        profile: String,

        /// Index name or pattern
        #[arg(short, long)]
        index: String,

        /// Query JSON file path or inline JSON
        #[arg(short, long)]
        query: String,

        /// DuckDB table name for output
        #[arg(short, long)]
        output: String,
    },

    /// Manage local DuckDB database
    #[command(subcommand)]
    Db(DbCommands),
}

#[derive(Subcommand)]
enum ProfileCommands {
    /// List all connection profiles
    List,

    /// Show details of a specific profile
    Show {
        /// Profile name
        name: String,
    },

    /// Delete a connection profile
    Delete {
        /// Profile name
        name: String,
    },
}

#[derive(Subcommand)]
enum IndexCommands {
    /// List all indices
    List {
        /// Profile name
        #[arg(long)]
        profile: String,
    },

    /// Create a new index
    Create {
        /// Profile name
        #[arg(long)]
        profile: String,

        /// Index name
        #[arg(short, long)]
        name: String,

        /// Number of shards
        #[arg(long, default_value = "1")]
        shards: u32,

        /// Number of replicas
        #[arg(long, default_value = "0")]
        replicas: u32,
    },

    /// Show index information
    Info {
        /// Profile name
        #[arg(long)]
        profile: String,

        /// Index name
        #[arg(short, long)]
        name: String,
    },
}

#[derive(Subcommand)]
enum DbCommands {
    /// List all tables in DuckDB
    List,

    /// Execute SQL query
    Query {
        /// SQL query string
        #[arg(short, long)]
        sql: String,
    },

    /// Show table data (preview)
    Show {
        /// Table name
        table: String,

        /// Number of rows to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Drop a table
    Drop {
        /// Table name
        table: String,

        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Profile(cmd) => handle_profile_command(cmd).await?,
        Commands::Connect {
            name,
            host,
            user,
            password,
            api_key,
            insecure,
        } => handle_connect_command(name, host, user, password, api_key, insecure).await?,
        Commands::Index(cmd) => handle_index_command(cmd).await?,
        Commands::Extract {
            profile,
            index,
            query,
            output,
        } => handle_extract_command(profile, index, query, output).await?,
        Commands::Db(cmd) => handle_db_command(cmd).await?,
    }

    Ok(())
}

async fn handle_profile_command(cmd: ProfileCommands) -> Result<(), Box<dyn std::error::Error>> {
    let config_service = ConfigService::new()?;

    match cmd {
        ProfileCommands::List => {
            let profiles = config_service.load_profiles()?;
            if profiles.is_empty() {
                println!("{}", "No profiles found.".yellow());
                println!("Use {} to create a new profile.", "es-client connect".cyan());
            } else {
                println!("{}", "Connection Profiles:".green().bold());
                println!();
                for profile in profiles {
                    println!("  {} {}", "•".cyan(), profile.name.bold());
                    println!("    Host:      {}", profile.host);
                    println!("    Auth:      {:?}", profile.auth_type);
                    println!("    SSL:       {}", if profile.use_ssl { "enabled" } else { "disabled" });
                    println!("    Created:   {}", profile.created_at.format("%Y-%m-%d %H:%M:%S"));
                    println!();
                }
            }
        }
        ProfileCommands::Show { name } => {
            let profile = config_service.get_profile(&name)?;
            println!("{}", format!("Profile: {}", name).green().bold());
            println!();
            println!("  Host:              {}", profile.host);
            println!("  Username:          {}", profile.username.as_deref().unwrap_or("N/A"));
            println!("  Auth Type:         {:?}", profile.auth_type);
            println!("  SSL Enabled:       {}", profile.use_ssl);
            println!("  Verify Certificate: {}", profile.verify_certificate);
            println!("  Created:           {}", profile.created_at.format("%Y-%m-%d %H:%M:%S"));
            println!("  Updated:           {}", profile.updated_at.format("%Y-%m-%d %H:%M:%S"));
        }
        ProfileCommands::Delete { name } => {
            config_service.delete_profile(&name)?;
            println!("{} Profile '{}' deleted successfully.", "✓".green(), name.bold());
        }
    }

    Ok(())
}

async fn handle_connect_command(
    name: String,
    host: String,
    user: Option<String>,
    password: Option<String>,
    api_key: Option<String>,
    insecure: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_service = ConfigService::new()?;
    let encryptor = Encryptor::new()?;

    // Determine auth type
    let auth_type = if api_key.is_some() {
        AuthType::ApiKey
    } else {
        AuthType::Basic
    };

    // Encrypt credentials
    let password_encrypted = if let Some(pwd) = password {
        Some(encryptor.encrypt(&pwd)?)
    } else {
        None
    };

    let api_key_encrypted = if let Some(key) = api_key {
        Some(encryptor.encrypt(&key)?)
    } else {
        None
    };

    // Create profile
    let mut profile = ProfileConfig::new(name.clone(), host, auth_type);
    profile.username = user;
    profile.password_encrypted = password_encrypted;
    profile.api_key_encrypted = api_key_encrypted;
    profile.verify_certificate = !insecure;

    // Test connection
    print!("Testing connection to {}... ", profile.host);
    let client = ESClient::new(&profile, &encryptor)?;
    match client.test_connection().await {
        Ok(true) => {
            println!("{}", "✓ Connected".green());
            let cluster_info = client.get_cluster_info().await?;
            println!("  Cluster: {} ({})", cluster_info.cluster_name.cyan(), cluster_info.version.number);
        }
        Ok(false) => {
            println!("{}", "✗ Connection failed".red());
            return Err("Connection test failed".into());
        }
        Err(e) => {
            println!("{}", "✗ Error".red());
            return Err(e);
        }
    }

    // Save profile
    config_service.save_profile(&profile)?;
    println!("{} Profile '{}' saved successfully.", "✓".green(), name.bold());

    Ok(())
}

async fn handle_index_command(cmd: IndexCommands) -> Result<(), Box<dyn std::error::Error>> {
    let config_service = ConfigService::new()?;
    let encryptor = Encryptor::new()?;

    match cmd {
        IndexCommands::List { profile } => {
            let profile_config = config_service.get_profile(&profile)?;
            let client = ESClient::new(&profile_config, &encryptor)?;

            println!("Fetching indices from {}...", profile_config.host);
            let indices = client.list_indices().await?;

            if indices.is_empty() {
                println!("{}", "No indices found.".yellow());
            } else {
                println!("{}", format!("Indices ({})", indices.len()).green().bold());
                println!();
                for index in indices {
                    println!("  {} {}", "•".cyan(), index);
                }
            }
        }
        IndexCommands::Create { profile, name, shards, replicas } => {
            let profile_config = config_service.get_profile(&profile)?;
            let client = ESClient::new(&profile_config, &encryptor)?;

            let settings = serde_json::json!({
                "number_of_shards": shards,
                "number_of_replicas": replicas
            });

            print!("Creating index '{}'... ", name);
            client.create_index(&name, Some(settings), None).await?;
            println!("{}", "✓ Created".green());
        }
        IndexCommands::Info { profile, name } => {
            let profile_config = config_service.get_profile(&profile)?;
            let client = ESClient::new(&profile_config, &encryptor)?;

            if client.index_exists(&name).await? {
                println!("{}", format!("Index: {}", name).green().bold());
                println!("  Status: {}", "exists".green());

                // Get document count
                let count = client.count(&name).await?;
                println!("  Documents: {}", count);
            } else {
                println!("{} Index '{}' does not exist.", "✗".red(), name);
            }
        }
    }

    Ok(())
}

async fn handle_extract_command(
    profile: String,
    index: String,
    query: String,
    output: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_service = ConfigService::new()?;
    let encryptor = Encryptor::new()?;
    let duckdb_service = DuckDBService::new()?;

    let profile_config = config_service.get_profile(&profile)?;
    let client = ESClient::new(&profile_config, &encryptor)?;

    // Parse query (file or inline JSON)
    let query_json: serde_json::Value = if std::path::Path::new(&query).exists() {
        let content = std::fs::read_to_string(&query)?;
        serde_json::from_str(&content)?
    } else {
        serde_json::from_str(&query)?
    };

    println!("Extracting data from index '{}'...", index);

    // Search documents
    let search_result = client.search(&index, query_json.clone()).await?;
    let hits = search_result["hits"]["hits"].as_array()
        .ok_or("Invalid search response")?;

    if hits.is_empty() {
        println!("{}", "No documents found.".yellow());
        return Ok(());
    }

    println!("Found {} documents", hits.len());

    // Extract _source from each hit
    let documents: Vec<serde_json::Value> = hits
        .iter()
        .filter_map(|hit| hit["_source"].clone().into())
        .collect();

    // Create table and insert data
    print!("Saving to DuckDB table '{}'... ", output);
    duckdb_service.create_data_table(&output, &documents)?;
    let inserted = duckdb_service.insert_data(&output, documents)?;
    println!("{} {} records saved", "✓".green(), inserted);

    // Save extraction history
    let job = es_client_lib::models::ExtractionJob::new(
        profile,
        index,
        serde_json::to_string(&query_json)?,
        output,
    );
    duckdb_service.save_extraction_job(&job)?;

    Ok(())
}

async fn handle_db_command(cmd: DbCommands) -> Result<(), Box<dyn std::error::Error>> {
    let duckdb_service = DuckDBService::new()?;

    match cmd {
        DbCommands::List => {
            let tables = duckdb_service.list_tables()?;
            if tables.is_empty() {
                println!("{}", "No tables found.".yellow());
                println!("Use {} to extract data from Elasticsearch.", "es-client extract".cyan());
            } else {
                println!("{}", format!("DuckDB Tables ({})", tables.len()).green().bold());
                println!();
                for table in tables {
                    println!("  {} {}", "•".cyan(), table);
                }
            }
        }
        DbCommands::Query { sql } => {
            println!("Executing query...");
            let results = duckdb_service.query_table(&sql, None)?;

            if results.is_empty() {
                println!("{}", "No results.".yellow());
            } else {
                println!();
                println!("{}", format!("Results ({})", results.len()).green().bold());
                println!();
                // Print as JSON
                for (i, row) in results.iter().enumerate() {
                    println!("{}:", format!("Row {}", i + 1).cyan());
                    println!("{}", serde_json::to_string_pretty(row)?);
                    if i < results.len() - 1 {
                        println!();
                    }
                }
            }
        }
        DbCommands::Show { table, limit } => {
            let sql = format!("SELECT * FROM {} LIMIT {}", table, limit);
            let results = duckdb_service.query_table(&sql, Some(limit))?;

            if results.is_empty() {
                println!("{}", "No data found.".yellow());
            } else {
                println!("{}", format!("Table: {} (showing {} rows)", table, results.len()).green().bold());
                println!();
                for (i, row) in results.iter().enumerate() {
                    println!("{}:", format!("Row {}", i + 1).cyan());
                    println!("{}", serde_json::to_string_pretty(row)?);
                    if i < results.len() - 1 {
                        println!();
                    }
                }
            }
        }
        DbCommands::Drop { table, yes } => {
            if !yes {
                print!("Are you sure you want to drop table '{}'? [y/N] ", table);
                use std::io::{self, Write};
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Cancelled.");
                    return Ok(());
                }
            }

            let sql = format!("DROP TABLE IF EXISTS {}", table);
            duckdb_service.query_table(&sql, None)?;
            println!("{} Table '{}' dropped successfully.", "✓".green(), table.bold());
        }
    }

    Ok(())
}
