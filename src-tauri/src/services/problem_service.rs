use crate::models::{CreateProblemRequest, Problem};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct ProblemService;

impl ProblemService {
    /// Create a new problem in the database
    pub async fn create(
        request: CreateProblemRequest,
        pool: &SqlitePool,
    ) -> Result<Problem, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        // Insert problem
        let problem = sqlx::query_as::<_, Problem>(
            "INSERT INTO problems (id, title, description, difficulty, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)
             RETURNING id, title, description, difficulty, created_at, updated_at",
        )
        .bind(&id)
        .bind(&request.title)
        .bind(&request.description)
        .bind(request.difficulty)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await?;

        // Create associated FSRS card
        Self::create_fsrs_card(&id, pool).await?;

        // Create associated study phase progress
        Self::create_study_phase(&id, pool).await?;

        // Create associated problem mastery
        Self::create_mastery(&id, pool).await?;

        Ok(problem)
    }

    /// Get a problem by ID
    pub async fn get(id: &str, pool: &SqlitePool) -> Result<Problem, sqlx::Error> {
        sqlx::query_as::<_, Problem>(
            "SELECT id, title, description, difficulty, created_at, updated_at 
             FROM problems 
             WHERE id = ?",
        )
        .bind(id)
        .fetch_one(pool)
        .await
    }

    /// List all problems with optional filtering
    pub async fn list(
        pool: &SqlitePool,
        difficulty: Option<i32>,
        solved_only: Option<bool>,
    ) -> Result<Vec<Problem>, sqlx::Error> {
        let mut query = String::from(
            "SELECT p.id, p.title, p.description, p.difficulty, p.created_at, p.updated_at 
             FROM problems p",
        );

        if solved_only.unwrap_or(false) {
            query.push_str(
                " JOIN problem_mastery pm ON p.id = pm.problem_id 
                  WHERE pm.solved = 1",
            );
        }

        if let Some(diff) = difficulty {
            if solved_only.is_some() {
                query.push_str(&format!(" AND p.difficulty = {}", diff));
            } else {
                query.push_str(&format!(" WHERE p.difficulty = {}", diff));
            }
        }

        query.push_str(" ORDER BY p.created_at DESC");

        sqlx::query_as::<_, Problem>(&query).fetch_all(pool).await
    }

    /// Delete a problem (cascades to related records)
    pub async fn delete(id: &str, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM problems WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Update an existing problem
    pub async fn update(
        id: &str,
        title: Option<String>,
        description: Option<String>,
        difficulty: Option<i32>,
        pool: &SqlitePool,
    ) -> Result<Problem, sqlx::Error> {
        // Get current problem
        let current = Self::get(id, pool).await?;

        // Use provided values or keep existing
        let new_title = title.unwrap_or(current.title);
        let new_description = description.or(current.description);
        let new_difficulty = difficulty.unwrap_or(current.difficulty);
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query_as::<_, Problem>(
            "UPDATE problems 
             SET title = ?, description = ?, difficulty = ?, updated_at = ?
             WHERE id = ?
             RETURNING id, title, description, difficulty, created_at, updated_at",
        )
        .bind(&new_title)
        .bind(&new_description)
        .bind(new_difficulty)
        .bind(&now)
        .bind(id)
        .fetch_one(pool)
        .await
    }

    /// Add a tag to a problem
    pub async fn add_tag(
        problem_id: &str,
        tag_name: &str,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT OR IGNORE INTO problem_tags (id, problem_id, tag_name)
             VALUES (?, ?, ?)",
        )
        .bind(&id)
        .bind(problem_id)
        .bind(tag_name)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Remove a tag from a problem
    pub async fn remove_tag(
        problem_id: &str,
        tag_name: &str,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM problem_tags WHERE problem_id = ? AND tag_name = ?")
            .bind(problem_id)
            .bind(tag_name)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Get all tags for a problem
    pub async fn get_tags(problem_id: &str, pool: &SqlitePool) -> Result<Vec<String>, sqlx::Error> {
        let tags: Vec<(String,)> = sqlx::query_as(
            "SELECT tag_name FROM problem_tags WHERE problem_id = ? ORDER BY tag_name",
        )
        .bind(problem_id)
        .fetch_all(pool)
        .await?;

        Ok(tags.into_iter().map(|(tag,)| tag).collect())
    }

    /// Full-text search problems
    pub async fn search(query: &str, pool: &SqlitePool) -> Result<Vec<Problem>, sqlx::Error> {
        sqlx::query_as::<_, Problem>(
            "SELECT p.id, p.title, p.description, p.difficulty, p.created_at, p.updated_at
             FROM problems p
             WHERE p.id IN (
                 SELECT id FROM problems_fts WHERE problems_fts MATCH ?
             )
             ORDER BY rank
             LIMIT 100",
        )
        .bind(format!("{}*", query)) // Prefix search
        .fetch_all(pool)
        .await
    }

    /// Get problems with multiple filters
    pub async fn filter(
        difficulty: Option<i32>,
        tags: Option<Vec<String>>,
        solved_only: Option<bool>,
        pool: &SqlitePool,
    ) -> Result<Vec<Problem>, sqlx::Error> {
        let mut query = String::from(
            "SELECT DISTINCT p.id, p.title, p.description, p.difficulty, p.created_at, p.updated_at
             FROM problems p",
        );

        let mut has_where = false;

        // Add tag filter
        if let Some(tag_list) = &tags {
            if !tag_list.is_empty() {
                query.push_str(" LEFT JOIN problem_tags pt ON p.id = pt.problem_id");
                query.push_str(" WHERE pt.tag_name IN (");
                for (i, _) in tag_list.iter().enumerate() {
                    if i > 0 {
                        query.push(',');
                    }
                    query.push('?');
                }
                query.push(')');
                has_where = true;
            }
        }

        // Add solved filter
        if solved_only.unwrap_or(false) {
            if has_where {
                query.push_str(
                    " AND p.id IN (SELECT problem_id FROM problem_mastery WHERE solved = 1)",
                );
            } else {
                query.push_str(
                    " WHERE p.id IN (SELECT problem_id FROM problem_mastery WHERE solved = 1)",
                );
                has_where = true;
            }
        }

        // Add difficulty filter
        if let Some(diff) = difficulty {
            if has_where {
                query.push_str(&format!(" AND p.difficulty = {}", diff));
            } else {
                query.push_str(&format!(" WHERE p.difficulty = {}", diff));
            }
        }

        query.push_str(" ORDER BY p.created_at DESC");

        sqlx::query_as::<_, Problem>(&query).fetch_all(pool).await
    }

    /// Bulk create problems from CSV-like data
    pub async fn bulk_create(
        problems_data: Vec<CreateProblemRequest>,
        pool: &SqlitePool,
    ) -> Result<Vec<Problem>, sqlx::Error> {
        let mut created = Vec::new();

        for request in problems_data {
            match Self::create(request, pool).await {
                Ok(problem) => created.push(problem),
                Err(_) => continue, // Skip failed inserts
            }
        }

        Ok(created)
    }

    /// Get problem count by difficulty
    pub async fn count_by_difficulty(pool: &SqlitePool) -> Result<Vec<(i32, i64)>, sqlx::Error> {
        sqlx::query_as::<_, (i32, i64)>(
            "SELECT difficulty, COUNT(*) as count FROM problems GROUP BY difficulty ORDER BY difficulty"
        )
        .fetch_all(pool)
        .await
    }

    /// Get total problem count
    pub async fn count_total(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM problems")
            .fetch_one(pool)
            .await?;
        Ok(count)
    }

    /// Get all problem details including tags and mastery
    pub async fn get_full(id: &str, pool: &SqlitePool) -> Result<serde_json::Value, sqlx::Error> {
        let problem = Self::get(id, pool).await?;
        let tags = Self::get_tags(id, pool).await?;

        let mastery: Option<(bool, f64)> = sqlx::query_as::<_, (i32, f64)>(
            "SELECT solved, mastery_percent FROM problem_mastery WHERE problem_id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .map(|(s, m)| (s != 0, m));

        Ok(serde_json::json!({
            "problem": problem,
            "tags": tags,
            "mastery": mastery,
        }))
    }

    /// Delete problem and all related data
    pub async fn delete_with_cascade(id: &str, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        // Delete cascade happens automatically due to FOREIGN KEY constraints
        sqlx::query("DELETE FROM problems WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Add multiple tags to a problem
    pub async fn add_tags(
        problem_id: &str,
        tag_names: Vec<String>,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        for tag in tag_names {
            Self::add_tag(problem_id, &tag, pool).await?;
        }
        Ok(())
    }

    /// Remove multiple tags from a problem
    pub async fn remove_tags(
        problem_id: &str,
        tag_names: Vec<String>,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        for tag in tag_names {
            Self::remove_tag(problem_id, &tag, pool).await?;
        }
        Ok(())
    }

    /// Replace all tags for a problem
    pub async fn set_tags(
        problem_id: &str,
        tag_names: Vec<String>,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        // Delete existing tags
        sqlx::query("DELETE FROM problem_tags WHERE problem_id = ?")
            .bind(problem_id)
            .execute(pool)
            .await?;

        // Add new tags
        Self::add_tags(problem_id, tag_names, pool).await
    }

    /// Get all unique tags in database
    pub async fn get_all_tags(pool: &SqlitePool) -> Result<Vec<String>, sqlx::Error> {
        let tags: Vec<(String,)> =
            sqlx::query_as("SELECT DISTINCT tag_name FROM problem_tags ORDER BY tag_name")
                .fetch_all(pool)
                .await?;

        Ok(tags.into_iter().map(|(tag,)| tag).collect())
    }

    /// Get problems by multiple tags (AND logic)
    pub async fn get_by_tags(
        tag_names: Vec<String>,
        pool: &SqlitePool,
    ) -> Result<Vec<Problem>, sqlx::Error> {
        if tag_names.is_empty() {
            return Ok(Vec::new());
        }

        let mut query = String::from(
            "SELECT p.id, p.title, p.description, p.difficulty, p.created_at, p.updated_at
             FROM problems p
             WHERE p.id IN (
                 SELECT problem_id FROM problem_tags 
                 WHERE tag_name IN (",
        );

        for (i, _) in tag_names.iter().enumerate() {
            if i > 0 {
                query.push(',');
            }
            query.push('?');
        }

        query.push_str(
            ")
                 GROUP BY problem_id
                 HAVING COUNT(DISTINCT tag_name) = ?",
        );

        let mut q = sqlx::query_as::<_, Problem>(&query);
        for tag in tag_names.iter() {
            q = q.bind(tag);
        }
        q = q.bind(tag_names.len() as i32);

        q.fetch_all(pool).await
    }

    // === Helper Methods ===

    async fn create_fsrs_card(problem_id: &str, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO fsrs_cards (id, problem_id, due, stability, difficulty, state)
             VALUES (?, ?, DATE('now'), 0.0, 5.0, 'new')",
        )
        .bind(&id)
        .bind(problem_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn create_study_phase(problem_id: &str, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO study_phase_progress (id, problem_id, current_phase, current_step)
             VALUES (?, ?, 1, 1)",
        )
        .bind(&id)
        .bind(problem_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn create_mastery(problem_id: &str, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO problem_mastery (id, problem_id, solved, mastery_percent)
             VALUES (?, ?, 0, 0)",
        )
        .bind(&id)
        .bind(problem_id)
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_create_problem() {
        // Test will be implemented in Phase 1, Task 1.13
        // For now, just shows the structure
    }
}
