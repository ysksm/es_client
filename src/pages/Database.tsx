import { useState } from 'react';
import { Card, Button, Input } from '../components/ui';
import {
  TableCellsIcon,
  PlayIcon,
  DocumentTextIcon,
  ArrowPathIcon,
} from '@heroicons/react/24/outline';
import toast from 'react-hot-toast';
import * as api from '../api/tauri';

export const Database = () => {
  const [tables, setTables] = useState<string[]>([]);
  const [sqlQuery, setSqlQuery] = useState('SELECT * FROM ');
  const [queryResults, setQueryResults] = useState<any[] | null>(null);
  const [isExecuting, setIsExecuting] = useState(false);
  const [isLoadingTables, setIsLoadingTables] = useState(false);

  const handleListTables = async () => {
    setIsLoadingTables(true);
    try {
      const tableList = await api.listTables();
      setTables(tableList);
      toast.success(`${tableList.length} 件のテーブルを読み込みました`);
    } catch (error) {
      toast.error('テーブル一覧の取得に失敗しました');
      console.error(error);
    } finally {
      setIsLoadingTables(false);
    }
  };

  const handleExecuteQuery = async () => {
    if (!sqlQuery.trim()) {
      toast.error('SQLクエリを入力してください');
      return;
    }

    setIsExecuting(true);
    try {
      const results = await api.queryLocal(sqlQuery);
      setQueryResults(results);
      toast.success('クエリを実行しました');
    } catch (error) {
      toast.error('クエリの実行に失敗しました');
      console.error(error);
      setQueryResults(null);
    } finally {
      setIsExecuting(false);
    }
  };

  const handleExportQuery = async () => {
    if (!sqlQuery.trim()) {
      toast.error('SQLクエリを入力してください');
      return;
    }

    try {
      const result = await api.exportToParquet(sqlQuery, 'export.parquet');
      toast.success(result);
    } catch (error) {
      toast.error('エクスポートに失敗しました');
      console.error(error);
    }
  };

  const handleQuickQuery = (tableName: string, queryType: 'preview' | 'count') => {
    if (queryType === 'preview') {
      setSqlQuery(`SELECT * FROM ${tableName} LIMIT 100`);
    } else if (queryType === 'count') {
      setSqlQuery(`SELECT COUNT(*) as count FROM ${tableName}`);
    }
  };

  const renderResults = () => {
    if (!queryResults || queryResults.length === 0) {
      return (
        <div className="text-center py-8 text-gray-500 dark:text-gray-400">
          結果がありません
        </div>
      );
    }

    const columns = Object.keys(queryResults[0]);

    return (
      <div className="overflow-x-auto">
        <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
          <thead className="bg-gray-50 dark:bg-gray-800">
            <tr>
              {columns.map((col) => (
                <th
                  key={col}
                  className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider"
                >
                  {col}
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-800">
            {queryResults.map((row, idx) => (
              <tr key={idx} className="hover:bg-gray-50 dark:hover:bg-gray-800">
                {columns.map((col) => (
                  <td
                    key={col}
                    className="px-4 py-3 text-sm text-gray-900 dark:text-gray-100 whitespace-nowrap"
                  >
                    {typeof row[col] === 'object'
                      ? JSON.stringify(row[col])
                      : String(row[col])}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    );
  };

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-3xl font-bold text-gray-900 dark:text-white">
          ローカルデータベース
        </h2>
        <p className="mt-2 text-gray-600 dark:text-gray-400">
          DuckDBデータベースの管理とクエリ実行
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <Card title="テーブル一覧" className="lg:col-span-1">
          <div className="space-y-4">
            <Button
              variant="secondary"
              className="w-full"
              onClick={handleListTables}
              isLoading={isLoadingTables}
            >
              <ArrowPathIcon className="w-5 h-5 mr-2" />
              テーブルを更新
            </Button>

            {tables.length === 0 ? (
              <div className="text-center py-8">
                <TableCellsIcon className="mx-auto w-10 h-10 text-gray-400" />
                <p className="mt-2 text-sm text-gray-500 dark:text-gray-400">
                  テーブルがありません
                </p>
              </div>
            ) : (
              <div className="space-y-2">
                {tables.map((table) => (
                  <div
                    key={table}
                    className="p-3 bg-gray-50 dark:bg-gray-800 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        <TableCellsIcon className="w-5 h-5 text-primary-600 dark:text-primary-400" />
                        <span className="font-medium text-gray-900 dark:text-white">
                          {table}
                        </span>
                      </div>
                    </div>
                    <div className="mt-2 flex gap-2">
                      <button
                        onClick={() => handleQuickQuery(table, 'preview')}
                        className="text-xs text-primary-600 dark:text-primary-400 hover:underline"
                      >
                        プレビュー
                      </button>
                      <button
                        onClick={() => handleQuickQuery(table, 'count')}
                        className="text-xs text-primary-600 dark:text-primary-400 hover:underline"
                      >
                        件数確認
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </Card>

        <Card title="SQLクエリ" className="lg:col-span-2">
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                SQLクエリを入力
              </label>
              <textarea
                value={sqlQuery}
                onChange={(e) => setSqlQuery(e.target.value)}
                rows={6}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 font-mono text-sm focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                placeholder="SELECT * FROM table_name LIMIT 100"
              />
            </div>

            <div className="flex gap-2">
              <Button
                onClick={handleExecuteQuery}
                disabled={isExecuting}
                isLoading={isExecuting}
                className="flex-1"
              >
                <PlayIcon className="w-5 h-5 mr-2" />
                {isExecuting ? '実行中...' : 'クエリを実行'}
              </Button>
              <Button
                variant="secondary"
                onClick={handleExportQuery}
                disabled={isExecuting}
              >
                <DocumentTextIcon className="w-5 h-5 mr-2" />
                Parquet出力
              </Button>
            </div>

            <div className="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
              <h4 className="text-sm font-semibold text-gray-900 dark:text-white mb-2">
                クエリ例
              </h4>
              <div className="space-y-2 text-xs text-gray-600 dark:text-gray-400">
                <div>
                  <code className="block bg-white dark:bg-gray-900 p-2 rounded">
                    SELECT * FROM table_name LIMIT 100
                  </code>
                </div>
                <div>
                  <code className="block bg-white dark:bg-gray-900 p-2 rounded">
                    SELECT COUNT(*) FROM table_name
                  </code>
                </div>
                <div>
                  <code className="block bg-white dark:bg-gray-900 p-2 rounded">
                    SELECT * FROM table_name WHERE status = 'active'
                  </code>
                </div>
              </div>
            </div>
          </div>
        </Card>
      </div>

      {queryResults !== null && (
        <Card title={`クエリ結果 (${queryResults.length} 件)`}>
          {renderResults()}
        </Card>
      )}
    </div>
  );
};
