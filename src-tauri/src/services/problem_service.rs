use crate::models::Problem;
use sqlx::SqlitePool;

pub struct ProblemService;

impl ProblemService {
    pub async fn create(
        pool: &SqlitePool,
        topic_id: String,
        theory_id: Option<String>,
        title: String,
        description: Option<String>,
        image_url: Option<String>,
        difficulty: i32,
    ) -> Result<Problem, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO problems (id, topic_id, theory_id, title, description, image_url, difficulty, is_solved, total_unresolved_errors, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, 0, 0, ?, ?)"
        )
        .bind(&id)
        .bind(&topic_id)
        .bind(&theory_id)
        .bind(&title)
        .bind(&description)
        .bind(&image_url)
        .bind(difficulty)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Problem {
            id,
            topic_id,
            theory_id,
            title,
            description,
            image_url,
            difficulty,
            is_solved: false,
            total_unresolved_errors: 0,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub async fn get_by_id(pool: &SqlitePool, id: String) -> Result<Problem, String> {
        sqlx::query_as::<_, Problem>(
            "SELECT id, topic_id, theory_id, title, description, image_url, difficulty, is_solved, total_unresolved_errors, created_at, updated_at FROM problems WHERE id = ?"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn list_by_topic(
        pool: &SqlitePool,
        topic_id: String,
    ) -> Result<Vec<Problem>, String> {
        sqlx::query_as::<_, Problem>(
            "SELECT id, topic_id, theory_id, title, description, image_url, difficulty, is_solved, total_unresolved_errors, created_at, updated_at 
             FROM problems WHERE topic_id = ? ORDER BY created_at DESC"
        )
        .bind(topic_id)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn list_by_theory(
        pool: &SqlitePool,
        theory_id: String,
    ) -> Result<Vec<Problem>, String> {
        sqlx::query_as::<_, Problem>(
            "SELECT id, topic_id, theory_id, title, description, image_url, difficulty, is_solved, total_unresolved_errors, created_at, updated_at 
             FROM problems WHERE theory_id = ? ORDER BY created_at DESC"
        )
        .bind(theory_id)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn mark_solved(pool: &SqlitePool, id: String) -> Result<Problem, String> {
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query("UPDATE problems SET is_solved = 1, updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(&id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Self::get_by_id(pool, id).await
    }

    pub async fn update_unresolved_errors(
        pool: &SqlitePool,
        problem_id: String,
        count: i32,
    ) -> Result<(), String> {
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query("UPDATE problems SET total_unresolved_errors = ?, updated_at = ? WHERE id = ?")
            .bind(count)
            .bind(&now)
            .bind(&problem_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn get_unresolved_error_count(
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

    pub async fn update(
        pool: &SqlitePool,
        id: String,
        title: Option<String>,
        description: Option<String>,
        image_url: Option<String>,
        difficulty: Option<i32>,
    ) -> Result<Problem, String> {
        let problem = Self::get_by_id(pool, id.clone()).await?;
        let now = chrono::Utc::now().to_rfc3339();

        let new_title = title.unwrap_or(problem.title.clone());
        let new_desc = description.or(problem.description.clone());
        let new_image = image_url.or(problem.image_url.clone());
        let new_difficulty = difficulty.unwrap_or(problem.difficulty);

        sqlx::query(
            "UPDATE problems SET title = ?, description = ?, image_url = ?, difficulty = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&new_title)
        .bind(&new_desc)
        .bind(&new_image)
        .bind(new_difficulty)
        .bind(&now)
        .bind(&id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Problem {
            id,
            topic_id: problem.topic_id,
            theory_id: problem.theory_id,
            title: new_title,
            description: new_desc,
            image_url: new_image,
            difficulty: new_difficulty,
            is_solved: problem.is_solved,
            total_unresolved_errors: problem.total_unresolved_errors,
            created_at: problem.created_at,
            updated_at: now,
        })
    }

    pub async fn delete(pool: &SqlitePool, id: String) -> Result<(), String> {
        sqlx::query("DELETE FROM problems WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
