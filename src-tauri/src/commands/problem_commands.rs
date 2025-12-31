use crate::db::Database;
use crate::models::{CreateProblemRequest, Problem};
use crate::services::ProblemService;

#[tauri::command]
pub async fn create_problem(
    title: String,
    description: Option<String>,
    difficulty: i32,
    db: tauri::State<'_, Database>,
) -> Result<Problem, String> {
    let request = CreateProblemRequest {
        title,
        description,
        difficulty,
    };

    ProblemService::create(request, db.pool())
        .await
        .map_err(|e| format!("Failed to create problem: {}", e))
}

#[tauri::command]
pub async fn get_problem(id: String, db: tauri::State<'_, Database>) -> Result<Problem, String> {
    ProblemService::get(&id, db.pool())
        .await
        .map_err(|e| format!("Failed to get problem: {}", e))
}

#[tauri::command]
pub async fn list_problems(
    difficulty: Option<i32>,
    solved_only: Option<bool>,
    db: tauri::State<'_, Database>,
) -> Result<Vec<Problem>, String> {
    ProblemService::list(db.pool(), difficulty, solved_only)
        .await
        .map_err(|e| format!("Failed to list problems: {}", e))
}

#[tauri::command]
pub async fn update_problem(
    id: String,
    title: Option<String>,
    description: Option<String>,
    difficulty: Option<i32>,
    db: tauri::State<'_, Database>,
) -> Result<Problem, String> {
    ProblemService::update(&id, title, description, difficulty, db.pool())
        .await
        .map_err(|e| format!("Failed to update problem: {}", e))
}

#[tauri::command]
pub async fn delete_problem(id: String, db: tauri::State<'_, Database>) -> Result<(), String> {
    ProblemService::delete(&id, db.pool())
        .await
        .map_err(|e| format!("Failed to delete problem: {}", e))
}

#[tauri::command]
pub async fn add_tag(
    problem_id: String,
    tag_name: String,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    ProblemService::add_tag(&problem_id, &tag_name, db.pool())
        .await
        .map_err(|e| format!("Failed to add tag: {}", e))
}

#[tauri::command]
pub async fn remove_tag(
    problem_id: String,
    tag_name: String,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    ProblemService::remove_tag(&problem_id, &tag_name, db.pool())
        .await
        .map_err(|e| format!("Failed to remove tag: {}", e))
}

#[tauri::command]
pub async fn get_problem_tags(
    problem_id: String,
    db: tauri::State<'_, Database>,
) -> Result<Vec<String>, String> {
    ProblemService::get_tags(&problem_id, db.pool())
        .await
        .map_err(|e| format!("Failed to get tags: {}", e))
}

#[tauri::command]
pub async fn search_problems(
    query: String,
    db: tauri::State<'_, Database>,
) -> Result<Vec<Problem>, String> {
    ProblemService::search(&query, db.pool())
        .await
        .map_err(|e| format!("Failed to search problems: {}", e))
}

#[tauri::command]
pub async fn filter_problems(
    difficulty: Option<i32>,
    tags: Option<Vec<String>>,
    solved_only: Option<bool>,
    db: tauri::State<'_, Database>,
) -> Result<Vec<Problem>, String> {
    ProblemService::filter(difficulty, tags, solved_only, db.pool())
        .await
        .map_err(|e| format!("Failed to filter problems: {}", e))
}

#[tauri::command]
pub async fn bulk_create_problems(
    problems: Vec<(String, Option<String>, i32)>,
    db: tauri::State<'_, Database>,
) -> Result<usize, String> {
    let requests: Vec<CreateProblemRequest> = problems
        .into_iter()
        .map(|(title, description, difficulty)| CreateProblemRequest {
            title,
            description,
            difficulty,
        })
        .collect();

    ProblemService::bulk_create(requests, db.pool())
        .await
        .map(|created| created.len())
        .map_err(|e| format!("Failed to bulk create: {}", e))
}

#[tauri::command]
pub async fn get_problem_count(db: tauri::State<'_, Database>) -> Result<i64, String> {
    ProblemService::count_total(db.pool())
        .await
        .map_err(|e| format!("Failed to count problems: {}", e))
}

#[tauri::command]
pub async fn get_problem_count_by_difficulty(
    db: tauri::State<'_, Database>,
) -> Result<Vec<(i32, i64)>, String> {
    ProblemService::count_by_difficulty(db.pool())
        .await
        .map_err(|e| format!("Failed to get counts: {}", e))
}
#[tauri::command]
pub async fn get_full_problem(
    id: String,
    db: tauri::State<'_, Database>,
) -> Result<serde_json::Value, String> {
    ProblemService::get_full(&id, db.pool())
        .await
        .map_err(|e| format!("Failed to get full problem: {}", e))
}

#[tauri::command]
pub async fn delete_problem_cascade(
    id: String,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    ProblemService::delete_with_cascade(&id, db.pool())
        .await
        .map_err(|e| format!("Failed to delete problem: {}", e))
}

#[tauri::command]
pub async fn add_tags(
    problem_id: String,
    tag_names: Vec<String>,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    ProblemService::add_tags(&problem_id, tag_names, db.pool())
        .await
        .map_err(|e| format!("Failed to add tags: {}", e))
}

#[tauri::command]
pub async fn remove_tags(
    problem_id: String,
    tag_names: Vec<String>,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    ProblemService::remove_tags(&problem_id, tag_names, db.pool())
        .await
        .map_err(|e| format!("Failed to remove tags: {}", e))
}

#[tauri::command]
pub async fn set_tags(
    problem_id: String,
    tag_names: Vec<String>,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    ProblemService::set_tags(&problem_id, tag_names, db.pool())
        .await
        .map_err(|e| format!("Failed to set tags: {}", e))
}

#[tauri::command]
pub async fn get_all_tags(db: tauri::State<'_, Database>) -> Result<Vec<String>, String> {
    ProblemService::get_all_tags(db.pool())
        .await
        .map_err(|e| format!("Failed to get tags: {}", e))
}

#[tauri::command]
pub async fn get_problems_by_tags(
    tag_names: Vec<String>,
    db: tauri::State<'_, Database>,
) -> Result<Vec<Problem>, String> {
    ProblemService::get_by_tags(tag_names, db.pool())
        .await
        .map_err(|e| format!("Failed to get problems by tags: {}", e))
}

#[tauri::command]
pub async fn import_csv_problems(
    csv_data: String,
    db: tauri::State<'_, Database>,
) -> Result<usize, String> {
    // Parse CSV: each line is "title|description|difficulty"
    let problems: Vec<CreateProblemRequest> = csv_data
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            CreateProblemRequest {
                title: parts.get(0).unwrap_or(&"").to_string(),
                description: parts.get(1).map(|s| s.to_string()),
                difficulty: parts
                    .get(2)
                    .and_then(|s| s.parse::<i32>().ok())
                    .unwrap_or(3),
            }
        })
        .collect();

    ProblemService::bulk_create(problems, db.pool())
        .await
        .map(|created| created.len())
        .map_err(|e| format!("Failed to import CSV: {}", e))
}
