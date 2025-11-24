import { useProfileStore } from '../store/profileStore';
import { Card, Loading } from '../components/ui';
import {
  ServerIcon,
  CircleStackIcon,
  TableCellsIcon,
} from '@heroicons/react/24/outline';

export const Dashboard = () => {
  const { profiles, currentProfile, clusterInfo, isLoading } = useProfileStore();

  if (isLoading) {
    return <Loading text="Loading dashboard..." />;
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-3xl font-bold text-gray-900 dark:text-white">
          Dashboard
        </h2>
        <p className="mt-2 text-gray-600 dark:text-gray-400">
          Elasticsearch Index Management
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <Card>
          <div className="flex items-center gap-4">
            <div className="p-3 bg-primary-100 dark:bg-primary-900 rounded-lg">
              <ServerIcon className="w-8 h-8 text-primary-600 dark:text-primary-400" />
            </div>
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Profiles</p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">
                {profiles.length}
              </p>
            </div>
          </div>
        </Card>

        <Card>
          <div className="flex items-center gap-4">
            <div className="p-3 bg-green-100 dark:bg-green-900 rounded-lg">
              <CircleStackIcon className="w-8 h-8 text-green-600 dark:text-green-400" />
            </div>
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Connection</p>
              <p className="text-lg font-semibold text-gray-900 dark:text-white">
                {currentProfile ? 'Connected' : 'Not Connected'}
              </p>
            </div>
          </div>
        </Card>

        <Card>
          <div className="flex items-center gap-4">
            <div className="p-3 bg-purple-100 dark:bg-purple-900 rounded-lg">
              <TableCellsIcon className="w-8 h-8 text-purple-600 dark:text-purple-400" />
            </div>
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-400">Status</p>
              <p className="text-lg font-semibold text-gray-900 dark:text-white">
                Ready
              </p>
            </div>
          </div>
        </Card>
      </div>

      {currentProfile && clusterInfo && (
        <Card title="Current Connection">
          <dl className="grid grid-cols-2 gap-4">
            <div>
              <dt className="text-sm font-medium text-gray-600 dark:text-gray-400">
                Profile
              </dt>
              <dd className="mt-1 text-lg text-gray-900 dark:text-white">
                {currentProfile.name}
              </dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-gray-600 dark:text-gray-400">
                Host
              </dt>
              <dd className="mt-1 text-lg text-gray-900 dark:text-white">
                {currentProfile.host}
              </dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-gray-600 dark:text-gray-400">
                Cluster Name
              </dt>
              <dd className="mt-1 text-lg text-gray-900 dark:text-white">
                {clusterInfo.cluster_name}
              </dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-gray-600 dark:text-gray-400">
                Version
              </dt>
              <dd className="mt-1 text-lg text-gray-900 dark:text-white">
                {clusterInfo.version.number}
              </dd>
            </div>
          </dl>
        </Card>
      )}

      {!currentProfile && (
        <Card>
          <div className="text-center py-12">
            <ServerIcon className="mx-auto w-12 h-12 text-gray-400" />
            <h3 className="mt-4 text-lg font-medium text-gray-900 dark:text-white">
              No connection selected
            </h3>
            <p className="mt-2 text-gray-600 dark:text-gray-400">
              Go to Connections to create or select a profile
            </p>
          </div>
        </Card>
      )}
    </div>
  );
};
