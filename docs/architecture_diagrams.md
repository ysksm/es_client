# ES Client - アーキテクチャ図（修正版）

**最終更新:** 2025-11-25

## 1. システム全体構成（ASCII図）

```
┌───────────────────────────────────────────────────────────────┐
│              ES Client Desktop Application                    │
│                                                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  Frontend (React + TypeScript)                      │    │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐         │    │
│  │  │  Pages   │  │  Stores  │  │    API   │         │    │
│  │  │  (5個)   │→ │ (Zustand)│→ │ Wrapper  │         │    │
│  │  │          │  │  (3個)   │  │ (21 cmd) │         │    │
│  │  └──────────┘  └──────────┘  └─────┬────┘         │    │
│  └─────────────────────────────────────┼──────────────┘    │
│                                         │                    │
│                                  Tauri IPC                   │
│                                         │                    │
│  ┌─────────────────────────────────────┼──────────────┐    │
│  │  Backend (Rust)                     ▼              │    │
│  │  ┌──────────────┐  ┌──────────────────────────┐   │    │
│  │  │   Commands   │  │       Services           │   │    │
│  │  │   (21個)     │→ │  - ConfigService         │   │    │
│  │  │              │  │  - ESClient              │   │    │
│  │  │              │  │  - DuckDBService         │   │    │
│  │  └──────────────┘  └───────┬──────────────────┘   │    │
│  └─────────────────────────────┼──────────────────────┘    │
└─────────────────────────────────┼──────────────────────────┘
                                  │
                  ┌───────────────┴────────────────┐
                  │                                │
                  ▼                                ▼
        ┌──────────────────┐          ┌────────────────────┐
        │ Elasticsearch    │          │  Local DuckDB      │
        │ (Remote Cluster) │          │  ~/.es_client/     │
        │                  │          │  data.duckdb       │
        └──────────────────┘          └────────────────────┘
```

## 2. データフロー - 接続管理（ASCII図）

```
User Action: Click "Connect"
      │
      ▼
┌──────────────────┐
│ Connections Page │
│ (React)          │
└────────┬─────────┘
         │ selectProfile(name)
         ▼
┌──────────────────┐
│  profileStore    │
│  (Zustand)       │
└────────┬─────────┘
         │ api.getClusterInfo(profileName)
         ▼
┌──────────────────┐
│   tauri.ts       │
│  (API Wrapper)   │
└────────┬─────────┘
         │ invoke('get_cluster_info')
         ▼
┌──────────────────┐
│ Tauri Command    │
│ (Rust)           │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│   ESClient       │
│   (Service)      │
└────────┬─────────┘
         │ GET /_cluster/health
         │ GET /
         ▼
┌──────────────────┐
│ Elasticsearch    │
│ (Remote)         │
└────────┬─────────┘
         │ Response: ClusterInfo
         │
         ▼ (reverse flow)
┌──────────────────┐
│  profileStore    │
│  Update state    │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ Header Component │
│ Show connection  │
│ status badge     │
└──────────────────┘
```

## 3. フロントエンド構造

```
src/
│
├── main.tsx ················· エントリーポイント
│
├── App.tsx ·················· ルートコンポーネント (React Router)
│
├── components/
│   ├── ui/ ·················· 共通UIコンポーネント
│   │   ├── Button.tsx ······· ボタン (3バリアント, 3サイズ)
│   │   ├── Input.tsx ········ 入力フィールド
│   │   ├── Card.tsx ········· カードコンテナ
│   │   ├── Loading.tsx ······ ローディングスピナー
│   │   └── index.ts
│   │
│   └── layout/ ·············· レイアウトコンポーネント
│       ├── Header.tsx ······· ヘッダー (テーマ切替, 接続状態)
│       ├── Sidebar.tsx ······ サイドバーナビゲーション
│       ├── Layout.tsx ······· メインレイアウト
│       └── index.ts
│
├── pages/ ··················· ページコンポーネント
│   ├── Dashboard.tsx ········ ダッシュボード (統計情報)
│   ├── Connections.tsx ······ 接続管理 (プロファイル一覧)
│   ├── Indices.tsx ·········· インデックス管理 (一覧, 削除)
│   ├── Extract.tsx ·········· データ抽出 (クエリ, 保存)
│   ├── Database.tsx ········· ローカルDB管理 (SQL実行)
│   └── index.ts
│
├── store/ ··················· 状態管理 (Zustand)
│   ├── appStore.ts ·········· アプリ設定 (テーマ)
│   ├── profileStore.ts ······ プロファイル管理
│   └── indexStore.ts ········ インデックス管理
│
├── api/
│   └── tauri.ts ············· Tauri APIラッパー (21コマンド)
│
├── types/
│   └── index.ts ············· TypeScript型定義
│
└── index.css ················ グローバルスタイル (Tailwind)
```

## 4. バックエンド構造

