import { invoke } from '@tauri-apps/api/core';
import type {
  ProfileConfig,
  AppConfig,
  ClusterInfo,
  SampleIndexConfig,
  ExtractionJob,
  SearchQuery,
} from '../types';

// Profile Management
export const listProfiles = async (): Promise<ProfileConfig[]> => {
  return invoke('list_profiles');
};

export const getProfile = async (name: string): Promise<ProfileConfig> => {
  return invoke('get_profile', { name });
};

export const saveProfile = async (profile: ProfileConfig): Promise<void> => {
  return invoke('save_profile', { profile });
};

export const deleteProfile = async (name: string): Promise<void> => {
  return invoke('delete_profile', { name });
};

export const encryptPassword = async (password: string): Promise<string> => {
  return invoke('encrypt_password', { password });
};

// App Configuration
export const loadAppConfig = async (): Promise<AppConfig> => {
  return invoke('load_app_config');
};

export const saveAppConfig = async (config: AppConfig): Promise<void> => {
  return invoke('save_app_config', { config });
};

// Connection
export const testConnection = async (profileName: string): Promise<boolean> => {
  return invoke('test_connection', { profileName });
};

export const getClusterInfo = async (profileName: string): Promise<ClusterInfo> => {
  return invoke('get_cluster_info', { profileName });
};

// Index Management
export const listIndices = async (profileName: string): Promise<string[]> => {
  return invoke('list_indices', { profileName });
};

export const createIndex = async (
  profileName: string,
  indexName: string,
  settings?: Record<string, any>,
  mappings?: Record<string, any>
): Promise<void> => {
  return invoke('create_index', {
    profileName,
    indexName,
    settings,
    mappings,
  });
};

export const deleteIndex = async (
  profileName: string,
  indexName: string
): Promise<void> => {
  return invoke('delete_index', { profileName, indexName });
};

export const indexExists = async (
  profileName: string,
  indexName: string
): Promise<boolean> => {
  return invoke('index_exists', { profileName, indexName });
};

// Sample Index Management
export const listSampleIndexTemplates = async (): Promise<SampleIndexConfig[]> => {
  return invoke('list_sample_index_templates');
};

export const createSampleIndex = async (
  profileName: string,
  templateName: string,
  documentCount: number
): Promise<string> => {
  return invoke('create_sample_index', {
    profileName,
    templateName,
    documentCount,
  });
};

export const searchDocuments = async (
  profileName: string,
  indexName: string,
  query: SearchQuery
): Promise<any> => {
  return invoke('search_documents', { profileName, indexName, query });
};

export const countDocuments = async (
  profileName: string,
  indexName: string,
  query: SearchQuery
): Promise<number> => {
  return invoke('count_documents', { profileName, indexName, query });
};

// Data Extraction
export const extractAndStoreData = async (
  profileName: string,
  indexName: string,
  query: SearchQuery,
  tableName: string
): Promise<string> => {
  return invoke('extract_and_store_data', {
    profileName,
    indexName,
    query,
    tableName,
  });
};

export const getExtractionHistory = async (): Promise<ExtractionJob[]> => {
  return invoke('get_extraction_history');
};

export const queryExtractedData = async (
  tableName: string,
  limit?: number
): Promise<any[]> => {
  return invoke('query_extracted_data', { tableName, limit });
};

export const listDuckdbTables = async (): Promise<string[]> => {
  return invoke('list_duckdb_tables');
};

// Local Database
export const queryLocal = async (sql: string): Promise<any[]> => {
  return invoke('query_local', { sql });
};

export const listTables = async (): Promise<string[]> => {
  return invoke('list_tables');
};

export const exportToParquet = async (
  tableName: string,
  outputPath: string
): Promise<string> => {
  return invoke('export_to_parquet', { tableName, outputPath });
};
