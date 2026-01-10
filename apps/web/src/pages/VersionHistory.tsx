import { useParams } from 'react-router-dom';

export default function VersionHistory() {
  const { id } = useParams<{ id: string }>();

  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold">Version History</h1>
      <div className="bg-slate-800 rounded-lg p-6">
        <p className="text-gray-400">Version history for config: {id}</p>
        <p className="text-sm text-gray-500 mt-2">(Feature not yet implemented)</p>
      </div>
    </div>
  );
}
