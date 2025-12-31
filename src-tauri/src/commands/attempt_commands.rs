use crate::models::attempt::CreateAttemptRequest;
use crate::models::Attempt;
use crate::services::{AttemptService, FsrsService, ProblemService};
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_attempt(
    db: State<'_, SqlitePool>,
    req: CreateAttemptRequest,
) -> Result<Attempt, String> {
    let attempt = AttemptService::create(
        db.inner(),
        req.problem_id.clone(),
        req.is_solved,
        req.commentary,
    )
    .await?;

    let attempt_count =
        AttemptService::get_count_by_problem(db.inner(), req.problem_id.clone()).await?;

    if attempt_count == 1 {
        // FIX: Add .clone() here. This ensures the original
        // req.problem_id is still available for the next block.
        let _ = FsrsService::get_or_create_card(db.inner(), req.problem_id.clone()).await;
    }

    if req.is_solved {
        // Now this works because the previous call didn't "eat" the original string.
        let _ = ProblemService::mark_solved(db.inner(), req.problem_id).await;
    }

    Ok(attempt)
}

#[tauri::command]
pub async fn get_attempt(db: State<'_, SqlitePool>, id: String) -> Result<Attempt, String> {
    AttemptService::get_by_id(db.inner(), id).await
}

#[tauri::command]
pub async fn list_attempts_by_problem(
    db: State<'_, SqlitePool>,
    problem_id: String,
) -> Result<Vec<Attempt>, String> {
    AttemptService::list_by_problem(db.inner(), problem_id).await
}

#[tauri::command]
pub async fn update_attempt_commentary(
    db: State<'_, SqlitePool>,
    id: String,
    commentary: String,
) -> Result<Attempt, String> {
    AttemptService::update_commentary(db.inner(), id, commentary).await
}

#[tauri::command]
pub async fn get_problem_attempt_stats(
    db: State<'_, SqlitePool>,
    problem_id: String,
) -> Result<serde_json::Value, String> {
    let total = AttemptService::get_count_by_problem(db.inner(), problem_id.clone()).await?;
    let successful = AttemptService::get_success_count_by_problem(db.inner(), problem_id).await?;

    Ok(serde_json::json!({
        "total_attempts": total,
        "successful_attempts": successful,
        "success_rate": if total > 0 { successful as f64 / total as f64 * 100.0 } else { 0.0 }
    }))
}
