use crate::app_state::AppState;
use crate::auth::jwt::Claims;
use crate::errors::{ScrapingRunsError, ScrapingRunsSuccess};
use crate::models::scraping_runs::ScrapingRuns;
use axum::extract::State;
use axum::Json;

pub(crate) async fn create_scraping_run(
    _claims: Claims,
    State(state): State<AppState>,
    Json(json): Json<ScrapingRuns>,
) -> Result<ScrapingRunsSuccess, ScrapingRunsError> {
    let row: (i32,) = sqlx::query_as::<_, (i32,)>(
        "INSERT INTO scraping_runs (start_time, end_time) VALUES ($1, $2) RETURNING id",
    )
    .bind(json.start_time)
    .bind(json.end_time)
    .fetch_one(&state.db)
    .await
    .map_err(ScrapingRunsError::insert_error)?;

    Ok(ScrapingRunsSuccess::created(row.0))
}

pub(crate) async fn get_last_scraping_run_by_time(
    State(state): State<AppState>,
) -> Result<Json<ScrapingRuns>, ScrapingRunsError> {
    let res: ScrapingRuns = match sqlx::query_as::<_, ScrapingRuns>(
        "SELECT * FROM scraping_runs ORDER BY end_time DESC LIMIT 1",
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err(ScrapingRunsError::fetch_error(e));
        }
    };

    Ok(Json(res))
}
