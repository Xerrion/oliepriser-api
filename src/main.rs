use crate::app_state::AppState;
use routes::router;
use sqlx::Executor;
use sqlx::PgPool;

mod app_state;
mod auth;
mod crud;
mod errors;
mod helpers;
mod models;
mod routes;

///
/// Main function
///
/// This function is the entry point of the application
///
/// # Arguments
///
/// * `db` - The database connection pool
///
/// # Returns
///
/// The application instance
///
#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] db: PgPool) -> shuttle_axum::ShuttleAxum {
    db.execute(include_str!("../migrations.sql")).await.unwrap();

    let state = AppState { db };

    Ok(router(state).into())
}
