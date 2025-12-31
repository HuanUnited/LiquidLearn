-- Subjects (e.g., "Differential Equations")
CREATE TABLE subjects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Topics (e.g., "1st Order ODEs")
CREATE TABLE topics (
    id TEXT PRIMARY KEY,
    subject_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    UNIQUE(subject_id, name)
);

-- Theory content (phases and explanations)
CREATE TABLE theories (
    id TEXT PRIMARY KEY,
    topic_id TEXT NOT NULL,
    phase_number INTEGER NOT NULL, -- 1.1, 1.2, 2.1, 2.2, 3, 4
    title TEXT NOT NULL,
    content TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (topic_id) REFERENCES topics(id) ON DELETE CASCADE
);

-- Problems (the actual questions)
CREATE TABLE problems (
    id TEXT PRIMARY KEY,
    topic_id TEXT NOT NULL,
    theory_id TEXT, -- Link to specific theory
    title TEXT NOT NULL,
    description TEXT,
    image_url TEXT, -- Optional image
    difficulty INTEGER DEFAULT 1, -- 1-5
    is_solved BOOLEAN DEFAULT 0,
    total_unresolved_errors INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (topic_id) REFERENCES topics(id) ON DELETE CASCADE,
    FOREIGN KEY (theory_id) REFERENCES theories(id) ON DELETE SET NULL
);

-- Attempts (each time someone tries to solve a problem)
CREATE TABLE attempts (
    id TEXT PRIMARY KEY,
    problem_id TEXT NOT NULL,
    is_solved BOOLEAN NOT NULL,
    commentary TEXT, -- Resolution notes, approach, etc.
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE
);

-- Errors (only logged if attempt is NOT solved)
CREATE TABLE attempt_errors (
    id TEXT PRIMARY KEY,
    attempt_id TEXT NOT NULL,
    error_type_id INTEGER NOT NULL,
    description TEXT,
    is_resolved BOOLEAN DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (attempt_id) REFERENCES attempts(id) ON DELETE CASCADE,
    FOREIGN KEY (error_type_id) REFERENCES error_types(id) ON DELETE RESTRICT
);

-- Error types (predefined)
CREATE TABLE error_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    multiplier REAL DEFAULT 1.0,
    created_at TEXT NOT NULL
);

-- FSRS Cards (spaced repetition representation of problems)
CREATE TABLE fsrs_cards (
    id TEXT PRIMARY KEY,
    problem_id TEXT NOT NULL UNIQUE,
    due TEXT NOT NULL, -- Next review date
    stability REAL DEFAULT 1.0,
    difficulty REAL DEFAULT 5.0,
    state TEXT DEFAULT 'new', -- new, learning, review, relearning
    reps INTEGER DEFAULT 0,
    lapses INTEGER DEFAULT 0,
    elapsed_days INTEGER DEFAULT 0,
    scheduled_days INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE
);

-- Indices for performance
CREATE INDEX idx_topics_subject ON topics(subject_id);
CREATE INDEX idx_theories_topic ON theories(topic_id);
CREATE INDEX idx_problems_topic ON problems(topic_id);
CREATE INDEX idx_attempts_problem ON attempts(problem_id);
CREATE INDEX idx_errors_attempt ON attempt_errors(attempt_id);
CREATE INDEX idx_fsrs_due ON fsrs_cards(due);
CREATE INDEX idx_fsrs_problem ON fsrs_cards(problem_id);
