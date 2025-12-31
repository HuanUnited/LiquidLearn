import { invoke } from '@tauri-apps/api/core';

export interface DueCard {
  card_id: string;
  problem_id: string;
  title: string;
  difficulty: number;
  state: string;
  due: string;
  card_difficulty: number;
  stability: number;
  reps: number;
  lapses: number;
  days_overdue: number;
}

export interface FsrsCard {
  id: string;
  problem_id: string;
  due: string;
  stability: number;
  difficulty: number;
  state: string;
  reps: number;
  lapses: number;
  last_review: string | null;
  scheduled_days: number;
}

export const useFsrs = () => {
  const processReview = async (
    cardId: string,
    rating: number,
    elapsedSeconds: number
  ): Promise<FsrsCard> => {
    return await invoke('process_review', { cardId, rating, elapsedSeconds });
  };

  const getNextDue = async (limit: number): Promise<DueCard[]> => {
    return await invoke('get_next_due_problems', { limit });
  };

  const getCardStats = async (cardId: string) => {
    return await invoke('get_card_stats', { cardId });
  };

  return {
    processReview,
    getNextDue,
    getCardStats,
  };
};
