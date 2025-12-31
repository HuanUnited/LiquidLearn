use crate::db::Database;
use crate::models::study_phase::{AdvancePhaseRequest, UpdatePhaseTimeRequest};
use crate::services::StudyPhaseService;

#[tauri::command]
pub async fn get_study_progress(
    problem_id: String,
    db: tauri::State<'_, Database>,
) -> Result<serde_json::Value, String> {
    StudyPhaseService::get_progress(&problem_id, db.pool())
        .await
        .map(|p| serde_json::to_value(p).unwrap_or(serde_json::json!({})))
        .map_err(|e| format!("Failed to get progress: {}", e))
}

#[tauri::command]
pub async fn get_all_study_progress(
    db: tauri::State<'_, Database>,
) -> Result<serde_json::Value, String> {
    StudyPhaseService::get_all_progress(db.pool())
        .await
        .map(|p| serde_json::to_value(p).unwrap_or(serde_json::json!([])))
        .map_err(|e| format!("Failed to get all progress: {}", e))
}

#[tauri::command]
pub async fn advance_study_phase(
    problem_id: String,
    time_spent_seconds: i32,
    notes: Option<String>,
    db: tauri::State<'_, Database>,
) -> Result<serde_json::Value, String> {
    let request = AdvancePhaseRequest {
        problem_id,
        time_spent_seconds,
        notes,
    };

    StudyPhaseService::advance_phase(request, db.pool())
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(serde_json::json!({})))
}

#[tauri::command]
pub async fn update_study_phase_time(
    problem_id: String,
    elapsed_seconds: i32,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    let request = UpdatePhaseTimeRequest {
        problem_id,
        elapsed_seconds,
    };

    StudyPhaseService::update_phase_time(request, db.pool()).await
}

#[tauri::command]
pub async fn get_study_summary(
    db: tauri::State<'_, Database>,
) -> Result<serde_json::Value, String> {
    StudyPhaseService::get_summary(db.pool()).await
}

#[tauri::command]
pub async fn get_phase_queue(db: tauri::State<'_, Database>) -> Result<serde_json::Value, String> {
    StudyPhaseService::get_phase_queue(db.pool()).await
}
