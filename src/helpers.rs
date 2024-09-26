use crate::errors::ProvidersError;

pub(crate) async fn provider_exists(id: i32, db: &sqlx::PgPool) -> Result<bool, ProvidersError> {
    let check_record: Option<(i32,)> =
        sqlx::query_as::<_, (i32,)>("SELECT id FROM providers WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(ProvidersError::fetch_error)?;

    Ok(check_record.is_some())
}

pub(crate) async fn zone_exists(id: i32, db: &sqlx::PgPool) -> Result<bool, ProvidersError> {
    let check_record: Option<(i32,)> =
        sqlx::query_as::<_, (i32,)>("SELECT id FROM delivery_zones WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(ProvidersError::fetch_error)?;

    Ok(check_record.is_some())
}

fn main() {}
