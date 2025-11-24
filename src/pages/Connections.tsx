import { useEffect } from 'react';
import { useProfileStore } from '../store/profileStore';
import { Card, Button, Loading } from '../components/ui';
import { ServerIcon, CheckCircleIcon } from '@heroicons/react/24/outline';
import toast from 'react-hot-toast';

export const Connections = () => {
  const { profiles, currentProfile, selectProfile, isLoading, loadProfiles } = useProfileStore();

  useEffect(() => {
    loadProfiles();
  }, []);

  const handleSelectProfile = async (name: string) => {
    try {
      await selectProfile(name);
      toast.success(`Connected to ${name}`);
    } catch (error) {
      toast.error('Failed to connect');
    }
  };

  if (isLoading) {
    return <Loading text="Loading profiles..." />;
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-3xl font-bold text-gray-900 dark:text-white">
            Connections
          </h2>
          <p className="mt-2 text-gray-600 dark:text-gray-400">
            Manage Elasticsearch connection profiles
          </p>
        </div>
        <Button>
          New Profile
        </Button>
      </div>

      {profiles.length === 0 ? (
        <Card>
          <div className="text-center py-12">
            <ServerIcon className="mx-auto w-12 h-12 text-gray-400" />
            <h3 className="mt-4 text-lg font-medium text-gray-900 dark:text-white">
              No profiles found
            </h3>
            <p className="mt-2 text-gray-600 dark:text-gray-400">
              Create your first connection profile to get started
            </p>
            <Button className="mt-4">
              Create Profile
            </Button>
          </div>
        </Card>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {profiles.map((profile) => (
            <Card key={profile.name}>
              <div className="space-y-4">
                <div className="flex items-start justify-between">
                  <div>
                    <h3 className="text-lg font-semibold text-gray-900 dark:text-white flex items-center gap-2">
                      {profile.name}
                      {currentProfile?.name === profile.name && (
                        <CheckCircleIcon className="w-5 h-5 text-green-500" />
                      )}
                    </h3>
                    <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
                      {profile.host}
                    </p>
                  </div>
                </div>

                <dl className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <dt className="text-gray-600 dark:text-gray-400">Auth:</dt>
                    <dd className="text-gray-900 dark:text-white font-medium">
                      {profile.auth_type}
                    </dd>
                  </div>
                  <div className="flex justify-between">
                    <dt className="text-gray-600 dark:text-gray-400">SSL:</dt>
                    <dd className="text-gray-900 dark:text-white font-medium">
                      {profile.use_ssl ? 'Enabled' : 'Disabled'}
                    </dd>
                  </div>
                </dl>

                <div className="flex gap-2">
                  <Button
                    variant="primary"
                    className="flex-1"
                    onClick={() => handleSelectProfile(profile.name)}
                    disabled={currentProfile?.name === profile.name}
                  >
                    {currentProfile?.name === profile.name ? 'Connected' : 'Connect'}
                  </Button>
                  <Button variant="secondary">
                    Edit
                  </Button>
                </div>
              </div>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
};
