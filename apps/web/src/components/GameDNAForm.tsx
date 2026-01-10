import type { GameDNA, TargetPlatform, PhysicsProfile, DifficultyMode, MonetizationModel, Tone, WorldScale } from '@entropic/types';
import { GenreSelector } from './GenreSelector';
import { CameraSelector } from './CameraSelector';

const PLATFORMS: TargetPlatform[] = ['Mobile', 'PC', 'Console', 'XR', 'CloudStreamed', 'MultiPlatform'];
const PHYSICS: PhysicsProfile[] = ['Arcade', 'SemiRealistic', 'Realistic'];
const DIFFICULTY: DifficultyMode[] = ['Easy', 'Medium', 'Hard', 'Dynamic'];
const MONETIZATION: MonetizationModel[] = ['FreeToPlay', 'PremiumBuy', 'Subscription', 'OneTimePay', 'Hybrid'];
const TONES: Tone[] = ['Realistic', 'Arcade', 'Cinematic', 'Stylized', 'Minimalist'];
const SCALES: WorldScale[] = ['TinyLevel', 'SmallLevel', 'MediumLevel', 'LargeLevel', 'OpenWorld', 'Planet', 'Galaxy'];

export function GameDNAForm({
  config,
  onChange,
}: {
  config: GameDNA;
  onChange: (updated: Partial<GameDNA>) => void;
}) {
  return (
    <div className="space-y-6">
      <div className="grid gap-4 md:grid-cols-2">
        {/* Basic Info */}
        <div className="space-y-2">
          <label className="block text-sm font-medium">Game Name</label>
          <input
            type="text"
            value={config.name}
            onChange={(e) => onChange({ name: e.target.value })}
            className="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded text-sm"
          />
        </div>

        <div className="space-y-2">
          <label className="block text-sm font-medium">Target Audience</label>
          <input
            type="text"
            value={config.target_audience}
            onChange={(e) => onChange({ target_audience: e.target.value })}
            className="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded text-sm"
          />
        </div>

        {/* Genre & Camera */}
        <GenreSelector value={config.genre} onChange={(genre) => onChange({ genre })} />
        <CameraSelector value={config.camera} onChange={(camera) => onChange({ camera })} />

        {/* Tone */}
        <div className="space-y-2">
          <label className="block text-sm font-medium">Tone</label>
          <select
            value={typeof config.tone === 'string' ? config.tone : 'Custom'}
            onChange={(e) => onChange({ tone: e.target.value as Tone })}
            className="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded text-sm"
          >
            {TONES.map((t) => (
              <option key={t} value={t}>{t}</option>
            ))}
          </select>
        </div>

        {/* World Scale */}
        <div className="space-y-2">
          <label className="block text-sm font-medium">World Scale</label>
          <select
            value={typeof config.world_scale === 'string' ? config.world_scale : 'Custom'}
            onChange={(e) => onChange({ world_scale: e.target.value as WorldScale })}
            className="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded text-sm"
          >
            {SCALES.map((s) => (
              <option key={s} value={s}>{s}</option>
            ))}
          </select>
        </div>

        {/* Physics */}
        <div className="space-y-2">
          <label className="block text-sm font-medium">Physics Profile</label>
          <select
            value={typeof config.physics_profile === 'string' ? config.physics_profile : 'Custom'}
            onChange={(e) => onChange({ physics_profile: e.target.value as PhysicsProfile })}
            className="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded text-sm"
          >
            {PHYSICS.map((p) => (
              <option key={p} value={p}>{p}</option>
            ))}
          </select>
        </div>

        {/* Difficulty */}
        <div className="space-y-2">
          <label className="block text-sm font-medium">Difficulty</label>
          <select
            value={typeof config.difficulty === 'string' ? config.difficulty : 'Custom'}
            onChange={(e) => onChange({ difficulty: e.target.value as DifficultyMode })}
            className="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded text-sm"
          >
            {DIFFICULTY.map((d) => (
              <option key={d} value={d}>{d}</option>
            ))}
          </select>
        </div>

        {/* Monetization */}
        <div className="space-y-2">
          <label className="block text-sm font-medium">Monetization</label>
          <select
            value={typeof config.monetization === 'string' ? config.monetization : 'Custom'}
            onChange={(e) => onChange({ monetization: e.target.value as MonetizationModel })}
            className="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded text-sm"
          >
            {MONETIZATION.map((m) => (
              <option key={m} value={m}>{m}</option>
            ))}
          </select>
        </div>

        {/* Max Players */}
        <div className="space-y-2">
          <label className="block text-sm font-medium">Max Players</label>
          <input
            type="number"
            value={config.max_players}
            onChange={(e) => onChange({ max_players: parseInt(e.target.value) })}
            className="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded text-sm"
          />
        </div>

        {/* Target FPS */}
        <div className="space-y-2">
          <label className="block text-sm font-medium">Target FPS</label>
          <input
            type="number"
            value={config.target_fps}
            onChange={(e) => onChange({ target_fps: parseInt(e.target.value) })}
            className="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded text-sm"
          />
        </div>
      </div>

      {/* Platforms */}
      <div className="space-y-2">
        <label className="block text-sm font-medium">Target Platforms</label>
        <div className="grid grid-cols-3 gap-2">
          {PLATFORMS.map((p) => (
            <label key={p} className="flex items-center space-x-2 cursor-pointer">
              <input
                type="checkbox"
                checked={config.target_platforms.includes(p)}
                onChange={(e) => {
                  const next = e.target.checked
                    ? [...config.target_platforms, p]
                    : config.target_platforms.filter((x) => x !== p);
                  onChange({ target_platforms: next });
                }}
                className="rounded"
              />
              <span className="text-sm">{p}</span>
            </label>
          ))}
        </div>
      </div>

      {/* Booleans */}
      <div className="grid gap-4 md:grid-cols-2">
        <label className="flex items-center space-x-2 cursor-pointer">
          <input
            type="checkbox"
            checked={config.is_competitive}
            onChange={(e) => onChange({ is_competitive: e.target.checked })}
            className="rounded"
          />
          <span className="text-sm">Competitive</span>
        </label>

        <label className="flex items-center space-x-2 cursor-pointer">
          <input
            type="checkbox"
            checked={config.supports_coop}
            onChange={(e) => onChange({ supports_coop: e.target.checked })}
            className="rounded"
          />
          <span className="text-sm">Co-op Support</span>
        </label>

        <label className="flex items-center space-x-2 cursor-pointer">
          <input
            type="checkbox"
            checked={config.weather_enabled}
            onChange={(e) => onChange({ weather_enabled: e.target.checked })}
            className="rounded"
          />
          <span className="text-sm">Weather Enabled</span>
        </label>

        <label className="flex items-center space-x-2 cursor-pointer">
          <input
            type="checkbox"
            checked={config.day_night_cycle}
            onChange={(e) => onChange({ day_night_cycle: e.target.checked })}
            className="rounded"
          />
          <span className="text-sm">Day/Night Cycle</span>
        </label>

        <label className="flex items-center space-x-2 cursor-pointer">
          <input
            type="checkbox"
            checked={config.ai_enabled}
            onChange={(e) => onChange({ ai_enabled: e.target.checked })}
            className="rounded"
          />
          <span className="text-sm">AI Enabled</span>
        </label>

        <label className="flex items-center space-x-2 cursor-pointer">
          <input
            type="checkbox"
            checked={config.has_campaign}
            onChange={(e) => onChange({ has_campaign: e.target.checked })}
            className="rounded"
          />
          <span className="text-sm">Campaign</span>
        </label>
      </div>
    </div>
  );
}
