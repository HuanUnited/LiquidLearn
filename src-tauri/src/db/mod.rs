use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteConnectOptions};
use std::path::PathBuf;
use std::str::FromStr;

/// Get application data directory path
pub fn get_app_data_path() -> Result<PathBuf> {
    let app_data = std::env::var("APPDATA")
        .unwrap_or_else(|_| ".".to_string());
    
    let path = PathBuf::from(app_data).join("LiquidLearn");
    
    // Create directory if it doesn't exist
    std::fs::create_dir_all(&path)?;
    
    Ok(path)
}

/// Ensure database exists and is initialized
pub async fn ensure_database() -> Result<SqlitePool> {
    // Get database path
    let db_path = get_app_data_path()?.join("liquidlearn.db");
    
    println!("Database path: {:?}", db_path);
    
    // Create connection options
    let options = SqliteConnectOptions::from_str(
        &format!("sqlite://{}?mode=rwc", db_path.display())
    )?
    .create_if_missing(true);
    
    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
    
    // Run migrations
    println!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    
    println!("Database initialized successfully!");
    
    Ok(pool)
}

/// Test database connection
pub async fn test_connection(pool: &SqlitePool) -> Result<()> {
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM problems")
        .fetch_one(pool)
        .await?;
    
    println!("Database test successful! Problem count: {}", result.0);
    Ok(())
}

#[cfg(test)]
mod tests;