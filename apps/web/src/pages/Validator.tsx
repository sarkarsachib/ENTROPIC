import { useState } from 'react';
import type { ValidationResult } from '@entropic/types';
import { validateConfigLocal } from '../services/validator';

/**
 * Render a Validator UI for pasting Game DNA JSON, performing local validation, and displaying validation results or parsing/validation errors.
 *
 * @returns A JSX element containing a textarea for JSON input and a results pane that shows an error message or the validation outcome and formatted ValidationResult.
 */
export default function Validator() {
  const [json, setJson] = useState('');
  const [result, setResult] = useState<ValidationResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function handleValidate(text: string) {
    setJson(text);
    try {
      setError(null);
      if (text.trim().length === 0) {
        setResult(null);
        return;
      }
      const config = JSON.parse(text);
      const res = await validateConfigLocal(config);
      setResult(res);
    } catch (e) {
      setError((e as Error).message);
      setResult(null);
    }
  }

  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold">Validator</h1>

      <div className="grid gap-6 lg:grid-cols-2">
        <div className="bg-slate-800 rounded-lg p-6">
          <h2 className="text-xl font-semibold mb-4">Paste Game DNA JSON</h2>
          <textarea
            value={json}
            onChange={(e) => handleValidate(e.target.value)}
            className="w-full h-80 px-3 py-2 bg-slate-900 border border-slate-700 rounded text-sm font-mono"
            placeholder="Paste Game DNA JSON here..."
          />
        </div>

        <div className="bg-slate-800 rounded-lg p-6">
          <h2 className="text-xl font-semibold mb-4">Results</h2>
          {error && <div className="text-red-400">{error}</div>}
          {result && (
            <div className="space-y-3">
              <div className={result.is_valid ? 'text-green-400' : 'text-red-400'}>
                {result.is_valid ? '✓ Valid' : '✗ Invalid'}
              </div>
              <pre className="text-xs bg-slate-950/40 border border-slate-800 rounded p-3 overflow-auto max-h-80">
                {JSON.stringify(result, null, 2)}
              </pre>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}