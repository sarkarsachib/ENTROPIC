import { useEffect, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import type { GameDNA } from '@entropic/types';
import { storage } from '../services/storage';

export default function Dashboard() {
  const navigate = useNavigate();
  const [configs, setConfigs] = useState<GameDNA[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadConfigs();
  }, []);

  async function loadConfigs() {
    setLoading(true);
    try {
      const items = await storage.listConfigs();
      setConfigs(items);
    } catch (error) {
      console.error('Failed to load configs:', error);
    } finally {
      setLoading(false);
    }
  }

  function handleCreateNew() {
    navigate('/editor');
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-gray-400">Loading...</div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">Game Configurations</h1>
        <button
          onClick={handleCreateNew}
          className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 rounded-lg transition-colors"
        >
          Create New Config
        </button>
      </div>

      <div className="bg-slate-800 rounded-lg p-6">
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {configs.length === 0 ? (
            <div className="col-span-full text-center py-8 text-gray-400">
              No configs found. Create your first configuration!
            </div>
          ) : (
            configs.map((config) => (
              <Link
                key={config.id}
                to={`/editor/${config.id}`}
                className="block p-4 bg-slate-700 hover:bg-slate-600 rounded-lg transition-colors"
              >
                <h3 className="text-lg font-semibold">{config.name}</h3>
                <p className="text-sm text-gray-400 mt-1">
                  {typeof config.genre === 'string' ? config.genre : 'Custom'}
                </p>
                <p className="text-xs text-gray-500 mt-2">
                  v{config.version.major}.{config.version.minor}.{config.version.patch}
                </p>
              </Link>
            ))
          )}
        </div>
      </div>

      <div className="grid gap-4 md:grid-cols-3">
        <div className="bg-slate-800 rounded-lg p-6">
          <h2 className="text-lg font-semibold">Total Configs</h2>
          <p className="text-3xl font-bold mt-2 text-indigo-400">{configs.length}</p>
        </div>
        <div className="bg-slate-800 rounded-lg p-6">
          <h2 className="text-lg font-semibold">Recent Edits</h2>
          <p className="text-3xl font-bold mt-2 text-purple-400">0</p>
        </div>
        <div className="bg-slate-800 rounded-lg p-6">
          <h2 className="text-lg font-semibold">Published</h2>
          <p className="text-3xl font-bold mt-2 text-green-400">0</p>
        </div>
      </div>
    </div>
  );
}
