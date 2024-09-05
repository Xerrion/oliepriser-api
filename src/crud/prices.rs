use crate::app_state::AppState;
use crate::models::{OilPriceWithProvider, PriceAdd, PriceDetails, ProviderPrices};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub(crate) async fn create_price(
    State(state): State<AppState>,
    Json(json): Json<PriceAdd>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("INSERT INTO oil_prices (provider_id, price) VALUES ($1, $2)")
        .bind(json.provider_id)
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
    let rows: Vec<OilPriceWithProvider> = match sqlx::query_as::<_, OilPriceWithProvider>(
        r#"
        SELECT 
            oil_prices.id AS oil_price_id,
            oil_prices.price,
            oil_prices.created_at,
            providers.name AS provider_name,
            providers.url AS provider_url
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

    // Transform the data into the desired structure
    let mut provider_map: std::collections::HashMap<String, Vec<(i32, PriceDetails)>> =
        std::collections::HashMap::new();

    for row in rows {
        let price_details = PriceDetails {
            price: row.price,
            created_at: row.created_at,
        };

        provider_map
            .entry(row.provider_name)
            .or_insert_with(Vec::new)
            .push((row.oil_price_id, price_details));
    }

    let response_data: Vec<ProviderPrices> = provider_map
        .into_iter()
        .map(|(provider_name, prices)| ProviderPrices {
            provider_name,
            prices,
        })
        .collect();

    Ok(Json(response_data))
}

pub(crate) async fn fetch_oil_prices_by_provider(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let query = r#"
        SELECT 
            oil_prices.id AS oil_price_id,
            oil_prices.price,
            providers.name AS provider_name,
            providers.url
        FROM 
            oil_prices
        JOIN 
            providers
        ON 
            oil_prices.provider_id = providers.id
        WHERE 
            providers.id = $1;
    "#;

    let results = match sqlx::query_as::<_, OilPriceWithProvider>(query)
        .bind(id) // Bind the provider_id parameter
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
