#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod db;
mod models;
mod services;

use db::Database;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
async fn init_db() -> Result<String, String> {
    match Database::new().await {
        Ok(_) => Ok("Database initialized successfully".to_string()),
        Err(e) => Err(format!("Database init failed: {}", e)),
    }
}

fn main() {
    // Initialize database on startup
    let rt = tokio::runtime::Runtime::new().unwrap();
    let db = rt.block_on(async {
        Database::new().await.unwrap_or_else(|e| {
            eprintln!("Failed to initialize database: {}", e);
            std::process::exit(1);
        })
    });

    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            greet,
            init_db,
            // ===== CRUD Commands =====
            commands::create_problem,
            commands::get_problem,
            commands::get_full_problem,
            commands::list_problems,
            commands::update_problem,
            commands::delete_problem,
            commands::delete_problem_cascade,
            // ===== Tag Commands =====
            commands::add_tag,
            commands::remove_tag,
            commands::get_problem_tags,
            commands::add_tags,
            commands::remove_tags,
            commands::set_tags,
            commands::get_all_tags,
            commands::get_problems_by_tags,
            // ===== Search & Filter Commands =====
            commands::search_problems,
            commands::filter_problems,
            // ===== Bulk & Import Commands =====
            commands::bulk_create_problems,
            commands::import_csv_problems,
            // ===== Stats Commands =====
            commands::get_problem_count,
            commands::get_problem_count_by_difficulty,
            // ===== FSRS Algorithm Commands =====
            commands::process_review,
            commands::get_due_cards_count,
            commands::get_fsrs_stats,
            commands::get_fsrs_parameters,
            // ===== Study Phase Commands =====
            commands::get_study_progress,
            commands::get_all_study_progress,
            commands::advance_study_phase,
            commands::update_study_phase_time,
            commands::get_study_summary,
            commands::get_phase_queue,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
