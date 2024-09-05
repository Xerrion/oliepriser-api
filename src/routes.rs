use axum::routing::delete;
use axum::{routing::get, Router};

use crate::app_state::AppState;
use crate::crud::prices::{
    create_price_for_provider, delete_price, fetch_prices, fetch_prices_by_provider,
};
use crate::crud::providers::{
    create_provider, delete_provider, fetch_provider, fetch_providers, update_provider,
};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub(crate) fn router(state: AppState) -> Router {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/providers", get(fetch_providers).post(create_provider))
        .route(
            "/providers/:id",
            get(fetch_provider)
                .put(update_provider)
                .delete(delete_provider),
        )
        .route(
            "/providers/:id/prices",
            get(fetch_prices_by_provider).post(create_price_for_provider),
        )
        .route("/prices", get(fetch_prices))
        .route("/prices/:id", delete(delete_price))
        .with_state(state);

    router
}
