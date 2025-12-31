import { invoke } from "@tauri-apps/api/tauri";
import type {
  Subject,
  Topic,
  Theory,
  Problem,
  Attempt,
  ErrorType,
  AttemptError,
  FsrsCard,
  FsrsStats,
} from "../types";

// ============ SUBJECTS ============
export const subjectAPI = {
  create: (name: string, description?: string) =>
    invoke<Subject>("create_subject", { req: { name, description } }),
  get: (id: string) => invoke<Subject>("get_subject", { id }),
  list: () => invoke<Subject[]>("list_subjects", {}),
  update: (id: string, name?: string, description?: string) =>
    invoke<Subject>("update_subject", { id, name, description }),
  delete: (id: string) => invoke<void>("delete_subject", { id }),
};

// ============ TOPICS ============
export const topicAPI = {
  create: (subject_id: string, name: string, description?: string) =>
    invoke<Topic>("create_topic", { subject_id, name, description }),
  get: (id: string) => invoke<Topic>("get_topic", { id }),
  listBySubject: (subject_id: string) =>
    invoke<Topic[]>("list_topics_by_subject", { subject_id }),
  update: (id: string, name?: string, description?: string) =>
    invoke<Topic>("update_topic", { id, name, description }),
  delete: (id: string) => invoke<void>("delete_topic", { id }),
};

// ============ THEORIES ============
export const theoryAPI = {
  create: (
    topic_id: string,
    phase_number: number,
    title: string,
    content?: string
  ) =>
    invoke<Theory>("create_theory", {
      topic_id,
      phase_number,
      title,
      content,
    }),
  get: (id: string) => invoke<Theory>("get_theory", { id }),
  listByTopic: (topic_id: string) =>
    invoke<Theory[]>("list_theories_by_topic", { topic_id }),
  getByPhase: (topic_id: string, phase_number: number) =>
    invoke<Theory | null>("get_theory_by_phase", { topic_id, phase_number }),
  update: (id: string, title?: string, content?: string) =>
    invoke<Theory>("update_theory", { id, title, content }),
  delete: (id: string) => invoke<void>("delete_theory", { id }),
};

// ============ PROBLEMS ============
export const problemAPI = {
  create: (
    topic_id: string,
    title: string,
    description?: string,
    image_url?: string,
    difficulty?: number,
    theory_id?: string
  ) =>
    invoke<Problem>("create_problem", {
      req: {
        topic_id,
        theory_id,
        title,
        description,
        image_url,
        difficulty: difficulty || 1,
      },
    }),
  get: (id: string) => invoke<Problem>("get_problem", { id }),
  listByTopic: (topic_id: string) =>
    invoke<Problem[]>("list_problems_by_topic", { topic_id }),
  listByTheory: (theory_id: string) =>
    invoke<Problem[]>("list_problems_by_theory", { theory_id }),
  update: (
    id: string,
    title?: string,
    description?: string,
    image_url?: string,
    difficulty?: number
  ) =>
    invoke<Problem>("update_problem", {
      id,
      req: { title, description, image_url, difficulty },
    }),
  delete: (id: string) => invoke<void>("delete_problem", { id }),
  markSolved: (id: string) => invoke<Problem>("mark_problem_solved", { id }),
  getWithDetails: (id: string) =>
    invoke<any>("get_problem_with_details", { id }),
};

// ============ ATTEMPTS ============
export const attemptAPI = {
  create: (problem_id: string, is_solved: boolean, commentary?: string) =>
    invoke<Attempt>("create_attempt", {
      req: { problem_id, is_solved, commentary },
    }),
  get: (id: string) => invoke<Attempt>("get_attempt", { id }),
  listByProblem: (problem_id: string) =>
    invoke<Attempt[]>("list_attempts_by_problem", { problem_id }),
  updateCommentary: (id: string, commentary: string) =>
    invoke<Attempt>("update_attempt_commentary", { id, commentary }),
  getStats: (problem_id: string) =>
    invoke<any>("get_problem_attempt_stats", { problem_id }),
};

// ============ ERRORS ============
export const errorAPI = {
  log: (attempt_id: string, error_type_id: number, description?: string) =>
    invoke<AttemptError>("log_error", {
      req: { attempt_id, error_type_id, description },
    }),
  resolve: (error_id: string) =>
    invoke<AttemptError>("resolve_error", { req: { error_id } }),
  getTypes: () => invoke<ErrorType[]>("get_error_types", {}),
  getByAttempt: (attempt_id: string) =>
    invoke<AttemptError[]>("get_errors_by_attempt", { attempt_id }),
  getUnresolvedByProblem: (problem_id: string) =>
    invoke<AttemptError[]>("get_unresolved_errors_by_problem", { problem_id }),
  init: () => invoke<void>("init_error_types", {}),
};

// ============ FSRS ============
export const fsrsAPI = {
  processReview: (
    problem_id: string,
    attempt_is_solved: boolean,
    quality: number,
    time_spent_seconds: number
  ) =>
    invoke<any>("process_review", {
      req: {
        problem_id,
        attempt_is_solved,
        quality,
        time_spent_seconds,
      },
    }),
  getDueCards: () => invoke<FsrsCard[]>("get_due_cards", {}),
  getStats: () => invoke<FsrsStats>("get_fsrs_stats", {}),
  getCard: (card_id: string) => invoke<FsrsCard>("get_fsrs_card", { card_id }),
  getCardByProblem: (problem_id: string) =>
    invoke<FsrsCard>("get_fsrs_card_by_problem", { problem_id }),
  getCardsByState: (state: string) =>
    invoke<FsrsCard[]>("get_cards_by_state", { state }),
};
