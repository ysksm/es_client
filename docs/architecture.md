# ES Client - アーキテクチャ図

**最終更新:** 2025-11-25

## 1. システム全体アーキテクチャ

```mermaid
graph TB
    subgraph "Desktop Application (Tauri)"
        subgraph "Frontend (React + TypeScript)"
            Pages[Pages<br/>Dashboard, Connections, Indices, Extract, Database]
            Components[Components<br/>Button, Input, Card, Loading, Header, Sidebar]
            Stores[State Management<br/>appStore, profileStore, indexStore]
            API[API Wrapper<br/>tauri.ts - 21 commands]

            Pages --> Stores
            Pages --> Components
            Stores --> API
        end

        subgraph "Backend (Rust)"
            Commands[Tauri Commands<br/>21 handlers]
            Services[Services<br/>ConfigService, ESClient, DuckDBService]
            Models[Data Models<br/>ProfileConfig, ClusterInfo, etc.]
            Utils[Utils<br/>Encryptor]

            Commands --> Services
            Services --> Models
            Services --> Utils
        end

        API -.Tauri IPC.-> Commands
    end

    Services --> ES[Elasticsearch<br/>Remote Cluster]
    Services --> DB[(Local DuckDB<br/>~/.es_client/data.duckdb)]
    Services --> Config[Config Files<br/>profiles.toml<br/>config.toml]

    style Pages fill:#e1f5ff
    style Components fill:#e8f5e9
    style Stores fill:#fff9c4
    style API fill:#ffe0b2
    style Commands fill:#f3e5f5
    style Services fill:#e1bee7
    style ES fill:#ffccbc
    style DB fill:#c5cae9
    style Config fill:#d7ccc8
```

## 2. データフロー - 接続管理

```mermaid
sequenceDiagram
    participant User
    participant UI as Connections Page
    participant Store as profileStore
    participant API as tauri.ts
    participant Cmd as Tauri Command
    participant Svc as ESClient
    participant ES as Elasticsearch

    User->>UI: Click "Connect" button
    UI->>Store: selectProfile(name)
    Store->>API: getClusterInfo(profileName)
    API->>Cmd: invoke('get_cluster_info')
    Cmd->>Svc: ESClient.get_cluster_info()
    Svc->>ES: GET /_cluster/health
    ES-->>Svc: {health: "green", ...}
    Svc->>ES: GET /
    ES-->>Svc: {version: {...}}
    Svc-->>Cmd: ClusterInfo
    Cmd-->>API: Result<ClusterInfo>
    API-->>Store: ClusterInfo
    Store->>Store: Update state
    Store-->>UI: Trigger re-render
    UI-->>User: Show connection status in Header
```

## 3. データフロー - データ抽出

```mermaid
sequenceDiagram
    participant User
    participant UI as Extract Page
    participant API as tauri.ts
    participant Cmd as Tauri Command
    participant ES as ESClient
    participant DB as DuckDBService
    participant Duck as DuckDB File

    User->>UI: Click "データを抽出"
    UI->>API: extractAndStoreData(profile, index, query, table)
    API->>Cmd: invoke('extract_and_store_data')
    Cmd->>ES: search(index, query)
    ES-->>Cmd: Vec<Document>
    Cmd->>DB: create_data_table(table, docs)
    DB->>Duck: CREATE TABLE ...
    Cmd->>DB: insert_data(table, docs)
    DB->>Duck: INSERT INTO ...
    Cmd->>DB: save_extraction_job(history)
    DB->>Duck: INSERT INTO extraction_history
    Cmd-->>API: Result<String>
    API-->>UI: Success message
    UI->>UI: Show toast notification
```

## 4. コンポーネント階層

```mermaid
graph TD
    App[App.tsx<br/>BrowserRouter]
    Layout[Layout<br/>Initialize app state]
    Header[Header<br/>Title, Connection Status, Theme Toggle]
    Sidebar[Sidebar<br/>Navigation x 5]
    Main[Main Content<br/>React Router Outlet]

    Dashboard[Dashboard Page<br/>Stats & Cluster Info]
    Connections[Connections Page<br/>Profile Management]
    Indices[Indices Page<br/>Index Management]
    Extract[Extract Page<br/>Data Extraction]
    Database[Database Page<br/>Local DB Management]

    Button[Button Component<br/>3 variants, 3 sizes]
    Input[Input Component<br/>Label, Error]
    Card[Card Component<br/>Container]
    Loading[Loading Component<br/>Spinner]

    App --> Layout
    Layout --> Header
    Layout --> Sidebar
    Layout --> Main

    Main --> Dashboard
    Main --> Connections
    Main --> Indices
    Main --> Extract
    Main --> Database

    Dashboard --> Card
    Dashboard --> Loading
    Connections --> Card
    Connections --> Button
    Indices --> Card
    Indices --> Button
    Indices --> Input
    Extract --> Card
    Extract --> Button
    Extract --> Input
    Database --> Card
    Database --> Button
    Database --> Input

    style App fill:#e3f2fd
    style Layout fill:#f3e5f5
    style Header fill:#fff9c4
    style Sidebar fill:#fff9c4
    style Main fill:#e8f5e9
    style Button fill:#ffccbc
    style Input fill:#ffccbc
    style Card fill:#ffccbc
    style Loading fill:#ffccbc
```

## 5. 状態管理 (Zustand)

