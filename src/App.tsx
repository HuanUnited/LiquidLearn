import { invoke } from '@tauri-apps/api/tauri';
import { useState, useEffect } from 'react';
import './styles/tailwind.css';
import { ThemeProvider, useTheme } from './components/layout/ThemeProvider';
import { BaseCard } from './components/cards/BaseCard';
import { TimerCard } from './components/cards/TimerCard';
import { DashboardCard } from './components/cards/DashboardCard';
import { StatsCard } from './components/cards/StatsCard';
import { Moon, Sun } from 'lucide-react';

function AppContent() {
  const { theme, toggleTheme } = useTheme();
  const [dbStatus, setDbStatus] = useState('');
  const [problems, setProblems] = useState<any[]>([]);
  const [selectedProblem, setSelectedProblem] = useState<any>(null);
  const [testTitle, setTestTitle] = useState('');
  const [testDifficulty, setTestDifficulty] = useState(3);

  async function initDb() {
    try {
      const result = await invoke('init_db');
      setDbStatus(result as string);
      await listProblems();
    } catch (error) {
      setDbStatus(`Error: ${error}`);
    }
  }

  async function createProblem() {
    try {
      await invoke('create_problem', {
        title: testTitle || `Problem ${Date.now()}`,
        description: 'Test description',
        difficulty: testDifficulty,
      });
      setTestTitle('');
      await listProblems();
    } catch (error) {
      console.error(error);
    }
  }

  async function listProblems() {
    try {
      const result = await invoke('list_problems', {
        difficulty: null,
        solvedOnly: false,
      });
      setProblems(result as any[]);
    } catch (error) {
      console.error(error);
    }
  }

  useEffect(() => {
    initDb();
  }, []);

  return (
    <div className="min-h-screen bg-slate-50 dark:bg-slate-900 transition-colors duration-200">
      {/* Header */}
      <header className="bg-white dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700 sticky top-0 z-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex items-center justify-between">
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">🚀 LiquidLearn</h1>
          <div className="flex items-center gap-4">
            <button
              onClick={initDb}
              className="btn-primary text-sm"
            >
              Init DB
            </button>
            <button
              onClick={toggleTheme}
              className="p-2 rounded-lg bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 transition-colors"
              aria-label="Toggle theme"
            >
              {theme === 'light' ? (
                <Moon size={20} className="text-slate-600 dark:text-slate-300" />
              ) : (
                <Sun size={20} className="text-slate-300" />
              )}
            </button>
          </div>
        </div>

        {dbStatus && (
          <div className="bg-green-50 dark:bg-green-900/20 border-t border-green-200 dark:border-green-700 px-4 sm:px-6 lg:px-8 py-3 text-sm text-green-800 dark:text-green-300">
            ✓ {dbStatus}
          </div>
        )}
      </header>

      {/* Main Grid */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 auto-rows-max">
          {/* Dashboard */}
          <DashboardCard />

          {/* Study Queue */}
          <StatsCard />

          {/* Timer */}
          <TimerCard mode="countup" />

          {/* Create Problem */}
          <BaseCard
            title="Create Problem"
            subtitle="Add a new problem to study"
            icon="➕"
            className="col-span-1"
          >
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  Title
                </label>
                <input
                  type="text"
                  placeholder="Problem title..."
                  value={testTitle}
                  onChange={(e) => setTestTitle(e.target.value)}
                  className="w-full px-3 py-2 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-900 dark:text-white placeholder-slate-500 dark:placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  Difficulty: {testDifficulty}
                </label>
                <input
                  type="range"
                  min="1"
                  max="5"
                  value={testDifficulty}
                  onChange={(e) => setTestDifficulty(parseInt(e.target.value))}
                  className="w-full"
                />
              </div>

              <button
                onClick={createProblem}
                className="btn-primary w-full"
              >
                Create Problem
              </button>
            </div>
          </BaseCard>

          {/* Problems List */}
          <BaseCard
            title="Problems"
            subtitle={`${problems.length} problems`}
            icon="📚"
            className="col-span-2 md:col-span-1"
          >
            <div className="space-y-2 max-h-80 overflow-y-auto">
              {problems.length === 0 ? (
                <p className="text-sm text-slate-500 dark:text-slate-400 text-center py-4">
                  No problems yet. Create one to start!
                </p>
              ) : (
                problems.map((p) => (
                  <div
                    key={p.id}
                    onClick={() => setSelectedProblem(p)}
                    className={`p-3 rounded-lg cursor-pointer transition-colors ${selectedProblem?.id === p.id
                        ? 'bg-blue-100 dark:bg-blue-900/30 border-2 border-blue-500'
                        : 'bg-slate-100 dark:bg-slate-700/50 border border-slate-200 dark:border-slate-600 hover:bg-slate-200 dark:hover:bg-slate-700'
                      }`}
                  >
                    <p className="font-semibold text-slate-900 dark:text-white text-sm">
                      {p.title.slice(0, 25)}
                    </p>
                    <p className="text-xs text-slate-600 dark:text-slate-400 mt-1">
                      Difficulty: {p.difficulty}
                    </p>
                  </div>
                ))
              )}
            </div>
          </BaseCard>

          {/* Selected Problem Details */}
          {selectedProblem && (
            <BaseCard
              title="Problem Details"
              subtitle="View and manage"
              icon="🔍"
              className="col-span-2 md:col-span-1"
            >
              <div className="space-y-4">
                <div>
                  <p className="text-xs text-slate-500 dark:text-slate-400 uppercase tracking-wide">
                    Title
                  </p>
                  <p className="text-lg font-semibold text-slate-900 dark:text-white">
                    {selectedProblem.title}
                  </p>
                </div>

                <div>
                  <p className="text-xs text-slate-500 dark:text-slate-400 uppercase tracking-wide">
                    Difficulty
                  </p>
                  <div className="flex gap-1 mt-1">
                    {Array.from({ length: selectedProblem.difficulty }).map((_, i) => (
                      <div
                        key={i}
                        className="w-2 h-2 rounded-full bg-blue-600 dark:bg-blue-400"
                      />
                    ))}
                  </div>
                </div>

                <div>
                  <p className="text-xs text-slate-500 dark:text-slate-400 uppercase tracking-wide">
                    Created
                  </p>
                  <p className="text-sm text-slate-600 dark:text-slate-300">
                    {new Date(selectedProblem.created_at).toLocaleDateString()}
                  </p>
                </div>

                <button
                  onClick={() => setSelectedProblem(null)}
                  className="btn-secondary w-full text-sm"
                >
                  Clear Selection
                </button>
              </div>
            </BaseCard>
          )}
        </div>
      </main>
    </div>
  );
}

function App() {
  return (
    <ThemeProvider>
      <AppContent />
    </ThemeProvider>
  );
}

export default App;
