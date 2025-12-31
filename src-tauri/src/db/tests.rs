#[cfg(test)]
mod tests {
    use crate::db::{ensure_database, test_connection};

    #[tokio::test]
    async fn test_database_initialization() {
        let pool = ensure_database().await;
        assert!(pool.is_ok(), "Database initialization failed");

        let pool = pool.unwrap();
        let result: Result<(), anyhow::Error> = test_connection(&pool).await;
        assert!(result.is_ok(), "Database connection test failed");
    }
}
