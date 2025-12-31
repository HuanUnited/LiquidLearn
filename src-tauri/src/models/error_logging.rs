use serde::{Deserialize, Serialize};

// ============================================================================
// ERROR TYPE MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorType {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub multiplier: f64,
    pub created_at: String,
}

// ============================================================================
// PROBLEM ERROR MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemError {
    pub id: i64,
    pub attempt_id: i64,
    pub error_type_id: i64,
    pub message: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemErrorWithType {
    pub id: i64,
    pub attempt_id: i64,
    pub error_type_id: i64,
    pub error_type_name: String,
    pub message: String,
    pub multiplier: f64,
    pub created_at: String,
}

// ============================================================================
// ERROR RESOLUTION MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResolution {
    pub id: i64,
    pub error_id: i64,
    pub resolution_notes: Option<String>,
    pub time_to_fix_seconds: Option<i64>,
    pub re_attempted: i64,
    pub successful: i64,
    pub created_at: String,
}

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogErrorRequest {
    pub attempt_id: i64,
    pub error_type_id: i64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveErrorRequest {
    pub error_id: i64,
    pub resolution_notes: String,
    pub time_to_fix_seconds: i64,
    pub successful: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorAnalyticsResponse {
    pub total_errors: i64,
    pub resolved_errors: i64,
    pub resolution_rate: f64,
    pub most_common_error: Option<String>,
    pub errors_by_type: Vec<ErrorTypeStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorTypeStats {
    pub error_type_id: i64,
    pub error_type_name: String,
    pub total_count: i64,
    pub resolved_count: i64,
    pub resolution_rate: f64,
    pub multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemErrorHistory {
    pub problem_id: i64,
    pub error_type_id: i64,
    pub error_type_name: String,
    pub total_occurrences: i64,
    pub resolution_count: i64,
    pub last_occurred: String,
}

// ============================================================================
// SMART RECOMMENDATION MODELS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecommendationTier {
    Tier1,
    Tier2,
    Tier3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemRecommendation {
    pub problem_id: i64,
    pub problem_name: String,
    pub tier: String,
    pub due: String,
    pub unresolved_error_count: i64,
    pub error_types: Vec<String>,
    pub priority_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRecommendationsResponse {
    pub tier_1_critical: Vec<ProblemRecommendation>,
    pub tier_2_due: Vec<ProblemRecommendation>,
    pub tier_3_remediation: Vec<ProblemRecommendation>,
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

impl ErrorType {
    pub fn is_high_impact(&self) -> bool {
        self.multiplier >= 1.5
    }

    pub fn is_low_impact(&self) -> bool {
        self.multiplier <= 0.7
    }
}

impl ErrorResolution {
    pub fn is_successful(&self) -> bool {
        self.successful == 1
    }
}

impl RecommendationTier {
    pub fn as_string(&self) -> String {
        match self {
            RecommendationTier::Tier1 => "Tier1".to_string(),
            RecommendationTier::Tier2 => "Tier2".to_string(),
            RecommendationTier::Tier3 => "Tier3".to_string(),
        }
    }

    pub fn color(&self) -> String {
        match self {
            RecommendationTier::Tier1 => "#EF4444".to_string(), // Red
            RecommendationTier::Tier2 => "#3B82F6".to_string(), // Blue
            RecommendationTier::Tier3 => "#F59E0B".to_string(), // Amber
        }
    }
}
