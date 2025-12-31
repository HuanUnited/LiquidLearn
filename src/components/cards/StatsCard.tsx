import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { BaseCard } from './BaseCard';
import { TrendingUp, Calendar, Target, Award } from 'lucide-react';

interface PhaseQueue {
  decode?: number;
  encode?: number;
  recall?: number;
  reflect?: number;
}

export const StatsCard: React.FC = () => {
  const [phaseQueue, setPhaseQueue] = useState<PhaseQueue | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadQueue();
  }, []);

  const loadQueue = async () => {
    try {
      setLoading(true);
      const queue = await invoke('get_phase_queue');
      setPhaseQueue(queue as PhaseQueue);
    } catch (error) {
      console.error('Failed to load queue:', error);
    } finally {
      setLoading(false);
    }
  };

  const phases = [
    { name: 'Decode', icon: '🔍', count: phaseQueue?.decode ?? 0, color: 'from-blue-500 to-blue-600' },
    { name: 'Encode', icon: '🧠', count: phaseQueue?.encode ?? 0, color: 'from-purple-500 to-purple-600' },
    { name: 'Recall', icon: '📝', count: phaseQueue?.recall ?? 0, color: 'from-green-500 to-green-600' },
    { name: 'Reflect', icon: '💭', count: phaseQueue?.reflect ?? 0, color: 'from-orange-500 to-orange-600' },
  ];

  const totalInQueue = Object.values(phaseQueue ?? {}).reduce((a, b) => (a ?? 0) + (b ?? 0), 0);

  return (
    <BaseCard
      title="Study Queue"
      subtitle="Problems waiting in each phase"
      icon="📈"
      className="col-span-2"
    >
      <div className="space-y-6">
        {/* Refresh Button */}
        <button
          onClick={loadQueue}
          disabled={loading}
          className="btn-secondary w-full"
        >
          {loading ? 'Loading...' : 'Refresh Queue'}
        </button>

        {/* Total Count */}
        <div className="text-center p-4 bg-slate-50 dark:bg-slate-900/50 rounded-lg">
          <p className="text-sm text-slate-600 dark:text-slate-400 mb-1">Total Waiting</p>
          <p className="text-4xl font-bold text-blue-600 dark:text-blue-400">{totalInQueue}</p>
        </div>

        {/* Phase Queue Grid */}
        <div className="grid grid-cols-2 gap-4">
          {phases.map((phase) => (
            <div
              key={phase.name}
              className={`stat-card bg-linear-to-br ${phase.color} p-4`}
            >
              <div className="flex items-center justify-between mb-2">
                <p className="text-sm font-medium opacity-90">{phase.name}</p>
                <span className="text-2xl">{phase.icon}</span>
              </div>
              <p className="text-3xl font-bold">{phase.count}</p>
              <p className="text-xs mt-2 opacity-75">waiting</p>
            </div>
          ))}
        </div>

        {/* Recommended Focus */}
        {phaseQueue && (
          <div className="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
            <h3 className="font-semibold text-blue-900 dark:text-blue-300 mb-2 flex items-center gap-2">
              <Target size={18} /> Recommended Focus
            </h3>
            <p className="text-sm text-blue-800 dark:text-blue-300">
              {phaseQueue.decode ? (
                `Start with ${phaseQueue.decode} problems in Decode phase`
              ) : phaseQueue.encode ? (
                `Continue with ${phaseQueue.encode} problems in Encode phase`
              ) : phaseQueue.recall ? (
                `Move to ${phaseQueue.recall} problems in Recall phase`
              ) : (
                'Great! All caught up. Review completed problems.'
              )}
            </p>
          </div>
        )}
      </div>
    </BaseCard>
  );
};
