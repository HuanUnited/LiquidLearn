// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use liquidlearn::db::ensure_database;
use tauri::Manager; // <-- ADD THIS IMPORT

fn main() {
    // Initialize Tauri runtime
    tauri::Builder::default()
        .setup(|app| {
            // Initialize database on app startup
            tauri::async_runtime::block_on(async {
                match ensure_database().await {
                    Ok(pool) => {
                        println!("✅ Database initialized successfully!");

                        // Store database pool in app state
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
