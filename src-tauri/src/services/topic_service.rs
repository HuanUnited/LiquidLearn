use crate::models::Topic;
use sqlx::SqlitePool;

pub struct TopicService;

impl TopicService {
    pub async fn create(
        pool: &SqlitePool,
        subject_id: String,
        name: String,
        description: Option<String>,
    ) -> Result<Topic, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO topics (id, subject_id, name, description, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&subject_id)
        .bind(&name)
        .bind(&description)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Topic {
            id,
            subject_id,
            name,
            description,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub async fn get_by_id(pool: &SqlitePool, id: String) -> Result<Topic, String> {
        sqlx::query_as::<_, Topic>(
            "SELECT id, subject_id, name, description, created_at, updated_at FROM topics WHERE id = ?"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn list_by_subject(
        pool: &SqlitePool,
        subject_id: String,
    ) -> Result<Vec<Topic>, String> {
        sqlx::query_as::<_, Topic>(
            "SELECT id, subject_id, name, description, created_at, updated_at 
             FROM topics WHERE subject_id = ? ORDER BY name",
        )
        .bind(subject_id)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn update(
        pool: &SqlitePool,
        id: String,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<Topic, String> {
        let topic = Self::get_by_id(pool, id.clone()).await?;
        let now = chrono::Utc::now().to_rfc3339();

        let new_name = name.unwrap_or(topic.name.clone());
        let new_desc = description.or(topic.description.clone());

        sqlx::query("UPDATE topics SET name = ?, description = ?, updated_at = ? WHERE id = ?")
            .bind(&new_name)
            .bind(&new_desc)
            .bind(&now)
            .bind(&id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Topic {
            id,
            subject_id: topic.subject_id,
            name: new_name,
            description: new_desc,
            created_at: topic.created_at,
            updated_at: now,
        })
    }

    pub async fn delete(pool: &SqlitePool, id: String) -> Result<(), String> {
        sqlx::query("DELETE FROM topics WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
