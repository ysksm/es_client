# ES Client

Elasticsearchのインデックス管理とデータ抽出を行うデスクトップアプリケーション

## 概要

ES Clientは、Elasticsearchからデータを抽出し、ローカルのDuckDBに保存・管理するためのGUIツールです。Tauri + React + TypeScriptで構築されています。

## 主な機能

- 🔌 **接続管理**: 複数のElasticsearchプロファイルを管理
- 📊 **ダッシュボード**: クラスタ情報と接続状況の表示
- 📑 **インデックス管理**: インデックスの一覧表示、検索、削除
- 📥 **データ抽出**: Elasticsearchからデータを抽出してDuckDBに保存
- 💾 **ローカルDB管理**: DuckDBへのSQLクエリ実行とParquetエクスポート
- 🌙 **ダークモード**: ライト/ダークテーマの切り替え

## 技術スタック

- **フロントエンド**: React 19, TypeScript, Tailwind CSS
- **状態管理**: Zustand
- **ルーティング**: React Router v7
- **UI**: Headless UI, Heroicons
- **バックエンド**: Rust, Tauri 2
- **データベース**: DuckDB
- **ビルドツール**: Vite

## 必要な環境

- Node.js 18以上
- Rust 1.70以上
- macOS: Xcode Command Line Tools

## インストール

```bash
# リポジトリのクローン
git clone <repository-url>
cd es_client

# 依存関係のインストール
npm install
```

## 実行方法

### 開発モード

```bash
# フロントエンドのみ（ブラウザ確認用）
npm run dev

# 完全なTauriアプリケーション（推奨）
npm run tauri dev
```

### プロダクションビルド

```bash
npm run tauri build
```

ビルドされたアプリケーションは `src-tauri/target/release/bundle/` に出力されます。

## 使い方

1. **接続設定**
   - Connectionsページで接続プロファイルを作成
   - プロファイルを選択して接続

2. **インデックス管理**
   - Indicesページでインデックス一覧を確認
   - インデックスの検索、選択、削除が可能

3. **データ抽出**
   - Extractページでインデックスを選択
   - 検索クエリを設定（JSON形式）
   - DuckDBテーブル名を指定して抽出実行

4. **ローカルDB管理**
   - Databaseページでテーブル一覧を確認
   - SQLクエリを実行してデータを確認
   - Parquet形式でエクスポート

## プロジェクト構成

```
es_client/
├── src/
│   ├── api/           # Tauri APIラッパー
│   ├── components/    # Reactコンポーネント
│   │   ├── ui/        # 共通UIコンポーネント
│   │   └── layout/    # レイアウトコンポーネント
│   ├── pages/         # ページコンポーネント
│   ├── store/         # Zustand状態管理
│   ├── types/         # TypeScript型定義
│   └── App.tsx        # アプリケーションルート
├── src-tauri/         # Rustバックエンド
│   └── src/
│       ├── config/    # 設定管理
│       ├── es/        # Elasticsearch連携
│       ├── db/        # DuckDB管理
│       └── main.rs    # エントリーポイント
└── package.json
```

## 開発

### フロントエンド開発

```bash
# 開発サーバー起動
npm run dev

# TypeScriptの型チェック
npx tsc --noEmit
```

### バックエンド開発

```bash
# Rustコードのチェック
cargo check --manifest-path=src-tauri/Cargo.toml

# テスト実行
cargo test --manifest-path=src-tauri/Cargo.toml
```

## ライセンス

MIT

## 作者

Built with [Tauri](https://tauri.app/) + [React](https://react.dev/)
