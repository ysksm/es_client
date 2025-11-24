import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { AppConfig } from '../types';
import * as api from '../api/tauri';

interface AppStore {
  config: AppConfig | null;
  theme: 'light' | 'dark';
  isLoading: boolean;
  error: string | null;

  // Actions
  loadConfig: () => Promise<void>;
  saveConfig: (config: AppConfig) => Promise<void>;
  setTheme: (theme: 'light' | 'dark') => void;
  toggleTheme: () => void;
  setError: (error: string | null) => void;
}

export const useAppStore = create<AppStore>()(
  persist(
    (set, get) => ({
      config: null,
      theme: 'dark',
      isLoading: false,
      error: null,

      loadConfig: async () => {
        set({ isLoading: true, error: null });
        try {
          const config = await api.loadAppConfig();
          set({ config, isLoading: false });
        } catch (error) {
          set({
            error: error instanceof Error ? error.message : 'Failed to load config',
            isLoading: false
          });
        }
      },

      saveConfig: async (config: AppConfig) => {
        set({ isLoading: true, error: null });
        try {
          await api.saveAppConfig(config);
          set({ config, isLoading: false });
        } catch (error) {
          set({
            error: error instanceof Error ? error.message : 'Failed to save config',
            isLoading: false
          });
        }
      },

      setTheme: (theme) => {
        set({ theme });
        // Update DOM
        if (theme === 'dark') {
          document.documentElement.classList.add('dark');
        } else {
          document.documentElement.classList.remove('dark');
        }
      },

      toggleTheme: () => {
        const newTheme = get().theme === 'light' ? 'dark' : 'light';
        get().setTheme(newTheme);
      },

      setError: (error) => set({ error }),
    }),
    {
      name: 'app-storage',
      partialize: (state) => ({ theme: state.theme }),
    }
  )
);
