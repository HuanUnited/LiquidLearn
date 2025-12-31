use serde::Serialize;
use sqlx::SqlitePool;
use tauri::State;

use crate::fsrs::FsrsParameters;
use crate::models::FsrsCard;
use crate::services::{CardStats, DueCard, FsrsService};

#[derive(Serialize)]
pub struct ReviewResultResponse {
    pub card: FsrsCard,
    pub new_interval: i32,
    pub new_state: String,
    pub is_lapse: bool,
}

#[tauri::command]
pub async fn process_review(
    card_id: String,
    rating: i32,
    elapsed_seconds: i64,
    pool: State<'_, SqlitePool>,
) -> Result<FsrsCard, String> {
    FsrsService::process_card_review(&card_id, rating, elapsed_seconds, &pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_next_due_problems(
    limit: i32,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<DueCard>, String> {
    FsrsService::get_next_due(limit, &pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_card_stats(
    card_id: String,
    pool: State<'_, SqlitePool>,
) -> Result<CardStats, String> {
    FsrsService::get_card_stats(&card_id, &pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_fsrs_config(pool: State<'_, SqlitePool>) -> Result<FsrsParameters, String> {
    FsrsService::load_parameters(&pool)
        .await
        .map_err(|e| e.to_string())
}
