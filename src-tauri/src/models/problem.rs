use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Problem {
    pub id: String,
    pub topic_id: String,
    pub theory_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub difficulty: i32,
    pub is_solved: bool,
    pub total_unresolved_errors: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProblemRequest {
    pub topic_id: String,
    pub theory_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub difficulty: i32,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct ProblemWithStats {
    pub problem: Problem,
    pub attempts_count: i64,
    pub unresolved_errors: i64,
    pub fsrs_state: String,
    pub days_until_review: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProblemRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub difficulty: Option<i32>,
}
