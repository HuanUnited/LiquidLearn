use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Problem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub difficulty: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProblemRequest {
    pub title: String,
    pub description: Option<String>,
    pub difficulty: i32,
}
