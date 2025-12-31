use serde::{Deserialize, Serialize};

/// FSRS Card state machine
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CardState {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "learning")]
    Learning,
    #[serde(rename = "review")]
    Review,
    #[serde(rename = "relearning")]
    Relearning,
}

impl CardState {
    pub fn to_string(&self) -> String {
        match self {
            CardState::New => "new".to_string(),
            CardState::Learning => "learning".to_string(),
            CardState::Review => "review".to_string(),
            CardState::Relearning => "relearning".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "learning" => CardState::Learning,
            "review" => CardState::Review,
            "relearning" => CardState::Relearning,
            _ => CardState::New,
        }
    }
}

/// FSRS Card representing a problem to review
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FsrsCard {
    pub id: String,
    pub problem_id: String,
    pub due: String,     // Next review date
    pub stability: f64,  // How stable the memory is
    pub difficulty: f64, // How difficult the problem is (1-10)
    pub state: String,   // new, learning, review, relearning
    pub reps: i32,       // Total repetitions
    pub lapses: i32,     // Times forgotten
    pub created_at: String,
    pub updated_at: String,
}

impl FsrsCard {
    pub fn get_state(&self) -> CardState {
        CardState::from_string(&self.state)
    }

    pub fn is_due_today(&self, today: &str) -> bool {
        self.due.as_str() <= today // Convert String to &str
    }
}

/// FSRS Parameters (w_1 through w_19)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FsrsParameters {
    pub id: String,
    pub w_1: f64,               // Initial stability for new cards
    pub w_2: f64,               // Stability growth multiplier
    pub w_3: f64,               // Initial difficulty
    pub w_4: f64,               // Difficulty decay
    pub w_5: f64,               // Difficulty increase
    pub w_6: f64,               // Stability increase for easy reviews
    pub w_7: f64,               // Minimum stability
    pub w_8: f64,               // Stability factor for hard reviews
    pub w_9: f64,               // Difficulty growth
    pub w_10: f64,              // Difficulty decrease
    pub w_11: f64,              // Stability increase for first review
    pub w_12: f64,              // Difficulty increase for lapses
    pub w_13: f64,              // Lapses reduction
    pub w_14: f64,              // New card interval
    pub w_15: f64,              // Learning card interval
    pub w_16: f64,              // Easy factor
    pub w_17: f64,              // Hard factor
    pub w_18: f64,              // Review interval factor
    pub w_19: f64,              // Relearning interval factor
    pub desired_retention: f64, // Target retention (0.8-0.99)
    pub total_reviews: i32,     // Total reviews done (for calibration)
    pub created_at: String,
    pub updated_at: String,
}

impl FsrsParameters {
    /// Get default FSRS v3 parameters
    pub fn default() -> Self {
        Self {
            id: "global".to_string(),
            w_1: 0.40,
            w_2: 1.86,
            w_3: 4.93,
            w_4: 0.94,
            w_5: 0.86,
            w_6: 0.01,
            w_7: 1.49,
            w_8: 0.04,
            w_9: 0.36,
            w_10: 0.86,
            w_11: 0.20,
            w_12: 2.50,
            w_13: 0.14,
            w_14: 0.94,
            w_15: 0.16,
            w_16: 0.10,
            w_17: 0.29,
            w_18: 0.34,
            w_19: 3.73,
            desired_retention: 0.95,
            total_reviews: 0,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Review request (user submits after reviewing card)
#[derive(Debug, Clone, Deserialize)]
pub struct ReviewRequest {
    pub card_id: String,
    pub problem_id: String,
    pub rating: u8,           // 1-10 (1=fail, 10=perfect)
    pub elapsed_seconds: i64, // Time spent on this card
}

impl ReviewRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.rating < 1 || self.rating > 10 {
            return Err("Rating must be between 1 and 10".to_string());
        }
        if self.elapsed_seconds < 0 {
            return Err("Elapsed time cannot be negative".to_string());
        }
        Ok(())
    }
}

/// Review result (calculated by algorithm)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResult {
    pub card_id: String,
    pub new_state: String,
    pub new_stability: f64,
    pub new_difficulty: f64,
    pub new_interval: i32, // Days until next review
    pub next_due: String,  // When to review next
    pub time_spent: i64,   // Time spent on this review
    pub is_correct: bool,  // Rating >= 5 is considered correct
}

/// Card for review (what user sees)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardForReview {
    pub id: String,
    pub problem_id: String,
    pub problem_title: String,
    pub problem_description: Option<String>,
    pub problem_difficulty: i32,
    pub card_state: String,
    pub reps: i32,
    pub lapses: i32,
    pub stability: f64,
    pub difficulty: f64,
}

/// Statistics about review state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewStats {
    pub total_cards: i64,
    pub new_count: i64,
    pub learning_count: i64,
    pub review_count: i64,
    pub relearning_count: i64,
    pub due_today: i64,
    pub retention_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_state_conversion() {
        assert_eq!(CardState::New.to_string(), "new");
        assert_eq!(CardState::from_string("review"), CardState::Review);
    }

    #[test]
    fn test_review_request_validation() {
        let valid = ReviewRequest {
            card_id: "id".to_string(),
            problem_id: "pid".to_string(),
            rating: 5,
            elapsed_seconds: 30,
        };
        assert!(valid.validate().is_ok());

        let invalid_rating = ReviewRequest {
            rating: 11,
            ..valid.clone()
        };
        assert!(invalid_rating.validate().is_err());

        let invalid_time = ReviewRequest {
            elapsed_seconds: -1,
            ..valid
        };
        assert!(invalid_time.validate().is_err());
    }

    #[test]
    fn test_default_parameters() {
        let params = FsrsParameters::default();
        assert_eq!(params.w_1, 0.40);
        assert_eq!(params.desired_retention, 0.95);
    }
}
