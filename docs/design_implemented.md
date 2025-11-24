# ES Client - 設計書（実装版）

**最終更新:** 2025-11-25
**ステータス:** GUI実装完了

## 1. システムアーキテクチャ

### 1.1 全体構成

```
┌─────────────────────────────────────────────────────┐
│              Desktop Application (Tauri)            │
│  ┌────────────────────────────────────────────┐    │
│  │         Frontend (React + TypeScript)      │    │
│  │  ┌──────────┐  ┌────────┐  ┌──────────┐  │    │
│  │  │  Pages   │→ │ Stores │→ │   API    │  │    │
│  │  │Components│  │(Zustand)│  │ Wrapper  │  │    │
│  │  └──────────┘  └────────┘  └─────┬────┘  │    │
│  └─────────────────────────────────│────────┘    │
│                                     │             │
│                              Tauri IPC            │
│                                     ↓             │
│  ┌──────────────────────────────────────────┐    │
│  │    Backend (Rust)                        │    │
│  │  ┌────────────┐  ┌──────────────────┐   │    │
│  │  │  Commands  │→ │    Services      │   │    │
│  │  │  (21個)    │  │ - ESClient       │   │    │
│  │  └────────────┘  │ - DuckDBService  │   │    │
│  │                  │ - ConfigService  │   │    │
│  │                  └──────┬───────────┘   │    │
│  └─────────────────────────│───────────────┘    │
└─────────────────────────────│───────────────────┘
                              │
              ┌───────────────┴────────────────┐
              │                                │
              ↓                                ↓
┌──────────────────────┐          ┌─────────────────────┐
│  Elasticsearch       │          │  Local DuckDB       │
│  (Remote)            │          │  (~/.es_client/     │
│                      │          │   data.duckdb)      │
└──────────────────────┘          └─────────────────────┘
```

### 1.2 データフロー

#### プロファイル管理フロー
```
[UI] Connections Page
    ↓ selectProfile(name)
[Store] profileStore
    ↓ api.getClusterInfo()
[Tauri] get_cluster_info command
    ↓
[Rust] ESClient.get_cluster_info()
    ↓ HTTP Request
[ES] GET /_cluster/health & GET /
    ↓ Response
[Rust] ClusterInfo
    ↓
[Store] Update state (currentProfile, clusterInfo)
    ↓
[UI] Header & Dashboard reflect changes
```

#### データ抽出フロー
```
[UI] Extract Page
    ↓ extractAndStoreData(profile, index, query, table)
[API] tauri.ts wrapper
    ↓
[Tauri] extract_and_store_data command
    ↓
[Rust] ESClient.search(query)
    ↓ HTTP POST /_search
[ES] Return documents
    ↓
[Rust] DuckDBService.create_data_table(schema)
    ↓
[Rust] DuckDBService.insert_data(documents)
    ↓
[DuckDB] Table created & data inserted
    ↓
[Rust] DuckDBService.save_extraction_job(history)
    ↓
[UI] Success toast notification
```

## 2. フロントエンド設計

### 2.1 コンポーネント階層

```
App (React Router)
├── Layout
│   ├── Header
│   │   ├── Title
│   │   ├── ConnectionStatus
│   │   └── ThemeToggle
│   ├── Sidebar
│   │   └── NavLink × 5
│   └── Main (Outlet)
│       ├── Dashboard
│       ├── Connections
│       ├── Indices
│       ├── Extract
│       └── Database
└── Toaster (react-hot-toast)
```

### 2.2 状態管理 (Zustand)

#### appStore
```typescript
{
  theme: 'light' | 'dark',
  config: AppConfig | null,
  isLoading: boolean,
  error: string | null,

  // Actions
  loadConfig(): Promise<void>,
  saveConfig(config): Promise<void>,
  setTheme(theme): void,
  toggleTheme(): void
}
```

