use crate::models::subject::{CreateSubjectRequest, Subject};
use crate::services::SubjectService;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_subject(
    db: State<'_, SqlitePool>,
    req: CreateSubjectRequest,
) -> Result<Subject, String> {
    SubjectService::create(db.inner(), req.name, req.description).await
}

#[tauri::command]
pub async fn get_subject(db: State<'_, SqlitePool>, id: String) -> Result<Subject, String> {
    SubjectService::get_by_id(db.inner(), id).await
}

#[tauri::command]
pub async fn list_subjects(db: State<'_, SqlitePool>) -> Result<Vec<Subject>, String> {
    SubjectService::list_all(db.inner()).await
}

#[tauri::command]
pub async fn update_subject(
    db: State<'_, SqlitePool>,
    id: String,
    name: Option<String>,
    description: Option<String>,
) -> Result<Subject, String> {
    SubjectService::update(db.inner(), id, name, description).await
}

#[tauri::command]
pub async fn delete_subject(db: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    SubjectService::delete(db.inner(), id).await
}
