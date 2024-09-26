use crate::app_state::AppState;
use crate::auth::jwt::Claims;
use crate::errors::{ScrapingRunsError, ScrapingRunsSuccess};
use crate::models::scraping_runs::{ScrapingRuns, ScrapingRunsInsertResponse};
use axum::extract::State;
use axum::Json;

pub(crate) async fn create_scraping_run(
    _claims: Claims,
    State(state): State<AppState>,
    Json(json): Json<ScrapingRuns>,
) -> Result<ScrapingRunsSuccess, ScrapingRunsError> {
    let row: ScrapingRunsInsertResponse = sqlx::query_as::<_, ScrapingRunsInsertResponse>(
        "INSERT INTO scraping_runs (start_time, end_time) VALUES ($1, $2) RETURNING id",
    )
    .bind(json.start_time)
    .bind(json.end_time)
    .fetch_one(&state.db)
    .await
    .map_err(ScrapingRunsError::insert_error)?;

    dbg!("Inserted row: {:?}", &row);

    Ok(ScrapingRunsSuccess::created(row.id))
}

pub(crate) async fn get_last_scraping_run_by_time(
    State(state): State<AppState>,
) -> Result<Json<ScrapingRuns>, ScrapingRunsError> {
    let res = sqlx::query_as::<_, ScrapingRuns>(
        "SELECT * FROM scraping_runs ORDER BY end_time DESC LIMIT 1",
    )
    .fetch_one(&state.db)
    .await
    .map_err(ScrapingRunsError::fetch_error)?;

    Ok(Json(res))
}
