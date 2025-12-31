-- ============================================================================
-- PHASE 6: ERROR LOGGING SYSTEM - DATABASE MIGRATION
-- ============================================================================
-- Created: December 31, 2025
-- Purpose: Add error logging tables and FSRS integration columns
-- ============================================================================

-- ============================================================================
-- 1. ERROR TYPES TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS error_types (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  description TEXT,
  multiplier REAL DEFAULT 1.0,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Insert the 10 error types with their multipliers
INSERT OR IGNORE INTO error_types (id, name, description, multiplier) VALUES
(1, 'Conceptual Error', 'Misunderstood core concept', 1.5),
(2, 'Terminology Error', 'Mixed up definitions or terms', 1.5),
(3, 'Logical Gap', 'Missing step in reasoning', 1.5),
(4, 'Performance Error', 'Algorithm too slow', 1.5),
(5, 'Off-by-One Error', 'Boundary or indexing mistake', 1.0),
(6, 'Edge Case Error', 'Missed special cases', 1.0),
(7, 'Careless Error', 'Typo or silly mistake', 1.0),
(8, 'Implementation Error', 'Code structure issue', 1.0),
(9, 'Unoptimized', 'Works but inefficient', 0.7),
(10, 'Over Time Limit', 'Exceeded time budget', 0.7);

-- ============================================================================
-- 2. PROBLEM ERRORS TABLE
-- ============================================================================
-- Stores each error logged during an attempt
CREATE TABLE IF NOT EXISTS problem_errors (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  attempt_id INTEGER NOT NULL,
  error_type_id INTEGER NOT NULL,
  message TEXT NOT NULL,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(attempt_id) REFERENCES problem_attempts(id) ON DELETE CASCADE,
  FOREIGN KEY(error_type_id) REFERENCES error_types(id)
);

-- ============================================================================
-- 3. ERROR RESOLUTIONS TABLE
-- ============================================================================
-- Tracks how each error was fixed
CREATE TABLE IF NOT EXISTS error_resolutions (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  error_id INTEGER NOT NULL,
  resolution_notes TEXT,
  time_to_fix_seconds INTEGER,
  re_attempted INTEGER DEFAULT 0,
  successful INTEGER DEFAULT 0,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(error_id) REFERENCES problem_errors(id) ON DELETE CASCADE
);

-- ============================================================================
-- 4. ERROR IMPACT MAPPING TABLE
-- ============================================================================
-- Maps error types to their FSRS interval multipliers
CREATE TABLE IF NOT EXISTS error_impact_mapping (
  error_type_id INTEGER PRIMARY KEY,
  interval_multiplier REAL NOT NULL,
  reason TEXT,
  FOREIGN KEY(error_type_id) REFERENCES error_types(id)
);

-- Pre-populate with multiplier mappings
INSERT OR IGNORE INTO error_impact_mapping (error_type_id, interval_multiplier, reason) VALUES
(1, 1.5, 'Conceptual errors require deeper understanding'),
(2, 1.5, 'Terminology errors need definition clarity'),
(3, 1.5, 'Logical gaps need step-by-step review'),
(4, 1.5, 'Performance errors need optimization study'),
(5, 1.0, 'Off-by-one errors are edge case awareness'),
(6, 1.0, 'Edge case errors are scenario awareness'),
(7, 1.0, 'Careless errors are attention checks'),
(8, 1.0, 'Implementation errors are structure checks'),
(9, 0.7, 'Unoptimized solutions need speed drilling'),
(10, 0.7, 'Time limit errors need speed drilling');

-- ============================================================================
-- 5. PROBLEM ERROR HISTORY TABLE
-- ============================================================================
-- Aggregated error tracking per problem
CREATE TABLE IF NOT EXISTS problem_error_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  problem_id INTEGER NOT NULL,
  error_type_id INTEGER NOT NULL,
  total_occurrences INTEGER DEFAULT 1,
  resolution_count INTEGER DEFAULT 0,
  last_occurred TEXT DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(problem_id, error_type_id),
  FOREIGN KEY(problem_id) REFERENCES problems(id) ON DELETE CASCADE,
  FOREIGN KEY(error_type_id) REFERENCES error_types(id)
);

-- ============================================================================
-- 6. FSRS CARDS MODIFICATIONS
-- ============================================================================
-- Add 3 new columns to track error impact on FSRS scheduling
ALTER TABLE fsrs_cards ADD COLUMN IF NOT EXISTS error_impact_modifier REAL DEFAULT 1.0;
ALTER TABLE fsrs_cards ADD COLUMN IF NOT EXISTS resolution_success_rate REAL DEFAULT 0.0;
ALTER TABLE fsrs_cards ADD COLUMN IF NOT EXISTS has_unresolved_errors INTEGER DEFAULT 0;

-- ============================================================================
-- 7. INDEXES FOR PERFORMANCE
-- ============================================================================
-- Problem errors indexes
CREATE INDEX IF NOT EXISTS idx_problem_errors_attempt ON problem_errors(attempt_id);
CREATE INDEX IF NOT EXISTS idx_problem_errors_type ON problem_errors(error_type_id);
CREATE INDEX IF NOT EXISTS idx_problem_errors_created ON problem_errors(created_at);

-- Error resolutions indexes
CREATE INDEX IF NOT EXISTS idx_error_resolutions_error ON error_resolutions(error_id);
CREATE INDEX IF NOT EXISTS idx_error_resolutions_successful ON error_resolutions(successful);

-- Problem error history indexes
CREATE INDEX IF NOT EXISTS idx_problem_error_history_problem ON problem_error_history(problem_id);
CREATE INDEX IF NOT EXISTS idx_problem_error_history_type ON problem_error_history(error_type_id);

-- FSRS cards indexes (update existing)
CREATE INDEX IF NOT EXISTS idx_fsrs_cards_errors ON fsrs_cards(has_unresolved_errors);
CREATE INDEX IF NOT EXISTS idx_fsrs_cards_due ON fsrs_cards(due);

-- ============================================================================
-- END OF MIGRATION
-- ============================================================================
