use crate::models::error_logging::*;
use sqlx::SqlitePool;

pub struct ErrorLoggingService;

impl ErrorLoggingService {
    // ========================================================================
    // ERROR LOGGING
    // ========================================================================

    /// Log a new error for a problem attempt
    pub async fn log_error(
        db: &SqlitePool,
        attempt_id: i64,
        error_type_id: i64,
        message: &str,
    ) -> Result<i64, String> {
        // Verify error type exists
        let _error_type_exists: (i64,) = sqlx::query_as("SELECT 1 FROM error_types WHERE id = ?")
            .bind(error_type_id)
            .fetch_optional(db)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| "Error type not found".to_string())?;

        // Insert the error
        let result = sqlx::query(
            "INSERT INTO problem_errors (attempt_id, error_type_id, message) 
             VALUES (?, ?, ?)",
        )
        .bind(attempt_id)
        .bind(error_type_id)
        .bind(message)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to log error: {}", e))?;

        let error_id = result.last_insert_rowid();

        // Get the problem_id from attempt
        let problem_id: (i64,) =
            sqlx::query_as("SELECT problem_id FROM problem_attempts WHERE id = ?")
                .bind(attempt_id)
                .fetch_one(db)
                .await
                .map_err(|e| format!("Failed to get problem: {}", e))?;

        // Update or insert problem_error_history
        sqlx::query(
            "INSERT INTO problem_error_history (problem_id, error_type_id, total_occurrences)
             VALUES (?, ?, 1)
             ON CONFLICT(problem_id, error_type_id) 
             DO UPDATE SET total_occurrences = total_occurrences + 1",
        )
        .bind(problem_id.0)
        .bind(error_type_id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to update history: {}", e))?;

        // Update fsrs_cards has_unresolved_errors flag
        sqlx::query(
            "UPDATE fsrs_cards SET has_unresolved_errors = 1 
     WHERE id = ?",
        )
        .bind(problem_id.0)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to update fsrs_cards: {}", e))?;

        // Update error impact modifier based on all errors
        // Convert i64 problem_id to String for FSRS function
        let _card_id: (String,) = sqlx::query_as("SELECT id FROM fsrs_cards WHERE problem_id = ?")
            .bind(problem_id.0)
            .fetch_one(db)
            .await
            .map_err(|e| format!("Failed to get card: {}", e))?;

        // Note: We'll call this from the service, not here
        // The error modifier will be recalculated on review

        Ok(error_id)
    }

    // ========================================================================
    // ERROR RESOLUTION
    // ========================================================================

    /// Resolve an error with details
    pub async fn resolve_error(
        db: &SqlitePool,
        error_id: i64,
        resolution_notes: &str,
        time_to_fix_seconds: i64,
        successful: bool,
    ) -> Result<i64, String> {
        // Insert resolution
        let result = sqlx::query(
            "INSERT INTO error_resolutions 
             (error_id, resolution_notes, time_to_fix_seconds, successful)
             VALUES (?, ?, ?, ?)",
        )
        .bind(error_id)
        .bind(resolution_notes)
        .bind(time_to_fix_seconds)
        .bind(if successful { 1 } else { 0 })
        .execute(db)
        .await
        .map_err(|e| format!("Failed to resolve error: {}", e))?;

        // Update problem_error_history resolution_count if successful
        if successful {
            let _error: (i64, i64) = sqlx::query_as(
                "SELECT pe.id, pe.error_type_id FROM problem_errors pe WHERE pe.id = ?",
            )
            .bind(error_id)
            .fetch_one(db)
            .await
            .map_err(|e| format!("Failed to get error: {}", e))?;
        }

        Ok(result.last_insert_rowid())
    }

    // ========================================================================
    // ANALYTICS & REPORTING
    // ========================================================================

    /// Get all error types
    pub async fn get_error_types(db: &SqlitePool) -> Result<Vec<ErrorType>, String> {
        let errors = sqlx::query_as::<_, (i64, String, Option<String>, f64, String)>(
            "SELECT id, name, description, multiplier, created_at FROM error_types 
             ORDER BY multiplier DESC",
        )
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to get error types: {}", e))?;

        Ok(errors
            .into_iter()
            .map(
                |(id, name, description, multiplier, created_at)| ErrorType {
                    id,
                    name,
                    description,
                    multiplier,
                    created_at,
                },
            )
            .collect())
    }

