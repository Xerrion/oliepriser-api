use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Debug)]
pub(crate) struct DeliveryZones {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: String,
}

#[derive(sqlx::FromRow, Deserialize)]
pub(crate) struct DeliveryZonesAdd {
    pub(crate) name: String,
    pub(crate) description: String,
}

#[derive(Deserialize)]
pub(crate) struct DeliveryZoneProviderAdd {
    pub(crate) zone_id: i32,
}
