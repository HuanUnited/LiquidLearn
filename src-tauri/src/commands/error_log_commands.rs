use crate::models::error_log::{LogErrorRequest, ResolveErrorRequest};
use crate::models::{AttemptError, ErrorType};
use crate::services::{ErrorService, ProblemService};
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn log_error(
    db: State<'_, SqlitePool>,
    req: LogErrorRequest,
) -> Result<AttemptError, String> {
    let error = ErrorService::log_error(
        db.inner(),
        req.attempt_id,
        req.error_type_id,
        req.description,
    )
    .await?;

    // Update problem's unresolved error count
    let attempt = sqlx::query_as::<_, (String,)>("SELECT problem_id FROM attempts WHERE id = ?")
        .bind(&error.attempt_id)
        .fetch_one(db.inner())
        .await
        .map_err(|e| e.to_string())?;

    let unresolved_count =
        ErrorService::count_unresolved_by_problem(db.inner(), attempt.0.clone()).await?;
    let _ = ProblemService::update_unresolved_errors(db.inner(), attempt.0, unresolved_count).await;

    Ok(error)
}

#[tauri::command]
pub async fn resolve_error(
    db: State<'_, SqlitePool>,
    req: ResolveErrorRequest,
) -> Result<AttemptError, String> {
    let error = ErrorService::resolve_error(db.inner(), req.error_id).await?;

    // Update problem's unresolved error count
    let attempt = sqlx::query_as::<_, (String,)>("SELECT problem_id FROM attempts WHERE id = ?")
        .bind(&error.attempt_id)
        .fetch_one(db.inner())
        .await
        .map_err(|e| e.to_string())?;

    let unresolved_count =
        ErrorService::count_unresolved_by_problem(db.inner(), attempt.0.clone()).await?;
    let _ = ProblemService::update_unresolved_errors(db.inner(), attempt.0, unresolved_count).await;

    Ok(error)
}

#[tauri::command]
pub async fn get_error_types(db: State<'_, SqlitePool>) -> Result<Vec<ErrorType>, String> {
    ErrorService::get_error_types(db.inner()).await
}

#[tauri::command]
pub async fn get_errors_by_attempt(
    db: State<'_, SqlitePool>,
    attempt_id: String,
) -> Result<Vec<AttemptError>, String> {
    ErrorService::get_errors_by_attempt(db.inner(), attempt_id).await
}

#[tauri::command]
pub async fn get_unresolved_errors_by_problem(
    db: State<'_, SqlitePool>,
    problem_id: String,
) -> Result<Vec<AttemptError>, String> {
    ErrorService::get_unresolved_errors_by_problem(db.inner(), problem_id).await
}

#[tauri::command]
pub async fn init_error_types(db: State<'_, SqlitePool>) -> Result<(), String> {
    ErrorService::init_default_error_types(db.inner()).await
}
