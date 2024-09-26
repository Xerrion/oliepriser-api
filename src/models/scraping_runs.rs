use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub(crate) struct ScrapingRuns {
    pub(crate) start_time: chrono::NaiveDateTime,
    pub(crate) end_time: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug)]
pub(crate) struct ScrapingRunsInsertResponse {
    pub(crate) id: i32,
}
