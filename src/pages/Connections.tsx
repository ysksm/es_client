import { useEffect, useState } from 'react';
import { useProfileStore } from '../store/profileStore';
import { Card, Button, Input, Loading } from '../components/ui';
import { ServerIcon, CheckCircleIcon, XMarkIcon, TrashIcon } from '@heroicons/react/24/outline';
import toast from 'react-hot-toast';
import * as api from '../api/tauri';
import type { AuthType } from '../types';

interface ProfileFormData {
  name: string;
  host: string;
  username: string;
  password: string;
  apiKey: string;
  authType: AuthType;
  useSsl: boolean;
  verifyCertificate: boolean;
}

const initialFormData: ProfileFormData = {
  name: '',
  host: 'http://localhost:9200',
  username: '',
  password: '',
  apiKey: '',
  authType: 'basic',
  useSsl: false,
  verifyCertificate: true,
};

export const Connections = () => {
  const { profiles, currentProfile, selectProfile, createProfile, deleteProfile, isLoading, loadProfiles } = useProfileStore();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [formData, setFormData] = useState<ProfileFormData>(initialFormData);
  const [isSaving, setIsSaving] = useState(false);
  const [deleteConfirm, setDeleteConfirm] = useState<string | null>(null);
  const [connectingProfile, setConnectingProfile] = useState<string | null>(null);

  useEffect(() => {
    loadProfiles();
  }, []);

  const handleSelectProfile = async (name: string) => {
    setConnectingProfile(name);
    try {
      // First test the connection
      await api.testConnection(name);
      // Only select profile if connection succeeds
      await selectProfile(name);
      toast.success(`Connected to ${name}`);
    } catch (error) {
      const errorMessage = typeof error === 'string' ? error : (error instanceof Error ? error.message : 'Failed to connect');
      toast.error(errorMessage);
    } finally {
      setConnectingProfile(null);
    }
  };

  const handleDeleteProfile = async (name: string) => {
    try {
      await deleteProfile(name);
      toast.success(`Profile "${name}" deleted`);
      setDeleteConfirm(null);
    } catch (error) {
      toast.error('Failed to delete profile');
    }
  };

  const openModal = () => {
    setFormData(initialFormData);
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
    setFormData(initialFormData);
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value, type } = e.target;
    if (type === 'checkbox') {
      const checked = (e.target as HTMLInputElement).checked;
      setFormData(prev => ({ ...prev, [name]: checked }));
    } else {
      setFormData(prev => ({ ...prev, [name]: value }));
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!formData.name.trim()) {
      toast.error('Profile name is required');
      return;
    }
    if (!formData.host.trim()) {
      toast.error('Host is required');
      return;
    }
    if (formData.authType === 'basic') {
      if (!formData.username.trim()) {
        toast.error('Username is required for Basic Auth');
        return;
      }
      if (!formData.password) {
        toast.error('Password is required for Basic Auth');
        return;
      }
    } else if (formData.authType === 'apikey') {
      if (!formData.apiKey) {
        toast.error('API Key is required');
        return;
      }
    }

    setIsSaving(true);
    try {
      let passwordEncrypted: string | undefined;
      let apiKeyEncrypted: string | undefined;

      if (formData.authType === 'basic' && formData.password) {
        passwordEncrypted = await api.encryptPassword(formData.password);
      } else if (formData.authType === 'apikey' && formData.apiKey) {
        apiKeyEncrypted = await api.encryptPassword(formData.apiKey);
      }

      const now = Math.floor(Date.now() / 1000);
      await createProfile({
        name: formData.name.trim(),
        host: formData.host.trim(),
        username: formData.authType === 'basic' ? formData.username : undefined,
        password_encrypted: passwordEncrypted,
        api_key_encrypted: apiKeyEncrypted,
        auth_type: formData.authType,
        use_ssl: formData.useSsl,
        verify_certificate: formData.verifyCertificate,
        created_at: now,
        updated_at: now,
      });

      toast.success(`Profile "${formData.name}" created`);
      closeModal();
    } catch (error) {
      toast.error(error instanceof Error ? error.message : 'Failed to create profile');
    } finally {
      setIsSaving(false);
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
        <Button onClick={openModal}>
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
            <Button className="mt-4" onClick={openModal}>
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
                    disabled={currentProfile?.name === profile.name || connectingProfile !== null}
                    isLoading={connectingProfile === profile.name}
                  >
                    {currentProfile?.name === profile.name ? 'Connected' : 'Connect'}
                  </Button>
                  {deleteConfirm === profile.name ? (
                    <div className="flex gap-1">
                      <Button
                        variant="danger"
                        size="sm"
                        onClick={() => handleDeleteProfile(profile.name)}
                      >
                        Yes
                      </Button>
                      <Button
                        variant="secondary"
                        size="sm"
                        onClick={() => setDeleteConfirm(null)}
                      >
                        No
                      </Button>
                    </div>
                  ) : (
                    <Button
                      variant="secondary"
                      onClick={() => setDeleteConfirm(profile.name)}
                      disabled={currentProfile?.name === profile.name}
                    >
                      <TrashIcon className="w-4 h-4" />
                    </Button>
                  )}
                </div>
              </div>
            </Card>
          ))}
        </div>
      )}

      {/* Profile Creation Modal */}
      {isModalOpen && (
        <div className="fixed inset-0 z-50 overflow-y-auto">
          <div className="flex min-h-full items-center justify-center p-4">
            {/* Backdrop */}
            <div
              className="fixed inset-0 bg-black/50 transition-opacity"
              onClick={closeModal}
            />

            {/* Modal */}
            <div className="relative w-full max-w-md transform rounded-lg bg-white dark:bg-gray-800 p-6 shadow-xl transition-all">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                  New Profile
                </h3>
                <button
                  onClick={closeModal}
                  className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                >
                  <XMarkIcon className="w-5 h-5" />
                </button>
              </div>

              <form onSubmit={handleSubmit} className="space-y-4">
                <Input
                  label="Profile Name"
                  name="name"
                  value={formData.name}
                  onChange={handleInputChange}
                  placeholder="my-cluster"
                  required
                />

                <Input
                  label="Host"
                  name="host"
                  value={formData.host}
                  onChange={handleInputChange}
                  placeholder="http://localhost:9200"
                  required
                />

                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                    Authentication Type
                  </label>
                  <select
                    name="authType"
                    value={formData.authType}
                    onChange={handleInputChange}
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  >
                    <option value="basic">Basic Auth</option>
                    <option value="apikey">API Key</option>
                  </select>
                </div>

                {formData.authType === 'basic' ? (
                  <>
                    <Input
                      label="Username"
                      name="username"
                      value={formData.username}
                      onChange={handleInputChange}
                      placeholder="elastic"
                    />
                    <Input
                      label="Password"
                      name="password"
                      type="password"
                      value={formData.password}
                      onChange={handleInputChange}
                      placeholder="••••••••"
                    />
                  </>
                ) : (
                  <Input
                    label="API Key"
                    name="apiKey"
                    type="password"
                    value={formData.apiKey}
                    onChange={handleInputChange}
                    placeholder="Enter API key"
                  />
                )}

                <div className="space-y-2">
                  <label className="flex items-center gap-2">
                    <input
                      type="checkbox"
                      name="useSsl"
                      checked={formData.useSsl}
                      onChange={handleInputChange}
                      className="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                    />
                    <span className="text-sm text-gray-700 dark:text-gray-300">Use SSL</span>
                  </label>
                  <label className="flex items-center gap-2">
                    <input
                      type="checkbox"
                      name="verifyCertificate"
                      checked={formData.verifyCertificate}
                      onChange={handleInputChange}
                      className="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                    />
                    <span className="text-sm text-gray-700 dark:text-gray-300">Verify Certificate</span>
                  </label>
                </div>

                <div className="flex gap-3 pt-4">
                  <Button
                    type="button"
                    variant="secondary"
                    className="flex-1"
                    onClick={closeModal}
                  >
                    Cancel
                  </Button>
                  <Button
                    type="submit"
                    variant="primary"
                    className="flex-1"
                    isLoading={isSaving}
                  >
                    Create
                  </Button>
                </div>
              </form>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
