use crate::models::Topic;
use crate::services::TopicService;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_topic(
    db: State<'_, SqlitePool>,
    subject_id: String,
    name: String,
    description: Option<String>,
) -> Result<Topic, String> {
    TopicService::create(db.inner(), subject_id, name, description).await
}

#[tauri::command]
pub async fn get_topic(db: State<'_, SqlitePool>, id: String) -> Result<Topic, String> {
    TopicService::get_by_id(db.inner(), id).await
}

#[tauri::command]
pub async fn list_topics_by_subject(
    db: State<'_, SqlitePool>,
    subject_id: String,
) -> Result<Vec<Topic>, String> {
    TopicService::list_by_subject(db.inner(), subject_id).await
}

#[tauri::command]
pub async fn update_topic(
    db: State<'_, SqlitePool>,
    id: String,
    name: Option<String>,
    description: Option<String>,
) -> Result<Topic, String> {
    TopicService::update(db.inner(), id, name, description).await
}

#[tauri::command]
pub async fn delete_topic(db: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    TopicService::delete(db.inner(), id).await
}
