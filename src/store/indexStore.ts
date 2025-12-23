import { create } from 'zustand';
import type { SearchQuery } from '../types';
import * as api from '../api/tauri';

interface IndexStore {
  indices: string[];
  selectedIndex: string | null;
  documentCount: number | null;
  searchResults: any[];
  isLoading: boolean;
  error: string | null;

  // Actions
  loadIndices: (profileName: string) => Promise<void>;
  selectIndex: (indexName: string) => void;
  createIndex: (
    profileName: string,
    indexName: string,
    settings?: Record<string, any>,
    mappings?: Record<string, any>
  ) => Promise<void>;
  deleteIndex: (profileName: string, indexName: string) => Promise<void>;
  searchDocuments: (
    profileName: string,
    indexName: string,
    query: SearchQuery
  ) => Promise<void>;
  countDocuments: (
    profileName: string,
    indexName: string
  ) => Promise<void>;
  setError: (error: string | null) => void;
}

export const useIndexStore = create<IndexStore>((set, get) => ({
  indices: [],
  selectedIndex: null,
  documentCount: null,
  searchResults: [],
  isLoading: false,
  error: null,

  loadIndices: async (profileName: string) => {
    set({ isLoading: true, error: null });
    try {
      const indices = await api.listIndices(profileName);
      set({ indices, isLoading: false });
    } catch (error) {
      const errorMessage = typeof error === 'string' ? error : (error instanceof Error ? error.message : 'Failed to load indices');
      set({
        error: errorMessage,
        isLoading: false
      });
    }
  },

  selectIndex: (indexName: string) => {
    set({ selectedIndex: indexName, searchResults: [], documentCount: null });
  },

  createIndex: async (profileName, indexName, settings, mappings) => {
    set({ isLoading: true, error: null });
    try {
      await api.createIndex(profileName, indexName, settings, mappings);
      await get().loadIndices(profileName);
      set({ isLoading: false });
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : 'Failed to create index',
        isLoading: false
      });
    }
  },

  deleteIndex: async (profileName, indexName) => {
    set({ isLoading: true, error: null });
    try {
      await api.deleteIndex(profileName, indexName);
      await get().loadIndices(profileName);
      if (get().selectedIndex === indexName) {
        set({ selectedIndex: null, searchResults: [], documentCount: null });
      }
      set({ isLoading: false });
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : 'Failed to delete index',
        isLoading: false
      });
    }
  },

  searchDocuments: async (profileName, indexName, query) => {
    set({ isLoading: true, error: null });
    try {
      const response = await api.searchDocuments(profileName, indexName, query);
      const results = response.hits?.hits || [];
      set({ searchResults: results, isLoading: false });
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : 'Failed to search documents',
        isLoading: false
      });
    }
  },

  countDocuments: async (profileName, indexName) => {
    try {
      const count = await api.countDocuments(profileName, indexName);
      set({ documentCount: count });
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : 'Failed to count documents',
      });
    }
  },

  setError: (error) => set({ error }),
}));
