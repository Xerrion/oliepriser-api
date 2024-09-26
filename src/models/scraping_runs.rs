use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub(crate) struct ScrapingRuns {
    pub(crate) start_time: chrono::NaiveDateTime,
    pub(crate) end_time: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow)]
pub(crate) struct ScrapingRunsInsertResponse {
    pub(crate) id: i32,
}
