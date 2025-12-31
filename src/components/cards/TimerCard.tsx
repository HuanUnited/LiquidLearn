import React, { useState } from 'react';
import { BaseCard } from './BaseCard';
import { useTimer, TimerMode } from '../../hooks/useTimer';
import { Play, Pause, RotateCcw } from 'lucide-react';

interface TimerCardProps {
  mode?: TimerMode;
  title?: string;
  onTimeUpdate?: (seconds: number) => void;
}

export const TimerCard: React.FC<TimerCardProps> = ({
  mode = 'countup',
  title = 'Study Timer',
  onTimeUpdate,
}) => {
  const timer = useTimer({ initialSeconds: 0, mode });
  const [timerMode, setTimerMode] = useState<TimerMode>(mode);
  const [countdownMinutes, setCountdownMinutes] = useState(5);

  const handleModeChange = (newMode: TimerMode) => {
    setTimerMode(newMode);
    timer.reset();
  };

  React.useEffect(() => {
    onTimeUpdate?.(Math.floor(timer.totalSeconds));
  }, [timer.totalSeconds, onTimeUpdate]);

  return (
    <BaseCard
      title={title}
      subtitle={timerMode === 'countup' ? 'Count Up' : 'Count Down'}
      icon="⏱️"
      className="col-span-2 md:col-span-1"
    >
      <div className="space-y-6">
        {/* Mode Toggle */}
        <div className="flex gap-2">
          <button
            onClick={() => handleModeChange('countup')}
            className={`flex-1 py-2 px-3 rounded-lg font-medium text-sm transition-colors ${timerMode === 'countup'
                ? 'bg-blue-600 text-white'
                : 'bg-slate-200 dark:bg-slate-700 text-slate-900 dark:text-white hover:bg-slate-300 dark:hover:bg-slate-600'
              }`}
          >
            Count Up
          </button>
          <button
            onClick={() => handleModeChange('countdown')}
            className={`flex-1 py-2 px-3 rounded-lg font-medium text-sm transition-colors ${timerMode === 'countdown'
                ? 'bg-blue-600 text-white'
                : 'bg-slate-200 dark:bg-slate-700 text-slate-900 dark:text-white hover:bg-slate-300 dark:hover:bg-slate-600'
              }`}
          >
            Count Down
          </button>
        </div>

        {/* Countdown Setup */}
        {timerMode === 'countdown' && !timer.isRunning && timer.totalSeconds === 0 && (
          <div className="space-y-2">
            <label className="text-sm font-medium text-slate-700 dark:text-slate-300">
              Minutes: {countdownMinutes}
            </label>
            <input
              type="range"
              min="1"
              max="60"
              value={countdownMinutes}
              onChange={(e) => setCountdownMinutes(parseInt(e.target.value))}
              className="w-full"
            />
            <button
              onClick={() => timer.setTotalSeconds(countdownMinutes * 60)}
              className="btn-primary w-full"
            >
              Set Countdown
            </button>
          </div>
        )}

        {/* Timer Display */}
        <div className="text-center">
          <div className="text-5xl font-bold font-mono text-blue-600 dark:text-blue-400 tracking-wider">
            {timer.formatTime()}
          </div>
        </div>

        {/* Progress Bar */}
        {timerMode === 'countdown' && countdownMinutes > 0 && (
          <div className="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-2 overflow-hidden">
            <div
              className="bg-blue-600 h-full transition-all duration-200"
              style={{
                width: `${((countdownMinutes * 60 - timer.totalSeconds) / (countdownMinutes * 60)) * 100}%`,
              }}
            />
          </div>
        )}

        {/* Controls */}
        <div className="flex gap-2">
          <button
            onClick={timer.isRunning ? timer.pause : timer.start}
            className="flex-1 btn-primary flex items-center justify-center gap-2"
          >
            {timer.isRunning ? (
              <>
                <Pause size={18} /> Pause
              </>
            ) : (
              <>
                <Play size={18} /> Start
              </>
            )}
          </button>
          <button
            onClick={timer.reset}
            className="btn-secondary px-4 flex items-center justify-center gap-2"
          >
            <RotateCcw size={18} />
          </button>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-2 gap-3 pt-4 border-t border-slate-200 dark:border-slate-700">
          <div className="text-center">
            <p className="text-xs text-slate-500 dark:text-slate-400">Seconds</p>
            <p className="text-xl font-semibold text-slate-900 dark:text-white">
              {Math.floor(timer.totalSeconds)}
            </p>
          </div>
          <div className="text-center">
            <p className="text-xs text-slate-500 dark:text-slate-400">Status</p>
            <p className="text-xl font-semibold text-slate-900 dark:text-white">
              {timer.isRunning ? '🔴 Running' : '⏸️ Paused'}
            </p>
          </div>
        </div>
      </div>
    </BaseCard>
  );
};
