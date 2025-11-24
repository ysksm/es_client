import { MoonIcon, SunIcon } from '@heroicons/react/24/outline';
import { useAppStore } from '../../store/appStore';
import { useProfileStore } from '../../store/profileStore';

export const Header = () => {
  const { theme, toggleTheme } = useAppStore();
  const { currentProfile, clusterInfo } = useProfileStore();

  return (
    <header className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-6 py-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">
            ES Client
          </h1>
          {currentProfile && (
            <div className="flex items-center gap-2 px-3 py-1.5 bg-primary-100 dark:bg-primary-900 rounded-lg">
              <div className="w-2 h-2 bg-green-500 rounded-full" />
              <span className="text-sm font-medium text-primary-900 dark:text-primary-100">
                {currentProfile.name}
              </span>
              {clusterInfo && (
                <span className="text-xs text-primary-700 dark:text-primary-300">
                  ({clusterInfo.version.number})
                </span>
              )}
            </div>
          )}
        </div>

        <div className="flex items-center gap-4">
          <button
            onClick={toggleTheme}
            className="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
            aria-label="Toggle theme"
          >
            {theme === 'dark' ? (
              <SunIcon className="w-6 h-6 text-gray-600 dark:text-gray-400" />
            ) : (
              <MoonIcon className="w-6 h-6 text-gray-600" />
            )}
          </button>
        </div>
      </div>
    </header>
  );
};