#### profileStore
```typescript
{
  profiles: ProfileConfig[],
  currentProfile: ProfileConfig | null,
  clusterInfo: ClusterInfo | null,
  isLoading: boolean,
  error: string | null,

  // Actions
  loadProfiles(): Promise<void>,
  selectProfile(name): Promise<void>,
  createProfile(profile): Promise<void>,
  updateProfile(name, profile): Promise<void>,
  deleteProfile(name): Promise<void>,
  testConnection(name): Promise<boolean>
}
```

#### indexStore
```typescript
{
  indices: string[],
  selectedIndex: string | null,
  documentCount: number | null,
  searchResults: any[],
  isLoading: boolean,
  error: string | null,

  // Actions
  loadIndices(profile): Promise<void>,
  selectIndex(index): void,
  createIndex(profile, name, settings, mappings): Promise<void>,
  deleteIndex(profile, index): Promise<void>,
  searchDocuments(profile, index, query): Promise<void>,
  countDocuments(profile, index, query): Promise<void>
}
```

### 2.3 ページコンポーネント設計

#### Dashboard (/)
- **目的**: 接続状況の概要表示
- **表示内容**:
  - プロファイル数
  - 接続状態（接続中 / 未接続）
  - 現在の接続情報（プロファイル名、ホスト、クラスタ名、バージョン）
- **状態**: profileStore
- **レイアウト**: 3カラムグリッド + 詳細カード

#### Connections (/connections)
- **目的**: プロファイル管理
- **表示内容**:
  - プロファイル一覧（カード形式）
  - 各プロファイルの接続情報（ホスト、認証方式、SSL）
  - 接続ボタン、編集ボタン
- **状態**: profileStore
- **レイアウト**: グリッド（max 3列）
- **インタラクション**:
  - Connect→ selectProfile() → Header更新
  - Edit→ モーダル表示（未実装）

#### Indices (/indices)
- **目的**: インデックス管理
- **表示内容**:
  - インデックス一覧（カード形式）
  - 検索バー（リアルタイムフィルター）
  - ドキュメント数（選択時）
  - 削除ボタン（確認付き）
- **状態**: indexStore, profileStore
- **レイアウト**: 検索バー + カードリスト
- **インタラクション**:
  - Select→ countDocuments() → ドキュメント数表示
  - Delete→ 確認UI→ deleteIndex()

#### Extract (/extract)
- **目的**: データ抽出とDuckDB保存
- **表示内容**:
  - インデックス選択（ドロップダウン）
  - テーブル名入力
  - 取得件数指定
  - JSONクエリエディタ
  - クイックガイド、クエリ例
  - 抽出結果メッセージ
- **状態**: indexStore, profileStore, ローカルstate
- **レイアウト**: 2カラム（設定 / ガイド）
- **インタラクション**:
  - Extract→ extractAndStoreData() → トースト通知

#### Database (/database)
- **目的**: ローカルDB管理
- **表示内容**:
  - テーブル一覧（サイドバー）
  - SQLクエリエディタ
  - クエリ結果テーブル
  - エクスポートボタン
- **状態**: ローカルstate
- **レイアウト**: 3カラム（テーブル / エディタ / 結果）
- **インタラクション**:
  - Execute→ queryLocal() → 結果表示
  - Export→ exportToParquet()

### 2.4 共通コンポーネント

#### Button
- **プロパティ**: variant (primary|secondary|danger), size (sm|md|lg), isLoading
- **スタイル**: Tailwind + dark mode対応
- **使用箇所**: 全ページ

#### Input
- **プロパティ**: label, error, type
- **スタイル**: Tailwind + dark mode対応
- **使用箇所**: Extract, Database

#### Card
- **プロパティ**: title, className
- **スタイル**: 白背景 + シャドウ + dark mode対応
- **使用箇所**: 全ページ

#### Loading
- **プロパティ**: size, text
- **スタイル**: スピナー + テキスト
- **使用箇所**: 全ページ

## 3. バックエンド設計

### 3.1 Tauriコマンド一覧