```mermaid
graph LR
    subgraph "Zustand Stores"
        AS[appStore<br/>theme, config]
        PS[profileStore<br/>profiles, currentProfile, clusterInfo]
        IS[indexStore<br/>indices, selectedIndex, documentCount]
    end

    subgraph "Pages"
        Dashboard
        Connections
        Indices
        Extract
        Database
    end

    subgraph "Persistence"
        LS[localStorage<br/>app-storage]
    end

    Dashboard --> AS
    Dashboard --> PS

    Connections --> PS

    Indices --> PS
    Indices --> IS

    Extract --> PS
    Extract --> IS

    Database --> AS

    AS <-.persist.-> LS

    style AS fill:#fff9c4
    style PS fill:#c5cae9
    style IS fill:#a5d6a7
    style LS fill:#ffccbc
```

## 6. バックエンドアーキテクチャ

```mermaid
graph TB
    subgraph "Tauri Commands Layer"
        PC[Profile Commands<br/>list, get, save, delete]
        IC[Index Commands<br/>list, create, delete, search]
        EC[Extract Commands<br/>extract_and_store_data]
        DC[DB Commands<br/>query_local, list_tables]
    end

    subgraph "Service Layer"
        CS[ConfigService<br/>Profile & App Config]
        ESC[ESClient<br/>Elasticsearch Operations]
        DBS[DuckDBService<br/>Local DB Operations]
    end

    subgraph "Data Layer"
        Files[Config Files<br/>profiles.toml<br/>config.toml]
        ES[Elasticsearch<br/>Remote]
        DB[(DuckDB<br/>data.duckdb)]
    end

    PC --> CS
    IC --> ESC
    EC --> ESC
    EC --> DBS
    DC --> DBS

    CS --> Files
    ESC --> ES
    DBS --> DB

    style PC fill:#e1bee7
    style IC fill:#e1bee7
    style EC fill:#e1bee7
    style DC fill:#e1bee7
    style CS fill:#c5cae9
    style ESC fill:#c5cae9
    style DBS fill:#c5cae9
```

## 7. セキュリティ層

```mermaid
graph LR
    subgraph "User Input"
        Pass[Password / API Key]
    end

    subgraph "Encryption Layer"
        Enc[Encryptor<br/>AES-256-GCM]
        Key[Machine-specific Key<br/>Generated on first run]
    end

    subgraph "Storage"
        File[profiles.toml<br/>600 permission]
    end

    subgraph "Runtime"
        Mem[Memory<br/>Cleared after use]
    end

    Pass --> Enc
    Key --> Enc
    Enc --> File
    Enc --> Mem

    File -.Decrypt.-> Enc
    Enc -.Use.-> ESC[ESClient]

    style Pass fill:#ffccbc
    style Enc fill:#c5cae9
    style Key fill:#a5d6a7
    style File fill:#fff9c4
    style Mem fill:#f48fb1
```

## 8. テクノロジースタック

```mermaid
graph TB
    subgraph "Frontend Stack"
        React[React 19.1.0]
        TS[TypeScript 5.8.3]
        Router[React Router 7.1.1]
        Zustand[Zustand 5.0.2]
        Tailwind[Tailwind CSS 3.4.17]
        Vite[Vite 7.0.4]
    end

    subgraph "Backend Stack"
        Rust[Rust 2024 Edition]
        Tauri[Tauri 2.x]
        Reqwest[reqwest]
        DuckDB[duckdb 1.1]
        Tokio[tokio async runtime]
        Serde[serde JSON]
    end

    subgraph "External"
        ES[Elasticsearch 7.x/8.x]
    end

    React --> TS
    React --> Router
    React --> Zustand
    React --> Tailwind
    TS --> Vite

    Rust --> Tauri
    Rust --> Reqwest
    Rust --> DuckDB
    Rust --> Tokio
    Rust --> Serde

    Tauri -.IPC.-> React
    Reqwest --> ES

    style React fill:#61dafb
    style Rust fill:#ff6b35
    style Tauri fill:#ffc947
    style ES fill:#00bfa5
```

## 9. Draw.io変換手順

このMermaid図をDraw.ioで使用するには:

### 方法1: 手動再作成
1. Draw.ioを開く
2. 上記の図を参考に手動で作成
3. レイアウトを調整

### 方法2: Mermaid Live Editorを使用
1. https://mermaid.live にアクセス
2. 上記のMermaidコードをコピー&ペースト
3. SVGまたはPNGでエクスポート
4. Draw.ioにインポート

### 方法3: VS Code拡張機能を使用
1. "Mermaid Markdown Syntax Highlighting"拡張機能をインストール
2. このファイルをVS Codeで開く
3. プレビューを表示
4. スクリーンショットを取得

## 10. 主要な設計判断

### 10.1 フロントエンド
- **React Router v7**: 最新のルーティングAPIとデータローディング
- **Zustand**: Reduxよりシンプルで軽量な状態管理
- **Tailwind CSS**: ユーティリティファーストで高速開発
- **Headless UI**: アクセシビリティに配慮した基礎コンポーネント

### 10.2 バックエンド
- **Rust 2024 Edition**: 最新の言語機能とモジュールシステム
- **reqwest**: 非同期HTTPクライアント、Elasticsearchとの通信
- **DuckDB**: 軽量で高速なOLAPデータベース
- **AES-256-GCM**: 業界標準の暗号化アルゴリズム

### 10.3 アーキテクチャパターン
- **コンポーネント分離**: UI/Layout/Pageの3層構造
- **状態管理の分離**: アプリ/プロファイル/インデックスで責務分割
- **サービス層**: ビジネスロジックをコマンドハンドラから分離
- **エラー境界**: 各層で適切なエラーハンドリング
