use axum::{routing::get, Router};
use sqlx::Executor;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] db: PgPool) -> shuttle_axum::ShuttleAxum {
    db.execute(include_str!("../migrations.sql")).await.unwrap();
}
