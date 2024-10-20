use crate::errors::{DeliveryZonesError, ProvidersError};

/// Checks if a provider exists in the database.
///
/// # Arguments
///
/// * `id` - The ID of the provider to check.
/// * `db` - The database connection pool.
///
/// # Returns
///
/// * `Result<bool, ProvidersError>` - `Ok(true)` if the provider exists, `Ok(false)` if not, or an error if the query fails.
pub(crate) async fn provider_exists(id: i32, db: &sqlx::PgPool) -> Result<bool, ProvidersError> {
    let check_record: Option<(i32,)> =
        sqlx::query_as::<_, (i32,)>("SELECT id FROM providers WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(ProvidersError::fetch_error)?;

    Ok(check_record.is_some())
}

/// Checks if a delivery zone exists in the database.
///
/// # Arguments
///
/// * `id` - The ID of the delivery zone to check.
/// * `db` - The database connection pool.
///
/// # Returns
///
/// * `Result<bool, DeliveryZonesError>` - `Ok(true)` if the delivery zone exists, `Ok(false)` if not, or an error if the query fails.
pub(crate) async fn zone_exists(id: i32, db: &sqlx::PgPool) -> Result<bool, DeliveryZonesError> {
    let check_record: Option<(i32,)> =
        sqlx::query_as::<_, (i32,)>("SELECT id FROM delivery_zones WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(DeliveryZonesError::fetch_error)?;

    Ok(check_record.is_some())
}
