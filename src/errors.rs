use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use sqlx::Error as SqlxError;

// Define the main application error enum
pub(crate) enum AppError {
    InsertError {
        resource: &'static str,
        error: SqlxError,
    },
    FetchError {
        resource: &'static str,
        error: SqlxError,
    },
    UpdateError {
        resource: &'static str,
        error: SqlxError,
    },
    DeleteError {
        resource: &'static str,
        error: SqlxError,
    },
    NotFound {
        resource: &'static str,
    },
}

// Implement IntoResponse for AppError to handle HTTP responses
impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let (status, body) = match self {
            AppError::InsertError { resource, error } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Error while inserting {}: {}", resource, error),
                })),
            ),
            AppError::FetchError { resource, error } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Error while fetching {}: {}", resource, error),
                })),
            ),
            AppError::UpdateError { resource, error } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Error while updating {}: {}", resource, error),
                })),
            ),
            AppError::DeleteError { resource, error } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Error while deleting {}: {}", resource, error),
                })),
            ),
            AppError::NotFound { resource } => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": format!("{} not found", resource),
                })),
            ),
        };
        (status, body).into_response()
    }
}

// Macro to implement specific error types
#[macro_export]
macro_rules! impl_error {
    ($name:ident, $resource:expr) => {
        pub(crate) enum $name {
            Inner(AppError),
        }

        impl From<AppError> for $name {
            fn from(error: AppError) -> Self {
                $name::Inner(error)
            }
        }

        impl IntoResponse for $name {
            fn into_response(self) -> Response<Body> {
                match self {
                    $name::Inner(err) => err.into_response(),
                }
            }
        }

        impl $name {
            pub fn insert_error(error: SqlxError) -> Self {
                AppError::InsertError {
                    resource: $resource,
                    error,
                }
                .into()
            }

            pub fn fetch_error(error: SqlxError) -> Self {
                AppError::FetchError {
                    resource: $resource,
                    error,
                }
                .into()
            }

            pub fn update_error(error: SqlxError) -> Self {
                AppError::UpdateError {
                    resource: $resource,
                    error,
                }
                .into()
            }

            pub fn delete_error(error: SqlxError) -> Self {
                AppError::DeleteError {
                    resource: $resource,
                    error,
                }
                .into()
            }

            pub fn not_found() -> Self {
                AppError::NotFound {
                    resource: $resource,
                }
                .into()
            }
        }
    };
}

// Define the main success enum
pub(crate) enum AppSuccess {
    Created { resource: &'static str, id: i32 },
    Updated { resource: &'static str, id: i32 },
    Deleted { resource: &'static str, id: i32 },
}

// Implement IntoResponse for AppSuccess
impl IntoResponse for AppSuccess {
    fn into_response(self) -> Response<Body> {
        let (status, body) = match self {
            AppSuccess::Created { resource, id } => (
                StatusCode::CREATED,
                Json(json!({
                    "message": format!("Created {} with id: {}", resource, id),
                })),
            ),
            AppSuccess::Updated { resource, id } => (
                StatusCode::OK,
                Json(json!({
                    "message": format!("Updated {} with id: {}", resource, id),
                })),
            ),
            AppSuccess::Deleted { resource, id } => (
                StatusCode::OK,
                Json(json!({
                    "message": format!("Deleted {} with id: {}", resource, id),
                })),
            ),
        };
        (status, body).into_response()
    }
}

// Macro to implement specific success types
#[macro_export]
macro_rules! impl_success {
    ($name:ident, $resource:expr) => {
        pub(crate) enum $name {
            Inner(AppSuccess),
        }

        impl From<AppSuccess> for $name {
            fn from(success: AppSuccess) -> Self {
                $name::Inner(success)
            }
        }

        impl IntoResponse for $name {
            fn into_response(self) -> Response<Body> {
                match self {
                    $name::Inner(success) => success.into_response(),
                }
            }
        }

        impl $name {
            pub fn created(id: i32) -> Self {
                AppSuccess::Created {
                    resource: $resource,
                    id: id,
                }
                .into()
            }
            pub fn deleted(id: i32) -> Self {
                AppSuccess::Deleted {
                    resource: $resource,
                    id: id,
                }
                .into()
            }
            pub fn updated(id: i32) -> Self {
                AppSuccess::Updated {
                    resource: $resource,
                    id: id,
                }
                .into()
            }
        }
    };
}

// Implement specific success enums using the macro
impl_success!(ProvidersSuccess, "provider");
impl_success!(DeliveryZonesSuccess, "delivery zone");
impl_success!(PricesSuccess, "price");
impl_success!(ScrapingRunsSuccess, "scraping run");

// Implement specific error enums using the macro
impl_error!(ProvidersError, "provider");
impl_error!(DeliveryZonesError, "delivery zone");
impl_error!(PricesError, "price");
impl_error!(ScrapingRunsError, "scraping run");

impl From<DeliveryZonesError> for ProvidersError {
    fn from(_err: DeliveryZonesError) -> Self {
        ProvidersError::fetch_error(sqlx::Error::RowNotFound)
    }
}
