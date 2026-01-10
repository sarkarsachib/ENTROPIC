import { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import type { GameDNA, ValidationResult } from '@entropic/types';
import { storage } from '../services/storage';
import { validateConfigLocal } from '../services/validator';
import { GameDNAForm } from '../components/GameDNAForm';
import { ValidationResults } from '../components/ValidationResults';
import { PreviewPanel } from '../components/PreviewPanel';

function createEmptyConfig(): GameDNA {
  return {
    id: crypto.randomUUID(),
    name: 'New Game',
    version: { major: 0, minor: 1, patch: 0 },
    genre: 'Casual',
    camera: 'Perspective2D',
    tone: 'Stylized',
    world_scale: 'SmallLevel',
    target_platforms: ['PC'],
    physics_profile: 'Arcade',
    max_players: 1,
    is_competitive: false,
    supports_coop: false,
    difficulty: 'Easy',
    monetization: 'FreeToPlay',
    target_audience: 'Everyone',
    esrb_rating: null,
    target_fps: 60,
    max_draw_distance: 1000,
    max_entities: 100,
    max_npc_count: 10,
    time_scale: 1.0,
    weather_enabled: false,
    seasons_enabled: false,
    day_night_cycle: false,
    persistent_world: false,
    npc_count: 0,
    ai_enabled: false,
    ai_difficulty_scaling: false,
    has_campaign: false,
    has_side_quests: false,
    dynamic_quests: false,
    tags: [],
    custom_properties: {},
  };
}

export default function ConfigEditor() {
  const { id } = useParams<{ id?: string }>();
  const navigate = useNavigate();
  const [config, setConfig] = useState<GameDNA | null>(null);
  const [validation, setValidation] = useState<ValidationResult | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadConfig();
  }, [id]);

  useEffect(() => {
    if (config) {
      validateAsync();
    }
  }, [config]);

  async function loadConfig() {
    setLoading(true);
    try {
      if (id) {
        const loaded = await storage.getConfig(id);
        setConfig(loaded ?? createEmptyConfig());
      } else {
        setConfig(createEmptyConfig());
      }
    } catch (error) {
      console.error('Failed to load config:', error);
    } finally {
      setLoading(false);
    }
  }

  async function validateAsync() {
    if (!config) return;
    try {
      const result = await validateConfigLocal(config);
      setValidation(result);
    } catch (error) {
      console.error('Validation error:', error);
    }
  }

  async function handleSave() {
    if (!config) return;
    await storage.putConfig(config);
    alert('Config saved locally!');
  }

  async function handlePublish() {
    if (!config || !validation?.is_valid) return;
    alert('Publishing not yet implemented (requires API server)');
  }

  function handleChange(updates: Partial<GameDNA>) {
    if (!config) return;
    setConfig({ ...config, ...updates });
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-gray-400">Loading...</div>
      </div>
    );
  }

  if (!config) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-red-400">Failed to load config</div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">{config.name}</h1>
        <div className="flex space-x-2">
          <button
            onClick={() => navigate('/')}
            className="px-4 py-2 bg-slate-700 hover:bg-slate-600 rounded-lg transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleSave}
            className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 rounded-lg transition-colors"
          >
            Save
          </button>
          <button
            onClick={handlePublish}
            disabled={!validation?.is_valid}
            className="px-4 py-2 bg-green-600 hover:bg-green-700 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Publish
          </button>
        </div>
      </div>

      <div className="grid gap-6 lg:grid-cols-2">
        <div className="space-y-6">
          <div className="bg-slate-800 rounded-lg p-6">
            <h2 className="text-xl font-semibold mb-4">Configuration</h2>
            <GameDNAForm config={config} onChange={handleChange} />
          </div>
        </div>

        <div className="space-y-6">
          <div className="bg-slate-800 rounded-lg p-6">
            <ValidationResults result={validation} />
          </div>
          <div className="bg-slate-800 rounded-lg p-6">
            <PreviewPanel config={config} />
          </div>
        </div>
      </div>
    </div>
  );
}
