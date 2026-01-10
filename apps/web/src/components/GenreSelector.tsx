import type { Genre } from '@entropic/types';

const GENRES: { value: Genre; label: string; description: string }[] = [
  { value: 'FPS', label: 'FPS', description: 'First-person shooter' },
  { value: 'RPG', label: 'RPG', description: 'Role-playing game' },
  { value: 'TPS', label: 'TPS', description: 'Third-person shooter' },
  { value: 'Strategy', label: 'Strategy', description: 'Strategy and tactics' },
  { value: 'Casual', label: 'Casual', description: 'Casual and accessible' },
  { value: 'Horror', label: 'Horror', description: 'Horror and suspense' },
  { value: 'Racing', label: 'Racing', description: 'Racing and driving' },
  { value: 'Simulation', label: 'Simulation', description: 'Simulation and sandbox' },
  { value: 'Puzzle', label: 'Puzzle', description: 'Puzzles and logic' },
  { value: 'Educational', label: 'Educational', description: 'Learning-focused' },
];

export function GenreSelector({
  value,
  onChange,
}: {
  value: Genre;
  onChange: (genre: Genre) => void;
}) {
  return (
    <div className="space-y-2">
      <label className="block text-sm font-medium">Genre</label>
      <select
        aria-label="Genre"
        value={typeof value === 'string' ? value : 'Custom'}
        onChange={(e) => onChange(e.target.value as Genre)}
        className="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded text-sm"
      >
        {GENRES.map((g) => (
          <option key={String(g.value)} value={g.value as string}>
            {g.label}
          </option>
        ))}
      </select>
      <p className="text-xs text-slate-400">
        {GENRES.find((g) => g.value === value)?.description ?? 'Custom genre'}
      </p>
    </div>
  );
}
