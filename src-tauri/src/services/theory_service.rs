use crate::models::Theory;
use sqlx::SqlitePool;

pub struct TheoryService;

impl TheoryService {
    pub async fn create(
        pool: &SqlitePool,
        topic_id: String,
        phase_number: i32,
        title: String,
        content: Option<String>,
    ) -> Result<Theory, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO theories (id, topic_id, phase_number, title, content, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&topic_id)
        .bind(phase_number)
        .bind(&title)
        .bind(&content)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Theory {
            id,
            topic_id,
            phase_number,
            title,
            content,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub async fn get_by_id(pool: &SqlitePool, id: String) -> Result<Theory, String> {
        sqlx::query_as::<_, Theory>(
            "SELECT id, topic_id, phase_number, title, content, created_at, updated_at FROM theories WHERE id = ?"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn list_by_topic(pool: &SqlitePool, topic_id: String) -> Result<Vec<Theory>, String> {
        sqlx::query_as::<_, Theory>(
            "SELECT id, topic_id, phase_number, title, content, created_at, updated_at 
             FROM theories WHERE topic_id = ? ORDER BY phase_number",
        )
        .bind(topic_id)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn get_by_phase(
        pool: &SqlitePool,
        topic_id: String,
        phase_number: i32,
    ) -> Result<Option<Theory>, String> {
        sqlx::query_as::<_, Theory>(
            "SELECT id, topic_id, phase_number, title, content, created_at, updated_at 
             FROM theories WHERE topic_id = ? AND phase_number = ?",
        )
        .bind(topic_id)
        .bind(phase_number)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn update(
        pool: &SqlitePool,
        id: String,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Theory, String> {
        let theory = Self::get_by_id(pool, id.clone()).await?;
        let now = chrono::Utc::now().to_rfc3339();

        let new_title = title.unwrap_or(theory.title.clone());
        let new_content = content.or(theory.content.clone());

        sqlx::query("UPDATE theories SET title = ?, content = ?, updated_at = ? WHERE id = ?")
            .bind(&new_title)
            .bind(&new_content)
            .bind(&now)
            .bind(&id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Theory {
            id,
            topic_id: theory.topic_id,
            phase_number: theory.phase_number,
            title: new_title,
            content: new_content,
            created_at: theory.created_at,
            updated_at: now,
        })
    }

    pub async fn delete(pool: &SqlitePool, id: String) -> Result<(), String> {
        sqlx::query("DELETE FROM theories WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
