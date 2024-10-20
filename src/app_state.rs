use sqlx::PgPool;

/// Application state.
///
/// This struct is used to store the database connection pool.
///
/// # Fields
///
/// * `db` - The database connection pool.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}
