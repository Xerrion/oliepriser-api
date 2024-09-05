use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub(crate) struct Providers {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) html_element: String,
    pub(crate) last_accessed: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub(crate) struct ProviderAdd {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) html_element: String,
}

// Ignore dead code warning
#[derive(sqlx::FromRow, Serialize)]
pub(crate) struct Prices {
    pub(crate) id: i32,
    pub(crate) provider_id: i32,
    pub(crate) price: f64,
    pub(crate) created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub(crate) struct ProviderPriceAdd {
    pub(crate) price: f64,
}

#[derive(sqlx::FromRow, Serialize)]
pub(crate) struct PriceDetails {
    pub(crate) price: f64,
    pub(crate) created_at: chrono::NaiveDateTime,
}
