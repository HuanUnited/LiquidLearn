// ============================================================================
// ERROR TYPE MODEL
// ============================================================================
export interface ErrorType {
  id: number;
  name: string;
  description?: string;
  multiplier: number;
  created_at: string;
}

// ============================================================================
// PROBLEM ERROR MODEL
// ============================================================================
export interface ProblemError {
  id: number;
  attempt_id: number;
  error_type_id: number;
  message: string;
  created_at: string;
}

export interface ProblemErrorWithType extends ProblemError {
  error_type_name: string;
  multiplier: number;
}

// ============================================================================
// ERROR RESOLUTION MODEL
// ============================================================================
export interface ErrorResolution {
  id: number;
  error_id: number;
  resolution_notes?: string;
  time_to_fix_seconds?: number;
  re_attempted: number;
  successful: number;
  created_at: string;
}

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================
export interface LogErrorRequest {
  attempt_id: number;
  error_type_id: number;
  message: string;
}

export interface ResolveErrorRequest {
  error_id: number;
  resolution_notes: string;
  time_to_fix_seconds: number;
  successful: boolean;
}

export interface ErrorTypeStats {
  error_type_id: number;
  error_type_name: string;
  total_count: number;
  resolved_count: number;
  resolution_rate: number;
  multiplier: number;
}

export interface ErrorAnalyticsResponse {
  total_errors: number;
  resolved_errors: number;
  resolution_rate: number;
  most_common_error?: string;
  errors_by_type: ErrorTypeStats[];
}

export interface ProblemErrorHistory {
  problem_id: number;
  error_type_id: number;
  error_type_name: string;
  total_occurrences: number;
  resolution_count: number;
  last_occurred: string;
}

export interface ProblemRecommendation {
  problem_id: number;
  problem_name: string;
  tier: string;
  due: string;
  unresolved_error_count: number;
  error_types: string[];
  priority_score: number;
}
