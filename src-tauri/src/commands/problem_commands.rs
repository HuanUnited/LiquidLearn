use serde::Serialize;
use sqlx::SqlitePool;
use tauri::State;

use crate::models::{CreateProblemRequest, Problem, ProblemWithMastery, UpdateProblemRequest};
use crate::services::ProblemService;

// Response types
#[derive(Serialize)]
pub struct BulkImportResponse {
    imported: i32,
    failed: i32,
    errors: Vec<String>,
}

#[derive(Serialize)]
pub struct ListProblemsResponse {
    problems: Vec<ProblemWithMastery>,
    total: usize,
}

// Commands

#[tauri::command]
pub async fn create_problem(
    title: String,
    description: Option<String>,
    difficulty: i32,
    pool: State<'_, SqlitePool>,
) -> Result<Problem, String> {
    let req = CreateProblemRequest {
        title,
        description,
        difficulty,
    };

    match ProblemService::create(req, &pool).await {
        Ok(problem) => Ok(problem),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn get_problem(
    id: String,
    pool: State<'_, SqlitePool>,
) -> Result<ProblemWithMastery, String> {
    match ProblemService::read_with_details(&id, &pool).await {
        Ok(problem) => Ok(problem),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn update_problem(
    id: String,
    title: Option<String>,
    description: Option<String>,
    difficulty: Option<i32>,
    pool: State<'_, SqlitePool>,
) -> Result<Problem, String> {
    let req = UpdateProblemRequest {
        title,
        description,
        difficulty,
    };

    match ProblemService::update(&id, req, &pool).await {
        Ok(problem) => Ok(problem),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn delete_problem(id: String, pool: State<'_, SqlitePool>) -> Result<(), String> {
    match ProblemService::delete(&id, &pool).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn list_problems(
    filter: Option<String>,
    sort: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
    pool: State<'_, SqlitePool>,
) -> Result<ListProblemsResponse, String> {
    match ProblemService::list(
        filter,
        sort,
        limit.unwrap_or(50),
        offset.unwrap_or(0),
        &pool,
    )
    .await
    {
        Ok(problems) => {
            let total = problems.len();
            Ok(ListProblemsResponse { problems, total })
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn search_problems(
    query: String,
    limit: Option<i32>,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Problem>, String> {
    match ProblemService::search(&query, limit.unwrap_or(20), &pool).await {
        Ok(problems) => Ok(problems),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn bulk_import_problems(
    csv_content: String,
    pool: State<'_, SqlitePool>,
) -> Result<BulkImportResponse, String> {
    match ProblemService::bulk_import(&csv_content, &pool).await {
        Ok((imported, failed, errors)) => Ok(BulkImportResponse {
            imported,
            failed,
            errors,
        }),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn add_problem_tag(
    problem_id: String,
    tag_name: String,
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    match ProblemService::add_tag(&problem_id, &tag_name, &pool).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
