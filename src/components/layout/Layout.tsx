import { ReactNode, useEffect } from 'react';
import { Toaster } from 'react-hot-toast';
import { Header } from './Header';
import { Sidebar } from './Sidebar';
import { useAppStore } from '../../store/appStore';
import { useProfileStore } from '../../store/profileStore';

interface LayoutProps {
  children: ReactNode;
}

export const Layout = ({ children }: LayoutProps) => {
  const { theme, loadConfig } = useAppStore();
  const { loadProfiles } = useProfileStore();

  useEffect(() => {
    // Initialize app
    loadConfig();
    loadProfiles();

    // Set initial theme
    if (theme === 'dark') {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }, []);

  return (
    <div className="flex flex-col h-screen bg-gray-50 dark:bg-gray-900">
      <Header />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <main className="flex-1 overflow-auto p-6">
          {children}
        </main>
      </div>
      <Toaster
        position="bottom-right"
        toastOptions={{
          className: 'dark:bg-gray-800 dark:text-white',
          duration: 4000,
        }}
      />
    </div>
  );
};
