use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{async_trait, Json};
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Struct to hold encoding and decoding keys for JWT.
pub(crate) struct Keys {
    pub(crate) encoding: EncodingKey,
    pub(crate) decoding: DecodingKey,
}

impl Keys {
    /// Creates a new `Keys` instance from a secret.
    ///
    /// # Arguments
    ///
    /// * `secret` - A byte slice containing the secret key.
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// Static instance of `Keys` initialized with a random secret.
pub(crate) static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = Alphanumeric.sample_string(&mut rand::thread_rng(), 60);
    Keys::new(secret.as_bytes())
});

/// Enum representing different types of authentication errors.
pub enum AuthError {
    InvalidToken,
    WrongCredentials,
    TokenCreation,
    MissingCredentials,
}

impl IntoResponse for AuthError {
    /// Converts `AuthError` into an HTTP response.
    ///
    /// # Returns
    ///
    /// * `Response` - The HTTP response containing the error message.
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

/// Struct representing JWT claims.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) username: String,
    pub(crate) exp: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    /// Extracts `Claims` from the request parts.
    ///
    /// # Arguments
    ///
    /// * `parts` - The request parts.
    /// * `_state` - The state.
    ///
    /// # Returns
    ///
    /// * `Result<Claims, AuthError>` - The extracted claims or an authentication error.
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let mut validation = Validation::default();
        validation.validate_exp = true; // Ensure expiration is validated

        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &validation)
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

/// Struct representing the authentication response body.
#[derive(Debug, Serialize)]
pub(crate) struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    /// Creates a new `AuthBody` instance.
    ///
    /// # Arguments
    ///
    /// * `access_token` - The access token.
    ///
    /// # Returns
    ///
    /// * `AuthBody` - The new authentication response body.
    pub(crate) fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

/// Struct representing the authentication payload.
#[derive(Debug, Deserialize)]
pub(crate) struct AuthPayload {
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
}
