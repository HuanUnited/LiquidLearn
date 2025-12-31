import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { Subject, Topic, Theory, TimerPhase } from "../types";

interface SelectedState {
  selectedSubject: Subject | null;
  selectedTopic: Topic | null;
  selectedTheory: Theory | null;
  currentTimerPhase: number; // 1-6 mapping to phase_number
}

interface AppStore extends SelectedState {
  setSelectedSubject: (subject: Subject | null) => void;
  setSelectedTopic: (topic: Topic | null) => void;
  setSelectedTheory: (theory: Theory | null) => void;
  setCurrentTimerPhase: (phase: number) => void;
  clearSelection: () => void;
}

export const useAppStore = create<AppStore>()(
  persist(
    (set) => ({
      selectedSubject: null,
      selectedTopic: null,
      selectedTheory: null,
      currentTimerPhase: 1,

      setSelectedSubject: (subject) => set({ selectedSubject: subject }),
      setSelectedTopic: (topic) => set({ selectedTopic: topic }),
      setSelectedTheory: (theory) => set({ selectedTheory: theory }),
      setCurrentTimerPhase: (phase) => set({ currentTimerPhase: phase }),

      clearSelection: () =>
        set({
          selectedSubject: null,
          selectedTopic: null,
          selectedTheory: null,
        }),
    }),
    {
      name: "app-store",
    }
  )
);
