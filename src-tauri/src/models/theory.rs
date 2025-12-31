use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Theory {
    pub id: String,
    pub topic_id: String,
    pub phase_number: i32, // 1, 2, 3, 4 (mapped as 1.1, 1.2, 2.1, 2.2 in UI)
    pub title: String,
    pub content: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateTheoryRequest {
    pub topic_id: String,
    pub phase_number: i32,
    pub title: String,
    pub content: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct TheoryWithPhase {
    pub theory: Theory,
    pub phase_label: String, // "1.1", "1.2", etc
}

#[allow(dead_code)]
impl Theory {
    pub fn get_phase_label(&self) -> String {
        match self.phase_number {
            1 => "1.1".to_string(),
            2 => "1.2".to_string(),
            3 => "2.1".to_string(),
            4 => "2.2".to_string(),
            5 => "3".to_string(),
            6 => "4".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}
