import type { ValidationResult } from '@entropic/types';

export function ValidationResults({ result }: { result: ValidationResult | null }) {
  if (!result) return null;

  return (
    <div className="space-y-3">
      <div className="flex items-center justify-between">
        <h3 className="font-semibold">Validation</h3>
        <span
          className={`text-sm px-2 py-1 rounded ${
            result.is_valid ? 'bg-green-600/20 text-green-300' : 'bg-red-600/20 text-red-300'
          }`}
        >
          {result.is_valid ? 'Valid' : 'Invalid'}
        </span>
      </div>

      {result.errors.length > 0 && (
        <div className="space-y-2">
          <div className="text-sm font-medium text-red-300">Errors</div>
          {result.errors.map((e, idx) => (
            <div key={idx} className="text-sm bg-red-950/40 border border-red-900 rounded p-2">
              <div className="font-mono text-xs text-red-200">[{e.code}] {e.field}</div>
              <div className="text-red-100">{e.message}</div>
              <div className="text-xs text-red-200/80">{e.details}</div>
            </div>
          ))}
        </div>
      )}

      {result.warnings.length > 0 && (
        <div className="space-y-2">
          <div className="text-sm font-medium text-yellow-300">Warnings</div>
          {result.warnings.map((w, idx) => (
            <div key={idx} className="text-sm bg-yellow-950/30 border border-yellow-900 rounded p-2">
              <div className="font-mono text-xs text-yellow-200">[{w.code}] {w.field}</div>
              <div className="text-yellow-100">{w.message}</div>
              <div className="text-xs text-yellow-200/80">{w.suggestion}</div>
            </div>
          ))}
        </div>
      )}

      {result.suggestions.length > 0 && (
        <div className="space-y-2">
          <div className="text-sm font-medium text-indigo-300">Suggestions</div>
          <ul className="text-sm text-indigo-100 list-disc list-inside">
            {result.suggestions.map((s, idx) => (
              <li key={idx}>{s}</li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}
