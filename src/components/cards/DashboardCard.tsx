import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { BaseCard } from './BaseCard';
import { TrendingUp, BookOpen, CheckCircle, Zap } from 'lucide-react';

interface DashboardStats {
  total_cards?: number;
  new?: number;
  learning?: number;
  review?: number;
  relearning?: number;
  due_today?: number;
  total_problems?: number;
  completed?: number;
  total_seconds_spent?: number;
}

export const DashboardCard: React.FC = () => {
  const [stats, setStats] = useState<DashboardStats | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadStats();
  }, []);

  const loadStats = async () => {
    try {
      setLoading(true);
      const fsrsStats = await invoke('get_fsrs_stats');
      const studySummary = await invoke('get_study_summary');

      setStats({
        ...(fsrsStats as object),
        ...(studySummary as object),
      });
    } catch (error) {
      console.error('Failed to load stats:', error);
    } finally {
      setLoading(false);
    }
  };

  const dueToday = stats?.due_today ?? 0;
  const completionPercent = stats?.total_problems
    ? ((stats.completed ?? 0) / stats.total_problems * 100).toFixed(1)
    : 0;
  const hoursSpent = stats?.total_seconds_spent
    ? (stats.total_seconds_spent / 3600).toFixed(1)
    : 0;

  return (
    <BaseCard
      title="Dashboard"
      subtitle="Your learning stats at a glance"
      icon="📊"
      className="col-span-2"
    >
      <div className="space-y-6">
        {/* Refresh Button */}
        <button
          onClick={loadStats}
          disabled={loading}
          className="btn-secondary w-full"
        >
          {loading ? 'Loading...' : 'Refresh Stats'}
        </button>

        {/* Main Stats Grid */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          {/* Due Today */}
          <div className="stat-card bg-gradient-to-br from-blue-500 to-blue-600">
            <div className="flex items-center justify-between mb-2">
              <p className="text-sm font-medium opacity-90">Due Today</p>
              <Zap size={20} />
            </div>
            <p className="text-3xl font-bold">{dueToday}</p>
            <p className="text-xs mt-2 opacity-75">reviews waiting</p>
          </div>

          {/* Completion Rate */}
          <div className="stat-card bg-gradient-to-br from-green-500 to-green-600">
            <div className="flex items-center justify-between mb-2">
              <p className="text-sm font-medium opacity-90">Completed</p>
              <CheckCircle size={20} />
            </div>
            <p className="text-3xl font-bold">{completionPercent}%</p>
            <p className="text-xs mt-2 opacity-75">{stats?.completed}/{stats?.total_problems}</p>
          </div>

          {/* Total Problems */}
          <div className="stat-card bg-gradient-to-br from-purple-500 to-purple-600">
            <div className="flex items-center justify-between mb-2">
              <p className="text-sm font-medium opacity-90">Total</p>
              <BookOpen size={20} />
            </div>
            <p className="text-3xl font-bold">{stats?.total_problems ?? 0}</p>
            <p className="text-xs mt-2 opacity-75">problems added</p>
          </div>

          {/* Time Invested */}
          <div className="stat-card bg-gradient-to-br from-orange-500 to-orange-600">
            <div className="flex items-center justify-between mb-2">
              <p className="text-sm font-medium opacity-90">Time</p>
              <TrendingUp size={20} />
            </div>
            <p className="text-3xl font-bold">{hoursSpent}</p>
            <p className="text-xs mt-2 opacity-75">hours spent</p>
          </div>
        </div>

        {/* Card State Distribution */}
        <div className="bg-slate-50 dark:bg-slate-900/50 rounded-lg p-4 space-y-3">
          <h3 className="font-semibold text-slate-900 dark:text-white">Card Distribution</h3>

          {[
            { label: 'New', count: stats?.new ?? 0, color: 'bg-blue-500' },
            { label: 'Learning', count: stats?.learning ?? 0, color: 'bg-yellow-500' },
            { label: 'Review', count: stats?.review ?? 0, color: 'bg-green-500' },
            { label: 'Relearning', count: stats?.relearning ?? 0, color: 'bg-red-500' },
          ].map((item) => (
            <div key={item.label}>
              <div className="flex items-center justify-between mb-1">
                <span className="text-sm text-slate-600 dark:text-slate-400">{item.label}</span>
                <span className="text-sm font-semibold text-slate-900 dark:text-white">
                  {item.count}
                </span>
              </div>
              <div className="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-2 overflow-hidden">
                <div
                  className={`${item.color} h-full transition-all duration-300`}
                  style={{
                    width: `${stats?.total_cards && stats.total_cards > 0
                        ? (item.count / stats.total_cards) * 100
                        : 0
                      }%`,
                  }}
                />
              </div>
            </div>
          ))}
        </div>
      </div>
    </BaseCard>
  );
};
