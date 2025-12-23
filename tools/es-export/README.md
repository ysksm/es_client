# ES Export

Elasticsearchからデータを取得してJSONとExcel形式でエクスポートする簡易CLIツール

## ビルド

```bash
cd tools/es-export
cargo build --release
```

ビルド成果物: `target/release/es-export`

## 使い方

### 1. 設定ファイルの作成

`config.example.toml` をコピーして `config.toml` を作成:

```bash
cp config.example.toml config.toml
```

設定ファイルを編集:

```toml
[elasticsearch]
host = "localhost"
port = 9200
username = "elastic"
password = "your_password"
use_ssl = false
verify_certificate = true

[query]
index = "my-index"
query = { match_all = {} }
size = 1000
```

### 2. 実行

```bash
# デフォルト設定 (config.toml, 両形式で出力)
./target/release/es-export

# 設定ファイルを指定
./target/release/es-export -c my-config.toml

# 出力形式を指定
./target/release/es-export -f json      # JSONのみ
./target/release/es-export -f excel     # Excelのみ
./target/release/es-export -f both      # 両方 (デフォルト)

# 出力ファイル名を指定
./target/release/es-export -o my-data
# -> my-data.json, my-data.xlsx
```

## コマンドオプション

| オプション | 短縮 | デフォルト | 説明 |
|-----------|------|----------|------|
| `--config` | `-c` | `config.toml` | 設定ファイルのパス |
| `--format` | `-f` | `both` | 出力形式 (json/excel/both) |
| `--output` | `-o` | `output` | 出力ファイル名 (拡張子なし) |

## クエリ例

### 全件取得
```toml
[query]
index = "logs-*"
query = { match_all = {} }
size = 10000
```

### 条件指定
```toml
[query]
index = "users"
query = { bool = { must = [{ match = { status = "active" } }] } }
size = 500
```

### 日付範囲
```toml
[query]
index = "events"
query = { range = { "@timestamp" = { gte = "2024-01-01", lte = "2024-12-31" } } }
size = 5000
```

## 出力形式

### JSON
- UTF-8エンコード
- Pretty print (整形済み)
- 配列形式で全ドキュメントを格納

### Excel
- xlsx形式
- 1行目: ヘッダー (フィールド名)
- 2行目以降: データ
- ネストしたオブジェクト/配列はJSON文字列として出力
