use axum::extract::{Path, State};
use axum::Json;

use crate::app_state::AppState;
use crate::auth::jwt::Claims;
use crate::errors::{DeliveryZonesError, DeliveryZonesSuccess};
use crate::models::delivery_zones::{DeliveryZones, DeliveryZonesAdd};

pub(crate) async fn create_delivery_zone(
    _claims: Claims,
    State(state): State<AppState>,
    Json(json): Json<DeliveryZonesAdd>,
) -> Result<DeliveryZonesSuccess, DeliveryZonesError> {
    let row: (i32,) = sqlx::query_as::<_, (i32,)>(
        "INSERT INTO delivery_zones (name, description) VALUES ($1, $2) RETURNING id",
    )
    .bind(json.name)
    .bind(json.description)
    .fetch_one(&state.db)
    .await
    .map_err(DeliveryZonesError::insert_error)?;

    Ok(DeliveryZonesSuccess::created(row.0))
}

pub(crate) async fn fetch_delivery_zones(
    State(state): State<AppState>,
) -> Result<Json<Vec<DeliveryZones>>, DeliveryZonesError> {
    let res: Vec<DeliveryZones> =
        match sqlx::query_as::<_, DeliveryZones>("SELECT * FROM delivery_zones")
            .fetch_all(&state.db)
            .await
        {
            Ok(res) => res,
            Err(e) => {
                return Err(DeliveryZonesError::fetch_error(e));
            }
        };

    Ok(Json(res))
}

pub(crate) async fn delete_delivery_zone(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<DeliveryZonesSuccess, DeliveryZonesError> {
    // Check to see if the record exists
    let check_record: Option<DeliveryZones> =
        sqlx::query_as("SELECT * FROM delivery_zones WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db)
            .await
            .map_err(DeliveryZonesError::fetch_error)?;

    if !check_record.is_some() {
        return Err(DeliveryZonesError::fetch_error(sqlx::Error::RowNotFound));
    }

    if let Err(e) = sqlx::query("DELETE FROM delivery_zones WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
    {
        return Err(DeliveryZonesError::delete_error(e));
    }

    Ok(DeliveryZonesSuccess::deleted(id))
}
