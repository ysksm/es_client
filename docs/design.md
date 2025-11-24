# Elasticsearch Index管理アプリケーション 設計書

## 1. システムアーキテクチャ

### 1.1 全体構成

```
┌─────────────────────────────────────────────────────────┐
│                  ユーザーインターフェース                    │
├──────────────────────┬──────────────────────────────────┤
│   GUI (React-TS)     │      CLI (Rust)                  │
│   - Tauri Frontend   │      - clap/clap-derive         │
└──────────┬───────────┴──────────────┬───────────────────┘
           │                          │
           │    Tauri IPC Command     │
           ▼                          ▼
┌─────────────────────────────────────────────────────────┐
│            Tauri Backend (Rust Core)                    │
├─────────────────────────────────────────────────────────┤
│  - Command Handlers                                     │
│  - Business Logic Layer                                 │
│  - Error Handling                                       │
└───────┬─────────────────────────────────┬───────────────┘
        │                                 │
        ▼                                 ▼
┌──────────────────┐            ┌──────────────────┐
│  Elasticsearch   │            │     DuckDB       │
│   Client         │            │     Client       │
│  (elasticsearch) │            │   (duckdb-rs)    │
└──────────────────┘            └──────────────────┘
        │                                 │
        ▼                                 ▼
┌──────────────────┐            ┌──────────────────┐
│  Elasticsearch   │            │  Local Database  │
│    Cluster       │            │   (*.duckdb)     │
└──────────────────┘            └──────────────────┘
```

### 1.2 Tauriアーキテクチャ

**フロントエンド（src-ui/）**
- React + TypeScript
- UIコンポーネント（shadcn/ui等を想定）
- 状態管理（Zustand または React Context）
- Tauri APIによるバックエンド呼び出し

**バックエンド（src-tauri/）**
- Rustによるビジネスロジック
- Tauri Commandハンドラ
- Elasticsearch操作
- DuckDB操作
- 設定管理

**通信方式**
- Tauri IPC（Inter-Process Communication）
- JSON-RPC形式でのコマンド/レスポンス

### 1.3 ディレクトリ構成

```
es_client/
├── src-ui/                  # フロントエンド
│   ├── src/
│   │   ├── components/      # Reactコンポーネント
│   │   ├── pages/           # 画面コンポーネント
│   │   ├── stores/          # 状態管理
│   │   ├── api/             # Tauri API呼び出し
│   │   └── types/           # TypeScript型定義
│   ├── public/
│   └── package.json
├── src-tauri/               # バックエンド
│   ├── src/
│   │   ├── main.rs          # エントリーポイント
│   │   ├── commands/        # Tauriコマンド
│   │   ├── services/        # ビジネスロジック
│   │   │   ├── es_client.rs # Elasticsearch操作
│   │   │   ├── duckdb.rs    # DuckDB操作
│   │   │   └── config.rs    # 設定管理
│   │   ├── models/          # データ構造
│   │   └── utils/           # ユーティリティ
│   ├── Cargo.toml
│   └── tauri.conf.json
├── cli/                     # CLI実装
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── docs/                    # ドキュメント
└── README.md
```

## 2. データモデル

### 2.1 DuckDBテーブル設計

DuckDBは抽出したデータと履歴の保存のみに使用します。接続プロファイルは設定ファイルで管理します。

#### 2.1.1 メタデータテーブル

**extraction_history** - 抽出履歴
```sql
CREATE TABLE extraction_history (
    id INTEGER PRIMARY KEY,
    profile_name VARCHAR,
    index_name VARCHAR NOT NULL,
    query_json TEXT,  -- 実行したクエリ
    target_table VARCHAR NOT NULL,  -- 保存先テーブル名
    records_count INTEGER,
    executed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR CHECK (status IN ('success', 'failed')),
    error_message TEXT
);
```

#### 2.1.2 抽出データテーブル

抽出したElasticsearchのデータは動的にテーブルを作成:
```sql
-- 例: logs_2024 というテーブルに保存
CREATE TABLE logs_2024 (
    _id VARCHAR,
    _index VARCHAR,
    _score DOUBLE,
    timestamp TIMESTAMP,
    message TEXT,
    level VARCHAR,
    -- その他のフィールドは動的に追加
);
```

