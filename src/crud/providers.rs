use crate::app_state::AppState;
use crate::auth::jwt::Claims;
use crate::errors::{ProvidersError, ProvidersSuccess};
use crate::helpers::{provider_exists, zone_exists};
use crate::models::delivery_zones::{DeliveryZoneProviderAdd, DeliveryZones};
use crate::models::providers::{
    ProviderAdd, ProviderIds, ProviderWithZones, ProviderZoneRow, Providers,
    ProvidersInsertResponse,
};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use std::collections::HashMap;

pub(crate) async fn create_provider(
    _claims: Claims,
    State(state): State<AppState>,
    Json(json): Json<ProviderAdd>,
) -> Result<ProvidersSuccess, ProvidersError> {
    let row: ProvidersInsertResponse = sqlx::query_as::<_, ProvidersInsertResponse>(
        "INSERT INTO providers (name, url, html_element) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(json.name)
    .bind(json.url)
    .bind(json.html_element)
    .fetch_one(&state.db)
    .await
    .map_err(ProvidersError::insert_error)?;

    Ok(ProvidersSuccess::created(row.id))
}

pub(crate) async fn add_delivery_zones_to_provider(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(json): Json<DeliveryZoneProviderAdd>,
) -> Result<ProvidersSuccess, ProvidersError> {
    if !provider_exists(id, &state.db).await? {
        return Err(ProvidersError::fetch_error(sqlx::Error::RowNotFound));
    }

    for zone_id in &json.zone_ids {
        if !zone_exists(*zone_id, &state.db).await? {
            return Err(ProvidersError::fetch_error(sqlx::Error::RowNotFound));
        }

        sqlx::query("INSERT INTO delivery_zone (provider_id, zone_id) VALUES ($1, $2)")
            .bind(id)
            .bind(zone_id)
            .execute(&state.db)
            .await
            .map_err(ProvidersError::insert_error)?;
    }

    Ok(ProvidersSuccess::updated(id))
}

pub(crate) async fn fetch_providers_ids(
    _claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<Vec<ProviderIds>>, ProvidersError> {
    let res = sqlx::query_as::<_, ProviderIds>("SELECT id FROM providers")
        .fetch_all(&state.db)
        .await
        .map_err(ProvidersError::fetch_error)?;

    Ok(Json(res))
}

pub(crate) async fn fetch_provider(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Providers>, ProvidersError> {
    let res = sqlx::query_as::<_, Providers>("SELECT * FROM providers WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(ProvidersError::fetch_error)?;

    update_last_accessed(State(state), Path(id)).await?;
    Ok(Json(res))
}

pub(crate) async fn update_provider(
    _claims: Claims,
    State(state): State<AppState>,
    Json(json): Json<Providers>,
) -> Result<StatusCode, ProvidersError> {
    sqlx::query("UPDATE providers SET name = $1, url = $2, html_element = $3 WHERE id = $4")
        .bind(json.name)
        .bind(json.url)
        .bind(json.html_element)
        .bind(json.id)
        .execute(&state.db)
        .await
        .map_err(ProvidersError::update_error)?;

    Ok(StatusCode::OK)
}

pub(crate) async fn fetch_providers_with_zones(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProviderWithZones>>, ProvidersError> {
    let rows = sqlx::query_as::<_, ProviderZoneRow>(
        r#"
        SELECT
            p.id as provider_id, p.name as provider_name, p.url, p.html_element,
            p.created_at, p.last_updated, p.last_accessed,
            z.id as zone_id, z.name as zone_name, z.description
        FROM
            providers p
        LEFT JOIN
            provider_delivery_zones pz ON p.id = pz.provider_id
        LEFT JOIN
            delivery_zones z ON pz.zone_id = z.id
        ORDER BY
            p.id
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(ProvidersError::fetch_error)?;

    let mut providers_map: HashMap<i32, ProviderWithZones> = HashMap::new();

    for row in rows {
        let provider_entry =
            providers_map
                .entry(row.provider_id)
                .or_insert_with(|| ProviderWithZones {
                    id: row.provider_id,
                    name: row.provider_name,
                    url: row.url,
                    created_at: row.created_at,
                    last_updated: row.last_updated,
                    zones: vec![],
                });

        if let Some(zone_id) = row.zone_id {
            provider_entry.zones.push(DeliveryZones {
                id: zone_id,
                name: row.zone_name.unwrap_or_default(),
                description: row.description.unwrap(),
            });
        }
    }

    Ok(Json(providers_map.into_values().collect()))
}

async fn update_last_accessed(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ProvidersSuccess, ProvidersError> {
    sqlx::query("UPDATE providers SET last_accessed = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(ProvidersError::update_error)?;

    Ok(ProvidersSuccess::updated(id))
}

pub(crate) async fn delete_provider(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ProvidersSuccess, ProvidersError> {
    sqlx::query("DELETE FROM providers WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(ProvidersError::delete_error)?;

    Ok(ProvidersSuccess::deleted(id))
}
