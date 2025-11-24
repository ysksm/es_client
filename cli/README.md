# ES Client CLI

ElasticsearchインデックスマネジメントのためのCLIツール

## 目次

- [ビルド](#ビルド)
- [インストール](#インストール)
- [動作確認](#動作確認)
- [使い方](#使い方)
- [コマンドリファレンス](#コマンドリファレンス)
- [設定ファイル](#設定ファイル)

## ビルド

```bash
# プロジェクトルートから
cd /path/to/es_client

# ワークスペース全体をチェック
cargo check --workspace

# CLIをビルド（デバッグ）
cargo build -p es-client

# CLIをビルド（リリース・最適化）
cargo build --release -p es-client
```

**ビルド成果物:**
- デバッグビルド: `./target/debug/es-client`
- リリースビルド: `./target/release/es-client`

## インストール

システムにインストールして、どこからでも `es-client` コマンドを使えるようにします：

```bash
cargo install --path cli
```

インストール後：
```bash
es-client --help
```

## 動作確認

### 前提条件

Elasticsearchが起動している必要があります。以下のいずれかの方法で起動してください：

```bash
# Dockerで起動する場合
docker run -d \
  --name elasticsearch \
  -p 9200:9200 \
  -e "discovery.type=single-node" \
  -e "xpack.security.enabled=false" \
  docker.elastic.co/elasticsearch/elasticsearch:8.11.0

# または既存のElasticsearchインスタンスを使用
```

### 基本動作確認

#### 1. ヘルプ表示の確認

```bash
./target/release/es-client --help
```

**期待される出力:**
```
Elasticsearch Index Management CLI

Usage: es-client <COMMAND>

Commands:
  profile  Manage connection profiles
  connect  Connect to Elasticsearch and create/update a profile
  index    Manage Elasticsearch indices
  extract  Extract data from Elasticsearch to DuckDB
  db       Manage local DuckDB database
  help     Print this message or the help of the given subcommand(s)
```

#### 2. 接続プロファイル作成

```bash
./target/release/es-client connect \
  --name local \
  --host http://localhost:9200 \
  --user elastic \
  --password changeme
```

**期待される動作:**
- Elasticsearchへの接続テスト実行
- 接続成功メッセージとクラスタ情報表示
- プロファイルが `~/.es_client/profiles.toml` に保存

**出力例:**
```
Testing connection to http://localhost:9200... ✓ Connected
  Cluster: elasticsearch (8.11.0)
✓ Profile 'local' saved successfully.
```

#### 3. プロファイル一覧確認

```bash
./target/release/es-client profile list
```

**期待される出力:**
```
Connection Profiles:

  • local
    Host:      http://localhost:9200
    Auth:      Basic
    SSL:       enabled
    Created:   2025-11-25 10:30:00
```

#### 4. インデックス一覧取得

```bash
./target/release/es-client index list --profile local
```

**期待される動作:**
- Elasticsearchからインデックス一覧を取得
- インデックス名を一覧表示

#### 5. テストインデックス作成

```bash
./target/release/es-client index create \
  --profile local \
  --name test-index \
  --shards 1 \
  --replicas 0
```

**期待される出力:**
```
Creating index 'test-index'... ✓ Created
```

#### 6. インデックス情報確認

```bash
./target/release/es-client index info \
  --profile local \
  --name test-index
```

**期待される出力:**
```
Index: test-index
  Status: exists
  Documents: 0
```

#### 7. データ抽出テスト（インデックスにデータがある場合）

```bash
./target/release/es-client extract \
  --profile local \
  --index test-index \
  --query '{"match_all":{}}' \
  --output test_table
```

**期待される動作:**
- Elasticsearchからデータを検索
- DuckDBにテーブル作成とデータ挿入
- 挿入レコード数を表示

#### 8. DuckDBテーブル一覧

```bash
./target/release/es-client db list
```

**期待される出力:**
```
DuckDB Tables (1)

  • test_table
```

#### 9. DuckDBデータ確認

```bash
./target/release/es-client db show test_table --limit 5
```

**期待される動作:**
- テーブルから最大5行を取得
- JSON形式でデータを表示

#### 10. SQLクエリ実行

```bash
./target/release/es-client db query \
  --sql "SELECT COUNT(*) as count FROM test_table"
```

**期待される動作:**
- SQLクエリを実行
- 結果をJSON形式で表示

### エラーケースの確認

#### 接続失敗時

```bash
# 存在しないホストに接続
./target/release/es-client connect \
  --name invalid \
  --host http://localhost:9999 \
  --user elastic \
  --password changeme
```

**期待される動作:**
- 接続エラーメッセージ表示
- プロファイルは保存されない

#### 存在しないプロファイル

```bash
./target/release/es-client index list --profile nonexistent
```

**期待される動作:**
- エラーメッセージ表示：プロファイルが見つからない

## 使い方

### 接続プロファイルの作成

#### Basic認証

```bash
es-client connect \
  --name local \
  --host https://localhost:9200 \
  --user elastic \
  --password changeme
```

#### API Key認証

```bash
es-client connect \
  --name prod \
  --host https://prod.example.com \
  --api-key "your-api-key"
```

#### SSL検証を無効化（開発環境用）

```bash
es-client connect \
  --name dev \
  --host https://localhost:9200 \
  --user elastic \
  --password changeme \
  --insecure
```

### プロファイル管理

```bash
# プロファイル一覧
es-client profile list

# プロファイル詳細表示
es-client profile show local

# プロファイル削除
es-client profile delete old-profile
```

### インデックス管理

```bash
# インデックス一覧
es-client index list --profile local

# インデックス作成
es-client index create \
  --profile local \
  --name my-index \
  --shards 1 \
  --replicas 0

# インデックス情報
es-client index info --profile local --name my-index
```

### データ抽出

```bash
# インラインクエリ
es-client extract \
  --profile local \
  --index logs-* \
  --query '{"match_all":{}}' \
  --output logs_table

# クエリファイルを使用
es-client extract \
  --profile local \
  --index logs-* \
  --query query.json \
  --output logs_table
```

**query.json の例:**
```json
{
  "query": {
    "range": {
      "timestamp": {
        "gte": "2024-01-01",
        "lte": "2024-12-31"
      }
    }
  },
  "size": 1000
}
```

### ローカルDBクエリ

```bash
# テーブル一覧
es-client db list

# テーブルデータプレビュー
es-client db show logs_table --limit 10

# SQLクエリ実行
es-client db query --sql "SELECT COUNT(*) FROM logs_table"

es-client db query --sql "SELECT * FROM logs_table WHERE level='ERROR' LIMIT 5"

# テーブル削除
es-client db drop logs_table
es-client db drop logs_table --yes  # 確認なし
```

## コマンドリファレンス

### `es-client connect`

Elasticsearchに接続し、プロファイルを作成します。

**オプション:**
- `--name, -n <NAME>`: プロファイル名（必須）
- `--host, -H <HOST>`: ElasticsearchホストURL（必須）
- `--user, -u <USER>`: Basic認証のユーザー名
- `--password, -p <PASSWORD>`: Basic認証のパスワード
- `--api-key, -a <API_KEY>`: APIキー認証のキー
- `--insecure`: SSL証明書検証を無効化

### `es-client profile`

プロファイルを管理します。

**サブコマンド:**
- `list`: すべてのプロファイルを一覧表示
- `show <NAME>`: プロファイルの詳細を表示
- `delete <NAME>`: プロファイルを削除

### `es-client index`

Elasticsearchインデックスを管理します。

**サブコマンド:**
- `list --profile <PROFILE>`: インデックス一覧
- `create --profile <PROFILE> --name <NAME> [--shards N] [--replicas N]`: インデックス作成
- `info --profile <PROFILE> --name <NAME>`: インデックス情報

### `es-client extract`

Elasticsearchからデータを抽出し、DuckDBに保存します。

**オプション:**
- `--profile <PROFILE>`: プロファイル名（必須）
- `--index, -i <INDEX>`: インデックス名またはパターン（必須）
- `--query, -q <QUERY>`: クエリJSON文字列またはファイルパス（必須）
- `--output, -o <OUTPUT>`: DuckDBテーブル名（必須）

### `es-client db`

ローカルDuckDBデータベースを管理します。

**サブコマンド:**
- `list`: テーブル一覧
- `query --sql <SQL>`: SQLクエリ実行
- `show <TABLE> [--limit N]`: テーブルデータプレビュー
- `drop <TABLE> [--yes]`: テーブル削除

## ワークフロー例

### 基本的なデータ分析ワークフロー

```bash
# 1. Elasticsearchに接続
es-client connect \
  --name local \
  --host http://localhost:9200 \
  --user elastic \
  --password changeme

# 2. インデックス一覧を確認
es-client index list --profile local

# 3. 2024年のログデータを抽出
es-client extract \
  --profile local \
  --index logs-2024-* \
  --query '{"match_all":{}}' \
  --output logs_table

# 4. DuckDBでデータ分析
es-client db query \
  --sql "SELECT level, COUNT(*) as count FROM logs_table GROUP BY level"

# 5. エラーログのみ抽出
es-client extract \
  --profile local \
  --index logs-2024-* \
  --query '{"query":{"term":{"level":"ERROR"}}}' \
  --output error_logs

# 6. エラーログを確認
es-client db show error_logs --limit 20

# 7. 時系列分析
es-client db query \
  --sql "SELECT DATE_TRUNC('hour', timestamp) as hour, COUNT(*) as count FROM logs_table GROUP BY hour ORDER BY hour"
```

### 複数環境の管理

```bash
# 開発環境
es-client connect --name dev --host http://dev.example.com --user elastic --password dev_pass

# ステージング環境
es-client connect --name staging --host http://staging.example.com --user elastic --password staging_pass

# 本番環境
es-client connect --name prod --host https://prod.example.com --api-key "prod_api_key"

# 環境ごとにデータ抽出
es-client extract --profile dev --index logs-* --query '{"match_all":{}}' --output dev_logs
es-client extract --profile staging --index logs-* --query '{"match_all":{}}' --output staging_logs
es-client extract --profile prod --index logs-* --query '{"match_all":{}}' --output prod_logs

# 環境間でデータ比較
es-client db query --sql "SELECT 'dev' as env, COUNT(*) FROM dev_logs UNION SELECT 'staging', COUNT(*) FROM staging_logs UNION SELECT 'prod', COUNT(*) FROM prod_logs"
```

## 設定ファイル

CLIは `~/.es_client/` ディレクトリに設定とデータを保存します：

- **プロファイル**: `~/.es_client/profiles.toml`
- **アプリ設定**: `~/.es_client/config.toml`
- **DuckDBデータ**: `~/.es_client/data.duckdb`
- **暗号化キー**: `~/.es_client/.key` (パーミッション 600)

### profiles.toml の例

```toml
[[profiles]]
name = "local"
host = "http://localhost:9200"
username = "elastic"
password_encrypted = "encrypted_hex_string"
auth_type = "basic"
use_ssl = true
verify_certificate = true
created_at = 1732550400
updated_at = 1732550400
```

## トラブルシューティング

### 接続エラー

```bash
Error: Connection refused
```

**解決方法:**
1. Elasticsearchが起動しているか確認
2. ホストURLが正しいか確認
3. ファイアウォール設定を確認

### SSL証明書エラー

```bash
Error: SSL certificate verification failed
```

**解決方法:**
1. `--insecure` フラグを使用（開発環境のみ）
2. 正しい証明書を設定

### プロファイルが見つからない

```bash
Error: Profile 'xxx' not found
```

**解決方法:**
1. `es-client profile list` でプロファイル名を確認
2. `es-client connect` で新しいプロファイルを作成

## ヘルプ

各コマンドの詳細は `--help` オプションで確認できます：

```bash
es-client --help
es-client connect --help
es-client extract --help
es-client db query --help
```

## ライセンス

MIT
