use crate::db::Database;
use crate::models::ReviewRequest;
use crate::services::FsrsService;

#[tauri::command]
pub async fn process_review(
    request: ReviewRequest,
    db: tauri::State<'_, Database>,
) -> Result<serde_json::Value, String> {
    FsrsService::process_review(request, db.pool())
        .await
        .map(|result| serde_json::to_value(result).unwrap_or(serde_json::json!({})))
        .map_err(|e| format!("Failed to process review: {}", e))
}

#[tauri::command]
pub async fn get_due_cards_count(db: tauri::State<'_, Database>) -> Result<i64, String> {
    FsrsService::get_due_cards(db.pool())
        .await
        .map_err(|e| format!("Failed to get due cards: {}", e))
}

#[tauri::command]
pub async fn get_fsrs_stats(db: tauri::State<'_, Database>) -> Result<serde_json::Value, String> {
    FsrsService::get_stats(db.pool())
        .await
        .map_err(|e| format!("Failed to get stats: {}", e))
}

#[tauri::command]
pub async fn get_fsrs_parameters(
    db: tauri::State<'_, Database>,
) -> Result<serde_json::Value, String> {
    FsrsService::get_parameters(db.pool())
        .await
        .map(|p| serde_json::to_value(p).unwrap_or(serde_json::json!({})))
        .map_err(|e| format!("Failed to get parameters: {}", e))
}