| コマンド | パラメータ | 戻り値 | 説明 |
|---------|----------|--------|------|
| list_profiles | - | ProfileConfig[] | プロファイル一覧 |
| get_profile | name | ProfileConfig | プロファイル取得 |
| save_profile | profile | void | プロファイル保存 |
| delete_profile | name | void | プロファイル削除 |
| test_connection | profileName | boolean | 接続テスト |
| get_cluster_info | profileName | ClusterInfo | クラスタ情報 |
| list_indices | profileName | string[] | インデックス一覧 |
| create_index | profileName, indexName, settings, mappings | void | インデックス作成 |
| delete_index | profileName, indexName | void | インデックス削除 |
| index_exists | profileName, indexName | boolean | インデックス存在確認 |
| search_documents | profileName, indexName, query | SearchResponse | ドキュメント検索 |
| count_documents | profileName, indexName, query | number | ドキュメント件数 |
| extract_and_store_data | profileName, indexName, query, tableName | string | データ抽出・保存 |
| list_tables | - | string[] | テーブル一覧 |
| query_local | sql | any[] | SQLクエリ実行 |
| export_to_parquet | query, outputPath | string | Parquetエクスポート |
| load_app_config | - | AppConfig | アプリ設定読み込み |
| save_app_config | config | void | アプリ設定保存 |
| encrypt_password | password | string | パスワード暗号化 |
| get_extraction_history | limit | ExtractionJob[] | 抽出履歴取得 |
| create_sample_index | profileName, templateName | void | サンプルインデックス作成 |

### 3.2 主要サービス

#### ConfigService
```rust
pub struct ConfigService {
    config_dir: PathBuf,
    encryptor: Encryptor,
}

impl ConfigService {
    pub fn new() -> Result<Self>;
    pub fn load_profiles() -> Result<Vec<ProfileConfig>>;
    pub fn save_profile(profile: &ProfileConfig) -> Result<()>;
    pub fn delete_profile(name: &str) -> Result<()>;
    pub fn get_profile(name: &str) -> Result<ProfileConfig>;
    pub fn load_app_config() -> Result<AppConfig>;
    pub fn save_app_config(config: &AppConfig) -> Result<()>;
}
```

#### ESClient
```rust
pub struct ESClient {
    url: String,
    client: reqwest::Client,
}

impl ESClient {
    pub fn new(profile: &ProfileConfig) -> Result<Self>;
    pub async fn test_connection(&self) -> Result<bool>;
    pub async fn get_cluster_info(&self) -> Result<ClusterInfo>;
    pub async fn list_indices(&self) -> Result<Vec<String>>;
    pub async fn create_index(&self, name: &str, ...) -> Result<()>;
    pub async fn delete_index(&self, name: &str) -> Result<()>;
    pub async fn search(&self, index: &str, query: &Value) -> Result<SearchResponse>;
    pub async fn count(&self, index: &str, query: &Value) -> Result<u64>;
    pub async fn bulk_insert(&self, index: &str, docs: Vec<Value>) -> Result<()>;
}
```

#### DuckDBService
```rust
pub struct DuckDBService {
    conn: Connection,
}

impl DuckDBService {
    pub fn new(db_path: &str) -> Result<Self>;
    pub fn init_tables(&self) -> Result<()>;
    pub fn create_data_table(&self, table_name: &str, docs: &[Value]) -> Result<()>;
    pub fn insert_data(&self, table_name: &str, docs: &[Value]) -> Result<usize>;
    pub fn list_tables(&self) -> Result<Vec<String>>;
    pub fn query_table(&self, sql: &str) -> Result<Vec<HashMap<String, Value>>>;
    pub fn save_extraction_job(&self, job: &ExtractionJob) -> Result<()>;
    pub fn get_extraction_history(&self, limit: usize) -> Result<Vec<ExtractionJob>>;
    pub fn export_to_parquet(&self, query: &str, path: &str) -> Result<()>;
}
```

## 4. データモデル

### 4.1 TypeScript型定義

