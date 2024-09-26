use crate::app_state::AppState;
use crate::auth::jwt::Claims;
use crate::errors::{PricesError, PricesSuccess};
use crate::models::prices::{
    PriceDetails, PriceInsertResponse, PriceQueryParams, Prices, ProviderPriceAdd,
};
use axum::extract::{Path, Query, State};
use axum::Json;

pub(crate) async fn create_price_for_provider(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(json): Json<ProviderPriceAdd>,
) -> Result<PricesSuccess, PricesError> {
    let row: PriceInsertResponse = sqlx::query_as::<_, PriceInsertResponse>(
        "INSERT INTO oil_prices (provider_id, price) VALUES ($1, $2) RETURNING id",
    )
    .bind(id)
    .bind(json.price)
    .fetch_one(&state.db)
    .await
    .map_err(PricesError::insert_error)?;

    dbg!("Created price: {:?}", &json.price);

    Ok(PricesSuccess::created(row.id))
}

pub(crate) async fn fetch_prices(
    State(state): State<AppState>,
) -> Result<Json<Vec<Prices>>, PricesError> {
    let rows = sqlx::query_as::<_, Prices>(
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
            oil_prices.provider_id = providers.id
        ORDER BY
            oil_prices.created_at ASC
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(PricesError::fetch_error)?;

    Ok(Json(rows))
}

pub(crate) async fn fetch_prices_by_provider(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Query(params): Query<PriceQueryParams>,
) -> Result<Json<Vec<PriceDetails>>, PricesError> {
    let query = r#"
        SELECT
            oil_prices.price,
            oil_prices.created_at
        FROM
            oil_prices
        WHERE
            oil_prices.provider_id = $1
            AND oil_prices.price IS NOT NULL
            AND ($4 IS NULL OR oil_prices.created_at > $4)
            AND ($5 IS NULL OR oil_prices.created_at < $5)
        ORDER BY
            oil_prices.created_at ASC
        LIMIT $2
        OFFSET $3;
    "#;

    let results = sqlx::query_as::<_, PriceDetails>(query)
        .bind(id)
        .bind(params.limit.unwrap_or(1000))
        .bind(params.offset.unwrap_or(0))
        .bind(params.start)
        .bind(params.end)
        .fetch_all(&state.db)
        .await
        .map_err(PricesError::fetch_error)?;

    Ok(Json(results))
}

pub(crate) async fn delete_price(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<PricesSuccess, PricesError> {
    sqlx::query_as::<_, Prices>("SELECT * FROM oil_prices WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(PricesError::fetch_error)?
        .ok_or_else(|| PricesError::fetch_error(sqlx::Error::RowNotFound))?;

    sqlx::query("DELETE FROM oil_prices WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(PricesError::delete_error)?;

    Ok(PricesSuccess::deleted(id))
}
