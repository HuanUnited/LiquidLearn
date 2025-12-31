use anyhow::{Context, Result};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{
    CreateProblemRequest, Problem, ProblemMastery, ProblemWithMastery, UpdateProblemRequest,
};

pub struct ProblemService;

impl ProblemService {
    /// Create new problem (also creates FSRS card, mastery, and study phase progress)
    pub async fn create(req: CreateProblemRequest, pool: &SqlitePool) -> Result<Problem> {
        let problem_id = Uuid::new_v4().to_string();
        let card_id = Uuid::new_v4().to_string();
        let mastery_id = Uuid::new_v4().to_string();
        let progress_id = Uuid::new_v4().to_string();

        // Validate difficulty
        if req.difficulty < 1 || req.difficulty > 5 {
            return Err(anyhow::anyhow!("Difficulty must be between 1 and 5"));
        }

        // Start transaction
        let mut tx = pool.begin().await?;

        // Insert problem
        sqlx::query(
            "INSERT INTO problems (id, title, description, difficulty, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&problem_id)
        .bind(&req.title)
        .bind(&req.description)
        .bind(req.difficulty)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&mut *tx)
        .await
        .context("Failed to insert problem")?;

        // Create FSRS card
        let fsrs_difficulty = match req.difficulty {
            1 => 2.0,
            2 => 3.0,
            3 => 5.0,
            4 => 7.0,
            5 => 9.0,
            _ => 5.0,
        };

        sqlx::query(
            "INSERT INTO fsrs_cards 
             (id, problem_id, due, stability, difficulty, state, created_at, updated_at)
             VALUES (?, ?, DATE('now'), 0.0, ?, 'new', ?, ?)",
        )
        .bind(&card_id)
        .bind(&problem_id)
        .bind(fsrs_difficulty)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&mut *tx)
        .await
        .context("Failed to create FSRS card")?;

        // Create problem mastery
        sqlx::query(
            "INSERT INTO problem_mastery 
             (id, problem_id, solved, mastery_percent, updated_at)
             VALUES (?, ?, 0, 0, ?)",
        )
        .bind(&mastery_id)
        .bind(&problem_id)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await
        .context("Failed to create problem mastery")?;

