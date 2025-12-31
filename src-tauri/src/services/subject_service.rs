use crate::models::Subject;
use sqlx::SqlitePool;

pub struct SubjectService;

impl SubjectService {
    pub async fn create(
        pool: &SqlitePool,
        name: String,
        description: Option<String>,
    ) -> Result<Subject, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO subjects (id, name, description, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&name)
        .bind(&description)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Subject {
            id,
            name,
            description,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub async fn get_by_id(pool: &SqlitePool, id: String) -> Result<Subject, String> {
        sqlx::query_as::<_, Subject>(
            "SELECT id, name, description, created_at, updated_at FROM subjects WHERE id = ?",
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn list_all(pool: &SqlitePool) -> Result<Vec<Subject>, String> {
        sqlx::query_as::<_, Subject>(
            "SELECT id, name, description, created_at, updated_at FROM subjects ORDER BY name",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn update(
        pool: &SqlitePool,
        id: String,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<Subject, String> {
        let subject = Self::get_by_id(pool, id.clone()).await?;
        let now = chrono::Utc::now().to_rfc3339();

        let new_name = name.unwrap_or(subject.name.clone());
        let new_desc = description.or(subject.description.clone());

        sqlx::query("UPDATE subjects SET name = ?, description = ?, updated_at = ? WHERE id = ?")
            .bind(&new_name)
            .bind(&new_desc)
            .bind(&now)
            .bind(&id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Subject {
            id,
            name: new_name,
            description: new_desc,
            created_at: subject.created_at,
            updated_at: now,
        })
    }

    pub async fn delete(pool: &SqlitePool, id: String) -> Result<(), String> {
        sqlx::query("DELETE FROM subjects WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
