use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::fsrs::{process_review, FsrsParameters};
use crate::models::FsrsCard;

// Helper struct to fetch parameters from database
#[derive(Debug, FromRow)]
struct FsrsParametersRow {
    w_1: f64,
    w_2: f64,
    w_3: f64,
    w_4: f64,
    w_5: f64,
    w_6: f64,
    w_7: f64,
    w_8: f64,
    w_9: f64,
    w_10: f64,
    w_11: f64,
    w_12: f64,
    w_13: f64,
    w_14: f64,
    w_15: f64,
    w_16: f64,
    w_17: f64,
    w_18: f64,
    w_19: f64,
    desired_retention: f64,
}

impl From<FsrsParametersRow> for FsrsParameters {
    fn from(row: FsrsParametersRow) -> Self {
        FsrsParameters {
            w_1: row.w_1,
            w_2: row.w_2,
            w_3: row.w_3,
            w_4: row.w_4,
            w_5: row.w_5,
            w_6: row.w_6,
            w_7: row.w_7,
            w_8: row.w_8,
            w_9: row.w_9,
            w_10: row.w_10,
            w_11: row.w_11,
            w_12: row.w_12,
            w_13: row.w_13,
            w_14: row.w_14,
            w_15: row.w_15,
            w_16: row.w_16,
            w_17: row.w_17,
            w_18: row.w_18,
            w_19: row.w_19,
            desired_retention: row.desired_retention,
        }
    }
}

pub struct FsrsService;

