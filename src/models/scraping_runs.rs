use serde::Deserialize;

#[derive(sqlx::FromRow, Deserialize)]
pub(crate) struct ScrapingRuns {
    pub(crate) start_time: chrono::NaiveDateTime,
    pub(crate) end_time: chrono::NaiveDateTime,
}