        // Create study phase progress
        sqlx::query(
            "INSERT INTO study_phase_progress 
             (id, problem_id, current_phase, current_step, time_spent_seconds, created_at, updated_at)
             VALUES (?, ?, 1, 1, 0, ?, ?)"
        )
        .bind(&progress_id)
        .bind(&problem_id)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&mut *tx)
        .await
        .context("Failed to create study phase progress")?;

        tx.commit().await?;

        // Fetch and return created problem
        Self::read(&problem_id, pool).await
    }

    /// Get single problem by ID
    pub async fn read(problem_id: &str, pool: &SqlitePool) -> Result<Problem> {
        let problem = sqlx::query_as::<_, Problem>("SELECT * FROM problems WHERE id = ?")
            .bind(problem_id)
            .fetch_one(pool)
            .await
            .context("Problem not found")?;

        Ok(problem)
    }

    /// Get problem with mastery and tags
    pub async fn read_with_details(
        problem_id: &str,
        pool: &SqlitePool,
    ) -> Result<ProblemWithMastery> {
        let problem = Self::read(problem_id, pool).await?;

        // Get mastery
        let mastery = sqlx::query_as::<_, ProblemMastery>(
            "SELECT * FROM problem_mastery WHERE problem_id = ?",
        )
        .bind(problem_id)
        .fetch_optional(pool)
        .await?;

        // Get tags
        let tags: Vec<String> =
            sqlx::query_scalar("SELECT tag_name FROM problem_tags WHERE problem_id = ?")
                .bind(problem_id)
                .fetch_all(pool)
                .await?;

        Ok(ProblemWithMastery {
            problem,
            mastery,
            tags,
        })
    }

    /// Update problem
    pub async fn update(
        problem_id: &str,
        req: UpdateProblemRequest,
        pool: &SqlitePool,
    ) -> Result<Problem> {
        // Validate difficulty if provided
        if let Some(diff) = req.difficulty {
            if diff < 1 || diff > 5 {
                return Err(anyhow::anyhow!("Difficulty must be between 1 and 5"));
            }
        }

        // Simple approach: update only provided fields
        if let Some(title) = req.title {
            sqlx::query("UPDATE problems SET title = ?, updated_at = ? WHERE id = ?")
                .bind(&title)
                .bind(Utc::now())
                .bind(problem_id)
                .execute(pool)
                .await?;
        }

        if let Some(desc) = req.description {
            sqlx::query("UPDATE problems SET description = ?, updated_at = ? WHERE id = ?")
                .bind(&desc)
                .bind(Utc::now())
                .bind(problem_id)
                .execute(pool)
                .await?;
        }

        if let Some(diff) = req.difficulty {
            sqlx::query("UPDATE problems SET difficulty = ?, updated_at = ? WHERE id = ?")
                .bind(diff)
                .bind(Utc::now())
                .bind(problem_id)
                .execute(pool)
                .await?;
        }

        Self::read(problem_id, pool).await
    }

    /// Delete problem (cascades to FSRS, mastery, tags, study progress)
    pub async fn delete(problem_id: &str, pool: &SqlitePool) -> Result<()> {
        let result = sqlx::query("DELETE FROM problems WHERE id = ?")
            .bind(problem_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Problem not found"));
        }

        Ok(())
    }

    /// List problems with optional filtering
    pub async fn list(
        filter: Option<String>, // 'solved', 'unsolved', 'all'
        sort: Option<String>,   // 'difficulty', 'created', 'mastery'
        limit: i32,
        offset: i32,
        pool: &SqlitePool,
    ) -> Result<Vec<ProblemWithMastery>> {
        // Build base query
        let base_query =
            "SELECT p.id, p.title, p.description, p.difficulty, p.created_at, p.updated_at
                      FROM problems p
                      LEFT JOIN problem_mastery pm ON p.id = pm.problem_id
                      WHERE 1=1";

        let mut query = String::from(base_query);

        // Apply filter
        if let Some(f) = &filter {
            match f.as_str() {
                "solved" => query.push_str(" AND pm.solved = 1"),
                "unsolved" => query.push_str(" AND pm.solved = 0"),
                _ => {} // "all" or invalid - no filter
            }
        }

        // Apply sort
        let order_by = match sort.as_deref() {
            Some("difficulty") => " ORDER BY p.difficulty DESC",
            Some("mastery") => " ORDER BY pm.mastery_percent DESC",
            _ => " ORDER BY p.created_at DESC", // default: newest first
        };
        query.push_str(order_by);

        // Add pagination
        query.push_str(" LIMIT ? OFFSET ?");

        // Execute query with explicit type annotation
        let problems = sqlx::query_as::<_, Problem>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        // Fetch details for each problem
        let mut results = Vec::new();
        for problem in problems {
            let details = Self::read_with_details(&problem.id, pool).await?;
            results.push(details);
        }

        Ok(results)
    }

    /// Search problems
    pub async fn search(query: &str, limit: i32, pool: &SqlitePool) -> Result<Vec<Problem>> {
        let problems = sqlx::query_as::<_, Problem>(
            "SELECT * FROM problems 
             WHERE title LIKE ? OR description LIKE ?
             ORDER BY created_at DESC
             LIMIT ?",
        )
        .bind(format!("%{}%", query))
        .bind(format!("%{}%", query))
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(problems)
    }

    /// Add tag to problem
    pub async fn add_tag(problem_id: &str, tag_name: &str, pool: &SqlitePool) -> Result<()> {
        let tag_id = Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO problem_tags (id, problem_id, tag_name, created_at)
             VALUES (?, ?, ?, ?)
             ON CONFLICT(problem_id, tag_name) DO NOTHING",
        )
        .bind(tag_id)
        .bind(problem_id)
        .bind(tag_name)
        .bind(Utc::now())
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Bulk import from CSV
    pub async fn bulk_import(
        csv_content: &str,
        pool: &SqlitePool,
    ) -> Result<(i32, i32, Vec<String>)> {
        let mut imported = 0;
        let mut failed = 0;
        let mut errors = Vec::new();

        for (line_num, line) in csv_content.lines().enumerate().skip(1) {
            let parts: Vec<&str> = line.split(',').collect();

            if parts.len() < 3 {
                failed += 1;
                errors.push(format!("Line {}: insufficient columns", line_num + 1));
                continue;
            }

            let title = parts[0].trim();
            let description = parts.get(1).map(|s| s.trim().to_string());
            let difficulty = parts[2].trim().parse::<i32>().unwrap_or(3);

            match Self::create(
                CreateProblemRequest {
                    title: title.to_string(),
                    description,
                    difficulty,
                },
                pool,
            )
            .await
            {
                Ok(_) => imported += 1,
                Err(e) => {
                    failed += 1;
                    errors.push(format!("Line {}: {}", line_num + 1, e));
                }
            }
        }

        Ok((imported, failed, errors))
    }
}
