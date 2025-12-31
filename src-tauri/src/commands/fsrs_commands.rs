use crate::models::fsrs::{FsrsStats, ReviewRequest, ReviewResult};
use crate::models::FsrsCard;
use crate::services::FsrsService;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn process_review(
    db: State<'_, SqlitePool>,
    req: ReviewRequest,
) -> Result<ReviewResult, String> {
    FsrsService::process_review(db.inner(), req).await
}

#[tauri::command]
pub async fn get_due_cards(db: State<'_, SqlitePool>) -> Result<Vec<FsrsCard>, String> {
    FsrsService::get_due_cards(db.inner()).await
}

#[tauri::command]
pub async fn get_fsrs_stats(db: State<'_, SqlitePool>) -> Result<FsrsStats, String> {
    FsrsService::get_stats(db.inner()).await
}

#[tauri::command]
pub async fn get_fsrs_card(db: State<'_, SqlitePool>, card_id: String) -> Result<FsrsCard, String> {
    FsrsService::get_card_by_id(db.inner(), card_id).await
}

#[tauri::command]
pub async fn get_fsrs_card_by_problem(
    db: State<'_, SqlitePool>,
    problem_id: String,
) -> Result<FsrsCard, String> {
    FsrsService::get_or_create_card(db.inner(), problem_id).await
}

#[tauri::command]
pub async fn get_cards_by_state(
    db: State<'_, SqlitePool>,
    state: String,
) -> Result<Vec<FsrsCard>, String> {
    FsrsService::get_cards_by_state(db.inner(), state).await
}
