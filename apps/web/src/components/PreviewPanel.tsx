import type { GameDNA } from '@entropic/types';

export function PreviewPanel({ config }: { config: GameDNA }) {
  return (
    <div className="space-y-2">
      <h3 className="font-semibold">Live Preview</h3>
      <pre className="text-xs bg-slate-950/40 border border-slate-800 rounded p-3 overflow-auto max-h-[500px]">
        {JSON.stringify(config, null, 2)}
      </pre>
    </div>
  );
}