impl FsrsService {
    /// Load FSRS parameters (global) or create defaults
    pub async fn load_parameters(pool: &SqlitePool) -> Result<FsrsParameters> {
        // Try to load existing parameters using the helper struct
        let row = sqlx::query_as::<_, FsrsParametersRow>(
            "SELECT w_1, w_2, w_3, w_4, w_5, w_6, w_7, w_8, w_9, w_10,
                    w_11, w_12, w_13, w_14, w_15, w_16, w_17, w_18, w_19, desired_retention
             FROM fsrs_parameters LIMIT 1",
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            Ok(row.into()) // Convert FsrsParametersRow to FsrsParameters
        } else {
            // Create default parameters
            let params = FsrsParameters::default();
            let id = Uuid::new_v4().to_string();

            sqlx::query(
                "INSERT INTO fsrs_parameters (
                    id, w_1, w_2, w_3, w_4, w_5, w_6, w_7, w_8, w_9, w_10,
                    w_11, w_12, w_13, w_14, w_15, w_16, w_17, w_18, w_19,
                    desired_retention, created_at, updated_at
                ) VALUES (
                    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?
                )",
            )
            .bind(&id)
            .bind(params.w_1)
            .bind(params.w_2)
            .bind(params.w_3)
            .bind(params.w_4)
            .bind(params.w_5)
            .bind(params.w_6)
            .bind(params.w_7)
            .bind(params.w_8)
            .bind(params.w_9)
            .bind(params.w_10)
            .bind(params.w_11)
            .bind(params.w_12)
            .bind(params.w_13)
            .bind(params.w_14)
            .bind(params.w_15)
            .bind(params.w_16)
            .bind(params.w_17)
            .bind(params.w_18)
            .bind(params.w_19)
            .bind(params.desired_retention)
            .bind(Utc::now())
            .bind(Utc::now())
            .execute(pool)
            .await?;

            Ok(params)
        }
    }

    /// Process a review and update card
    pub async fn process_card_review(
        card_id: &str,
        rating: i32,
        elapsed_seconds: i64,
        pool: &SqlitePool,
    ) -> Result<FsrsCard> {
        // Validate rating
        if !(1..=10).contains(&rating) {
            return Err(anyhow::anyhow!("Rating must be between 1 and 10"));
        }

        // Get card
        let card: FsrsCard = sqlx::query_as("SELECT * FROM fsrs_cards WHERE id = ?")
            .bind(card_id)
            .fetch_one(pool)
            .await
            .context("Card not found")?;

        // Load parameters
        let params = Self::load_parameters(pool).await?;

        // Process review using FSRS algorithm
        let result = process_review(
            &card.state,
            card.stability,
            card.difficulty,
            rating,
            &params,
        );

        // Calculate new due date
        let new_due = Utc::now() + Duration::days(result.new_interval as i64);

        // Start transaction
        let mut tx = pool.begin().await?;

        // Update card
        sqlx::query(
            "UPDATE fsrs_cards SET
                due = ?,
                stability = ?,
                difficulty = ?,
                state = ?,
                reps = reps + 1,
                lapses = CASE WHEN ? THEN lapses + 1 ELSE lapses END,
                last_review = ?,
                scheduled_days = ?,
                updated_at = ?
             WHERE id = ?",
        )
        .bind(new_due.date_naive())
        .bind(result.new_stability)
        .bind(result.new_difficulty)
        .bind(&result.new_state)
        .bind(result.is_lapse)
        .bind(Utc::now())
        .bind(result.new_interval)
        .bind(Utc::now())
        .bind(card_id)
        .execute(&mut *tx)
        .await?;

        // Save review to history
        let review_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO fsrs_reviews (
                id, card_id, problem_id, rating, state_before,
                elapsed_seconds, scheduled_days_before, scheduled_days_after,
                created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&review_id)
        .bind(card_id)
        .bind(&card.problem_id)
        .bind(rating)
        .bind(&card.state)
        .bind(elapsed_seconds)
        .bind(card.scheduled_days)
        .bind(result.new_interval)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await?;

        // Update problem mastery
        let is_solved =
            result.new_state == "review" && result.new_stability >= 21.0 && card.reps >= 1;

        let mastery_percent = if is_solved {
            100
        } else {
            // Calculate mastery based on stability (0-100%)
            ((result.new_stability / 21.0) * 100.0).min(99.0) as i32
        };

        sqlx::query(
            "UPDATE problem_mastery SET
                solved = ?,
                mastery_percent = ?,
                last_attempted = ?,
                attempt_count = attempt_count + 1,
                updated_at = ?
             WHERE problem_id = ?",
        )
        .bind(is_solved)
        .bind(mastery_percent)
        .bind(Utc::now())
        .bind(Utc::now())
        .bind(&card.problem_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        // Fetch and return updated card
        let updated_card: FsrsCard = sqlx::query_as("SELECT * FROM fsrs_cards WHERE id = ?")
            .bind(card_id)
            .fetch_one(pool)
            .await?;

        Ok(updated_card)
    }

    /// Get next due cards
    pub async fn get_next_due(limit: i32, pool: &SqlitePool) -> Result<Vec<DueCard>> {
        let cards = sqlx::query_as::<_, DueCard>(
            "SELECT 
                fc.id as card_id,
                fc.problem_id,
                p.title,
                p.difficulty,
                fc.state,
                fc.due,
                fc.difficulty as card_difficulty,
                fc.stability,
                fc.reps,
                fc.lapses,
                CAST(JULIANDAY('now') - JULIANDAY(fc.due) AS INTEGER) as days_overdue
             FROM fsrs_cards fc
             JOIN problems p ON fc.problem_id = p.id
             WHERE fc.due <= DATE('now')
             ORDER BY 
                days_overdue DESC,
                fc.stability ASC,
                fc.difficulty DESC
             LIMIT ?",
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(cards)
    }

    /// Get card statistics
    pub async fn get_card_stats(card_id: &str, pool: &SqlitePool) -> Result<CardStats> {
        let stats: CardStats = sqlx::query_as(
            "SELECT 
                fc.state,
                fc.reps,
                fc.lapses,
                fc.stability,
                fc.difficulty,
                fc.due,
                COUNT(fr.id) as total_reviews,
                AVG(fr.rating) as avg_rating
             FROM fsrs_cards fc
             LEFT JOIN fsrs_reviews fr ON fc.id = fr.card_id
             WHERE fc.id = ?
             GROUP BY fc.id",
        )
        .bind(card_id)
        .fetch_one(pool)
        .await?;

        Ok(stats)
    }
}

// Response types
#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct DueCard {
    pub card_id: String,
    pub problem_id: String,
    pub title: String,
    pub difficulty: i32,
    pub state: String,
    pub due: String,
    pub card_difficulty: f64,
    pub stability: f64,
    pub reps: i32,
    pub lapses: i32,
    pub days_overdue: i32,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct CardStats {
    pub state: String,
    pub reps: i32,
    pub lapses: i32,
    pub stability: f64,
    pub difficulty: f64,
    pub due: String,
    pub total_reviews: i32,
    pub avg_rating: Option<f64>,
}