### 2.2 設定ファイル形式

#### 2.2.1 アプリケーション設定 (config.toml)

保存場所: `~/.es_client/config.toml` (Linux/macOS), `%APPDATA%\es_client\config.toml` (Windows)

```toml
[app]
default_profile = "local"
log_level = "info"

[duckdb]
database_path = "~/.es_client/data.duckdb"
max_memory = "2GB"

[ui]
theme = "dark"
recent_profiles = ["local", "production"]
```

#### 2.2.2 接続プロファイル (profiles.toml)

保存場所: `~/.es_client/profiles.toml` (Linux/macOS), `%APPDATA%\es_client\profiles.toml` (Windows)

```toml
[[profiles]]
name = "local"
host = "https://localhost:9200"
username = "elastic"
password_encrypted = "a1b2c3d4e5f6..."  # HEX文字列として保存された暗号化パスワード
auth_type = "basic"  # "basic" | "api_key"
use_ssl = true
verify_certificate = false
created_at = "2024-01-01T00:00:00Z"
updated_at = "2024-01-01T00:00:00Z"

[[profiles]]
name = "production"
host = "https://es-prod.example.com:9200"
api_key_encrypted = "f6e5d4c3b2a1..."  # APIキーの場合
auth_type = "api_key"
use_ssl = true
verify_certificate = true
created_at = "2024-01-01T00:00:00Z"
updated_at = "2024-01-01T00:00:00Z"
```

**注意事項:**
- `password_encrypted` と `api_key_encrypted` は暗号化されたバイナリデータをHEX文字列に変換して保存
- ファイルパーミッションを600に設定して、所有者のみ読み書き可能にする
- プロファイルの追加・編集はアプリケーション経由で行い、ファイルの直接編集は推奨しない

### 2.3 Elasticsearch連携データ構造

#### Rustでのデータモデル例

```rust
// models/es_document.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct ESDocument {
    pub _id: String,
    pub _index: String,
    pub _score: Option<f64>,
    pub _source: serde_json::Value,  // 動的なフィールド
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub index: String,
    pub query: serde_json::Value,
    pub size: Option<u32>,
    pub from: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub total: u64,
    pub documents: Vec<ESDocument>,
}
```

## 3. UI/UX設計

### 3.1 CLI設計

#### コマンド体系

```bash
es-client <COMMAND> [OPTIONS]

Commands:
  connect         Elasticsearch接続を設定
  profile         プロファイル管理
  index           インデックス操作
  extract         データ抽出
  db              ローカルDB操作
  help            ヘルプ表示
```

#### 詳細コマンド仕様

**connect** - 接続設定
```bash
es-client connect \
  --host <URL> \
  --user <USERNAME> \
  --password <PASSWORD> \
  --profile <PROFILE_NAME>

# 例
es-client connect \
  --host https://localhost:9200 \
  --user elastic \
  --password changeme \
  --profile local
```

**profile** - プロファイル管理
```bash
# プロファイル一覧
es-client profile list

# プロファイル切り替え
es-client profile use <PROFILE_NAME>

# プロファイル削除
es-client profile delete <PROFILE_NAME>
```

**index** - インデックス操作
```bash
# インデックス作成
es-client index create \
  --name <INDEX_NAME> \
  --template <TEMPLATE_NAME>

# インデックス一覧
es-client index list

# サンプルデータ投入
es-client index load-sample \
  --name <INDEX_NAME> \
  --file <JSON_FILE>
```

**extract** - データ抽出
```bash
# クエリファイル指定
es-client extract \
  --index <INDEX_NAME> \
  --query <QUERY_FILE.json> \
  --output <TABLE_NAME>

# シンプルクエリ
es-client extract \
  --index logs-* \
  --field level \
  --value ERROR \
  --output error_logs

# 時間範囲指定
es-client extract \
  --index logs-* \
  --from "2024-01-01" \
  --to "2024-01-31" \
  --output january_logs
```

**db** - ローカルDB操作
```bash
# テーブル一覧
es-client db list

# テーブルプレビュー
es-client db show <TABLE_NAME> --limit 10

# SQLクエリ実行
es-client db query --sql "SELECT * FROM logs WHERE level='ERROR'"

# テーブル削除
es-client db drop <TABLE_NAME>
```

