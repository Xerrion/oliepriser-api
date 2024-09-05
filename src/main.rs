use crate::app_state::AppState;
use routes::router;
use sqlx::Executor;
use sqlx::PgPool;

mod app_state;
mod crud;
mod models;
mod routes;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] db: PgPool) -> shuttle_axum::ShuttleAxum {
    db.execute(include_str!("../migrations.sql")).await.unwrap();

    let state = AppState { db };

    Ok(router(state).into())
}
