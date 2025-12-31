import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { CardLayout } from "../types";

interface UIStore {
  layout: CardLayout[];
  setLayout: (layout: CardLayout[]) => void;
  toggleCardVisibility: (cardId: string) => void;
  darkMode: boolean;
  setDarkMode: (dark: boolean) => void;
  modals: {
    guidelines: boolean;
    attempt: boolean;
    error: boolean;
    createResource: boolean;
  };
  openModal: (modal: keyof UIStore["modals"]) => void;
  closeModal: (modal: keyof UIStore["modals"]) => void;
}

export const useUIStore = create<UIStore>()(
  persist(
    (set) => ({
      layout: [],
      setLayout: (layout) => set({ layout }),
      toggleCardVisibility: (cardId) =>
        set((state) => ({
          layout: state.layout.map((item) =>
            item.i === cardId ? { ...item, hidden: !item.hidden } : item
          ),
        })),

      darkMode: true,
      setDarkMode: (dark) => set({ darkMode: dark }),

      modals: {
        guidelines: false,
        attempt: false,
        error: false,
        createResource: false,
      },

      openModal: (modal) =>
        set((state) => ({
          modals: { ...state.modals, [modal]: true },
        })),

      closeModal: (modal) =>
        set((state) => ({
          modals: { ...state.modals, [modal]: false },
        })),
    }),
    {
      name: "ui-store",
    }
  )
);