### 3.2 GUI設計

#### 画面一覧

1. **ダッシュボード** (`/`)
   - 接続状態の表示
   - 最近使用したプロファイル
   - クイックアクション

2. **接続設定** (`/connections`)
   - プロファイル一覧
   - 新規プロファイル作成
   - プロファイル編集・削除
   - 接続テスト

3. **インデックス管理** (`/indices`)
   - インデックス一覧（検索可能）
   - インデックス詳細情報
   - 新規インデックス作成
   - サンプルデータ投入

4. **データ抽出** (`/extract`)
   - 検索条件入力（クエリビルダー）
   - クエリJSON編集
   - 検索結果プレビュー
   - DuckDB保存設定

5. **ローカルDB管理** (`/database`)
   - テーブル一覧
   - テーブルデータ表示
   - SQL実行画面
   - 抽出履歴

#### レイアウト構成

```
┌─────────────────────────────────────────────────────────┐
│  Header: タイトル | 接続状態 | プロファイル選択          │
├──────────┬──────────────────────────────────────────────┤
│          │                                              │
│  Sidebar │         Main Content Area                    │
│          │                                              │
│  - Home  │  各ページのコンテンツ                          │
│  - Conn  │                                              │
│  - Index │                                              │
│  - Extrt │                                              │
│  - DB    │                                              │
│          │                                              │
│          │                                              │
└──────────┴──────────────────────────────────────────────┘
```

## 4. 主要コンポーネント設計

### 4.1 Elasticsearch接続管理

#### ESClientService (Rust)

```rust
pub struct ESClient {
    url: String,
    credentials: Credentials,
    client: elasticsearch::Elasticsearch,
}

impl ESClient {
    pub async fn new(config: ESConfig) -> Result<Self>;
    pub async fn test_connection(&self) -> Result<ClusterInfo>;
    pub async fn list_indices(&self) -> Result<Vec<IndexInfo>>;
    pub async fn create_index(&self, name: &str, mapping: &str) -> Result<()>;
    pub async fn search(&self, req: SearchRequest) -> Result<SearchResponse>;
}
```

### 4.2 DuckDB操作

#### DuckDBService (Rust)

```rust
pub struct DuckDBService {
    conn: Connection,
}

impl DuckDBService {
    pub fn new(db_path: &str) -> Result<Self>;
    pub fn create_history_table(&self) -> Result<()>;  // 履歴テーブルの作成
    pub fn save_documents(&self, table_name: &str, docs: Vec<ESDocument>) -> Result<usize>;
    pub fn list_tables(&self) -> Result<Vec<String>>;
    pub fn query(&self, sql: &str) -> Result<Vec<Row>>;
    pub fn drop_table(&self, table_name: &str) -> Result<()>;
    pub fn save_extraction_history(&self, history: &ExtractionHistory) -> Result<()>;  // 履歴の保存
}
```

### 4.2.5 設定管理

#### ConfigService (Rust)

```rust
pub struct ConfigService {
    config_dir: PathBuf,
    encryptor: Encryptor,
}

impl ConfigService {
    pub fn new() -> Result<Self>;  // 設定ディレクトリの初期化

    // プロファイル管理
    pub fn load_profiles(&self) -> Result<Vec<ProfileConfig>>;
    pub fn save_profile(&self, profile: &ProfileConfig) -> Result<()>;
    pub fn delete_profile(&self, name: &str) -> Result<()>;
    pub fn get_profile(&self, name: &str) -> Result<ProfileConfig>;

    // アプリケーション設定
    pub fn load_app_config(&self) -> Result<AppConfig>;
    pub fn save_app_config(&self, config: &AppConfig) -> Result<()>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub name: String,
    pub host: String,
    pub username: Option<String>,
    pub password_encrypted: Option<String>,  // HEX文字列
    pub api_key_encrypted: Option<String>,   // HEX文字列
    pub auth_type: AuthType,
    pub use_ssl: bool,
    pub verify_certificate: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthType {
    Basic,
    ApiKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub default_profile: Option<String>,
    pub log_level: String,
    pub database_path: String,
    pub max_memory: String,
    pub theme: String,
    pub recent_profiles: Vec<String>,
}
```

