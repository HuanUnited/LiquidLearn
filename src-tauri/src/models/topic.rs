use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Topic {
    pub id: String,
    pub subject_id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateTopicRequest {
    pub subject_id: String,
    pub name: String,
    pub description: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct TopicWithStats {
    pub topic: Topic,
    pub total_problems: i64,
    pub solved_problems: i64,
    pub theories_count: i64,
}
