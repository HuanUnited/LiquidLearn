use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attempt {
    pub id: String,
    pub problem_id: String,
    pub is_solved: bool,
    pub commentary: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAttemptRequest {
    pub problem_id: String,
    pub is_solved: bool,
    pub commentary: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct AttemptWithErrors {
    pub attempt: Attempt,
    pub errors: Vec<AttemptError>,
}

// Re-export from error module
use super::error_log::AttemptError;
