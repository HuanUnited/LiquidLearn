import React, { useEffect } from "react";
import { BaseCard } from "./BaseCard";
import { useTimerStore, TIMER_PHASES } from "@/stores/timerStore";
import { useAppStore } from "../../stores/appStore";
import { Play, Pause, SkipForward, RotateCcw } from "lucide-react";

export const TimerCard: React.FC = () => {
  const {
    isRunning,
    timeRemaining,
    currentPhase,
    totalTimeSpent,
    setIsRunning,
    setTimeRemaining,
    setCurrentPhase,
    addTimeSpent,
  } = useTimerStore();

  const { setCurrentTimerPhase } = useAppStore();

  // Timer tick effect
  useEffect(() => {
    if (!isRunning) return;

    const interval = setInterval(() => {
      setTimeRemaining(Math.max(0, timeRemaining - 1));
      addTimeSpent(1);
    }, 1000);

    return () => clearInterval(interval);
  }, [isRunning, timeRemaining, setTimeRemaining, addTimeSpent]);

  // Auto advance phase when timer ends
  useEffect(() => {
    if (timeRemaining === 0 && isRunning) {
      setIsRunning(false);
      if (currentPhase < 6) {
        setCurrentPhase(currentPhase + 1);
      }
    }
  }, [timeRemaining, isRunning, currentPhase, setIsRunning, setCurrentPhase]);

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
  };

  const currentPhaseData = TIMER_PHASES.find((p) => p.number === currentPhase);
  const progressPercent = (
    ((TIMER_PHASES[currentPhase - 1].duration - timeRemaining) /
      TIMER_PHASES[currentPhase - 1].duration) *
    100
  ).toFixed(0);

  return (
    <BaseCard id="timer" title="Study Timer" className="col-span-1">
      <div className="flex flex-col items-center gap-6">
        {/* Phase display */}
        <div className="text-center">
          <p className="text-xs text-slate-400 mb-1">Current Phase</p>
          <p className="text-3xl font-bold text-amber-400">
            {currentPhaseData?.label}
          </p>
        </div>

        {/* Timer display */}
        <div className="relative w-32 h-32 flex items-center justify-center">
          <svg className="absolute w-full h-full transform -rotate-90">
            <circle
              cx="64"
              cy="64"
              r="56"
              fill="none"
              stroke="#334155"
              strokeWidth="2"
            />
            <circle
              cx="64"
              cy="64"
              r="56"
              fill="none"
              stroke="#3b82f6"
              strokeWidth="2"
              strokeDasharray={`${(356 * Number(progressPercent)) / 100} 356`}
              className="transition-all duration-500"
            />
          </svg>
          <div className="text-center">
            <p className="text-4xl font-mono font-bold text-blue-400">
              {formatTime(timeRemaining)}
            </p>
          </div>
        </div>

        {/* Controls */}
        <div className="flex gap-2">
          <button
            onClick={() => setIsRunning(!isRunning)}
            className="p-2 bg-blue-500/20 hover:bg-blue-500/30 rounded border border-blue-500/50 text-blue-400 transition-colors"
          >
            {isRunning ? <Pause size={18} /> : <Play size={18} />}
          </button>
          <button
            onClick={() => setCurrentPhase(currentPhase + 1)}
            className="p-2 bg-slate-700/50 hover:bg-slate-700 rounded border border-slate-600/50 text-slate-300 transition-colors disabled:opacity-50"
            disabled={currentPhase >= 6}
          >
            <SkipForward size={18} />
          </button>
          <button
            onClick={() => {
              setIsRunning(false);
              setCurrentPhase(1);
            }}
            className="p-2 bg-slate-700/50 hover:bg-slate-700 rounded border border-slate-600/50 text-slate-300 transition-colors"
          >
            <RotateCcw size={18} />
          </button>
        </div>

        {/* Total time spent */}
        <div className="text-center text-xs text-slate-400">
          <p>Total time today: {formatTime(totalTimeSpent)}</p>
        </div>
      </div>
    </BaseCard>
  );
};
