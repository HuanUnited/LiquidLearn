import { create } from "zustand";

export const TIMER_PHASES = [
  { number: 1, label: "1.1", duration: 25 * 60 },
  { number: 2, label: "1.2", duration: 25 * 60 },
  { number: 3, label: "2.1", duration: 25 * 60 },
  { number: 4, label: "2.2", duration: 25 * 60 },
  { number: 5, label: "3", duration: 25 * 60 },
  { number: 6, label: "4", duration: 25 * 60 },
];

interface TimerStore {
  isRunning: boolean;
  timeRemaining: number;
  currentPhase: number;
  totalTimeSpent: number;

  setIsRunning: (running: boolean) => void;
  setTimeRemaining: (time: number) => void;
  setCurrentPhase: (phase: number) => void;
  addTimeSpent: (seconds: number) => void;
  resetTimer: () => void;
  skipPhase: () => void;
}

export const useTimerStore = create<TimerStore>((set) => ({
  isRunning: false,
  timeRemaining: TIMER_PHASES[0].duration,
  currentPhase: 1,
  totalTimeSpent: 0,

  setIsRunning: (running) => set({ isRunning: running }),
  setTimeRemaining: (time) => set({ timeRemaining: time }),
  setCurrentPhase: (phase) => {
    const idx = TIMER_PHASES.findIndex((p) => p.number === phase);
    if (idx !== -1) {
      set({
        currentPhase: phase,
        timeRemaining: TIMER_PHASES[idx].duration,
        isRunning: false,
      });
    }
  },
  addTimeSpent: (seconds) =>
    set((state) => ({ totalTimeSpent: state.totalTimeSpent + seconds })),
  resetTimer: () =>
    set({
      isRunning: false,
      timeRemaining: TIMER_PHASES[0].duration,
      currentPhase: 1,
      totalTimeSpent: 0,
    }),
  skipPhase: () =>
    set((state) => {
      const nextPhase = Math.min(state.currentPhase + 1, 6);
      const idx = TIMER_PHASES.findIndex((p) => p.number === nextPhase);
      return {
        currentPhase: nextPhase,
        timeRemaining: TIMER_PHASES[idx].duration,
        isRunning: false,
      };
    }),
}));
