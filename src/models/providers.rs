use crate::models::delivery_zones::DeliveryZones;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub(crate) struct Providers {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) html_element: String,
    pub(crate) last_accessed: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub(crate) struct ProviderAdd {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) html_element: String,
}

#[derive(Serialize)]
pub(crate) struct ProviderWithZones {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) html_element: String,
    pub(crate) created_at: chrono::NaiveDateTime,
    pub(crate) last_updated: chrono::NaiveDateTime,
    pub(crate) last_accessed: chrono::NaiveDateTime,
    pub(crate) zones: Vec<DeliveryZones>,
}

#[derive(sqlx::FromRow)]
pub(crate) struct ProviderZoneRow {
    pub(crate) provider_id: i32,
    pub(crate) provider_name: String,
    pub(crate) url: String,
    pub(crate) html_element: String,
    pub(crate) created_at: chrono::NaiveDateTime,
    pub(crate) last_updated: chrono::NaiveDateTime,
    pub(crate) last_accessed: chrono::NaiveDateTime,
    pub(crate) zone_id: Option<i32>,
    pub(crate) zone_name: Option<String>,
    pub(crate) description: Option<String>,
}
