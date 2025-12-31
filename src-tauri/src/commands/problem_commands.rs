use crate::models::problem::{CreateProblemRequest, Problem, UpdateProblemRequest};
use crate::services::{FsrsService, ProblemService};
use serde::Serialize;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_problem(
    db: State<'_, SqlitePool>,
    req: CreateProblemRequest,
) -> Result<Problem, String> {
    let problem = ProblemService::create(
        db.inner(),
        req.topic_id,
        req.theory_id,
        req.title,
        req.description,
        req.image_url,
        req.difficulty,
    )
    .await?;

    // Create FSRS card for this problem
    FsrsService::create_card(db.inner(), problem.id.clone()).await?;

    Ok(problem)
}

#[tauri::command]
pub async fn get_problem(db: State<'_, SqlitePool>, id: String) -> Result<Problem, String> {
    ProblemService::get_by_id(db.inner(), id).await
}

#[tauri::command]
pub async fn list_problems_by_topic(
    db: State<'_, SqlitePool>,
    topic_id: String,
) -> Result<Vec<Problem>, String> {
    ProblemService::list_by_topic(db.inner(), topic_id).await
}

#[tauri::command]
pub async fn list_problems_by_theory(
    db: State<'_, SqlitePool>,
    theory_id: String,
) -> Result<Vec<Problem>, String> {
    ProblemService::list_by_theory(db.inner(), theory_id).await
}

#[tauri::command]
pub async fn update_problem(
    db: State<'_, SqlitePool>,
    id: String,
    req: UpdateProblemRequest,
) -> Result<Problem, String> {
    ProblemService::update(
        db.inner(),
        id,
        req.title,
        req.description,
        req.image_url,
        req.difficulty,
    )
    .await
}

#[tauri::command]
pub async fn delete_problem(db: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    ProblemService::delete(db.inner(), id).await
}

#[tauri::command]
pub async fn mark_problem_solved(db: State<'_, SqlitePool>, id: String) -> Result<Problem, String> {
    ProblemService::mark_solved(db.inner(), id).await
}

#[derive(Debug, Serialize)]
pub struct ProblemWithDetails {
    pub problem: Problem,
    pub attempts_count: i64,
    pub unresolved_errors: i32,
}

#[tauri::command]
pub async fn get_problem_with_details(
    db: State<'_, SqlitePool>,
    id: String,
) -> Result<ProblemWithDetails, String> {
    let problem = ProblemService::get_by_id(db.inner(), id.clone()).await?;

    // Get attempts count and unresolved errors
    let attempts_count =
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM attempts WHERE problem_id = ?")
            .bind(&id)
            .fetch_one(db.inner())
            .await
            .map_err(|e| e.to_string())?
            .0;

    let unresolved_errors = ProblemService::get_unresolved_error_count(db.inner(), id).await?;

    Ok(ProblemWithDetails {
        problem,
        attempts_count,
        unresolved_errors,
    })
}
