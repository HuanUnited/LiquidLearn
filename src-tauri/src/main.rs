// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use liquidlearn::db::ensure_database;
use tauri::Manager;

mod commands;
mod fsrs;
mod models;
mod services;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                match ensure_database().await {
                    Ok(pool) => {
                        println!("✅ Database initialized successfully!");
                        app.manage(pool);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to initialize database: {}", e);
                        std::process::exit(1);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Problem commands
            commands::create_problem,
            commands::get_problem,
            commands::update_problem,
            commands::delete_problem,
            commands::list_problems,
            commands::search_problems,
            commands::bulk_import_problems,
            commands::add_problem_tag,
            // FSRS commands
            commands::process_review,
            commands::get_next_due_problems,
            commands::get_card_stats,
            commands::get_fsrs_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
