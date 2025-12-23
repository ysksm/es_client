import { useEffect, useState } from 'react';
import { useProfileStore } from '../store/profileStore';
import { useIndexStore } from '../store/indexStore';
import { Card, Button, Loading, Input } from '../components/ui';
import {
  CircleStackIcon,
  MagnifyingGlassIcon,
  TrashIcon,
  ArrowPathIcon,
  PlusIcon,
} from '@heroicons/react/24/outline';
import toast from 'react-hot-toast';

export const Indices = () => {
  const { currentProfile } = useProfileStore();
  const {
    indices,
    selectedIndex,
    documentCount,
    isLoading,
    error,
    loadIndices,
    selectIndex,
    deleteIndex,
    countDocuments,
  } = useIndexStore();

  const [searchTerm, setSearchTerm] = useState('');
  const [showDeleteConfirm, setShowDeleteConfirm] = useState<string | null>(null);

  useEffect(() => {
    if (currentProfile) {
      loadIndices(currentProfile.name);
    }
  }, [currentProfile]);

  const handleRefresh = async () => {
    if (!currentProfile) return;
    try {
      await loadIndices(currentProfile.name);
      toast.success('インデックスリストを更新しました');
    } catch (error) {
      toast.error('更新に失敗しました');
    }
  };

  const handleSelectIndex = async (indexName: string) => {
    if (!currentProfile) return;
    selectIndex(indexName);
    try {
      await countDocuments(currentProfile.name, indexName);
    } catch (error) {
      toast.error('ドキュメント数の取得に失敗しました');
    }
  };

  const handleDeleteIndex = async (indexName: string) => {
    if (!currentProfile) return;
    try {
      await deleteIndex(currentProfile.name, indexName);
      toast.success(`インデックス "${indexName}" を削除しました`);
      setShowDeleteConfirm(null);
    } catch (error) {
      toast.error('インデックスの削除に失敗しました');
    }
  };

  const filteredIndices = indices.filter((index) =>
    index.toLowerCase().includes(searchTerm.toLowerCase())
  );

  if (!currentProfile) {
    return (
      <div className="space-y-6">
        <h2 className="text-3xl font-bold text-gray-900 dark:text-white">
          インデックス管理
        </h2>
        <Card>
          <div className="text-center py-12">
            <CircleStackIcon className="mx-auto w-12 h-12 text-gray-400" />
            <h3 className="mt-4 text-lg font-medium text-gray-900 dark:text-white">
              接続が必要です
            </h3>
            <p className="mt-2 text-gray-600 dark:text-gray-400">
              インデックスを管理するには、まず接続プロファイルを選択してください
            </p>
          </div>
        </Card>
      </div>
    );
  }

  if (isLoading && indices.length === 0) {
    return <Loading text="インデックスを読み込み中..." />;
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-3xl font-bold text-gray-900 dark:text-white">
            インデックス管理
          </h2>
          <p className="mt-2 text-gray-600 dark:text-gray-400">
            {currentProfile.name} のインデックス一覧
          </p>
        </div>
        <div className="flex gap-2">
          <Button
            variant="secondary"
            onClick={handleRefresh}
            disabled={isLoading}
          >
            <ArrowPathIcon className="w-5 h-5 mr-2" />
            更新
          </Button>
          <Button>
            <PlusIcon className="w-5 h-5 mr-2" />
            新規作成
          </Button>
        </div>
      </div>

      {error && (
        <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
          <p className="text-red-800 dark:text-red-200">{error}</p>
        </div>
      )}

      <Card>
        <div className="flex items-center gap-2">
          <MagnifyingGlassIcon className="w-5 h-5 text-gray-400" />
          <Input
            type="text"
            placeholder="インデックスを検索..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="flex-1"
          />
        </div>
      </Card>

      {filteredIndices.length === 0 ? (
        <Card>
          <div className="text-center py-12">
            <CircleStackIcon className="mx-auto w-12 h-12 text-gray-400" />
            <h3 className="mt-4 text-lg font-medium text-gray-900 dark:text-white">
              {searchTerm ? 'インデックスが見つかりません' : 'インデックスがありません'}
            </h3>
            <p className="mt-2 text-gray-600 dark:text-gray-400">
              {searchTerm
                ? '検索条件を変更してください'
                : '新しいインデックスを作成してください'}
            </p>
          </div>
        </Card>
      ) : (
        <div className="grid grid-cols-1 gap-4">
          {filteredIndices.map((index) => (
            <Card
              key={index}
              className={`transition-all cursor-pointer hover:shadow-lg ${
                selectedIndex === index
                  ? 'ring-2 ring-primary-500 dark:ring-primary-400'
                  : ''
              }`}
              onClick={() => handleSelectIndex(index)}
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-4 flex-1">
                  <div className="p-3 bg-primary-100 dark:bg-primary-900 rounded-lg">
                    <CircleStackIcon className="w-6 h-6 text-primary-600 dark:text-primary-400" />
                  </div>
                  <div className="flex-1">
                    <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                      {index}
                    </h3>
                    {selectedIndex === index && documentCount !== null && (
                      <p className="text-sm text-gray-600 dark:text-gray-400">
                        {documentCount.toLocaleString()} ドキュメント
                      </p>
                    )}
                  </div>
                </div>

                <div className="flex items-center gap-2">
                  {showDeleteConfirm === index ? (
                    <div className="flex items-center gap-2">
                      <span className="text-sm text-gray-600 dark:text-gray-400">
                        削除しますか?
                      </span>
                      <Button
                        variant="danger"
                        size="sm"
                        onClick={(e) => {
                          e.stopPropagation();
                          handleDeleteIndex(index);
                        }}
                      >
                        はい
                      </Button>
                      <Button
                        variant="secondary"
                        size="sm"
                        onClick={(e) => {
                          e.stopPropagation();
                          setShowDeleteConfirm(null);
                        }}
                      >
                        いいえ
                      </Button>
                    </div>
                  ) : (
                    <Button
                      variant="danger"
                      size="sm"
                      onClick={(e) => {
                        e.stopPropagation();
                        setShowDeleteConfirm(index);
                      }}
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

      <div className="text-sm text-gray-600 dark:text-gray-400">
        {filteredIndices.length} 件のインデックス
        {searchTerm && ` (全 ${indices.length} 件中)`}
      </div>
    </div>
  );
};
