use serde::{Deserialize, Serialize};

/// The 4 phases of decoupled learning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StudyPhase {
    #[serde(rename = "decode")]
    Decode, // Phase 1: Understand the problem
    #[serde(rename = "encode")]
    Encode, // Phase 2: Create mental model
    #[serde(rename = "recall")]
    Recall, // Phase 3: Retrieve from memory
    #[serde(rename = "reflect")]
    Reflect, // Phase 4: Analyze and optimize
}

impl StudyPhase {
    pub fn to_number(&self) -> i32 {
        match self {
            StudyPhase::Decode => 1,
            StudyPhase::Encode => 2,
            StudyPhase::Recall => 3,
            StudyPhase::Reflect => 4,
        }
    }

    pub fn from_number(n: i32) -> Self {
        match n {
            2 => StudyPhase::Encode,
            3 => StudyPhase::Recall,
            4 => StudyPhase::Reflect,
            _ => StudyPhase::Decode,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            StudyPhase::Decode => "decode".to_string(),
            StudyPhase::Encode => "encode".to_string(),
            StudyPhase::Recall => "recall".to_string(),
            StudyPhase::Reflect => "reflect".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "encode" => StudyPhase::Encode,
            "recall" => StudyPhase::Recall,
            "reflect" => StudyPhase::Reflect,
            _ => StudyPhase::Decode,
        }
    }

    /// Get description of this phase
    pub fn description(&self) -> &str {
        match self {
            StudyPhase::Decode => "Understand the problem statement and context",
            StudyPhase::Encode => "Create a mental model and solution approach",
            StudyPhase::Recall => "Retrieve solution from memory without looking",
            StudyPhase::Reflect => "Analyze mistakes and optimize solution",
        }
    }
}

/// Progress through study phases for a problem
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StudyPhaseProgress {
    pub id: String,
    pub problem_id: String,
    pub current_phase: i32, // 1-4
    pub current_step: i32,  // 1-N (substeps within phase)
    pub phase_time_total: i32,
    pub completed_at: Option<String>, // When all 4 phases finished
    pub created_at: String,
    pub updated_at: String,
}

impl StudyPhaseProgress {
    pub fn get_current_phase(&self) -> StudyPhase {
        StudyPhase::from_number(self.current_phase)
    }

    pub fn get_total_time(&self) -> i32 {
        self.phase_time_total
    }

    pub fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }

    pub fn get_completion_percent(&self) -> f64 {
        ((self.current_phase - 1) as f64 / 4.0 * 100.0).min(100.0)
    }
}

/// Request to advance to next phase
#[derive(Debug, Clone, Deserialize)]
pub struct AdvancePhaseRequest {
    pub problem_id: String,
    pub time_spent_seconds: i32,
    pub notes: Option<String>,
}

/// Response after advancing phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseAdvanceResult {
    pub problem_id: String,
    pub from_phase: i32,
    pub to_phase: i32,
    pub is_completed: bool,
}

/// Time update for current phase
#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePhaseTimeRequest {
    pub problem_id: String,
    pub elapsed_seconds: i32,
}

/// Current study session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudySessionData {
    pub problem_id: String,
    pub current_phase: String,
    pub current_step: i32,
    pub phase_description: String,
    pub time_in_phase: i32,
    pub total_completion: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_study_phase_conversion() {
        assert_eq!(StudyPhase::Decode.to_number(), 1);
        assert_eq!(StudyPhase::from_number(2), StudyPhase::Encode);
        assert_eq!(StudyPhase::Reflect.to_string(), "reflect");
        assert_eq!(StudyPhase::from_string("recall"), StudyPhase::Recall);
    }

    #[test]
    fn test_completion_percent() {
        let progress = StudyPhaseProgress {
            id: "test".to_string(),
            problem_id: "p1".to_string(),
            current_phase: 2,
            current_step: 1,
            phase_time_total: 100,
            completed_at: None,
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        };

        assert_eq!(progress.get_completion_percent(), 25.0);
        assert!(!progress.is_completed());
    }

    #[test]
    fn test_total_time_calculation() {
        let progress = StudyPhaseProgress {
            id: "test".to_string(),
            problem_id: "p1".to_string(),
            current_phase: 4,
            current_step: 1,
            phase_time_total: 200,
            completed_at: Some("2024-01-02".to_string()),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-02".to_string(),
        };

        assert_eq!(progress.get_total_time(), 450);
        assert!(progress.is_completed());
    }
}
