use crate::app_state::AppState;
use crate::auth::jwt::Claims;
use crate::errors::{DeliveryZonesError, DeliveryZonesSuccess};
use crate::helpers::zone_exists;
use crate::models::delivery_zones::{DeliveryZones, DeliveryZonesAdd, DeliveryZonesInsertResponse};
use axum::extract::{Path, State};
use axum::Json;

/// Creates a new delivery zone in the database.
///
/// # Arguments
///
/// * `_claims` - The JWT claims of the authenticated user.
/// * `state` - The application state containing the database connection pool.
/// * `json` - The JSON payload containing the delivery zone details.
///
/// # Returns
///
/// * `Result<DeliveryZonesSuccess, DeliveryZonesError>` - The result of the operation, either a success or an error.
pub(crate) async fn create_delivery_zone(
    _claims: Claims,
    State(state): State<AppState>,
    Json(json): Json<DeliveryZonesAdd>,
) -> Result<DeliveryZonesSuccess, DeliveryZonesError> {
    let row: DeliveryZonesInsertResponse = sqlx::query_as::<_, DeliveryZonesInsertResponse>(
        "INSERT INTO delivery_zones (name, description) VALUES ($1, $2) RETURNING id",
    )
    .bind(json.name)
    .bind(json.description)
    .fetch_one(&state.db)
    .await
    .map_err(DeliveryZonesError::insert_error)?;

    Ok(DeliveryZonesSuccess::created(row.id))
}

/// Fetches all delivery zones from the database.
///
/// # Arguments
///
/// * `state` - The application state containing the database connection pool.
///
/// # Returns
///
/// * `Result<Json<Vec<DeliveryZones>>, DeliveryZonesError>` - The result of the operation, either a list of delivery zones or an error.
pub(crate) async fn fetch_delivery_zones(
    State(state): State<AppState>,
) -> Result<Json<Vec<DeliveryZones>>, DeliveryZonesError> {
    let res = sqlx::query_as::<_, DeliveryZones>("SELECT * FROM delivery_zones")
        .fetch_all(&state.db)
        .await
        .map_err(DeliveryZonesError::fetch_error)?;

    Ok(Json(res))
}

/// Deletes a delivery zone from the database.
///
/// # Arguments
///
/// * `_claims` - The JWT claims of the authenticated user.
/// * `state` - The application state containing the database connection pool.
/// * `id` - The ID of the delivery zone to delete.
///
/// # Returns
///
/// * `Result<DeliveryZonesSuccess, DeliveryZonesError>` - The result of the operation, either a success or an error.
pub(crate) async fn delete_delivery_zone(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<DeliveryZonesSuccess, DeliveryZonesError> {
    // Check if the record exists
    if !zone_exists(id, &state.db).await? {
        return Err(DeliveryZonesError::fetch_error(sqlx::Error::RowNotFound));
    }

    sqlx::query("DELETE FROM delivery_zones WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(DeliveryZonesError::delete_error)?;

    Ok(DeliveryZonesSuccess::deleted(id))
}
