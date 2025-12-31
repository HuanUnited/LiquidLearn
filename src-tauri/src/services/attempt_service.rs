use crate::models::Attempt;
use sqlx::SqlitePool;

pub struct AttemptService;

impl AttemptService {
    pub async fn create(
        pool: &SqlitePool,
        problem_id: String,
        is_solved: bool,
        commentary: Option<String>,
    ) -> Result<Attempt, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO attempts (id, problem_id, is_solved, commentary, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&problem_id)
        .bind(is_solved as i32)
        .bind(&commentary)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Attempt {
            id,
            problem_id,
            is_solved,
            commentary,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub async fn get_by_id(pool: &SqlitePool, id: String) -> Result<Attempt, String> {
        sqlx::query_as::<_, Attempt>(
            "SELECT id, problem_id, is_solved, commentary, created_at, updated_at FROM attempts WHERE id = ?"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn list_by_problem(
        pool: &SqlitePool,
        problem_id: String,
    ) -> Result<Vec<Attempt>, String> {
        sqlx::query_as::<_, Attempt>(
            "SELECT id, problem_id, is_solved, commentary, created_at, updated_at 
             FROM attempts WHERE problem_id = ? ORDER BY created_at DESC",
        )
        .bind(problem_id)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn get_count_by_problem(
        pool: &SqlitePool,
        problem_id: String,
    ) -> Result<i64, String> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM attempts WHERE problem_id = ?")
            .bind(problem_id)
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(count)
    }

    pub async fn get_success_count_by_problem(
        pool: &SqlitePool,
        problem_id: String,
    ) -> Result<i64, String> {
        let (count,): (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM attempts WHERE problem_id = ? AND is_solved = 1")
                .bind(problem_id)
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;

        Ok(count)
    }

    pub async fn update_commentary(
        pool: &SqlitePool,
        id: String,
        commentary: String,
    ) -> Result<Attempt, String> {
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query("UPDATE attempts SET commentary = ?, updated_at = ? WHERE id = ?")
            .bind(&commentary)
            .bind(&now)
            .bind(&id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Self::get_by_id(pool, id).await
    }
}
