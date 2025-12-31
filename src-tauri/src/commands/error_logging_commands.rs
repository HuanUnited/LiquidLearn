use crate::db::Database;
use crate::models::error_logging::*;
use crate::services::error_logging_service::ErrorLoggingService;
use tauri::State;

// ============================================================================
// ERROR LOGGING COMMANDS
// ============================================================================

#[tauri::command]
pub async fn log_error(request: LogErrorRequest, db: State<'_, Database>) -> Result<i64, String> {
    ErrorLoggingService::log_error(
        db.pool(),
        request.attempt_id,
        request.error_type_id,
        &request.message,
    )
    .await
}

// ============================================================================
// ERROR RESOLUTION COMMANDS
// ============================================================================

#[tauri::command]
pub async fn resolve_error(
    request: ResolveErrorRequest,
    db: State<'_, Database>,
) -> Result<i64, String> {
    ErrorLoggingService::resolve_error(
        db.pool(),
        request.error_id,
        &request.resolution_notes,
        request.time_to_fix_seconds,
        request.successful,
    )
    .await
}

// ============================================================================
// ANALYTICS COMMANDS
// ============================================================================

#[tauri::command]
pub async fn get_error_types(db: State<'_, Database>) -> Result<Vec<ErrorType>, String> {
    ErrorLoggingService::get_error_types(db.pool()).await
}

#[tauri::command]
pub async fn get_error_frequency(
    db: State<'_, Database>,
) -> Result<ErrorAnalyticsResponse, String> {
    ErrorLoggingService::get_error_frequency(db.pool()).await
}

#[tauri::command]
pub async fn get_problem_error_history(
    problem_id: i64,
    db: State<'_, Database>,
) -> Result<Vec<ProblemErrorHistory>, String> {
    ErrorLoggingService::get_problem_error_history(db.pool(), problem_id).await
}

#[tauri::command]
pub async fn get_unresolved_errors(
    db: State<'_, Database>,
) -> Result<Vec<ProblemErrorWithType>, String> {
    ErrorLoggingService::get_unresolved_errors(db.pool()).await
}