```
src-tauri/src/
│
├── main.rs ·················· エントリーポイント
│                              - Tauri app setup
│                              - Command registration
│
├── models.rs ················ データモデル
│                              - ProfileConfig
│                              - ClusterInfo
│                              - ExtractionJob
│
├── config/ ·················· 設定管理
│   └── mod.rs ··············· ConfigService
│                              - load_profiles()
│                              - save_profile()
│                              - get_profile()
│                              - delete_profile()
│
├── es/ ·····················  Elasticsearch連携
│   └── mod.rs ··············· ESClient
│                              - test_connection()
│                              - get_cluster_info()
│                              - list_indices()
│                              - search()
│                              - create_index()
│                              - delete_index()
│
├── db/ ·····················  DuckDB管理
│   └── mod.rs ··············· DuckDBService
│                              - init_tables()
│                              - create_data_table()
│                              - insert_data()
│                              - query_table()
│                              - list_tables()
│                              - save_extraction_job()
│
└── utils.rs ················· ユーティリティ
                               - Encryptor (AES-256-GCM)
                               - encrypt()
                               - decrypt()
```

## 5. Tauri コマンド一覧

```
┌────────────────────────────────────────────────────────┐
│                  Tauri Commands (21個)                 │
├────────────────────────────────────────────────────────┤
│                                                        │
│  [プロファイル管理]                                     │
│  • list_profiles          プロファイル一覧取得         │
│  • get_profile            プロファイル取得             │
│  • save_profile           プロファイル保存             │
│  • delete_profile         プロファイル削除             │
│  • encrypt_password       パスワード暗号化             │
│                                                        │
│  [接続管理]                                             │
│  • test_connection        接続テスト                   │
│  • get_cluster_info       クラスタ情報取得             │
│                                                        │
│  [インデックス管理]                                     │
│  • list_indices           インデックス一覧             │
│  • create_index           インデックス作成             │
│  • delete_index           インデックス削除             │
│  • index_exists           インデックス存在確認         │
│  • search_documents       ドキュメント検索             │
│  • count_documents        ドキュメント件数取得         │
│                                                        │
│  [サンプルデータ]                                       │
│  • create_sample_index    サンプルインデックス作成     │
│                                                        │
│  [データ抽出]                                           │
│  • extract_and_store_data データ抽出・DuckDB保存      │
│  • get_extraction_history 抽出履歴取得                │
│                                                        │
│  [ローカルDB]                                           │
│  • list_tables            テーブル一覧                 │
│  • query_local            SQLクエリ実行                │
│  • export_to_parquet      Parquetエクスポート          │
│                                                        │
│  [アプリ設定]                                           │
│  • load_app_config        アプリ設定読み込み           │
│  • save_app_config        アプリ設定保存               │
│                                                        │
└────────────────────────────────────────────────────────┘
```

## 6. 状態管理フロー (Zustand)

```
┌─────────────┐
│  appStore   │  テーマ設定、アプリ設定
├─────────────┤
│ State:      │
│ • theme     │  'light' | 'dark'
│ • config    │  AppConfig | null
│ • isLoading │
│ • error     │
├─────────────┤
│ Actions:    │
│ • loadConfig()
│ • saveConfig()
│ • setTheme()
│ • toggleTheme()
└─────────────┘

┌─────────────────────┐
│  profileStore       │  プロファイル・接続管理
├─────────────────────┤
│ State:              │
│ • profiles          │  ProfileConfig[]
│ • currentProfile    │  ProfileConfig | null
│ • clusterInfo       │  ClusterInfo | null
│ • isLoading         │
│ • error             │
├─────────────────────┤
│ Actions:            │
│ • loadProfiles()
│ • selectProfile()
│ • createProfile()
│ • updateProfile()
│ • deleteProfile()
│ • testConnection()
└─────────────────────┘

┌─────────────────────┐
│  indexStore         │  インデックス管理
├─────────────────────┤
│ State:              │
│ • indices           │  string[]
│ • selectedIndex     │  string | null
│ • documentCount     │  number | null
│ • searchResults     │  any[]
│ • isLoading         │
│ • error             │
├─────────────────────┤
│ Actions:            │
│ • loadIndices()
│ • selectIndex()
│ • createIndex()
│ • deleteIndex()
│ • searchDocuments()
│ • countDocuments()
└─────────────────────┘
```

## 7. データフロー - データ抽出

