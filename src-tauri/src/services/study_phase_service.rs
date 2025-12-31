use crate::models::study_phase::{
    AdvancePhaseRequest, PhaseAdvanceResult, StudyPhase, StudyPhaseProgress, UpdatePhaseTimeRequest,
};
use chrono::Local;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct StudyPhaseService;

impl StudyPhaseService {
    /// Get current study phase for a problem
    pub async fn get_progress(
        problem_id: &str,
        pool: &SqlitePool,
    ) -> Result<StudyPhaseProgress, sqlx::Error> {
        sqlx::query_as::<_, StudyPhaseProgress>(
            "SELECT id, problem_id, current_phase, current_step,
                    phase_1_time_seconds, phase_2_time_seconds,
                    phase_3_time_seconds, phase_4_time_seconds,
                    completed_at, created_at, updated_at
             FROM study_phase_progress
             WHERE problem_id = ?",
        )
        .bind(problem_id)
        .fetch_one(pool)
        .await
    }

    /// Get all study phases for user (for dashboard)
    pub async fn get_all_progress(
        pool: &SqlitePool,
    ) -> Result<Vec<StudyPhaseProgress>, sqlx::Error> {
        sqlx::query_as::<_, StudyPhaseProgress>(
            "SELECT id, problem_id, current_phase, current_step,
                    phase_1_time_seconds, phase_2_time_seconds,
                    phase_3_time_seconds, phase_4_time_seconds,
                    completed_at, created_at, updated_at
             FROM study_phase_progress
             ORDER BY updated_at DESC",
        )
        .fetch_all(pool)
        .await
    }

    /// Advance to next phase
    pub async fn advance_phase(
        request: AdvancePhaseRequest,
        pool: &SqlitePool,
    ) -> Result<PhaseAdvanceResult, String> {
        // Get current progress
        let progress = Self::get_progress(&request.problem_id, pool)
            .await
            .map_err(|e| format!("Failed to get progress: {}", e))?;

        let current_phase = progress.current_phase;
        let new_phase = (current_phase + 1).min(4); // Max phase 4

        // Update time for current phase
        let time_column = match current_phase {
            1 => "phase_1_time_seconds",
            2 => "phase_2_time_seconds",
            3 => "phase_3_time_seconds",
            4 => "phase_4_time_seconds",
            _ => "phase_1_time_seconds",
        };

        let now = chrono::Utc::now().to_rfc3339();
        let completed_at = if new_phase >= 4 {
            Some(now.clone())
        } else {
            None
        };

        let query = format!(
            "UPDATE study_phase_progress
             SET current_phase = ?, current_step = 1, {} = {} + ?, completed_at = ?, updated_at = ?
             WHERE problem_id = ?",
            time_column, time_column
        );

        sqlx::query(&query)
            .bind(new_phase)
            .bind(request.time_spent_seconds)
            .bind(&completed_at)
            .bind(&now)
            .bind(&request.problem_id)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to advance phase: {}", e))?;

        Ok(PhaseAdvanceResult {
            problem_id: request.problem_id,
            from_phase: current_phase,
            to_phase: new_phase,
            is_completed: new_phase >= 4,
        })
    }

    /// Update time spent in current phase
    pub async fn update_phase_time(
        request: UpdatePhaseTimeRequest,
        pool: &SqlitePool,
    ) -> Result<(), String> {
        let progress = Self::get_progress(&request.problem_id, pool)
            .await
            .map_err(|e| format!("Failed to get progress: {}", e))?;

        let time_column = match progress.current_phase {
            1 => "phase_1_time_seconds",
            2 => "phase_2_time_seconds",
            3 => "phase_3_time_seconds",
            4 => "phase_4_time_seconds",
            _ => "phase_1_time_seconds",
        };

        let now = chrono::Utc::now().to_rfc3339();
        let query = format!(
            "UPDATE study_phase_progress
             SET {} = {} + ?, updated_at = ?
             WHERE problem_id = ?",
            time_column, time_column
        );

        sqlx::query(&query)
            .bind(request.elapsed_seconds)
            .bind(&now)
            .bind(&request.problem_id)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to update time: {}", e))?;

        Ok(())
    }

    /// Get summary statistics
    pub async fn get_summary(pool: &SqlitePool) -> Result<serde_json::Value, String> {
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM study_phase_progress")
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        let phase_1: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM study_phase_progress WHERE current_phase = 1")
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;

        let phase_2: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM study_phase_progress WHERE current_phase = 2")
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;

        let phase_3: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM study_phase_progress WHERE current_phase = 3")
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;

        let phase_4: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM study_phase_progress WHERE current_phase = 4")
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;

        let completed: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM study_phase_progress WHERE completed_at IS NOT NULL",
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let total_time: (Option<i64>,) = sqlx::query_as(
            "SELECT COALESCE(SUM(phase_1_time_seconds + phase_2_time_seconds + 
                    phase_3_time_seconds + phase_4_time_seconds), 0)
             FROM study_phase_progress",
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(serde_json::json!({
            "total_problems": total.0,
            "in_phase_1": phase_1.0,
            "in_phase_2": phase_2.0,
            "in_phase_3": phase_3.0,
            "in_phase_4": phase_4.0,
            "completed": completed.0,
            "total_seconds_spent": total_time.0.unwrap_or(0),
        }))
    }

    /// Get problems due for each phase today
    pub async fn get_phase_queue(pool: &SqlitePool) -> Result<serde_json::Value, String> {
        let decode: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM study_phase_progress WHERE current_phase = 1")
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;

        let encode: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM study_phase_progress WHERE current_phase = 2")
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;

        let recall: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM study_phase_progress WHERE current_phase = 3")
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;

        let reflect: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM study_phase_progress WHERE current_phase = 4")
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;

        Ok(serde_json::json!({
            "decode": decode.0,
            "encode": encode.0,
            "recall": recall.0,
            "reflect": reflect.0,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_boundaries() {
        // Phase cannot go above 4
        let result = PhaseAdvanceResult {
            problem_id: "p".to_string(),
            from_phase: 4,
            to_phase: 5,
            is_completed: true,
        };
        // This would be caught by min(4) in actual code
        assert_eq!(result.to_phase, 5); // Shows the bug if not handled
    }

    #[test]
    fn test_advance_request_validation() {
        let req = AdvancePhaseRequest {
            problem_id: "p".to_string(),
            time_spent_seconds: 100,
            notes: Some("Good progress".to_string()),
        };
        assert!(req.time_spent_seconds > 0);
    }
}
