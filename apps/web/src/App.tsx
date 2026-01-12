import { Routes, Route, Link } from 'react-router-dom';

import Dashboard from './pages/Dashboard';
import ConfigEditor from './pages/ConfigEditor';
import Validator from './pages/Validator';
import VersionHistory from './pages/VersionHistory';
import Settings from './pages/Settings';

function App() {
  return (
    <div className="min-h-screen bg-slate-900 text-slate-100">
      <nav className="bg-slate-800 border-b border-slate-700">
        <div className="container mx-auto px-4">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center space-x-8">
              <Link to="/" className="text-xl font-bold text-indigo-400">
                Entropic Dev Portal
              </Link>
              <div className="flex space-x-4">
                <Link to="/" className="hover:text-indigo-400">
                  Dashboard
                </Link>
                <Link to="/validator" className="hover:text-indigo-400">
                  Validator
                </Link>
              </div>
            </div>
          </div>
        </div>
      </nav>

      <main className="container mx-auto px-4 py-8">
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/editor/:id?" element={<ConfigEditor />} />
          <Route path="/validator" element={<Validator />} />
          <Route path="/history/:id" element={<VersionHistory />} />
          <Route path="/settings" element={<Settings />} />
        </Routes>
      </main>
    </div>
  );
}

export default App;
