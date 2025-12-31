#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod migrations;
mod models;
mod services;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;
use tauri::State;

#[tauri::command]
async fn initialize_db(db: State<'_, SqlitePool>) -> Result<String, String> {
    // Run migrations
    migrations::run_migrations(db.inner())
        .await
        .map_err(|e| e.to_string())?;

    // Initialize default error types
    services::ErrorService::init_default_error_types(db.inner()).await?;

    Ok("Database initialized successfully".to_string())
}

#[tokio::main]
async fn main() {
    // Create database directory if it doesn't exist
    let app_data_dir = tauri::api::path::app_data_dir(&tauri::Config::default())
        .unwrap_or_else(|| std::path::PathBuf::from("./"));

    std::fs::create_dir_all(&app_data_dir).ok();

    // Database path
    let db_path = app_data_dir.join("liquidlearn.db");
    let database_url = format!("sqlite:{}", db_path.to_string_lossy());

    // SQLite connection options
    let connect_options = SqliteConnectOptions::from_str(&database_url)
        .expect("Invalid database URL")
        .create_if_missing(true);

    // Create pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .expect("Failed to create pool");

    // Run migrations on startup
    migrations::run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    // Initialize default error types
    services::ErrorService::init_default_error_types(&pool)
        .await
        .expect("Failed to initialize error types");

    println!("✓ Database initialized at: {}", db_path.display());

    tauri::Builder::default()
        .manage(pool)
        .invoke_handler(tauri::generate_handler![
            // System
            initialize_db,
            // Subjects
            commands::create_subject,
            commands::get_subject,
            commands::list_subjects,
            commands::update_subject,
            commands::delete_subject,
            // Topics
            commands::create_topic,
            commands::get_topic,
            commands::list_topics_by_subject,
            commands::update_topic,
            commands::delete_topic,
            // Theories
            commands::create_theory,
            commands::get_theory,
            commands::list_theories_by_topic,
            commands::get_theory_by_phase,
            commands::update_theory,
            commands::delete_theory,
            // Problems
            commands::create_problem,
            commands::get_problem,
            commands::list_problems_by_topic,
            commands::list_problems_by_theory,
            commands::update_problem,
            commands::delete_problem,
            commands::mark_problem_solved,
            commands::get_problem_with_details,
            // Attempts
            commands::create_attempt,
            commands::get_attempt,
            commands::list_attempts_by_problem,
            commands::update_attempt_commentary,
            commands::get_problem_attempt_stats,
            // Errors
            commands::log_error,
            commands::resolve_error,
            commands::get_error_types,
            commands::get_errors_by_attempt,
            commands::get_unresolved_errors_by_problem,
            commands::init_error_types,
            // FSRS
            commands::process_review,
            commands::get_due_cards,
            commands::get_fsrs_stats,
            commands::get_fsrs_card,
            commands::get_fsrs_card_by_problem,
            commands::get_cards_by_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
