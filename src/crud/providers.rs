use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sqlx::Executor;

use crate::app_state::AppState;
use crate::models::{ProviderAdd, Providers};

pub(crate) async fn create_provider(
    State(state): State<AppState>,
    Json(json): Json<ProviderAdd>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) =
        sqlx::query("INSERT INTO providers (name, url, html_element) VALUES ($1, $2, $3)")
            .bind(json.name)
            .bind(json.url)
            .bind(json.html_element)
            .execute(&state.db)
            .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while inserting a record: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

pub(crate) async fn fetch_providers(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let res: Vec<Providers> = match sqlx::query_as::<_, Providers>("SELECT * FROM providers")
        .fetch_all(&state.db)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    Ok(Json(res))
}

pub(crate) async fn fetch_provider(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let res: Providers =
        match sqlx::query_as::<_, Providers>("SELECT * FROM providers WHERE id = $1")
            .bind(id)
            .fetch_one(&state.db)
            .await
        {
            Ok(res) => {
                update_last_accessed(State(state), Path(id)).await?;
                res
            }
            Err(e) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
            }
        };

    Ok(Json(res))
}

pub(crate) async fn update_provider(
    State(state): State<AppState>,
    Json(json): Json<Providers>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) =
        sqlx::query("UPDATE providers SET name = $1, url = $2, html_element = $3 WHERE id = $4")
            .bind(json.name)
            .bind(json.url)
            .bind(json.html_element)
            .bind(json.id)
            .execute(&state.db)
            .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while updating a record: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

pub(crate) async fn update_last_accessed(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    if let Err(e) = sqlx::query("UPDATE providers SET last_accessed = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while updating a record: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

pub(crate) async fn delete_provider(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("DELETE FROM providers WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while deleting a record: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}