```typescript
// プロファイル設定
export interface ProfileConfig {
  name: string;
  host: string;
  username?: string;
  password_encrypted?: string;
  api_key_encrypted?: string;
  auth_type: 'basic' | 'apikey';
  use_ssl: boolean;
  verify_certificate: boolean;
  created_at: number;
  updated_at: number;
}

// クラスタ情報
export interface ClusterInfo {
  name: string;
  cluster_name: string;
  cluster_uuid: string;
  version: VersionInfo;
}

// 検索クエリ
export interface SearchQuery {
  query?: Record<string, any>;
  size?: number;
  from?: number;
  sort?: Array<Record<string, any>>;
}

// アプリ設定
export interface AppConfig {
  default_profile?: string;
  log_level: string;
  database_path: string;
  theme: string;
}
```

### 4.2 Rustデータ構造

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileConfig {
    pub name: String,
    pub host: String,
    pub username: Option<String>,
    pub password_encrypted: Option<String>,
    pub api_key_encrypted: Option<String>,
    pub auth_type: AuthType,
    pub use_ssl: bool,
    pub verify_certificate: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterInfo {
    pub name: String,
    pub cluster_name: String,
    pub cluster_uuid: String,
    pub version: VersionInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractionJob {
    pub id: Option<i64>,
    pub profile_name: String,
    pub index_name: String,
    pub query_json: String,
    pub target_table: String,
    pub records_count: i64,
    pub executed_at: i64,
    pub status: String,
    pub error_message: Option<String>,
}
```

## 5. セキュリティ設計

### 5.1 認証情報の暗号化

```rust
// AES-256-GCM暗号化
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};

pub struct Encryptor {
    key: LessSafeKey,
}

impl Encryptor {
    // マシン固有のキー生成
    fn generate_machine_key() -> Result<[u8; 32]>;

    // 暗号化
    pub fn encrypt(&self, plaintext: &str) -> Result<Vec<u8>>;

    // 復号化
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<String>;
}
```

### 5.2 ファイルパーミッション

- profiles.toml: 600 (所有者のみ読み書き)
- data.duckdb: 600
- config.toml: 644

## 6. エラーハンドリング

### 6.1 エラーの伝搬

```
[UI] User Action
    ↓
[Store] Async Action (try/catch)
    ↓ error
[Store] Update error state
    ↓
[UI] Display error (Toast notification)
```

```
[Rust] Service Layer
    ↓ Result::Err
[Rust] Command Handler (?)
    ↓ Err(String)
[Tauri] IPC Error
    ↓
[TypeScript] Promise rejection
    ↓ catch
[React] Toast.error()
```

### 6.2 エラーメッセージ

すべて日本語で表示:
- "接続に失敗しました"
- "インデックスの削除に失敗しました"
- "データ抽出に失敗しました"
- "SQLクエリの実行に失敗しました"

## 7. パフォーマンス最適化

### 7.1 フロントエンド
- React.memo でコンポーネントの不要な再レンダリング防止
- Zustandのselectorsで必要な状態のみ購読
- Lazy loadingでコード分割（未実装）

### 7.2 バックエンド
- 非同期処理（Tokio）
- DuckDB Appenderによる高速挿入
- 接続プールの再利用

## 8. テスト戦略

### 8.1 ユニットテスト（Rust）
- 暗号化・復号化
- 設定ファイル読み書き
- DuckDB操作

### 8.2 E2Eテスト（未実装）
- Tauri E2Eテスト
- React Testing Library

## 9. ビルドとデプロイ

### 9.1 開発モード
```bash
npm run tauri dev
```

### 9.2 プロダクションビルド
```bash
npm run tauri build
```

出力先: `src-tauri/target/release/bundle/`

## 10. 今後の改善点

1. **モーダル実装**: プロファイル作成・編集、インデックス作成
2. **エラーリカバリ**: 再試行ロジック
3. **テストカバレッジ**: E2Eテストの追加
4. **パフォーマンス**: 仮想スクロール、コード分割
5. **アクセシビリティ**: キーボードナビゲーション、スクリーンリーダー対応
