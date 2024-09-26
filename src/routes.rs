use axum::routing::{delete, post};
use axum::{routing::get, Router};

use crate::app_state::AppState;
use crate::auth::routes::{authorize, create_user};
use crate::crud::delivery_zones::{
    create_delivery_zone, delete_delivery_zone, fetch_delivery_zones,
};
use crate::crud::prices::{
    create_price_for_provider, delete_price, fetch_prices, fetch_prices_by_provider,
};
use crate::crud::providers::{
    add_delivery_zones_to_provider, create_provider, delete_provider, fetch_provider,
    fetch_providers_ids, fetch_providers_with_zones, update_provider,
};
use crate::crud::scraping_runs::{create_scraping_run, get_last_scraping_run_by_time};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub(crate) fn router(state: AppState) -> Router {
    // Auth routes
    let auth_routes = Router::new()
        .route("/login", post(authorize))
        .route("/create", post(create_user));

    // Provider routes
    let provider_routes = Router::new()
        .route("/", get(fetch_providers_with_zones).post(create_provider))
        .route(
            "/:id",
            get(fetch_provider)
                .put(update_provider)
                .delete(delete_provider),
        )
        .route(
            "/:id/prices",
            get(fetch_prices_by_provider).post(create_price_for_provider),
        )
        .route("/:id/zones", post(add_delivery_zones_to_provider));

    // Price routes
    let price_routes = Router::new()
        .route("/", get(fetch_prices))
        .route("/:id", delete(delete_price));

    // Zone routes
    let zone_routes = Router::new()
        .route("/", get(fetch_delivery_zones).post(create_delivery_zone))
        .route("/:id", delete(delete_delivery_zone));

    let scrape_run_routes = Router::new()
        .route(
            "/",
            get(get_last_scraping_run_by_time).post(create_scraping_run),
        )
        .route("/providers", get(fetch_providers_ids));

    // Main router combining everything
    Router::new()
        .route("/", get(hello_world))
        .nest("/auth", auth_routes)
        .nest("/providers", provider_routes)
        .nest("/prices", price_routes)
        .nest("/zones", zone_routes)
        .nest("/scraping_runs", scrape_run_routes)
        .with_state(state)
}
