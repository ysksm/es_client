# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ES Client is an Elasticsearch index management and data extraction desktop application built with **Tauri 2 + React 19 + Rust**. It provides both a GUI and CLI for managing Elasticsearch clusters, extracting data to DuckDB, and performing local SQL operations.

## Build & Development Commands

### Full Application (Recommended)
```bash
npm install              # Install dependencies
npm run tauri dev        # Run Tauri app in development mode
npm run tauri build      # Build production app
```

### Frontend Only
```bash
npm run dev              # Vite dev server (browser only, port 1420)
npx tsc --noEmit         # TypeScript type checking
```

### Backend (Rust)
```bash
cargo check --workspace                           # Check all crates
cargo build -p es-client                          # Build CLI (debug)
cargo build --release -p es-client                # Build CLI (release)
cargo test --manifest-path=src-tauri/Cargo.toml   # Run backend tests
cargo run --manifest-path=cli/Cargo.toml          # Run CLI directly
```

### CLI Installation
```bash
cargo install --path cli    # Install CLI system-wide as 'es-client'
```

## Architecture

```
┌─────────────────────────────────────┐
│   React Frontend (TypeScript)       │
│   src/pages/ → src/api/tauri.ts     │
└────────────────┬────────────────────┘
                 │ IPC (Tauri invoke)
┌────────────────▼────────────────────┐
│   Tauri Backend (Rust)              │
│   src-tauri/src/commands.rs         │
│   └── Services:                     │
│       ConfigService, ESClient,      │
│       DuckDBService                 │
└────────────────┬────────────────────┘
    ┌────────────┴───────────┐
    ▼                        ▼
 Elasticsearch            DuckDB
 (HTTP/REST)            (~/.es_client/data.duckdb)
```

### Key Backend Files (src-tauri/src/)
- `commands.rs` - Tauri IPC command handlers (entry points from frontend)
- `services.rs` - Business logic: ConfigService, ESClient, DuckDBService
- `models.rs` - Data structures: ProfileConfig, AuthType, ClusterInfo, etc.
- `utils.rs` - AES-256-GCM encryption for credentials

### Key Frontend Files (src/)
- `api/tauri.ts` - Wrapper for Tauri IPC calls
- `pages/` - Dashboard, Connections, Indices, Extract, Database
- `store/` - Zustand stores (appStore, profileStore, indexStore)

### CLI (cli/src/main.rs)
Uses the same backend services as Tauri GUI. Subcommands: `connect`, `profile`, `index`, `extract`, `db`.

## Data Storage

All data stored in `~/.es_client/`:
- `profiles.toml` - Connection profiles (credentials encrypted)
- `config.toml` - App configuration
- `data.duckdb` - Local DuckDB database
- `.key` - Encryption key (mode 600)

## Tech Stack

- **Frontend**: React 19, TypeScript, Tailwind CSS, Zustand, React Router v7, Vite
- **Backend**: Rust 2024 edition, Tauri 2, reqwest (rustls-tls), tokio, duckdb, ring (crypto)
- **Requirements**: Node.js 18+, Rust 1.70+, macOS requires Xcode CLI tools

## Workspace Structure

This is a Cargo workspace with shared dependencies defined in root `Cargo.toml`:
- `src-tauri/` - Tauri desktop app crate
- `cli/` - CLI tool crate (package name: `es-client`)

Both crates share the same service layer for consistency.
