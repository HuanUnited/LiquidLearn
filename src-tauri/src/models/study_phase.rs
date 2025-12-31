use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StudyPhaseProgress {
    pub id: String,
    pub problem_id: String,
    pub current_phase: i32, // 1-4
    pub current_step: i32,  // 1-3
    pub time_spent_seconds: i32,
    pub phase_1_completed: bool,
    pub phase_2_completed: bool,
    pub phase_3_completed: bool,
    pub phase_4_completed: bool,
    pub all_phases_completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct AdvanceStepRequest {
    pub problem_id: String,
    pub direction: String, // 'next' or 'prev'
}

#[derive(Debug, Deserialize)]
pub struct JumpToPhaseRequest {
    pub problem_id: String,
    pub phase: i32,
    pub step: i32,
}

#[derive(Debug, Serialize)]
pub struct PhaseContent {
    pub phase: i32,
    pub step: i32,
    pub title: String,
    pub instruction: String,
    pub question: Option<String>,
    pub timer_minutes: i32,
    pub checkpoint: Option<String>,
}
