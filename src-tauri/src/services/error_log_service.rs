use crate::models::{AttemptError, ErrorType};
use sqlx::SqlitePool;

pub struct ErrorService;

impl ErrorService {
    /// Initialize default error types on first run
    pub async fn init_default_error_types(pool: &SqlitePool) -> Result<(), String> {
        let defaults = crate::models::error_log::get_default_error_types();

        for (id, name, desc, multiplier) in defaults {
            let now = chrono::Utc::now().to_rfc3339();

            // Skip if already exists
            let exists: Result<(i32,), _> =
                sqlx::query_as("SELECT id FROM error_types WHERE id = ?")
                    .bind(id)
                    .fetch_one(pool)
                    .await;

            if exists.is_ok() {
                continue;
            }

            sqlx::query(
                "INSERT INTO error_types (id, name, description, multiplier, created_at) 
                 VALUES (?, ?, ?, ?, ?)",
            )
            .bind(id)
            .bind(name)
            .bind(desc)
            .bind(multiplier)
            .bind(&now)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub async fn log_error(
        pool: &SqlitePool,
        attempt_id: String,
        error_type_id: i32,
        description: Option<String>,
    ) -> Result<AttemptError, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO attempt_errors (id, attempt_id, error_type_id, description, is_resolved, created_at, updated_at) 
             VALUES (?, ?, ?, ?, 0, ?, ?)"
        )
        .bind(&id)
        .bind(&attempt_id)
        .bind(error_type_id)
        .bind(&description)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(AttemptError {
            id,
            attempt_id,
            error_type_id,
            description,
            is_resolved: false,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub async fn resolve_error(
        pool: &SqlitePool,
        error_id: String,
    ) -> Result<AttemptError, String> {
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query("UPDATE attempt_errors SET is_resolved = 1, updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(&error_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Self::get_error_by_id(pool, error_id).await
    }

    pub async fn get_error_by_id(pool: &SqlitePool, id: String) -> Result<AttemptError, String> {
        sqlx::query_as::<_, AttemptError>(
            "SELECT id, attempt_id, error_type_id, description, is_resolved, created_at, updated_at FROM attempt_errors WHERE id = ?"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn get_errors_by_attempt(
        pool: &SqlitePool,
        attempt_id: String,
    ) -> Result<Vec<AttemptError>, String> {
        sqlx::query_as::<_, AttemptError>(
            "SELECT id, attempt_id, error_type_id, description, is_resolved, created_at, updated_at 
             FROM attempt_errors WHERE attempt_id = ? ORDER BY created_at DESC"
        )
        .bind(attempt_id)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn get_unresolved_errors_by_problem(
        pool: &SqlitePool,
        problem_id: String,
    ) -> Result<Vec<AttemptError>, String> {
        sqlx::query_as::<_, AttemptError>(
            "SELECT ae.id, ae.attempt_id, ae.error_type_id, ae.description, ae.is_resolved, ae.created_at, ae.updated_at
             FROM attempt_errors ae
             JOIN attempts a ON ae.attempt_id = a.id
             WHERE a.problem_id = ? AND ae.is_resolved = 0
             ORDER BY ae.created_at DESC"
        )
        .bind(problem_id)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn get_error_types(pool: &SqlitePool) -> Result<Vec<ErrorType>, String> {
        sqlx::query_as::<_, ErrorType>(
            "SELECT id, name, description, multiplier, created_at FROM error_types ORDER BY multiplier DESC"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }

    #[allow(dead_code)]
    pub async fn get_error_type_by_id(pool: &SqlitePool, id: i32) -> Result<ErrorType, String> {
        sqlx::query_as::<_, ErrorType>(
            "SELECT id, name, description, multiplier, created_at FROM error_types WHERE id = ?",
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn count_unresolved_by_problem(
        pool: &SqlitePool,
        problem_id: String,
    ) -> Result<i32, String> {
        let (count,): (i32,) = sqlx::query_as(
            "SELECT COUNT(*) FROM attempt_errors ae
             JOIN attempts a ON ae.attempt_id = a.id
             WHERE a.problem_id = ? AND ae.is_resolved = 0",
        )
        .bind(&problem_id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(count)
    }
}
