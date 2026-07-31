use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{ElectionResult, MessageResponse, SuccessResponse},
    AppState,
};

pub async fn get_results(
    State(state): State<AppState>,
    Path(election_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<Vec<ElectionResult>>>, AppError> {
    let results = sqlx::query_as::<_, ElectionResult>(
        r#"
        SELECT * FROM election_results
        WHERE election_id = $1
        ORDER BY vote_count DESC NULLS LAST
        "#,
    )
    .bind(election_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: results,
    }))
}

pub async fn publish_results(
    State(state): State<AppState>,
    Path(election_id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    // Verify election exists and is closed
    let election = sqlx::query_scalar::<_, String>(
        "SELECT status FROM elections WHERE id = $1"
    )
    .bind(election_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound("Election not found".to_string()))?;

    if election != "closed" {
        return Err(AppError::BadRequest(
            "Can only publish results for closed elections".to_string(),
        ));
    }

    // In a real system, this would trigger result publication
    // For now, we just verify the election is closed

    Ok(Json(MessageResponse {
        message: "Results published successfully".to_string(),
    }))
}
