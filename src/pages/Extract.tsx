import { useState, useEffect } from 'react';
import { useProfileStore } from '../store/profileStore';
import { useIndexStore } from '../store/indexStore';
import { Card, Button, Input, Loading } from '../components/ui';
import {
  ArrowDownTrayIcon,
  CircleStackIcon,
  MagnifyingGlassIcon,
} from '@heroicons/react/24/outline';
import toast from 'react-hot-toast';
import * as api from '../api/tauri';
import type { SearchQuery } from '../types';

export const Extract = () => {
  const { currentProfile } = useProfileStore();
  const { indices, loadIndices } = useIndexStore();

  const [selectedIndex, setSelectedIndex] = useState('');
  const [tableName, setTableName] = useState('');
  const [queryJson, setQueryJson] = useState('{}');
  const [size, setSize] = useState('1000');
  const [isExtracting, setIsExtracting] = useState(false);
  const [lastResult, setLastResult] = useState<string | null>(null);

  useEffect(() => {
    if (currentProfile) {
      loadIndices(currentProfile.name);
    }
  }, [currentProfile]);

  const handleExtract = async () => {
    if (!currentProfile || !selectedIndex || !tableName) {
      toast.error('プロファイル、インデックス、テーブル名を指定してください');
      return;
    }

    let query: SearchQuery;
    try {
      const parsedQuery = JSON.parse(queryJson);
      query = {
        ...parsedQuery,
        size: parseInt(size) || 1000,
      };
    } catch (error) {
      toast.error('クエリのJSON形式が正しくありません');
      return;
    }

    setIsExtracting(true);
    try {
      const result = await api.extractAndStoreData(
        currentProfile.name,
        selectedIndex,
        query,
        tableName
      );
      setLastResult(result);
      toast.success('データ抽出が完了しました');
    } catch (error) {
      toast.error('データ抽出に失敗しました');
      console.error(error);
    } finally {
      setIsExtracting(false);
    }
  };

  const handleQuickExtract = (queryType: 'all' | 'recent') => {
    if (queryType === 'all') {
      setQueryJson('{}');
      setSize('10000');
    } else if (queryType === 'recent') {
      setQueryJson(JSON.stringify({
        sort: [{ '@timestamp': { order: 'desc' } }]
      }, null, 2));
      setSize('1000');
    }
  };

  if (!currentProfile) {
    return (
      <div className="space-y-6">
        <h2 className="text-3xl font-bold text-gray-900 dark:text-white">
          データ抽出
        </h2>
        <Card>
          <div className="text-center py-12">
            <ArrowDownTrayIcon className="mx-auto w-12 h-12 text-gray-400" />
            <h3 className="mt-4 text-lg font-medium text-gray-900 dark:text-white">
              接続が必要です
            </h3>
            <p className="mt-2 text-gray-600 dark:text-gray-400">
              データを抽出するには、まず接続プロファイルを選択してください
            </p>
          </div>
        </Card>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-3xl font-bold text-gray-900 dark:text-white">
          データ抽出
        </h2>
        <p className="mt-2 text-gray-600 dark:text-gray-400">
          Elasticsearchからデータを抽出してDuckDBに保存
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card title="抽出設定">
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                インデックス
              </label>
              <select
                value={selectedIndex}
                onChange={(e) => setSelectedIndex(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent"
              >
                <option value="">インデックスを選択...</option>
                {indices.map((index) => (
                  <option key={index} value={index}>
                    {index}
                  </option>
                ))}
              </select>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                テーブル名
              </label>
              <Input
                type="text"
                value={tableName}
                onChange={(e) => setTableName(e.target.value)}
                placeholder="保存先のテーブル名"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                DuckDBに保存されるテーブル名を指定してください
              </p>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                取得件数
              </label>
              <Input
                type="number"
                value={size}
                onChange={(e) => setSize(e.target.value)}
                placeholder="1000"
                min="1"
                max="10000"
              />
            </div>

            <div>
              <div className="flex items-center justify-between mb-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  検索クエリ (JSON)
                </label>
                <div className="flex gap-2">
                  <button
                    onClick={() => handleQuickExtract('all')}
                    className="text-xs text-primary-600 dark:text-primary-400 hover:underline"
                  >
                    全件
                  </button>
                  <button
                    onClick={() => handleQuickExtract('recent')}
                    className="text-xs text-primary-600 dark:text-primary-400 hover:underline"
                  >
                    最新順
                  </button>
                </div>
              </div>
              <textarea
                value={queryJson}
                onChange={(e) => setQueryJson(e.target.value)}
                rows={8}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 font-mono text-sm focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                placeholder='{"query": {"match_all": {}}}'
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                Elasticsearch検索クエリをJSON形式で入力してください
              </p>
            </div>

            <Button
              className="w-full"
              onClick={handleExtract}
              disabled={isExtracting || !selectedIndex || !tableName}
              isLoading={isExtracting}
            >
              <ArrowDownTrayIcon className="w-5 h-5 mr-2" />
              {isExtracting ? '抽出中...' : 'データを抽出'}
            </Button>
          </div>
        </Card>

        <div className="space-y-6">
          <Card title="クイックガイド">
            <div className="space-y-4 text-sm text-gray-600 dark:text-gray-400">
              <div>
                <h4 className="font-semibold text-gray-900 dark:text-white mb-2">
                  基本的な使い方
                </h4>
                <ol className="list-decimal list-inside space-y-1">
                  <li>抽出元のインデックスを選択</li>
                  <li>保存先のテーブル名を入力</li>
                  <li>取得件数を指定（デフォルト: 1000件）</li>
                  <li>検索クエリを設定（空の場合は全件）</li>
                  <li>「データを抽出」ボタンをクリック</li>
                </ol>
              </div>

              <div>
                <h4 className="font-semibold text-gray-900 dark:text-white mb-2">
                  クエリの例
                </h4>
                <div className="space-y-2">
                  <div>
                    <p className="font-medium">全件取得:</p>
                    <code className="block bg-gray-100 dark:bg-gray-900 p-2 rounded text-xs mt-1">
                      {'{}'}
                    </code>
                  </div>
                  <div>
                    <p className="font-medium">条件検索:</p>
                    <code className="block bg-gray-100 dark:bg-gray-900 p-2 rounded text-xs mt-1 whitespace-pre">
                      {`{
  "query": {
    "match": {
      "status": "active"
    }
  }
}`}
                    </code>
                  </div>
                  <div>
                    <p className="font-medium">時間範囲指定:</p>
                    <code className="block bg-gray-100 dark:bg-gray-900 p-2 rounded text-xs mt-1 whitespace-pre">
                      {`{
  "query": {
    "range": {
      "@timestamp": {
        "gte": "now-7d"
      }
    }
  }
}`}
                    </code>
                  </div>
                </div>
              </div>
            </div>
          </Card>

          {lastResult && (
            <Card title="抽出結果">
              <div className="space-y-2">
                <div className="flex items-start gap-2">
                  <MagnifyingGlassIcon className="w-5 h-5 text-green-500 flex-shrink-0 mt-0.5" />
                  <div className="flex-1">
                    <p className="text-sm text-gray-900 dark:text-white">
                      {lastResult}
                    </p>
                  </div>
                </div>
              </div>
            </Card>
          )}
        </div>
      </div>
    </div>
  );
};
