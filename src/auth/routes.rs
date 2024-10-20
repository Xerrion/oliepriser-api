use argon2::password_hash::Error as ArgonError;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Utc;
use jsonwebtoken::{encode, Header};
use sqlx::query_as;

use crate::app_state::AppState;
use crate::auth::jwt::{AuthBody, AuthPayload, KEYS};
use crate::auth::jwt::{AuthError, Claims};
use crate::auth::security::{hash_password, verify_password};

#[derive(Debug, sqlx::FromRow)]
struct User {
    password_hash: String,
}

// Implement conversion from Argon2 error to your AuthError
impl From<ArgonError> for AuthError {
    /// Converts an Argon2 error into an AuthError.
    ///
    /// # Arguments
    ///
    /// * `_` - The Argon2 error.
    ///
    /// # Returns
    ///
    /// * `AuthError` - The authentication error.
    fn from(_: ArgonError) -> Self {
        AuthError::TokenCreation // Or another variant depending on your context
    }
}

/// Authorizes a user by verifying their credentials and generating a JWT token.
///
/// # Arguments
///
/// * `state` - The application state containing the database connection pool.
/// * `payload` - The JSON payload containing the client ID and client secret.
///
/// # Returns
///
/// * `Result<Json<AuthBody>, AuthError>` - The result of the operation, either a JSON response with the JWT token or an authentication error.
pub(crate) async fn authorize(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    // Fetch user from the database
    let user: User = match query_as::<_, User>(
        "SELECT client_id, password_hash FROM users WHERE client_id = $1",
    )
    .bind(&payload.client_id)
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => user,
        Err(_) => return Err(AuthError::WrongCredentials), // Return error if user not found
    };

    // Verify the hashed password
    if !verify_password(&user.password_hash, &payload.client_secret)
        .await
        .unwrap()
    {
        return Err(AuthError::WrongCredentials);
    }

    // Create the JWT token upon successful verification
    let exp = (Utc::now().naive_utc() + chrono::Duration::days(30))
        .and_utc()
        .timestamp() as usize;

    let claims = Claims {
        username: payload.client_id,
        exp,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}

/// Creates a new user in the database.
///
/// # Arguments
///
/// * `state` - The application state containing the database connection pool.
/// * `payload` - The JSON payload containing the client ID and client secret.
///
/// # Returns
///
/// * `Result<impl IntoResponse, impl IntoResponse>` - The result of the operation, either a success status code or an error response.
pub(crate) async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let password_hash = hash_password(payload.client_secret).await.unwrap();
    let check_user: Option<User> = query_as("SELECT client_id FROM users WHERE client_id = $1")
        .bind(&payload.client_id)
        .fetch_optional(&state.db)
        .await
        .unwrap();

    if !check_user.is_some() {
        if let Err(e) = sqlx::query("INSERT INTO users (client_id, password_hash) VALUES ($1, $2)")
            .bind(payload.client_id)
            .bind(password_hash)
            .execute(&state.db)
            .await
        {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Error while creating user: {e}"),
            ));
        }
    } else {
        return Err((StatusCode::BAD_REQUEST, String::from("User already exists")));
    }

    Ok(StatusCode::OK)
}
