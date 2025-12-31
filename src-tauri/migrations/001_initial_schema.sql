-- Migration 001: Initial Schema
-- Date: 2025-12-31
-- LiquidLearn Database Schema (9 tables, no auth)

-- 1. PROBLEMS TABLE
CREATE TABLE IF NOT EXISTS problems (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    difficulty INTEGER CHECK(difficulty >= 1 AND difficulty <= 5),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_problems_difficulty ON problems(difficulty);
CREATE INDEX IF NOT EXISTS idx_problems_created_at ON problems(created_at);

-- 2. PROBLEM TAGS TABLE
CREATE TABLE IF NOT EXISTS problem_tags (
    id TEXT PRIMARY KEY,
    problem_id TEXT NOT NULL,
    tag_name TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE,
    UNIQUE(problem_id, tag_name)
);

CREATE INDEX IF NOT EXISTS idx_problem_tags_tag_name ON problem_tags(tag_name);
CREATE INDEX IF NOT EXISTS idx_problem_tags_problem_id ON problem_tags(problem_id);

-- 3. PROBLEM MASTERY TABLE
CREATE TABLE IF NOT EXISTS problem_mastery (
    id TEXT PRIMARY KEY,
    problem_id TEXT NOT NULL UNIQUE,
    solved BOOLEAN DEFAULT 0,
    mastery_percent REAL DEFAULT 0 CHECK(mastery_percent >= 0 AND mastery_percent <= 100),
    last_attempted DATETIME,
    attempt_count INTEGER DEFAULT 0,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_problem_mastery_solved ON problem_mastery(solved);
CREATE INDEX IF NOT EXISTS idx_problem_mastery_mastery_percent ON problem_mastery(mastery_percent);
CREATE INDEX IF NOT EXISTS idx_problem_mastery_problem_id ON problem_mastery(problem_id);

-- 4. FSRS CARDS TABLE
CREATE TABLE IF NOT EXISTS fsrs_cards (
    id TEXT PRIMARY KEY,
    problem_id TEXT NOT NULL,
    due DATE NOT NULL DEFAULT CURRENT_DATE,
    stability REAL NOT NULL DEFAULT 0.0,
    difficulty REAL NOT NULL DEFAULT 5.0 CHECK(difficulty >= 1.0 AND difficulty <= 10.0),
    state TEXT NOT NULL DEFAULT 'new' CHECK(state IN ('new', 'learning', 'review', 'relearning')),
    reps INTEGER NOT NULL DEFAULT 0,
    lapses INTEGER NOT NULL DEFAULT 0,
    last_review DATETIME,
    last_elapsed_days INTEGER DEFAULT 0,
    scheduled_days INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE,
    UNIQUE(problem_id)
);

CREATE INDEX IF NOT EXISTS idx_fsrs_cards_due ON fsrs_cards(due);
CREATE INDEX IF NOT EXISTS idx_fsrs_cards_state ON fsrs_cards(state);
CREATE INDEX IF NOT EXISTS idx_fsrs_cards_last_review ON fsrs_cards(last_review);
CREATE INDEX IF NOT EXISTS idx_fsrs_cards_problem_id ON fsrs_cards(problem_id);

-- 5. FSRS REVIEWS TABLE
CREATE TABLE IF NOT EXISTS fsrs_reviews (
    id TEXT PRIMARY KEY,
    card_id TEXT NOT NULL,
    problem_id TEXT NOT NULL,
    rating INTEGER NOT NULL CHECK(rating >= 1 AND rating <= 10),
    state_before TEXT NOT NULL,
    elapsed_seconds INTEGER NOT NULL DEFAULT 0,
    elapsed_days INTEGER NOT NULL DEFAULT 0,
    scheduled_days_before INTEGER DEFAULT 0,
    scheduled_days_after INTEGER DEFAULT 0,
    ease REAL DEFAULT 0.0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (card_id) REFERENCES fsrs_cards(id) ON DELETE CASCADE,
    FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_fsrs_reviews_created_at ON fsrs_reviews(created_at);
CREATE INDEX IF NOT EXISTS idx_fsrs_reviews_card_id ON fsrs_reviews(card_id);
CREATE INDEX IF NOT EXISTS idx_fsrs_reviews_rating ON fsrs_reviews(rating);
CREATE INDEX IF NOT EXISTS idx_fsrs_reviews_problem_id ON fsrs_reviews(problem_id);

-- 6. FSRS PARAMETERS TABLE (Single Global Record)
CREATE TABLE IF NOT EXISTS fsrs_parameters (
    id TEXT PRIMARY KEY DEFAULT 'global',
    w_1 REAL DEFAULT 0.40,
    w_2 REAL DEFAULT 1.86,
    w_3 REAL DEFAULT 4.93,
    w_4 REAL DEFAULT 0.94,
    w_5 REAL DEFAULT 0.86,
    w_6 REAL DEFAULT 0.01,
    w_7 REAL DEFAULT 1.49,
    w_8 REAL DEFAULT 0.04,
    w_9 REAL DEFAULT 0.36,
    w_10 REAL DEFAULT 0.86,
    w_11 REAL DEFAULT 0.20,
    w_12 REAL DEFAULT 2.50,
    w_13 REAL DEFAULT 0.14,
    w_14 REAL DEFAULT 0.94,
    w_15 REAL DEFAULT 0.16,
    w_16 REAL DEFAULT 0.10,
    w_17 REAL DEFAULT 0.29,
    w_18 REAL DEFAULT 0.34,
    w_19 REAL DEFAULT 3.73,
    desired_retention REAL DEFAULT 0.95 CHECK(desired_retention >= 0.80 AND desired_retention <= 0.99),
    total_reviews INTEGER DEFAULT 0,
    last_calibrated DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 7. STUDY PHASE PROGRESS TABLE
CREATE TABLE IF NOT EXISTS study_phase_progress (
    id TEXT PRIMARY KEY,
    problem_id TEXT NOT NULL UNIQUE,
    current_phase INTEGER NOT NULL DEFAULT 1 CHECK(current_phase >= 1 AND current_phase <= 4),
    current_step INTEGER NOT NULL DEFAULT 1 CHECK(current_step >= 1 AND current_step <= 3),
    time_spent_seconds INTEGER NOT NULL DEFAULT 0,
    phase_1_completed BOOLEAN DEFAULT 0,
    phase_2_completed BOOLEAN DEFAULT 0,
    phase_3_completed BOOLEAN DEFAULT 0,
    phase_4_completed BOOLEAN DEFAULT 0,
    all_phases_completed BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_study_phase_progress_problem_id ON study_phase_progress(problem_id);
CREATE INDEX IF NOT EXISTS idx_study_phase_progress_completed ON study_phase_progress(all_phases_completed);

-- 8. SETTINGS TABLE
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Initialize default global parameters and settings
INSERT OR IGNORE INTO fsrs_parameters (id) VALUES ('global');

INSERT OR IGNORE INTO settings (key, value) VALUES
('app_version', '1.0.0'),
('auto_calibrate', 'true'),
('auto_start_phases', 'true'),
('show_hints', 'true'),
('theme', 'light');
