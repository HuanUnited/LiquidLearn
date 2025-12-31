export interface Subject {
  id: string;
  name: string;
  description?: string;
  created_at: string;
  updated_at: string;
}

export interface Topic {
  id: string;
  subject_id: string;
  name: string;
  description?: string;
  created_at: string;
  updated_at: string;
}

export interface Theory {
  id: string;
  topic_id: string;
  phase_number: number;
  title: string;
  content?: string;
  created_at: string;
  updated_at: string;
}

export interface Problem {
  id: string;
  topic_id: string;
  theory_id?: string;
  title: string;
  description?: string;
  image_url?: string;
  difficulty: number;
  is_solved: boolean;
  total_unresolved_errors: number;
  created_at: string;
  updated_at: string;
}

export interface Attempt {
  id: string;
  problem_id: string;
  is_solved: boolean;
  commentary?: string;
  created_at: string;
  updated_at: string;
}

export interface ErrorType {
  id: number;
  name: string;
  description?: string;
  multiplier: number;
  created_at: string;
}

export interface AttemptError {
  id: string;
  attempt_id: string;
  error_type_id: number;
  description?: string;
  is_resolved: boolean;
  created_at: string;
  updated_at: string;
}

export interface FsrsCard {
  id: string;
  problem_id: string;
  due: string;
  stability: number;
  difficulty: number;
  state: "new" | "learning" | "review" | "relearning";
  reps: number;
  lapses: number;
  elapsed_days: number;
  scheduled_days: number;
  created_at: string;
  updated_at: string;
}

export interface FsrsStats {
  total_cards: number;
  new_count: number;
  learning_count: number;
  review_count: number;
  relearning_count: number;
  due_today: number;
  retention_rate: number;
}

export interface CardLayout {
  i: string;
  x: number;
  y: number;
  w: number;
  h: number;
  static?: boolean;
  hidden?: boolean;
}

export interface TimerPhase {
  number: number;
  label: string;
  duration: number; // in seconds
}
