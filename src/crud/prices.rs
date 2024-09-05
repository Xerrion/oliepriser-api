use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::app_state::AppState;
use crate::models::{PriceDetails, Prices, ProviderPriceAdd};

pub(crate) async fn create_price_for_provider(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(json): Json<ProviderPriceAdd>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("INSERT INTO oil_prices (provider_id, price) VALUES ($1, $2)")
        .bind(id)
        .bind(json.price)
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

pub(crate) async fn fetch_prices(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let rows: Vec<Prices> = match sqlx::query_as::<_, Prices>(
        r#"
        SELECT 
            oil_prices.id AS id,
            oil_prices.price,
            oil_prices.created_at,
            providers.id AS provider_id
        FROM
            oil_prices
        JOIN 
            providers
        ON 
            oil_prices.provider_id = providers.id;
    "#,
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    Ok(Json(rows))
}

pub(crate) async fn fetch_prices_by_provider(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let query = r#"
        SELECT 
            oil_prices.price,
            oil_prices.created_at
        FROM
            oil_prices
        WHERE 
            oil_prices.provider_id = $1;
    "#;

    let results: Vec<PriceDetails> = match sqlx::query_as::<_, PriceDetails>(query)
        .bind(id)
        .fetch_all(&state.db)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    Ok(Json(results))
}

pub(crate) async fn delete_price(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("DELETE FROM oil_prices WHERE id = $1")
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
