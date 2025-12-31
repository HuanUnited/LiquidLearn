use crate::models::fsrs::{FsrsCard, FsrsStats, ReviewRequest, ReviewResult};
use sqlx::SqlitePool;

pub struct FsrsService;

// FSRS v3 Default Parameters
const W: [f64; 19] = [
    0.40, 1.86, 4.93, 0.94, 0.86, 0.01, 1.49, 0.04, 0.36, 0.86, 0.20, 2.50, 0.14, 0.94, 0.16, 0.10,
    0.29, 0.34, 3.73,
];

impl FsrsService {
    /// Initialize FSRS card for a new problem (first attempt)
    pub async fn create_card(pool: &SqlitePool, problem_id: String) -> Result<FsrsCard, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let due = now.clone(); // Due immediately for first review

        sqlx::query(
            "INSERT INTO fsrs_cards (id, problem_id, due, stability, difficulty, state, reps, lapses, elapsed_days, scheduled_days, created_at, updated_at) 
             VALUES (?, ?, ?, 1.0, 5.0, 'new', 0, 0, 0, 1, ?, ?)"
        )
        .bind(&id)
        .bind(&problem_id)
        .bind(&due)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(FsrsCard {
            id,
            problem_id,
            due,
            stability: 1.0,
            difficulty: 5.0,
            state: "new".to_string(),
            reps: 0,
            lapses: 0,
            elapsed_days: 0,
            scheduled_days: 1,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    /// Get or create card for problem
    pub async fn get_or_create_card(
        pool: &SqlitePool,
        problem_id: String,
    ) -> Result<FsrsCard, String> {
        // Try to fetch existing card
        match sqlx::query_as::<_, FsrsCard>(
            "SELECT id, problem_id, due, stability, difficulty, state, reps, lapses, elapsed_days, scheduled_days, created_at, updated_at 
             FROM fsrs_cards WHERE problem_id = ?"
        )
        .bind(&problem_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())? {
            Some(card) => Ok(card),
            None => Self::create_card(pool, problem_id).await,
        }
    }

    /// Process a review/attempt and update card state
    pub async fn process_review(
        pool: &SqlitePool,
        req: ReviewRequest,
    ) -> Result<ReviewResult, String> {
        let card = Self::get_or_create_card(pool, req.problem_id.clone()).await?;

        // Calculate new state using FSRS algorithm
        let (new_state, new_difficulty, new_stability, new_interval) =
            Self::calculate_fsrs_update(&card, req.attempt_is_solved, req.quality);

        let next_due = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(new_interval as i64))
            .unwrap()
            .to_rfc3339();

        let now = chrono::Utc::now().to_rfc3339();
        let is_correct = req.quality >= 3; // Quality 3+ is considered passing

        // Update card in database
        sqlx::query(
            "UPDATE fsrs_cards 
             SET state = ?, difficulty = ?, stability = ?, due = ?, 
                 reps = reps + 1, lapses = lapses + ?, scheduled_days = ?, 
                 elapsed_days = ?, updated_at = ? 
             WHERE id = ?",
        )
        .bind(&new_state)
        .bind(new_difficulty)
        .bind(new_stability)
        .bind(&next_due)
        .bind(if req.quality < 3 { 1 } else { 0 }) // Increment lapses on fail
        .bind(new_interval)
        .bind(Self::calculate_elapsed_days(req.time_spent_seconds))
        .bind(&now)
        .bind(&card.id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(ReviewResult {
            problem_id: req.problem_id,
            card_id: card.id,
            new_state,
            new_stability,
            new_difficulty,
            new_interval_days: new_interval,
            next_due,
            is_correct,
        })
    }

    /// Core FSRS algorithm: calculate new state based on quality
    fn calculate_fsrs_update(
        card: &FsrsCard,
        _attempt_solved: bool,
        quality: u8,
    ) -> (String, f64, f64, i32) {
        let quality = quality.clamp(1, 5);

        // Update difficulty (1-10 scale)
        let new_difficulty = card.difficulty + (5.0 - quality as f64) * W[4];
        let new_difficulty = new_difficulty.max(1.0).min(10.0);

        // Update stability based on card state and quality
        let new_stability = match card.state.as_str() {
            "new" => {
                if quality >= 3 {
                    W[0] // Initial stability for "pass" on new card
                } else {
                    W[1] * (card.stability + 1.0) // Reduced for "fail"
                }
            }
            "learning" => {
                if quality >= 3 {
                    W[2] * card.stability // Good progress
                } else {
                    W[1] * card.stability // Back to learning
                }
            }
            "review" => {
                if quality >= 3 {
                    card.stability * (1.0 + W[6] * (quality as f64 - 3.0))
                } else {
                    card.stability * W[7] // Significant decay
                }
            }
            "relearning" => {
                if quality >= 3 {
                    W[2] * card.stability
                } else {
                    W[1] * card.stability
                }
            }
            _ => card.stability,
        };

        let new_stability = new_stability.max(0.1);

        // Determine new state and interval
        let (new_state, new_interval) = match card.state.as_str() {
            "new" => {
                if quality >= 3 {
                    ("learning".to_string(), 1) // Start learning phase
                } else {
                    ("learning".to_string(), 1) // Stay in learning
                }
            }
            "learning" => {
                if quality >= 3 {
                    ("review".to_string(), ((new_stability * 10.0) as i32).max(1))
                // Move to review
                } else {
                    ("learning".to_string(), 1) // More practice
                }
            }
            "review" => {
                if quality >= 3 {
                    ("review".to_string(), ((new_stability * 10.0) as i32).max(1))
                // Continue review
                } else {
                    ("relearning".to_string(), 1) // Need relearning
                }
            }
            "relearning" => {
                if quality >= 3 {
                    ("review".to_string(), ((new_stability * 5.0) as i32).max(1))
                // Back to review
                } else {
                    ("relearning".to_string(), 1) // More relearning
                }
            }
            _ => ("new".to_string(), 1),
        };

        (new_state, new_difficulty, new_stability, new_interval)
    }

    /// Convert time spent to elapsed days
    fn calculate_elapsed_days(seconds: i64) -> i32 {
        ((seconds as f64) / 86400.0).ceil() as i32
    }

    /// Get card by ID
    pub async fn get_card_by_id(pool: &SqlitePool, id: String) -> Result<FsrsCard, String> {
        sqlx::query_as::<_, FsrsCard>(
            "SELECT id, problem_id, due, stability, difficulty, state, reps, lapses, elapsed_days, scheduled_days, created_at, updated_at 
             FROM fsrs_cards WHERE id = ?"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
    }

    /// Get due cards (problems to review)
    pub async fn get_due_cards(pool: &SqlitePool) -> Result<Vec<FsrsCard>, String> {
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query_as::<_, FsrsCard>(
            "SELECT id, problem_id, due, stability, difficulty, state, reps, lapses, elapsed_days, scheduled_days, created_at, updated_at 
             FROM fsrs_cards WHERE due <= ? ORDER BY due"
        )
        .bind(&now)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }

    /// Get FSRS statistics
    pub async fn get_stats(pool: &SqlitePool) -> Result<FsrsStats, String> {
        let now = chrono::Utc::now().to_rfc3339();

        let (total,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM fsrs_cards")
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        let (new,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM fsrs_cards WHERE state = 'new'")
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        let (learning,): (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM fsrs_cards WHERE state = 'learning'")
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;

        let (review,): (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM fsrs_cards WHERE state = 'review'")
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;

        let (relearning,): (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM fsrs_cards WHERE state = 'relearning'")
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;

        let (due,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM fsrs_cards WHERE due <= ?")
            .bind(&now)
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        let retention_rate = if review + relearning > 0 {
            ((review as f64) / (total as f64) * 100.0).min(100.0)
        } else {
            0.0
        };

        Ok(FsrsStats {
            total_cards: total,
            new_count: new,
            learning_count: learning,
            review_count: review,
            relearning_count: relearning,
            due_today: due,
            retention_rate,
        })
    }

    /// Get cards by state
    pub async fn get_cards_by_state(
        pool: &SqlitePool,
        state: String,
    ) -> Result<Vec<FsrsCard>, String> {
        sqlx::query_as::<_, FsrsCard>(
            "SELECT id, problem_id, due, stability, difficulty, state, reps, lapses, elapsed_days, scheduled_days, created_at, updated_at 
             FROM fsrs_cards WHERE state = ? ORDER BY due"
        )
        .bind(state)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }
}
