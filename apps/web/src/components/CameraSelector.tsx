import type { CameraMode } from '@entropic/types';

const CAMERAS: { value: CameraMode; label: string }[] = [
  { value: 'Perspective2D', label: '2D Perspective' },
  { value: 'Perspective2_5D', label: '2.5D Perspective' },
  { value: 'Perspective3D', label: '3D Perspective' },
  { value: 'Isometric', label: 'Isometric' },
  { value: 'VR', label: 'Virtual Reality' },
];

export function CameraSelector({
  value,
  onChange,
}: {
  value: CameraMode;
  onChange: (camera: CameraMode) => void;
}) {
  return (
    <div className="space-y-2">
      <label className="block text-sm font-medium">Camera Mode</label>
      <select
        value={typeof value === 'string' ? value : 'Custom'}
        onChange={(e) => onChange(e.target.value as CameraMode)}
        className="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded text-sm"
      >
        {CAMERAS.map((c) => (
          <option key={String(c.value)} value={c.value as string}>
            {c.label}
          </option>
        ))}
      </select>
    </div>
  );
}
