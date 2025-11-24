# Elasticsearch Index管理アプリケーション 開発タスク（成果物ベース）

## タスク管理について

- [ ] : 未着手
- [🔄] : 作業中
- [x] : 完了
- 優先度: 🔴 必須 / 🟡 推奨 / 🟢 オプション

---

## 📊 全体進捗

| 機能 | ステータス | 進捗 | 優先度 |
|------|-----------|------|--------|
| [F1: プロジェクトセットアップ](#feature-1-プロジェクトセットアップ) | ✅ 完了 | 100% | 🔴 |
| [F2: 接続管理機能](#feature-2-接続管理機能) | ✅ 完了 | 100% | 🔴 |
| [F3: インデックス管理機能](#feature-3-インデックス管理機能) | ✅ 完了 | 100% | 🔴 |
| [F4: データ抽出・保存機能](#feature-4-データ抽出保存機能) | ✅ 完了 | 100% | 🔴 |
| [F5: ローカルDB管理機能](#feature-5-ローカルdb管理機能) | ✅ 完了 | 100% | 🔴 |
| [F6: CLI実装](#feature-6-cli実装) | ✅ 完了 | 100% | 🟡 |
| [F7: GUI実装](#feature-7-gui実装) | 未着手 | 0% | 🔴 |
| [F8: セキュリティ・品質](#feature-8-セキュリティ品質) | 未着手 | 0% | 🟡 |
| [F9: パッケージング・リリース](#feature-9-パッケージングリリース) | 未着手 | 0% | 🟡 |

**総合進捗: 67%** ■■■■■■▰▱▱▱

---

## 🎯 マイルストーン（成果物ベース）

### Milestone 1: 動作するCLIプロトタイプ
**完了条件:**
- ✅ Elasticsearchに接続できる
- ✅ シンプルなクエリでデータを抽出できる
- ✅ 抽出したデータをDuckDBに保存できる
- ✅ 保存したデータをSQLで確認できる

**含まれる機能:** F1, F2, F4, F5（コア部分のみ）, F6（基本コマンド）

**デモシナリオ:**
```bash
# 1. 接続設定
es-client connect --host https://localhost:9200 --user elastic --password changeme

# 2. データ抽出
es-client extract --index logs-2024 --query '{"match_all":{}}' --output logs_table

# 3. データ確認
es-client db query --sql "SELECT * FROM logs_table LIMIT 10"
```

---

### Milestone 2: 使えるGUIアプリ
**完了条件:**
- ✅ Milestone 1の全機能がGUIから利用可能
- ✅ 接続プロファイルをGUIで管理できる
- ✅ インデックス一覧が見える
- ✅ クエリビルダーでデータ抽出できる
- ✅ 結果をテーブル表示できる

**含まれる機能:** F7（全画面）

**デモシナリオ:**
1. アプリ起動 → 接続設定画面でプロファイル作成
2. インデックス管理画面でインデックス一覧確認
3. データ抽出画面でクエリ作成 → 実行 → プレビュー → 保存
4. ローカルDB画面でテーブル確認

---

### Milestone 3: プロダクションレディ
**完了条件:**
- ✅ Milestone 2の全機能
- ✅ 認証情報が安全に暗号化保存される
- ✅ エラーハンドリングが適切
- ✅ ユニットテスト・統合テストがある
- ✅ 各プラットフォームでビルドできる

**含まれる機能:** F8, F9

---

## Feature 1: プロジェクトセットアップ

### 📦 成果物
- Tauriプロジェクトが動作する
- 必要な依存関係がすべて揃っている
- 開発環境が整っている

### ✅ 完了条件
- [ ] `cargo run` でTauriアプリが起動する
- [ ] フロントエンドの開発サーバーが動作する
- [ ] 基本的なディレクトリ構成ができている

### 📋 タスク

#### 1.1 環境構築 🔴
- [ ] Rustツールチェーンのインストール確認 (`rustc --version`)
- [ ] Node.js/npmのインストール確認 (`node --version`)
- [ ] Tauriプロジェクト作成
  ```bash
  npm create tauri-app@latest
  # Project name: es_client
  # Framework: React
  # Variant: TypeScript
  ```
- [ ] プロジェクトディレクトリ構成の整理（design.mdに従う）

#### 1.2 Rust依存関係の追加 🔴
`src-tauri/Cargo.toml` に以下を追加:
- [ ] `elasticsearch = "8.15"`
- [ ] `duckdb = "1.1"`
- [ ] `tokio = { version = "1", features = ["full"] }`
- [ ] `serde = { version = "1", features = ["derive"] }`
- [ ] `serde_json = "1"`
- [ ] `toml = "0.8"`
- [ ] `ring = "0.17"` または `aes-gcm = "0.10"`
- [ ] `thiserror = "1"`
- [ ] `tracing = "0.1"`
- [ ] `tracing-subscriber = "0.3"`
- [ ] `hex = "0.4"` (暗号化データのHEX変換用)
- [ ] `dirs = "5"` (ホームディレクトリ取得用)

#### 1.3 フロントエンド依存関係の追加 🔴
`src-ui/package.json` に以下を追加:
- [ ] `@tauri-apps/api`
- [ ] `zustand` (状態管理)
- [ ] `react-router-dom`
- [ ] `@tanstack/react-table` (テーブル表示用)

#### 1.4 開発環境設定 🔴
- [ ] `.gitignore` の設定
- [ ] ESLint/Prettier設定 (フロントエンド)
- [ ] `rustfmt.toml` 設定
- [ ] 基本的なREADME.md更新（セットアップ手順）

#### 1.5 プロジェクト構造の作成 🔴
以下のディレクトリとファイルを作成:
- [ ] `src-tauri/src/commands/` (空ディレクトリ + mod.rs)
- [ ] `src-tauri/src/services/` (空ディレクトリ + mod.rs)
- [ ] `src-tauri/src/models/` (空ディレクトリ + mod.rs)
- [ ] `src-tauri/src/utils/` (空ディレクトリ + mod.rs)
- [ ] `src-ui/src/api/` (Tauri APIラッパー用)
- [ ] `src-ui/src/components/` (Reactコンポーネント)
- [ ] `src-ui/src/pages/` (画面コンポーネント)

---

## Feature 2: 接続管理機能

### 📦 成果物
Elasticsearchへの接続設定を管理し、接続テストができる機能

### 🎯 デモシナリオ
1. 新しい接続プロファイルを作成（ホスト、ユーザー名、パスワードを入力）
2. 接続テストボタンを押す → 成功/失敗が表示される
3. プロファイルを保存 → `~/.es_client/profiles.toml` に暗号化して保存される
4. 保存したプロファイルを一覧表示できる
5. プロファイルを選択してElasticsearchに接続できる

### ✅ 完了条件
- [ ] 接続プロファイルをCRUD操作できる
- [ ] 接続テストが成功する
- [ ] パスワードが暗号化されて保存される
- [ ] 複数のプロファイルを管理できる

### 📋 タスク

#### 2.1 データモデル定義 🔴
`src-tauri/src/models/config.rs` を作成:
- [ ] `ProfileConfig` 構造体の定義
- [ ] `AppConfig` 構造体の定義
- [ ] `AuthType` enum の定義

#### 2.2 暗号化ユーティリティ 🔴
`src-tauri/src/utils/crypto.rs` を作成:
- [ ] `Encryptor` 構造体の実装
- [ ] マシン固有のキー生成ロジック
- [ ] `encrypt()` メソッド
- [ ] `decrypt()` メソッド
- [ ] HEX変換ユーティリティ

#### 2.3 設定管理サービス 🔴
`src-tauri/src/services/config.rs` を作成:
- [ ] `ConfigService` 構造体の実装
- [ ] 設定ディレクトリの作成 (`~/.es_client/`)
- [ ] `profiles.toml` の読み込み
- [ ] `profiles.toml` への書き込み
- [ ] プロファイルのCRUD操作
  - [ ] `load_profiles()` - 全プロファイル読み込み
  - [ ] `get_profile(name)` - 特定プロファイル取得
  - [ ] `save_profile(profile)` - プロファイル保存
  - [ ] `delete_profile(name)` - プロファイル削除
- [ ] `config.toml` の管理

#### 2.4 Elasticsearchクライアント 🔴
`src-tauri/src/services/es_client.rs` を作成:
- [ ] `ESClient` 構造体の実装
- [ ] クライアント初期化 (`new()`)
- [ ] Basic認証対応
- [ ] APIキー認証対応
- [ ] SSL設定対応
- [ ] 接続テスト (`test_connection()`)
- [ ] クラスタ情報取得 (`get_cluster_info()`)

#### 2.5 Tauriコマンド（接続管理） 🔴
`src-tauri/src/commands/connection.rs` を作成:
- [ ] `test_connection` - 接続テスト
- [ ] `list_profiles` - プロファイル一覧取得
- [ ] `get_profile` - プロファイル取得
- [ ] `save_profile` - プロファイル保存
- [ ] `delete_profile` - プロファイル削除
- [ ] エラーハンドリング（Result型を統一）

#### 2.6 ユニットテスト 🟡
- [ ] 暗号化・復号化のテスト
- [ ] 設定ファイル読み書きのテスト
- [ ] プロファイルCRUD操作のテスト

---

## Feature 3: インデックス管理機能

### 📦 成果物
Elasticsearchのインデックス一覧表示、作成、サンプルデータ投入ができる機能

### 🎯 デモシナリオ
1. プロファイルを選択してElasticsearchに接続
2. インデックス一覧を表示（名前、ドキュメント数、サイズ）
3. 新しいインデックスを作成（名前、マッピング指定）
4. サンプルデータを投入（JSON形式）

### ✅ 完了条件
- [ ] インデックス一覧が取得できる
- [ ] インデックスを新規作成できる
- [ ] サンプルデータを投入できる

### 📋 タスク

#### 3.1 Elasticsearchクライアント拡張 🔴
`src-tauri/src/services/es_client.rs` に追加:
- [ ] `list_indices()` - インデックス一覧取得
- [ ] `get_index_info(name)` - インデックス詳細情報取得
- [ ] `create_index(name, mapping)` - インデックス作成
- [ ] `index_document(index, doc)` - ドキュメント投入
- [ ] `bulk_index(index, docs)` - 複数ドキュメント一括投入

#### 3.2 データモデル 🔴
`src-tauri/src/models/index.rs` を作成:
- [ ] `IndexInfo` 構造体（名前、ドキュメント数、サイズ等）
- [ ] `IndexMapping` 構造体
- [ ] `CreateIndexRequest` 構造体

#### 3.3 Tauriコマンド（インデックス管理） 🔴
`src-tauri/src/commands/index.rs` を作成:
- [ ] `list_indices` - インデックス一覧
- [ ] `get_index_info` - インデックス詳細
- [ ] `create_index` - インデックス作成
- [ ] `load_sample_data` - サンプルデータ投入

#### 3.4 サンプルテンプレート 🟡
- [ ] よく使うインデックスのテンプレートを用意（logs, metricsなど）
- [ ] テンプレートJSONファイルを `src-tauri/templates/` に配置

---

## Feature 4: データ抽出・保存機能

### 📦 成果物
Elasticsearchからデータを検索・抽出し、DuckDBに保存できる機能（**コア機能**）

### 🎯 デモシナリオ
1. インデックスを選択
2. クエリを入力（JSON形式 または シンプルクエリ）
3. 検索実行 → 結果をプレビュー表示
4. 保存先テーブル名を指定
5. DuckDBに保存 → 完了メッセージ表示
6. 保存したデータをSQLで確認

### ✅ 完了条件
- [ ] クエリDSLでデータ検索ができる
- [ ] 検索結果をプレビューできる
- [ ] DuckDBにデータを保存できる
- [ ] 大量データ取得時もメモリ効率良く動作する（Scroll API）

### 📋 タスク

#### 4.1 DuckDBサービス 🔴
`src-tauri/src/services/duckdb.rs` を作成:
- [ ] `DuckDBService` 構造体の実装
- [ ] データベース接続 (`new()`)
- [ ] 履歴テーブルの作成 (`create_history_table()`)
- [ ] 動的テーブル作成 (`create_table_from_schema()`)
- [ ] データ挿入 (`save_documents()`) - Appender使用
- [ ] テーブル一覧取得 (`list_tables()`)
- [ ] SQLクエリ実行 (`query()`)
- [ ] テーブル削除 (`drop_table()`)
- [ ] 抽出履歴の保存 (`save_extraction_history()`)

#### 4.2 データモデル 🔴
`src-tauri/src/models/document.rs` を作成:
- [ ] `ESDocument` 構造体（_id, _index, _source等）
- [ ] `SearchRequest` 構造体
- [ ] `SearchResponse` 構造体
- [ ] `TableSchema` 構造体
- [ ] `ExtractionHistory` 構造体

#### 4.3 データ変換ロジック 🔴
`src-tauri/src/services/converter.rs` を作成:
- [ ] `DocumentConverter` の実装
- [ ] スキーマ推測 (`infer_schema()`)
  - JSON型からDuckDB型へのマッピング
  - ネストされたオブジェクトの処理
- [ ] ドキュメント変換 (`convert_to_rows()`)
- [ ] 型変換処理（timestamp, 数値、文字列等）

#### 4.4 Elasticsearchクライアント拡張 🔴
`src-tauri/src/services/es_client.rs` に追加:
- [ ] `search()` - 通常の検索
- [ ] `search_with_scroll()` - Scroll APIを使った大量データ取得
- [ ] `scroll_next()` - 次のページ取得
- [ ] `clear_scroll()` - Scrollのクリア

#### 4.5 Tauriコマンド（データ抽出） 🔴
`src-tauri/src/commands/extract.rs` を作成:
- [ ] `search_documents` - ドキュメント検索
- [ ] `extract_and_save` - 検索 + DuckDB保存を一括処理
- [ ] `preview_data` - データプレビュー（件数制限）
- [ ] プログレス通知（大量データ取得時）

#### 4.6 Tauriコマンド（ローカルDB） 🔴
`src-tauri/src/commands/database.rs` を作成:
- [ ] `list_local_tables` - テーブル一覧
- [ ] `query_local_db` - SQLクエリ実行
- [ ] `preview_table` - テーブルプレビュー
- [ ] `drop_local_table` - テーブル削除
- [ ] `get_extraction_history` - 抽出履歴取得

#### 4.7 統合テスト 🟡
- [ ] Testcontainersを使ったElasticsearchテスト
- [ ] データ抽出 → 変換 → DuckDB保存のE2Eテスト

---

## Feature 5: ローカルDB管理機能

### 📦 成果物
DuckDBに保存したデータを確認・管理できる機能

### 🎯 デモシナリオ
1. 保存済みテーブル一覧を表示
2. テーブルをクリック → データをプレビュー表示
3. カスタムSQLクエリを実行
4. 不要なテーブルを削除
5. 抽出履歴を確認

### ✅ 完了条件
- [ ] テーブル一覧が表示できる
- [ ] テーブルの中身をプレビューできる
- [ ] SQLクエリを実行して結果を表示できる
- [ ] テーブルを削除できる

### 📋 タスク

これらのタスクは **Feature 4** でほぼ完了しているため、残りは：

#### 5.1 追加機能 🟡
- [ ] テーブル統計情報の取得（行数、サイズ等）
- [ ] クエリ履歴の保存・表示
- [ ] テーブルのエクスポート機能（CSV, JSON）

---

## Feature 6: CLI実装

### 📦 成果物
すべての機能をコマンドラインから利用できるCLIツール

### 🎯 デモシナリオ
```bash
# 接続設定
es-client connect --host https://localhost:9200 --user elastic --password changeme --profile local

# プロファイル一覧
es-client profile list

# インデックス作成
es-client index create --name test-index --profile local

# データ抽出
es-client extract --index logs-* --query query.json --output logs_table --profile local

# ローカルDB確認
es-client db list
es-client db query --sql "SELECT COUNT(*) FROM logs_table"
```

### ✅ 完了条件
- [ ] 主要なコマンドがすべて実装されている
- [ ] エラーメッセージが分かりやすい
- [ ] ヘルプメッセージが充実している

### 📋 タスク

#### 6.1 CLIプロジェクトセットアップ 🟡
- [ ] `cli/` ディレクトリ作成
- [ ] Cargoワークスペース設定（サービスコードの再利用）
- [ ] `clap` の依存関係追加

#### 6.2 CLIコマンド実装 🟡
`cli/src/main.rs` と各サブコマンド:
- [ ] `connect` - 接続設定
- [ ] `profile` サブコマンド
  - [ ] `list` - 一覧
  - [ ] `show` - 詳細
  - [ ] `delete` - 削除
- [ ] `index` サブコマンド
  - [ ] `list` - 一覧
  - [ ] `create` - 作成
  - [ ] `info` - 詳細
- [ ] `extract` - データ抽出
- [ ] `db` サブコマンド
  - [ ] `list` - テーブル一覧
  - [ ] `query` - SQLクエリ実行
  - [ ] `show` - テーブルプレビュー
  - [ ] `drop` - テーブル削除

#### 6.3 CLI改善 🟢
- [ ] カラー出力対応（`colored` crate）
- [ ] プログレスバー表示（`indicatif` crate）
- [ ] テーブル表示の整形（`prettytable` crate）
- [ ] 自動補完スクリプト生成（Bash, Zsh, Fish）

---

## Feature 7: GUI実装

### 📦 成果物
すべての機能を直感的に操作できるデスクトップアプリケーション

### 🎯 デモシナリオ
1. アプリ起動 → ダッシュボード表示
2. サイドバーから「接続設定」を選択 → プロファイル作成
3. 「インデックス管理」でインデックス一覧確認
4. 「データ抽出」で以下を実行:
   - インデックス選択
   - クエリ入力（JSONエディタ）
   - 検索実行
   - 結果プレビュー（ページネーション付きテーブル）
   - 保存先指定 → DuckDB保存
5. 「ローカルDB」で保存データ確認

### ✅ 完了条件
- [ ] すべての画面が実装されている
- [ ] 直感的に操作できる
- [ ] エラーが適切に表示される
- [ ] レスポンシブなUI

### 📋 タスク

#### 7.1 基本セットアップ 🔴
- [ ] React Routerの設定
- [ ] レイアウトコンポーネント作成
  - [ ] `Layout` - 全体レイアウト
  - [ ] `Header` - ヘッダー（接続状態、プロファイル選択）
  - [ ] `Sidebar` - サイドバーナビゲーション
- [ ] Tauri APIラッパー作成 (`src-ui/src/api/tauri.ts`)
- [ ] Zustand ストアセットアップ
  - [ ] `useConnectionStore` - 接続状態管理
  - [ ] `useProfileStore` - プロファイル管理

#### 7.2 ダッシュボード画面 🔴
`src-ui/src/pages/Dashboard.tsx`:
- [ ] 接続状態の表示
- [ ] 最近使用したプロファイル
- [ ] クイックアクションボタン
- [ ] システム情報表示

#### 7.3 接続設定画面 🔴
`src-ui/src/pages/Connections.tsx`:
- [ ] プロファイル一覧表示（カード形式）
- [ ] 新規プロファイル作成モーダル
  - [ ] ホスト、ユーザー名、パスワード入力フォーム
  - [ ] 認証方式選択（Basic / APIキー）
  - [ ] SSL設定
- [ ] プロファイル編集機能
- [ ] 接続テストボタン → 結果表示
- [ ] プロファイル削除（確認ダイアログ付き）

#### 7.4 インデックス管理画面 🔴
`src-ui/src/pages/Indices.tsx`:
- [ ] インデックス一覧表示（テーブル形式）
  - [ ] 名前、ドキュメント数、サイズ、ヘルス状態
  - [ ] 検索フィルター
- [ ] インデックス作成ボタン → モーダル
  - [ ] インデックス名入力
  - [ ] マッピング設定（JSON編集）
  - [ ] テンプレート選択
- [ ] インデックス詳細表示

#### 7.5 データ抽出画面 🔴
`src-ui/src/pages/Extract.tsx`:
- [ ] インデックス選択ドロップダウン
- [ ] クエリ入力エリア
  - [ ] シンプルモード（フォーム入力）
  - [ ] アドバンスモード（JSONエディタ - Monaco Editor）
  - [ ] モード切り替えボタン
- [ ] 検索実行ボタン
- [ ] 結果プレビューエリア
  - [ ] テーブル表示（ページネーション付き）
  - [ ] ヒット件数表示
  - [ ] JSON表示モード切り替え
- [ ] DuckDB保存設定
  - [ ] テーブル名入力
  - [ ] 保存モード選択（追記/上書き）
  - [ ] 保存実行ボタン
- [ ] プログレス表示（大量データ保存時）

#### 7.6 ローカルDB管理画面 🔴
`src-ui/src/pages/Database.tsx`:
- [ ] テーブル一覧表示（サイドバー）
- [ ] テーブルプレビュー（メインエリア）
  - [ ] データテーブル表示
  - [ ] ページネーション
- [ ] SQLエディタ
  - [ ] クエリ入力エリア
  - [ ] 実行ボタン
  - [ ] 結果表示
- [ ] テーブル削除ボタン
- [ ] 抽出履歴表示タブ

#### 7.7 共通コンポーネント 🔴
`src-ui/src/components/`:
- [ ] `Loading` - ローディングスピナー
- [ ] `ErrorToast` - エラー表示トースト（react-hot-toast等）
- [ ] `ConfirmDialog` - 確認ダイアログ
- [ ] `DataTable` - 汎用データテーブル（ページネーション付き）
- [ ] `JsonEditor` - JSONエディタラッパー
- [ ] `StatusBadge` - ステータス表示バッジ

#### 7.8 UIライブラリの追加 🟡
- [ ] `@mui/material` または `shadcn/ui` の導入
- [ ] `react-hook-form` + `zod` (フォーム管理)
- [ ] `@monaco-editor/react` (JSONエディタ)
- [ ] `react-hot-toast` (トースト通知)
- [ ] `@tanstack/react-table` (テーブル)

#### 7.9 スタイリング 🟡
- [ ] ダークモード対応
- [ ] レスポンシブデザイン
- [ ] アニメーション（ページ遷移等）

---

## Feature 8: セキュリティ・品質

### 📦 成果物
安全で信頼性の高いアプリケーション

### ✅ 完了条件
- [ ] 認証情報が安全に保存される
- [ ] エラーハンドリングが適切
- [ ] ログが記録される
- [ ] テストカバレッジが十分

### 📋 タスク

#### 8.1 セキュリティ強化 🟡
- [ ] ファイルパーミッションの設定（profiles.toml を 600 に）
- [ ] メモリ上のパスワードを使用後即座にクリア
- [ ] SSL証明書検証の実装
- [ ] SQLインジェクション対策の確認

#### 8.2 エラーハンドリング 🟡
- [ ] 統一的なエラー型の定義 (`src-tauri/src/errors.rs`)
- [ ] 各レイヤーでの適切なエラー変換
- [ ] ユーザーフレンドリーなエラーメッセージ
- [ ] フロントエンドでのエラー表示改善

#### 8.3 ログ機能 🟡
- [ ] Tracingの設定
- [ ] ログファイル出力（`~/.es_client/logs/`）
- [ ] ログレベルの設定（config.toml）
- [ ] ログローテーション

#### 8.4 ユニットテスト 🟡
- [ ] 暗号化ユーティリティのテスト
- [ ] 設定管理のテスト
- [ ] データ変換ロジックのテスト
- [ ] DuckDBサービスのテスト
- [ ] テストカバレッジ計測

#### 8.5 統合テスト 🟡
- [ ] Testcontainersを使ったElasticsearchテスト
- [ ] E2Eシナリオテスト（接続→抽出→保存）
- [ ] エラーケースのテスト

#### 8.6 フロントエンドテスト 🟢
- [ ] React Testing Libraryでのコンポーネントテスト
- [ ] Tauri E2Eテスト

---

## Feature 9: パッケージング・リリース

### 📦 成果物
各プラットフォーム向けにビルドされた配布可能なアプリケーション

### ✅ 完了条件
- [ ] Windows, macOS, Linuxでビルドできる
- [ ] インストーラーが作成される
- [ ] ドキュメントが整備されている

### 📋 タスク

#### 9.1 ビルド設定 🟡
- [ ] `tauri.conf.json` の設定
  - [ ] アプリ名、バージョン、アイコン
  - [ ] バンドル設定
- [ ] アイコンの作成（各サイズ）
- [ ] ビルドスクリプトの作成

#### 9.2 各プラットフォーム向けビルド 🟡
- [ ] Windows
  - [ ] `.msi` インストーラー
  - [ ] `.exe` ポータブル版
- [ ] macOS
  - [ ] `.dmg` ディスクイメージ
  - [ ] `.app` バンドル
  - [ ] コード署名設定
- [ ] Linux
  - [ ] `.deb` パッケージ
  - [ ] `.AppImage`
  - [ ] `.rpm` パッケージ（オプション）

#### 9.3 CLIのビルド 🟡
- [ ] 各プラットフォーム向けバイナリ
- [ ] クロスコンパイル設定

#### 9.4 ドキュメント整備 🟡
- [ ] README.mdの充実
  - [ ] 機能紹介
  - [ ] インストール方法
  - [ ] クイックスタート
  - [ ] スクリーンショット
- [ ] ユーザーガイド作成
- [ ] CLI ヘルプの整備
- [ ] CHANGELOG.md

#### 9.5 リリース準備 🟢
- [ ] GitHub Releasesの設定
- [ ] CI/CDパイプライン（GitHub Actions）
  - [ ] 自動ビルド
  - [ ] 自動テスト
  - [ ] リリースの自動作成
- [ ] バージョン管理戦略

---

## 📝 実装の推奨順序

実装は以下の順序で進めることを強く推奨します：

1. **Feature 1** - プロジェクトセットアップ（必須）
2. **Feature 2** - 接続管理機能（コア機能）
3. **Feature 4** - データ抽出・保存機能（コア機能）
4. **Feature 5** - ローカルDB管理機能（コア機能）
5. **Feature 3** - インデックス管理機能（便利機能）
6. **Feature 6** - CLI実装（Milestone 1達成）
7. **Feature 7** - GUI実装（Milestone 2達成）
8. **Feature 8** - セキュリティ・品質（Milestone 3達成）
9. **Feature 9** - パッケージング・リリース

---

## 🔄 進捗の更新方法

1. タスクを開始したら `- [🔄]` にマークする
2. タスクを完了したら `- [x]` にマークする
3. 各機能の進捗率を手動で更新する（完了タスク数 / 全タスク数）
4. マイルストーン達成時は上部の表を更新する

---

## 📌 注意事項

- **design.mdとの整合性を保つ**: 設計書が更新された場合、このタスクリストも更新すること
- **小さくリリース**: 各Featureが完成したら動作確認を行い、問題があれば早期に修正
- **テストを忘れずに**: 機能実装と並行してテストを書く習慣をつける
- **ドキュメントも成果物**: コードだけでなく、使い方のドキュメントも整備する

---

## 🎉 実装完了記録

### Feature 1: プロジェクトセットアップ ✅
**完了日:** 2025-11-25

**実装内容:**
- Tauriプロジェクト作成（React + TypeScript）
- Rust依存関係追加（reqwest, duckdb, tokio, serde, toml, ring, hex, base64, chrono, dirs, thiserror, tracing）
- Rust 2024エディション形式のモジュール構成
- 基本的なディレクトリ構造

**コミット:** Initial commit

---

### Feature 2: 接続管理機能 ✅
**完了日:** 2025-11-25

**実装内容:**
- **データモデル:** ProfileConfig, AppConfig, AuthType, ClusterInfo, VersionInfo
- **暗号化ユーティリティ:** AES-256-GCM暗号化、マシン固有キー生成
- **ConfigService:** プロファイル管理（CRUD）、設定ファイル管理（profiles.toml, config.toml）
- **ESClient:** reqwestベースのElasticsearch REST APIクライアント
  - Basic認証とAPIキー認証対応
  - 接続テスト、クラスター情報取得
  - インデックス一覧、作成、削除、存在確認
- **Tauriコマンド:** 13個のコマンドハンドラ実装
  - プロファイル管理: list_profiles, get_profile, save_profile, delete_profile, encrypt_password
  - アプリ設定: load_app_config, save_app_config
  - 接続: test_connection, get_cluster_info
  - インデックス管理: list_indices, create_index, delete_index, index_exists

**テスト:** すべてのユニットテスト成功（5 passed）

**コミット:** feat: Elasticsearch接続管理機能を実装

---

### Feature 3: インデックス管理機能 ✅
**完了日:** 2025-11-25

**実装内容:**
- **SampleIndexConfig:** 3種類のサンプルインデックステンプレート
  - ecommerce_products: EC商品カタログ
  - application_logs: アプリケーションログ
  - user_analytics: ユーザー行動分析
- **sample_data.rs:** サンプルデータ生成モジュール
  - generate_products: 商品データ生成（カテゴリ、価格、在庫情報）
  - generate_logs: ログデータ生成（レベル、メッセージ、サービス情報）
  - generate_analytics: 分析データ生成（ユーザー行動、デバイス、地域情報）
- **ESClient拡張:**
  - bulk_insert: Bulk APIでドキュメント一括挿入
  - search: ドキュメント検索
  - count: ドキュメント件数取得
- **Tauriコマンド追加:**
  - list_sample_index_templates: テンプレート一覧取得
  - create_sample_index: サンプルインデックス作成とデータ投入
  - search_documents: ドキュメント検索
  - count_documents: ドキュメント件数取得

**テスト:** すべてのユニットテスト成功（8 passed）

**コミット:** feat: サンプルインデックス作成機能を実装

---

### Feature 4 & 5: データ抽出・保存機能 & ローカルDB管理機能 ✅
**完了日:** 2025-11-25

**実装内容:**
- **ExtractionJob:** データ抽出履歴モデル
- **DuckDBService:** ローカルデータベースサービス
  - init_tables: extraction_historyテーブル初期化
  - save_extraction_job: 抽出ジョブ履歴保存
  - get_extraction_history: 抽出履歴取得（最新100件）
  - create_data_table: 抽出データ用テーブル自動生成（スキーマ推測）
  - insert_data: ドキュメントデータ挿入（型変換対応）
  - query_table: テーブルデータクエリ
  - list_tables: テーブル一覧取得
- **Tauriコマンド追加:**
  - extract_and_store_data: Elasticsearchから検索してDuckDBに保存
  - get_extraction_history: 抽出履歴取得
  - query_extracted_data: 抽出済みデータクエリ
  - list_duckdb_tables: DuckDBテーブル一覧
- **AppState:** DuckDBService追加

**テスト:** すべてのユニットテスト成功

**コミット:** feat: データ抽出・保存機能を実装

---

### Feature 6: CLI実装 ✅
**完了日:** 2025-11-25

**実装内容:**
- **Cargoワークスペース:** ルートレベルでワークスペース設定、src-tauriとcliの共有依存関係管理
- **CLIバイナリプロジェクト:** cli/ディレクトリにバイナリプロジェクト作成
- **clapフレームワーク:** derive APIを使用したCLI実装
- **colored出力:** カラフルで見やすいターミナル出力
- **実装コマンド:**
  - `connect`: 接続設定（Basic認証/APIキー認証、SSL設定）
  - `profile list/show/delete`: プロファイル管理
  - `index list/create/info`: インデックス管理
  - `extract`: データ抽出とDuckDB保存
  - `db list/query/show/drop`: ローカルDB管理
- **ライブラリ再利用:** src-tauriのmodels/services/utilsモジュールを公開して再利用
- **包括的なREADME:**
  - ビルド・インストール手順
  - 10ステップの詳細な動作確認手順（期待される出力付き）
  - 全コマンドの使用例とリファレンス
  - 実用的なワークフロー例
  - トラブルシューティングガイド

**ファイル:**
- `/Cargo.toml`: ワークスペース設定
- `/cli/Cargo.toml`: CLIプロジェクト設定（依存関係修正済み）
- `/cli/src/main.rs`: CLI実装（約600行、ビルドエラー修正済み）
- `/cli/README.md`: CLI包括的使用ガイド（約400行）

**ビルド・テスト結果:**
- ✅ `cargo check --workspace`: 成功
- ✅ `cargo build --release -p es-client`: 成功
- ✅ `./target/release/es-client --help`: 動作確認済み
- ✅ 全コマンドのヘルプ表示確認

**コミット:**
1. feat: CLI実装を完成 & Milestone 1達成
2. fix: CLIのビルドエラーを修正
3. docs: CLIの包括的なREADMEを追加

---

### 📊 現在の状態

**実装済み機能:**
- ✅ Elasticsearchへの接続・認証
- ✅ プロファイル管理（暗号化保存）
- ✅ インデックス管理（一覧、作成、削除）
- ✅ サンプルデータ生成・投入
- ✅ データ検索・抽出
- ✅ DuckDBへのデータ保存
- ✅ 抽出履歴管理
- ✅ ローカルDBクエリ
- ✅ CLI実装（全コマンド）

**🎉 Milestone 1達成！**
- ✅ Elasticsearchに接続できる
- ✅ シンプルなクエリでデータを抽出できる
- ✅ 抽出したデータをDuckDBに保存できる
- ✅ 保存したデータをSQLで確認できる
- ✅ CLIから全機能を利用可能

**次のステップ:**
1. ✅ Milestone 1達成（動作するCLIプロトタイプ）
2. GUI実装（Feature 7）でMilestone 2達成
3. セキュリティ・品質強化（Feature 8）
4. パッケージング・リリース（Feature 9）でMilestone 3達成
