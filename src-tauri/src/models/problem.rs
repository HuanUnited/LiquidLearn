use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Problem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub difficulty: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProblemTag {
    pub id: String,
    pub problem_id: String,
    pub tag_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProblemMastery {
    pub id: String,
    pub problem_id: String,
    pub solved: bool,
    pub mastery_percent: f64,
    pub last_attempted: Option<DateTime<Utc>>,
    pub attempt_count: i32,
    pub updated_at: DateTime<Utc>,
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct CreateProblemRequest {
    pub title: String,
    pub description: Option<String>,
    pub difficulty: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProblemRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ProblemWithMastery {
    pub problem: Problem,
    pub mastery: Option<ProblemMastery>,
    pub tags: Vec<String>,
}
