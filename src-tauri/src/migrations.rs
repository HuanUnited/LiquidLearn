use sqlx::SqlitePool;

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Subjects table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS subjects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            description TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Topics table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS topics (
            id TEXT PRIMARY KEY,
            subject_id TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
            UNIQUE(subject_id, name)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Theories table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS theories (
            id TEXT PRIMARY KEY,
            topic_id TEXT NOT NULL,
            phase_number INTEGER NOT NULL,
            title TEXT NOT NULL,
            content TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (topic_id) REFERENCES topics(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Problems table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS problems (
            id TEXT PRIMARY KEY,
            topic_id TEXT NOT NULL,
            theory_id TEXT,
            title TEXT NOT NULL,
            description TEXT,
            image_url TEXT,
            difficulty INTEGER DEFAULT 1,
            is_solved BOOLEAN DEFAULT 0,
            total_unresolved_errors INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (topic_id) REFERENCES topics(id) ON DELETE CASCADE,
            FOREIGN KEY (theory_id) REFERENCES theories(id) ON DELETE SET NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Attempts table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS attempts (
            id TEXT PRIMARY KEY,
            problem_id TEXT NOT NULL,
            is_solved BOOLEAN NOT NULL,
            commentary TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Error types table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS error_types (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            description TEXT,
            multiplier REAL DEFAULT 1.0,
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Attempt errors table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS attempt_errors (
            id TEXT PRIMARY KEY,
            attempt_id TEXT NOT NULL,
            error_type_id INTEGER NOT NULL,
            description TEXT,
            is_resolved BOOLEAN DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (attempt_id) REFERENCES attempts(id) ON DELETE CASCADE,
            FOREIGN KEY (error_type_id) REFERENCES error_types(id) ON DELETE RESTRICT
        )
        "#,
    )
    .execute(pool)
    .await?;

    // FSRS Cards table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS fsrs_cards (
            id TEXT PRIMARY KEY,
            problem_id TEXT NOT NULL UNIQUE,
            due TEXT NOT NULL,
            stability REAL DEFAULT 1.0,
            difficulty REAL DEFAULT 5.0,
            state TEXT DEFAULT 'new',
            reps INTEGER DEFAULT 0,
            lapses INTEGER DEFAULT 0,
            elapsed_days INTEGER DEFAULT 0,
            scheduled_days INTEGER DEFAULT 1,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create indices for performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_topics_subject ON topics(subject_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_theories_topic ON theories(topic_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_problems_topic ON problems(topic_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_problems_theory ON problems(theory_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_attempts_problem ON attempts(problem_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_errors_attempt ON attempt_errors(attempt_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_fsrs_due ON fsrs_cards(due)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_fsrs_problem ON fsrs_cards(problem_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_fsrs_state ON fsrs_cards(state)")
        .execute(pool)
        .await?;

    Ok(())
}
