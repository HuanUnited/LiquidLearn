import { invoke } from '@tauri-apps/api/core';

export interface Problem {
  id: string;
  title: string;
  description: string | null;
  difficulty: number;
  created_at: string;
  updated_at: string;
}

export interface ProblemMastery {
  id: string;
  problem_id: string;
  solved: boolean;
  mastery_percent: number;
  last_attempted: string | null;
  attempt_count: number;
  updated_at: string;
}

export interface ProblemWithMastery {
  problem: Problem;
  mastery: ProblemMastery | null;
  tags: string[];
}

export interface ListProblemsResponse {
  problems: ProblemWithMastery[];
  total: number;
}

export const useProblems = () => {
  const createProblem = async (
    title: string,
    description: string | null,
    difficulty: number
  ): Promise<Problem> => {
    return await invoke('create_problem', { title, description, difficulty });
  };

  const getProblem = async (id: string): Promise<ProblemWithMastery> => {
    return await invoke('get_problem', { id });
  };

  const updateProblem = async (
    id: string,
    title?: string,
    description?: string,
    difficulty?: number
  ): Promise<Problem> => {
    return await invoke('update_problem', { id, title, description, difficulty });
  };

  const deleteProblem = async (id: string): Promise<void> => {
    return await invoke('delete_problem', { id });
  };

  const listProblems = async (
    filter?: string,
    sort?: string,
    limit?: number,
    offset?: number
  ): Promise<ListProblemsResponse> => {
    return await invoke('list_problems', { filter, sort, limit, offset });
  };

  const searchProblems = async (query: string, limit?: number): Promise<Problem[]> => {
    return await invoke('search_problems', { query, limit });
  };

  const bulkImport = async (csvContent: string) => {
    return await invoke('bulk_import_problems', { csvContent });
  };

  const addTag = async (problemId: string, tagName: string): Promise<void> => {
    return await invoke('add_problem_tag', { problemId, tagName });
  };

  return {
    createProblem,
    getProblem,
    updateProblem,
    deleteProblem,
    listProblems,
    searchProblems,
    bulkImport,
    addTag,
  };
};
