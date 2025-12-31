use crate::models::fsrs::{CardState, FsrsCard, FsrsParameters, ReviewRequest, ReviewResult};
use chrono::{Duration, Local};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct FsrsService;

impl FsrsService {
    /// Get global FSRS parameters
    pub async fn get_parameters(pool: &SqlitePool) -> Result<FsrsParameters, sqlx::Error> {
        sqlx::query_as::<_, FsrsParameters>(
            "SELECT id, w_1, w_2, w_3, w_4, w_5, w_6, w_7, w_8, w_9, w_10,
                    w_11, w_12, w_13, w_14, w_15, w_16, w_17, w_18, w_19,
                    desired_retention, total_reviews, created_at, updated_at
             FROM fsrs_parameters WHERE id = 'global'",
        )
        .fetch_one(pool)
        .await
    }

    /// Process a review and calculate new card state
    pub async fn process_review(
        request: ReviewRequest,
        pool: &SqlitePool,
    ) -> Result<ReviewResult, String> {
        // Validate request
        request.validate()?;

        // Get card
        let card = sqlx::query_as::<_, FsrsCard>(
            "SELECT id, problem_id, due, stability, difficulty, state, reps, lapses, created_at, updated_at
             FROM fsrs_cards WHERE id = ?"
        )
        .bind(&request.card_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to get card: {}", e))?;

        // Get parameters
        let params = Self::get_parameters(pool)
            .await
            .map_err(|e| format!("Failed to get parameters: {}", e))?;

        // Calculate new state
        let result = Self::calculate_review(card, request, params);

        // Save review
        Self::save_review(&result, pool)
            .await
            .map_err(|e| format!("Failed to save review: {}", e))?;

        // Update card
        Self::update_card(&result, pool)
            .await
            .map_err(|e| format!("Failed to update card: {}", e))?;

        Ok(result)
    }

    /// Core FSRS algorithm - calculates new card state from review
    fn calculate_review(
        card: FsrsCard,
        request: ReviewRequest,
        params: FsrsParameters,
    ) -> ReviewResult {
        let rating = request.rating as f64;
        let old_state = CardState::from_string(&card.state);
        let is_correct = request.rating >= 5;

        // Step 1: Calculate new difficulty
        let new_difficulty =
            Self::calculate_difficulty(rating, card.difficulty, old_state.clone(), &params);

        // Step 2: Calculate new stability
        let new_stability = Self::calculate_stability(
            rating,
            card.stability,
            card.difficulty,
            old_state.clone(),
            &params,
        );

        // Step 3: Calculate interval (days until next review)
        let new_interval = Self::calculate_interval(new_stability, params.desired_retention);

        // Step 4: Determine new state
        let new_state = Self::determine_state(old_state, is_correct, card.reps);

        // Step 5: Calculate next due date
        let today = Local::now().date_naive();
        let next_due = today + Duration::days(new_interval as i64);

        ReviewResult {
            card_id: card.id,
            new_state: new_state.to_string(),
            new_stability,
            new_difficulty,
            new_interval,
            next_due: next_due.to_string(),
            time_spent: request.elapsed_seconds,
            is_correct,
        }
    }

    /// Calculate new difficulty based on FSRS formula
    fn calculate_difficulty(
        rating: f64,
        old_difficulty: f64,
        _state: CardState,
        params: &FsrsParameters,
    ) -> f64 {
        // Higher rating = lower difficulty increase (easier perception)
        // FSRS formula: difficulty += (w_4 * (3 - rating) + w_5 * (rating - 1)) / 17
        let delta = params.w_4 * (3.0 - rating) + params.w_5 * (rating - 1.0);
        let normalized_delta = delta / 17.0;

        let new_difficulty = old_difficulty + normalized_delta;

        // Ensure bounds (1-10)
        let bounded = new_difficulty.max(1.0).min(10.0);

        bounded
    }

    /// Calculate new stability based on FSRS formula
    fn calculate_stability(
        rating: f64,
        old_stability: f64,
        _difficulty: f64,
        state: CardState,
        params: &FsrsParameters,
    ) -> f64 {
        let new_stability = match state {
            CardState::New => {
                // New card: w_1 + w_2 * (rating - 1) / 9
                params.w_1 + params.w_2 * (rating - 1.0) / 9.0
            }
            CardState::Learning => {
                // Learning card: similar to new
                params.w_1 + params.w_2 * (rating - 1.0) / 9.0
            }
            CardState::Review => {
                // Review card: stability *= (1 + w_6 * (rating - 3) / 10)
                // Higher rating = higher multiplier = higher stability
                old_stability * (1.0 + params.w_6 * (rating - 3.0) / 10.0)
            }
            CardState::Relearning => {
                // Relearning: similar to review but with lower multiplier
                old_stability * (1.0 + params.w_8 * (rating - 3.0) / 10.0)
            }
        };

        // Ensure minimum stability
        new_stability.max(params.w_7)
    }

    /// Calculate interval in days until next review
    fn calculate_interval(stability: f64, desired_retention: f64) -> i32 {
        // Formula: interval = stability * ln(desired_retention) / ln(0.9)
        let interval = (stability * desired_retention.ln() / 0.9_f64.ln()).ceil();
        interval.max(1.0) as i32
    }

    /// Determine new card state based on old state and correctness
    fn determine_state(old_state: CardState, is_correct: bool, reps: i32) -> CardState {
        match (old_state, is_correct) {
            (CardState::New, true) => {
                if reps < 2 {
                    CardState::Learning
                } else {
                    CardState::Review
                }
            }
            (CardState::New, false) => CardState::Relearning,
            (CardState::Learning, true) => CardState::Review,
            (CardState::Learning, false) => CardState::Relearning,
            (CardState::Review, true) => CardState::Review,
            (CardState::Review, false) => CardState::Relearning,
            (CardState::Relearning, true) => CardState::Review,
            (CardState::Relearning, false) => CardState::Relearning,
        }
    }

    /// Save review to database
    async fn save_review(result: &ReviewResult, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO fsrs_reviews (id, card_id, problem_id, rating, state_before, elapsed_seconds)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&result.card_id)
        .bind("") // problem_id will be filled from card
        .bind(if result.is_correct { 5 } else { 1 }) // Simplified rating
        .bind("") // state_before
        .bind(result.time_spent)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Update card with new values (with error multiplier applied)
    async fn update_card(result: &ReviewResult, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let now = chrono::Utc::now().to_rfc3339();

        // Get error impact modifier for this card
        let error_modifier: (f64,) = sqlx::query_as(
            "SELECT COALESCE(error_impact_modifier, 1.0) FROM fsrs_cards WHERE id = ?",
        )
        .bind(&result.card_id)
        .fetch_one(pool)
        .await
        .unwrap_or((1.0,));

        // Apply error multiplier to interval
        // Error multiplier: 1.5x (high impact), 1.0x (medium), 0.7x (low)
        let adjusted_interval = ((result.new_interval as f64) * error_modifier.0).ceil() as i32;

        // Recalculate due date with adjusted interval
        let today = Local::now().date_naive();
        let adjusted_due = today + Duration::days(adjusted_interval as i64);

        sqlx::query(
            "UPDATE fsrs_cards
        SET due = ?, stability = ?, difficulty = ?, state = ?,
        reps = reps + 1, updated_at = ?
        WHERE id = ?",
        )
        .bind(adjusted_due.to_string())
        .bind(result.new_stability)
        .bind(result.new_difficulty)
        .bind(&result.new_state)
        .bind(&now)
        .bind(&result.card_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Get cards due for review today
    pub async fn get_due_cards(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
        let today = Local::now().date_naive().to_string();
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM fsrs_cards WHERE due <= ?")
            .bind(&today)
            .fetch_one(pool)
            .await?;
        Ok(count)
    }

    /// Get review statistics
    pub async fn get_stats(pool: &SqlitePool) -> Result<serde_json::Value, sqlx::Error> {
        let today = Local::now().date_naive().to_string();

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM fsrs_cards")
            .fetch_one(pool)
            .await?;

        let new_count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM fsrs_cards WHERE state = 'new'")
                .fetch_one(pool)
                .await?;

        let learning: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM fsrs_cards WHERE state = 'learning'")
                .fetch_one(pool)
                .await?;

        let review: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM fsrs_cards WHERE state = 'review'")
                .fetch_one(pool)
                .await?;

        let relearning: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM fsrs_cards WHERE state = 'relearning'")
                .fetch_one(pool)
                .await?;

        let due_today: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM fsrs_cards WHERE due <= ?")
            .bind(&today)
            .fetch_one(pool)
            .await?;

        Ok(serde_json::json!({
            "total": total.0,
            "new": new_count.0,
            "learning": learning.0,
            "review": review.0,
            "relearning": relearning.0,
            "due_today": due_today.0,
        }))
    }

    /// Update error impact modifier for a card based on unresolved errors
    pub async fn update_error_modifier(card_id: &str, pool: &SqlitePool) -> Result<(), String> {
        // Get problem_id for this card
        let problem_result: (String,) =
            sqlx::query_as("SELECT problem_id FROM fsrs_cards WHERE id = ?")
                .bind(card_id)
                .fetch_one(pool)
                .await
                .map_err(|e| format!("Failed to get problem: {}", e))?;

        let problem_id = problem_result.0;

        // Calculate average error multiplier from problem_error_history
        let modifier_result: (f64,) = sqlx::query_as(
            "SELECT COALESCE(AVG(et.multiplier), 1.0)
         FROM problem_error_history peh
         JOIN error_types et ON peh.error_type_id = et.id
         WHERE peh.problem_id = ?",
        )
        .bind(&problem_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to calculate modifier: {}", e))?;

        let avg_multiplier = modifier_result.0;

        // Update fsrs_cards with the new modifier
        sqlx::query(
            "UPDATE fsrs_cards 
         SET error_impact_modifier = ?
         WHERE id = ?",
        )
        .bind(avg_multiplier)
        .bind(card_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update modifier: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Sanity Check 1: New card starts with correct initial values
    #[test]
    fn test_new_card_initialization() {
        let params = FsrsParameters::default();
        assert_eq!(params.w_1, 0.40);
        assert!(params.w_1 > 0.0);
    }

    // Sanity Check 2: Rating affects difficulty
    #[test]
    fn test_rating_affects_difficulty() {
        let params = FsrsParameters::default();
        let card = FsrsCard {
            id: "test".to_string(),
            problem_id: "p1".to_string(),
            due: "2024-01-01".to_string(),
            stability: 1.0,
            difficulty: 5.0,
            state: "new".to_string(),
            reps: 0,
            lapses: 0,
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        };

        let diff_high_rating =
            FsrsService::calculate_difficulty(8.0, card.difficulty, CardState::New, &params);

        let diff_low_rating =
            FsrsService::calculate_difficulty(2.0, card.difficulty, CardState::New, &params);

        assert!(
            diff_high_rating < diff_low_rating,
            "Higher rating should result in lower difficulty"
        );
    }

    // Sanity Check 3: Stability increases with good reviews
    #[test]
    fn test_stability_increases_good_review() {
        let params = FsrsParameters::default();
        let old_stability = 5.0;

        let new_stability = FsrsService::calculate_stability(
            8.0, // Good rating
            old_stability,
            5.0,
            CardState::Review,
            &params,
        );

        assert!(
            new_stability > old_stability,
            "Good review should increase stability"
        );
    }

    // Sanity Check 4: Stability decreases with poor reviews
    #[test]
    fn test_stability_decreases_poor_review() {
        let params = FsrsParameters::default();
        let old_stability = 5.0;

        let new_stability = FsrsService::calculate_stability(
            2.0, // Poor rating
            old_stability,
            5.0,
            CardState::Review,
            &params,
        );

        assert!(
            new_stability < old_stability,
            "Poor review should decrease stability"
        );
    }

    // Sanity Check 5: Interval is always positive
    #[test]
    fn test_interval_always_positive() {
        for stability in [0.1, 1.0, 5.0, 100.0].iter() {
            let interval = FsrsService::calculate_interval(*stability, 0.95);
            assert!(interval > 0, "Interval must be positive");
        }
    }

    // Sanity Check 6: State transitions are correct
    #[test]
    fn test_state_transitions() {
        // New -> Learning (correct)
        assert_eq!(
            FsrsService::determine_state(CardState::New, true, 0),
            CardState::Learning
        );

        // New -> Relearning (incorrect)
        assert_eq!(
            FsrsService::determine_state(CardState::New, false, 0),
            CardState::Relearning
        );

        // Learning -> Review (correct)
        assert_eq!(
            FsrsService::determine_state(CardState::Learning, true, 2),
            CardState::Review
        );
    }

    // Sanity Check 7: Difficulty bounds (1-10)
    #[test]
    fn test_difficulty_bounds() {
        let params = FsrsParameters::default();

        // Very high rating should not exceed 10
        let high = FsrsService::calculate_difficulty(10.0, 8.0, CardState::New, &params);
        assert!(high <= 10.0, "Difficulty should not exceed 10");

        // Very low rating should not go below 1
        let low = FsrsService::calculate_difficulty(1.0, 2.0, CardState::New, &params);
        assert!(low >= 1.0, "Difficulty should not go below 1");
    }

    // Sanity Check 8: Stability minimum is respected
    #[test]
    fn test_stability_minimum() {
        let params = FsrsParameters::default();
        let min_stability =
            FsrsService::calculate_stability(1.0, 0.0, 1.0, CardState::New, &params);
        assert!(
            min_stability >= params.w_7,
            "Stability should respect minimum"
        );
    }

    // Sanity Check 9: Interval respects retention
    #[test]
    fn test_interval_respects_retention() {
        let high_retention = FsrsService::calculate_interval(10.0, 0.95);
        let low_retention = FsrsService::calculate_interval(10.0, 0.70);
        assert!(
            high_retention < low_retention,
            "Higher retention target = shorter intervals"
        );
    }

    // Sanity Check 10: Review correctness classification
    #[test]
    fn test_review_correctness() {
        assert!(ReviewRequest {
            card_id: "test".to_string(),
            problem_id: "p".to_string(),
            rating: 5,
            elapsed_seconds: 30,
        }
        .validate()
        .is_ok());

        assert!(ReviewRequest {
            card_id: "test".to_string(),
            problem_id: "p".to_string(),
            rating: 11,
            elapsed_seconds: 30,
        }
        .validate()
        .is_err());
    }
}
