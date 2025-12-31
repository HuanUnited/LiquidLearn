use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FsrsCard {
    pub id: String,
    pub problem_id: String,
    pub due: String,
    pub stability: f64,
    pub difficulty: f64,
    pub state: String, // new, learning, review, relearning
    pub reps: i32,
    pub lapses: i32,
    pub elapsed_days: i32,
    pub scheduled_days: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewRequest {
    pub problem_id: String,
    pub attempt_is_solved: bool, // Whether the attempt was successful
    pub quality: u8,             // 1-5 (1=fail, 5=perfect)
    pub time_spent_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResult {
    pub problem_id: String,
    pub card_id: String,
    pub new_state: String,
    pub new_stability: f64,
    pub new_difficulty: f64,
    pub new_interval_days: i32,
    pub next_due: String,
    pub is_correct: bool,
}

#[derive(Debug, Serialize)]
pub struct FsrsStats {
    pub total_cards: i64,
    pub new_count: i64,
    pub learning_count: i64,
    pub review_count: i64,
    pub relearning_count: i64,
    pub due_today: i64,
    pub retention_rate: f64,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct ReviewStats {
    pub total_problems: i64,
    pub due_problems: i64,
    pub new_problems: i64,
}

#[allow(dead_code)]
impl FsrsCard {
    pub fn is_due(&self) -> bool {
        self.due <= chrono::Utc::now().to_rfc3339()
    }

    pub fn get_state_display(&self) -> String {
        match self.state.as_str() {
            "new" => "New",
            "learning" => "Learning",
            "review" => "Review",
            "relearning" => "Relearning",
            _ => "Unknown",
        }
        .to_string()
    }
}
