import { create } from 'zustand';
import type { ProfileConfig, ClusterInfo } from '../types';
import * as api from '../api/tauri';

interface ProfileStore {
  profiles: ProfileConfig[];
  currentProfile: ProfileConfig | null;
  clusterInfo: ClusterInfo | null;
  isLoading: boolean;
  error: string | null;

  // Actions
  loadProfiles: () => Promise<void>;
  selectProfile: (name: string) => Promise<void>;
  createProfile: (profile: ProfileConfig) => Promise<void>;
  updateProfile: (profile: ProfileConfig) => Promise<void>;
  deleteProfile: (name: string) => Promise<void>;
  testConnection: (profileName: string) => Promise<boolean>;
  loadClusterInfo: (profileName: string) => Promise<void>;
  setError: (error: string | null) => void;
}

export const useProfileStore = create<ProfileStore>((set, get) => ({
  profiles: [],
  currentProfile: null,
  clusterInfo: null,
  isLoading: false,
  error: null,

  loadProfiles: async () => {
    set({ isLoading: true, error: null });
    try {
      const profiles = await api.listProfiles();
      set({ profiles, isLoading: false });
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : 'Failed to load profiles',
        isLoading: false
      });
    }
  },

  selectProfile: async (name: string) => {
    set({ isLoading: true, error: null });
    try {
      const profile = await api.getProfile(name);
      set({ currentProfile: profile, isLoading: false });

      // Automatically load cluster info
      await get().loadClusterInfo(name);
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : 'Failed to select profile',
        isLoading: false
      });
    }
  },

  createProfile: async (profile: ProfileConfig) => {
    set({ isLoading: true, error: null });
    try {
      await api.saveProfile(profile);
      await get().loadProfiles();
      set({ isLoading: false });
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : 'Failed to create profile',
        isLoading: false
      });
    }
  },

  updateProfile: async (profile: ProfileConfig) => {
    set({ isLoading: true, error: null });
    try {
      await api.saveProfile(profile);
      await get().loadProfiles();
      if (get().currentProfile?.name === profile.name) {
        set({ currentProfile: profile });
      }
      set({ isLoading: false });
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : 'Failed to update profile',
        isLoading: false
      });
    }
  },

  deleteProfile: async (name: string) => {
    set({ isLoading: true, error: null });
    try {
      await api.deleteProfile(name);
      await get().loadProfiles();
      if (get().currentProfile?.name === name) {
        set({ currentProfile: null, clusterInfo: null });
      }
      set({ isLoading: false });
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : 'Failed to delete profile',
        isLoading: false
      });
    }
  },

  testConnection: async (profileName: string) => {
    set({ isLoading: true, error: null });
    try {
      const result = await api.testConnection(profileName);
      set({ isLoading: false });
      return result;
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : 'Connection test failed',
        isLoading: false
      });
      return false;
    }
  },

  loadClusterInfo: async (profileName: string) => {
    try {
      const clusterInfo = await api.getClusterInfo(profileName);
      set({ clusterInfo });
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : 'Failed to load cluster info',
        clusterInfo: null
      });
    }
  },

  setError: (error) => set({ error }),
}));
