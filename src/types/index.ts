// Type definitions matching Rust backend models

export type AuthType = 'basic' | 'apikey';

export interface ProfileConfig {
  name: string;
  host: string;
  username?: string;
  password_encrypted?: string;
  api_key_encrypted?: string;
  auth_type: AuthType;
  use_ssl: boolean;
  verify_certificate: boolean;
  created_at: number;
  updated_at: number;
}

export interface AppConfig {
  default_profile?: string;
  log_level: string;
  database_path: string;
  max_memory: string;
  theme: string;
  recent_profiles: string[];
}

export interface ClusterInfo {
  name: string;
  cluster_name: string;
  cluster_uuid: string;
  version: VersionInfo;
}

export interface VersionInfo {
  number: string;
  build_flavor: string;
  build_type: string;
}

export interface SampleIndexConfig {
  name: string;
  description: string;
  index_name: string;
  mappings: Record<string, any>;
  settings: Record<string, any>;
}

export interface ExtractionJob {
  id: number;
  profile_name: string;
  index_name: string;
  query: string;
  table_name: string;
  record_count: number;
  created_at: string;
  status: string;
}

export interface SearchQuery {
  query?: Record<string, any>;
  size?: number;
  from?: number;
  sort?: Array<Record<string, any>>;
}