### 4.3 データ変換エンジン

#### ESDocument → DuckDB変換

```rust
pub struct DocumentConverter;

impl DocumentConverter {
    // ESドキュメントからDuckDBスキーマを推測
    pub fn infer_schema(docs: &[ESDocument]) -> Result<TableSchema>;

    // ESドキュメントをDuckDBに挿入可能な形式に変換
    pub fn convert_to_rows(docs: &[ESDocument], schema: &TableSchema) -> Result<Vec<Row>>;
}
```

### 4.4 認証情報暗号化

#### 暗号化ユーティリティ

```rust
use ring::aead;

pub struct Encryptor {
    key: aead::LessSafeKey,
}

impl Encryptor {
    pub fn new() -> Result<Self>;  // マシン固有のキー生成
    pub fn encrypt(&self, plaintext: &str) -> Result<Vec<u8>>;
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<String>;
}
```

## 5. Tauriコマンド定義

### 5.1 接続関連

```rust
#[tauri::command]
async fn test_connection(profile: ESConfig) -> Result<ClusterInfo, String>;

#[tauri::command]
async fn save_profile(profile: ESConfig) -> Result<(), String>;

#[tauri::command]
async fn list_profiles() -> Result<Vec<ProfileInfo>, String>;
```

### 5.2 インデックス関連

```rust
#[tauri::command]
async fn list_indices(profile: String) -> Result<Vec<IndexInfo>, String>;

#[tauri::command]
async fn create_index(profile: String, name: String, mapping: String) -> Result<(), String>;
```

### 5.3 データ抽出関連

```rust
#[tauri::command]
async fn search_documents(
    profile: String,
    index: String,
    query: String,
    size: Option<u32>
) -> Result<SearchResponse, String>;

#[tauri::command]
async fn save_to_duckdb(
    table_name: String,
    documents: Vec<ESDocument>
) -> Result<usize, String>;
```

### 5.4 DuckDB関連

```rust
#[tauri::command]
async fn list_local_tables() -> Result<Vec<String>, String>;

#[tauri::command]
async fn query_local_db(sql: String) -> Result<QueryResult, String>;

#[tauri::command]
async fn drop_local_table(table_name: String) -> Result<(), String>;
```

## 6. エラーハンドリング

### 6.1 エラー型定義

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Elasticsearch connection failed: {0}")]
    ESConnectionError(String),

    #[error("DuckDB operation failed: {0}")]
    DuckDBError(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("Encryption/Decryption failed: {0}")]
    CryptoError(String),

    #[error("Query parsing failed: {0}")]
    QueryError(String),
}
```

### 6.2 エラー伝搬

- RustのResult型を統一的に使用
- Tauriコマンドでは `Result<T, String>` に変換
- フロントエンドでエラーメッセージを表示

## 7. セキュリティ設計

### 7.1 認証情報の保護

- **保存時**: AES-GCM暗号化
- **メモリ上**: 使用後は即座にクリア
- **設定ファイル**: パーミッション制限（600）

### 7.2 通信セキュリティ

- HTTPS通信を推奨（設定可能）
- SSL証明書検証（本番環境では必須）
- タイムアウト設定

### 7.3 入力検証

- SQLインジェクション対策（プレースホルダ使用）
- パス トラバーサル対策
- JSONスキーマ検証

## 8. パフォーマンス最適化

### 8.1 大量データ処理

- ストリーミング処理（scroll API使用）
- バッチ挿入（DuckDBのAPPENDER使用）
- メモリ使用量の監視

### 8.2 非同期処理

- Tokioランタイムの活用
- 並行処理（複数インデックスからの抽出）
- プログレスバー表示

## 9. テスト戦略

### 9.1 ユニットテスト
- 各サービスモジュールのテスト
- データ変換ロジックのテスト

### 9.2 統合テスト
- Testcontainersを使用したElasticsearchテスト
- DuckDB操作のテスト

### 9.3 E2Eテスト
- Tauriアプリケーションの自動テスト
- CLI コマンドのテスト