    /// Get error frequency statistics
    pub async fn get_error_frequency(db: &SqlitePool) -> Result<ErrorAnalyticsResponse, String> {
        // Get error statistics
        let stats: Vec<(i64, String, i64, i64, f64)> = sqlx::query_as(
            "SELECT 
                et.id,
                et.name,
                COUNT(pe.id) as total_count,
                COUNT(DISTINCT CASE WHEN er.id IS NOT NULL THEN pe.id END) as resolved_count,
                ROUND(100.0 * COUNT(DISTINCT CASE WHEN er.successful = 1 THEN pe.id END) 
                  / NULLIF(COUNT(DISTINCT CASE WHEN er.id IS NOT NULL THEN pe.id END), 0), 1) as resolution_rate
             FROM error_types et
             LEFT JOIN problem_errors pe ON et.id = pe.error_type_id
             LEFT JOIN error_resolutions er ON pe.id = er.error_id
             GROUP BY et.id, et.name
             ORDER BY total_count DESC"
        )
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to get error frequency: {}", e))?;

        let total_errors: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM problem_errors")
            .fetch_one(db)
            .await
            .map_err(|e| format!("Failed to get total errors: {}", e))?;

        let resolved_errors: (i64,) = sqlx::query_as(
            "SELECT COUNT(DISTINCT pe.id) FROM problem_errors pe
             JOIN error_resolutions er ON pe.id = er.error_id",
        )
        .fetch_one(db)
        .await
        .map_err(|e| format!("Failed to get resolved errors: {}", e))?;

        let resolution_rate = if total_errors.0 > 0 {
            (resolved_errors.0 as f64 / total_errors.0 as f64) * 100.0
        } else {
            0.0
        };

        let most_common = if !stats.is_empty() {
            Some(stats[0].1.clone())
        } else {
            None
        };

        let errors_by_type: Vec<ErrorTypeStats> = stats
            .into_iter()
            .map(|(id, name, total, resolved, rate)| ErrorTypeStats {
                error_type_id: id,
                error_type_name: name,
                total_count: total,
                resolved_count: resolved,
                resolution_rate: rate,
                multiplier: 1.0,
            })
            .collect();

        Ok(ErrorAnalyticsResponse {
            total_errors: total_errors.0,
            resolved_errors: resolved_errors.0,
            resolution_rate,
            most_common_error: most_common,
            errors_by_type,
        })
    }

    /// Get problem error history
    pub async fn get_problem_error_history(
        db: &SqlitePool,
        problem_id: i64,
    ) -> Result<Vec<ProblemErrorHistory>, String> {
        let history = sqlx::query_as::<_, (i64, i64, String, i64, i64, String)>(
            "SELECT 
                peh.problem_id,
                peh.error_type_id,
                et.name,
                peh.total_occurrences,
                peh.resolution_count,
                peh.last_occurred
             FROM problem_error_history peh
             JOIN error_types et ON peh.error_type_id = et.id
             WHERE peh.problem_id = ?
             ORDER BY peh.total_occurrences DESC",
        )
        .bind(problem_id)
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to get error history: {}", e))?;

        Ok(history
            .into_iter()
            .map(
                |(problem_id, error_type_id, name, total, resolved, last_occurred)| {
                    ProblemErrorHistory {
                        problem_id,
                        error_type_id,
                        error_type_name: name,
                        total_occurrences: total,
                        resolution_count: resolved,
                        last_occurred,
                    }
                },
            )
            .collect())
    }

    /// Get unresolved errors
    pub async fn get_unresolved_errors(
        db: &SqlitePool,
    ) -> Result<Vec<ProblemErrorWithType>, String> {
        let errors = sqlx::query_as::<_, (i64, i64, i64, String, String, f64, String)>(
            "SELECT 
                pe.id,
                pe.attempt_id,
                pe.error_type_id,
                et.name,
                pe.message,
                et.multiplier,
                pe.created_at
             FROM problem_errors pe
             JOIN error_types et ON pe.error_type_id = et.id
             LEFT JOIN error_resolutions er ON pe.id = er.error_id
             WHERE er.id IS NULL
             ORDER BY pe.created_at DESC",
        )
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to get unresolved errors: {}", e))?;

        Ok(errors
            .into_iter()
            .map(
                |(id, attempt_id, error_type_id, name, message, multiplier, created_at)| {
                    ProblemErrorWithType {
                        id,
                        attempt_id,
                        error_type_id,
                        error_type_name: name,
                        message,
                        multiplier,
                        created_at,
                    }
                },
            )
            .collect())
    }
}
