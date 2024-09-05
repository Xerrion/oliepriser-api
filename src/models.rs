use serde::{Deserialize, Serialize};

#[warn(dead_code)]
#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub(crate) struct Providers {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) html_element: String,
}

#[warn(dead_code)]
#[derive(Deserialize)]
pub(crate) struct ProviderAdd {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) html_element: String,
}

#[warn(dead_code)]
#[derive(Deserialize)]
pub(crate) struct ProviderUpdate {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) html_element: String,
}

#[warn(dead_code)]
#[derive(sqlx::FromRow)]
pub(crate) struct Prices {
    pub(crate) id: i32,
    pub(crate) provider_id: i32,
    pub(crate) price: f64,
    pub(crate) created_at: chrono::NaiveDateTime,
}

#[warn(dead_code)]
#[derive(Deserialize)]
pub(crate) struct PriceAdd {
    pub(crate) provider_id: i32,
    pub(crate) price: f64,
}

#[warn(dead_code)]
#[derive(sqlx::FromRow, Serialize)]
pub(crate) struct OilPriceWithProvider {
    pub(crate) oil_price_id: i32,
    pub(crate) price: f64,
    pub(crate) provider_name: String,
    pub(crate) url: String,
}
