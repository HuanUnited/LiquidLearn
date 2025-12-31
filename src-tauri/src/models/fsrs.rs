use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FsrsCard {
    pub id: String,
    pub problem_id: String,
    pub due: NaiveDate,
    pub stability: f64,
    pub difficulty: f64,
    pub state: String, // 'new', 'learning', 'review', 'relearning'
    pub reps: i32,
    pub lapses: i32,
    pub last_review: Option<DateTime<Utc>>,
    pub last_elapsed_days: i32,
    pub scheduled_days: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FsrsReview {
    pub id: String,
    pub card_id: String,
    pub problem_id: String,
    pub rating: i32, // 1-10
    pub state_before: String,
    pub elapsed_seconds: i32,
    pub elapsed_days: i32,
    pub scheduled_days_before: i32,
    pub scheduled_days_after: i32,
    pub ease: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FsrsParameters {
    pub id: String, // Always 'global'
    pub w_1: f64,
    pub w_2: f64,
    pub w_3: f64,
    pub w_4: f64,
    pub w_5: f64,
    pub w_6: f64,
    pub w_7: f64,
    pub w_8: f64,
    pub w_9: f64,
    pub w_10: f64,
    pub w_11: f64,
    pub w_12: f64,
    pub w_13: f64,
    pub w_14: f64,
    pub w_15: f64,
    pub w_16: f64,
    pub w_17: f64,
    pub w_18: f64,
    pub w_19: f64,
    pub desired_retention: f64,
    pub total_reviews: i32,
    pub last_calibrated: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct ProcessReviewRequest {
    pub card_id: String,
    pub rating: i32,
    pub elapsed_seconds: i32,
}

#[derive(Debug, Serialize)]
pub struct ReviewResult {
    pub new_interval: i32,
    pub new_difficulty: f64,
    pub new_stability: f64,
    pub next_due: NaiveDate,
    pub is_lapse: bool,
    pub new_state: String,
}
