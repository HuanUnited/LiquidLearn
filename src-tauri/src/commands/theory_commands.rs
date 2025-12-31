use crate::models::Theory;
use crate::services::TheoryService;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_theory(
    db: State<'_, SqlitePool>,
    topic_id: String,
    phase_number: i32,
    title: String,
    content: Option<String>,
) -> Result<Theory, String> {
    TheoryService::create(db.inner(), topic_id, phase_number, title, content).await
}

#[tauri::command]
pub async fn get_theory(db: State<'_, SqlitePool>, id: String) -> Result<Theory, String> {
    TheoryService::get_by_id(db.inner(), id).await
}

#[tauri::command]
pub async fn list_theories_by_topic(
    db: State<'_, SqlitePool>,
    topic_id: String,
) -> Result<Vec<Theory>, String> {
    TheoryService::list_by_topic(db.inner(), topic_id).await
}

#[tauri::command]
pub async fn get_theory_by_phase(
    db: State<'_, SqlitePool>,
    topic_id: String,
    phase_number: i32,
) -> Result<Option<Theory>, String> {
    TheoryService::get_by_phase(db.inner(), topic_id, phase_number).await
}

#[tauri::command]
pub async fn update_theory(
    db: State<'_, SqlitePool>,
    id: String,
    title: Option<String>,
    content: Option<String>,
) -> Result<Theory, String> {
    TheoryService::update(db.inner(), id, title, content).await
}

#[tauri::command]
pub async fn delete_theory(db: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    TheoryService::delete(db.inner(), id).await
}
