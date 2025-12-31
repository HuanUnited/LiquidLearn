use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ErrorType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub multiplier: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AttemptError {
    pub id: String,
    pub attempt_id: String,
    pub error_type_id: i32,
    pub description: Option<String>,
    pub is_resolved: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct LogErrorRequest {
    pub attempt_id: String,
    pub error_type_id: i32,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResolveErrorRequest {
    pub error_id: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct AttemptErrorWithType {
    pub error: AttemptError,
    pub error_type: ErrorType,
}

// Seed error types
pub fn get_default_error_types() -> Vec<(i32, &'static str, &'static str, f64)> {
    vec![
        (1, "Conceptual Error", "Misunderstood core concept", 1.5),
        (2, "Terminology Error", "Mixed up definitions", 1.5),
        (3, "Logical Gap", "Missing step in reasoning", 1.5),
        (4, "Performance Error", "Algorithm too slow", 1.5),
        (5, "Off-by-One Error", "Boundary mistake", 1.0),
        (6, "Edge Case Error", "Missed special cases", 1.0),
        (7, "Careless Error", "Typo or silly mistake", 1.0),
        (8, "Implementation Error", "Code structure issue", 1.0),
        (9, "Unoptimized", "Works but inefficient", 0.7),
        (10, "Over Time Limit", "Exceeded time budget", 0.7),
    ]
}
