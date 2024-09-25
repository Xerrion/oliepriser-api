use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub(crate) struct Prices {
    pub(crate) id: i32,
    pub(crate) provider_id: i32,
    pub(crate) price: f64,
    pub(crate) created_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow, Deserialize)]
pub(crate) struct ProviderPriceAdd {
    pub(crate) price: f64,
    pub id: i32,
}

#[derive(sqlx::FromRow, Serialize)]
pub(crate) struct PriceDetails {
    pub(crate) price: f64,
    pub(crate) created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub(crate) struct PriceQueryParams {
    pub(crate) limit: Option<i64>,
    pub(crate) offset: Option<i64>,
    pub(crate) start: Option<chrono::NaiveDateTime>,
    pub(crate) end: Option<chrono::NaiveDateTime>,
}
