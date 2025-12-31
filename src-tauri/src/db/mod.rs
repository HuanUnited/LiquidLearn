use sqlx::sqlite::SqlitePool;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = Self::get_db_path();

        // Create directory if needed
        std::fs::create_dir_all(db_path.parent().unwrap())?;

        // Create database URL
        let database_url = format!("sqlite://{}?mode=rwc", db_path.display());

        // Create pool
        let pool = SqlitePool::connect(&database_url).await?;

        // Initialize schema
        Self::init_schema(&pool).await?;

        Ok(Database { pool })
    }

    fn get_db_path() -> PathBuf {
        let app_data = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(app_data)
            .join("LiquidLearn")
            .join("liquidlearn.db")
    }

    async fn init_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        // Create problems table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS problems (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                difficulty INTEGER CHECK(difficulty >= 1 AND difficulty <= 5),
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create indexes
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_problems_difficulty ON problems(difficulty)")
            .execute(pool)
            .await?;

        // Create problem_tags table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS problem_tags (
                id TEXT PRIMARY KEY,
                problem_id TEXT NOT NULL,
                tag_name TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE,
                UNIQUE(problem_id, tag_name)
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create fsrs_cards table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS fsrs_cards (
                id TEXT PRIMARY KEY,
                problem_id TEXT NOT NULL UNIQUE,
                due DATE NOT NULL DEFAULT CURRENT_DATE,
                stability REAL NOT NULL DEFAULT 0.0,
                difficulty REAL NOT NULL DEFAULT 5.0,
                state TEXT NOT NULL DEFAULT 'new',
                reps INTEGER NOT NULL DEFAULT 0,
                lapses INTEGER NOT NULL DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create problem_mastery table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS problem_mastery (
                id TEXT PRIMARY KEY,
                problem_id TEXT NOT NULL UNIQUE,
                solved BOOLEAN DEFAULT 0,
                mastery_percent REAL DEFAULT 0,
                attempt_count INTEGER DEFAULT 0,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create fsrs_parameters table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS fsrs_parameters (
                id TEXT PRIMARY KEY,
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
                desired_retention REAL DEFAULT 0.95,
                total_reviews INTEGER DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Insert default parameters if not exists
        sqlx::query("INSERT OR IGNORE INTO fsrs_parameters (id) VALUES ('global')")
            .execute(pool)
            .await?;

        // Create study_phase_progress table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS study_phase_progress (
                id TEXT PRIMARY KEY,
                problem_id TEXT NOT NULL UNIQUE,
                current_phase INTEGER NOT NULL DEFAULT 1,
                current_step INTEGER NOT NULL DEFAULT 1,
                time_spent_seconds INTEGER NOT NULL DEFAULT 0,
                all_phases_completed BOOLEAN DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create fsrs_reviews table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS fsrs_reviews (
                id TEXT PRIMARY KEY,
                card_id TEXT NOT NULL,
                problem_id TEXT NOT NULL,
                rating INTEGER NOT NULL CHECK(rating >= 1 AND rating <= 10),
                state_before TEXT NOT NULL,
                elapsed_seconds INTEGER NOT NULL DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (card_id) REFERENCES fsrs_cards(id) ON DELETE CASCADE,
                FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create settings table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Insert default settings
        sqlx::query("INSERT OR IGNORE INTO settings (key, value) VALUES ('app_version', '1.0.0')")
            .execute(pool)
            .await?;

        // Create FTS5 virtual table for full-text search
        sqlx::query(
            r#"
            CREATE VIRTUAL TABLE IF NOT EXISTS problems_fts USING fts5(
                id UNINDEXED,
                title,
                description,
                tags
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create trigger to auto-update FTS on insert
        sqlx::query(
            r#"
            CREATE TRIGGER IF NOT EXISTS problems_ai AFTER INSERT ON problems BEGIN
              INSERT INTO problems_fts(id, title, description) 
              VALUES (new.id, new.title, new.description);
            END
            "#,
        )
        .execute(pool)
        .await?;

        // Create trigger to auto-update FTS on delete
        sqlx::query(
            r#"
            CREATE TRIGGER IF NOT EXISTS problems_ad AFTER DELETE ON problems BEGIN
              INSERT INTO problems_fts(problems_fts, id, title, description) 
              VALUES('delete', old.id, old.title, old.description);
            END
            "#,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