```
[1] User Input
    ┌────────────────────────────────────────┐
    │ Extract Page                           │
    │ • インデックス: "logs-2024"            │
    │ • テーブル名: "my_logs"                │
    │ • クエリ: {"match_all": {}}            │
    │ • 件数: 1000                           │
    └─────────────────┬──────────────────────┘
                      │
                      ▼
[2] API Call
    ┌────────────────────────────────────────┐
    │ api.extractAndStoreData()              │
    └─────────────────┬──────────────────────┘
                      │
                      ▼
[3] Tauri Command
    ┌────────────────────────────────────────┐
    │ extract_and_store_data                 │
    └─────────────────┬──────────────────────┘
                      │
                      ├──[3a]──────────────────┐
                      │                         │
                      ▼                         ▼
[4] Elasticsearch Search        [5] DuckDB操作
    ┌───────────────────┐       ┌──────────────────┐
    │ ESClient.search() │       │ DuckDBService    │
    │                   │       │                  │
    │ POST /_search     │       │ 1. create_table()│
    │ → 1000 documents  │──────>│ 2. insert_data() │
    └───────────────────┘       │ 3. save_job()    │
                                └─────────┬────────┘
                                          │
                                          ▼
[6] Result
    ┌────────────────────────────────────────┐
    │ Success: "1000件のデータを保存しました" │
    └─────────────────┬──────────────────────┘
                      │
                      ▼
[7] UI Update
    ┌────────────────────────────────────────┐
    │ Toast Notification                     │
    │ ✓ データ抽出が完了しました              │
    └────────────────────────────────────────┘
```

## 8. セキュリティフロー

```
[入力] Password: "mypassword"
         │
         ▼
[暗号化前処理]
┌────────────────────────┐
│ Encryptor              │
│ • AES-256-GCM          │
│ • Machine-specific key │
└───────────┬────────────┘
            │
            ▼
[暗号化]
┌────────────────────────┐
│ Encrypted bytes        │
│ [0x1a, 0x2b, ...]      │
└───────────┬────────────┘
            │
            ▼
[HEX変換]
┌────────────────────────┐
│ HEX string             │
│ "1a2b3c4d..."          │
└───────────┬────────────┘
            │
            ▼
[保存]
┌────────────────────────┐
│ profiles.toml          │
│ password_encrypted =   │
│   "1a2b3c4d..."        │
│                        │
│ Permission: 600        │
│ (owner only)           │
└────────────────────────┘

[読み込み時は逆順]
HEX string → bytes → decrypt → Password
```

## 9. 技術スタック

```
┌──────────────────────────────────────────────────────┐
│                   Frontend Stack                     │
├──────────────────────────────────────────────────────┤
│  React                 19.1.0    UIライブラリ        │
│  TypeScript             5.8.3    型安全性            │
│  React Router           7.1.1    ルーティング        │
│  Zustand                5.0.2    状態管理            │
│  Tailwind CSS          3.4.17    スタイリング        │
│  Headless UI            2.2.0    UIコンポーネント    │
│  Heroicons              2.2.0    アイコン            │
│  React Hot Toast        2.4.1    通知                │
│  Vite                   7.0.4    ビルドツール        │
└──────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────┐
│                   Backend Stack                      │
├──────────────────────────────────────────────────────┤
│  Rust               2024 Edition  言語               │
│  Tauri                      2.x   デスクトップFW     │
│  reqwest                  latest  HTTPクライアント   │
│  duckdb                      1.1  ローカルDB         │
│  tokio                   latest   非同期ランタイム   │
│  serde/serde_json        latest   JSON処理           │
│  ring                    latest   暗号化             │
│  toml                    latest   設定ファイル       │
└──────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────┐
│                   External Services                  │
├──────────────────────────────────────────────────────┤
│  Elasticsearch          7.x/8.x   データソース       │
└──────────────────────────────────────────────────────┘
```

## 10. ファイル構成の詳細

```
es_client/
│
├── src/                     フロントエンド (約2,100行)
│   ├── api/                 (150行)
│   ├── components/          (250行)
│   ├── pages/               (1,025行)
│   ├── store/               (330行)
│   ├── types/               (220行)
│   ├── App.tsx              (25行)
│   ├── main.tsx             (10行)
│   └── index.css            (30行)
│
├── src-tauri/               バックエンド (約1,500行)
│   ├── src/
│   │   ├── main.rs          (100行)
│   │   ├── models.rs        (200行)
│   │   ├── config/          (300行)
│   │   ├── es/              (400行)
│   │   ├── db/              (400行)
│   │   └── utils.rs         (100行)
│   └── Cargo.toml
│
├── cli/                     CLIバイナリ (約600行)
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
│
├── docs/                    ドキュメント
│   ├── requirements_implemented.md
│   ├── design_implemented.md
│   ├── tasks_completed.md
│   └── architecture_diagrams.md  (このファイル)
│
├── README.md
├── package.json
└── tailwind.config.js
```

## 簡易版 - 最も重要な3つの図

### 図1: コンポーネント→Store→API→Rust

```
[React Component]
        ↓
    [Zustand Store]
        ↓
    [API Wrapper]
        ↓ Tauri IPC
    [Rust Command]
        ↓
    [Service Layer]
        ↓
[External Service]
```

### 図2: 5つの画面

```
1. Dashboard    → 統計情報
2. Connections  → プロファイル管理
3. Indices      → インデックス管理
4. Extract      → データ抽出
5. Database     → ローカルDB管理
```

### 図3: 3つのStore

```
1. appStore     → テーマ設定
2. profileStore → 接続管理
3. indexStore   → インデックス管理
```

---

これらの図はテキストベースなので、どの環境でも確実に表示できます！
