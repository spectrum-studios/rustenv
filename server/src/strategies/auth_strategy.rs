use std::env;

use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::response::{IntoResponse, Response};
use axum::{Json, RequestPartsExt, async_trait};
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use base64::prelude::*;
use http::request::Parts;
use http::{HeaderMap, StatusCode};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, get_current_timestamp,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use struct_iterable::Iterable;

use super::user_strategy::get_db_user_by_uuid;
use crate::types::auth::{AuthErrorBody, AuthErrorType, AuthToken};

// Keys for encode and decode authentication tokens
static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET").expect("Missing JWT_SECRET environment variable");
    Keys::new(secret.as_bytes())
});

// Authentication token lifetime
static TOKEN_LIFETIME: Lazy<u64> = Lazy::new(|| {
    let expiry =
        env::var("AUTH_TOKEN_EXPIRY").expect("Missing AUTH_TOKEN_EXPIRY environment variable");
    u64::from_str_radix(&expiry, 10).expect("Cannot pase AUTH_TOKEN_EXPIRY as u64")
});

// Authentication request token lifetime
static REQUEST_TOKEN_LIFETIME: Lazy<u64> = Lazy::new(|| {
    let expiry = env::var("REQUEST_TOKEN_EXPIRY")
        .expect("Missing REQUEST_TOKEN_EXPIRY environment variable");
    u64::from_str_radix(&expiry, 10).expect("Cannot pase REQUEST_TOKEN_EXPIRY as u64")
});

// Encode/decode keys
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

// Define trait for JWT claims
pub trait JWTClaims {
    // Create new claims from UUID
    async fn new(uuid: String) -> Result<Self, AuthError>
    where
        Self: Sized;

    // Create claims from X-Claims header
    fn from_header(header: &HeaderMap) -> Self
    where
        Self: for<'de> Deserialize<'de>,
    {
        let claims = header.get("X-Claims").unwrap();
        serde_json::from_str(&String::from_utf8(BASE64_STANDARD.decode(claims).unwrap()).unwrap())
            .unwrap()
    }

    // Create claims from encoded string
    fn from_string(encoded_str: &str) -> Result<Self, AuthError>
    where
        Self: Sized,
        Self: for<'de> Deserialize<'de>,
    {
        // Get environment variables
        let aud = env::var("JWT_AUDIENCE").expect("Missing JWT_AUDIENCE environment variable");
        let iss = env::var("JWT_ISSUER").expect("Missing JWT_ISSUER environment variable");

        // Build validation
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 5;
        validation.set_audience(&[aud]);
        validation.set_issuer(&[iss]);

        // Decode token
        match decode::<Self>(encoded_str, &KEYS.decoding, &validation) {
            Ok(token_data) => Ok(token_data.claims),
            Err(_) => Err(AuthError::from_type(AuthErrorType::InvalidToken)),
        }
    }

    // Create default claims
    fn default() -> Self;

    // Generate token from claims
    fn generate_token(&self) -> Result<AuthToken, AuthError>
    where
        Self: Serialize,
    {
        match encode(&Header::default(), &self, &KEYS.encoding) {
            Ok(encoded_string) => Ok(AuthToken::new(encoded_string)),
            Err(error) => {
                println!("Error generating token: {:?}", error);
                Err(AuthError::from_type(AuthErrorType::TokenGeneration))
            }
        }
    }
}

// Build claims from authorization header
async fn from_request_parts<T>(parts: &mut Parts) -> Result<T, AuthError>
where
    T: for<'de> Deserialize<'de>,
{
    // Get environment variables
    let aud = env::var("JWT_AUDIENCE").expect("Missing JWT_AUDIENCE environment variable");
    let iss = env::var("JWT_ISSUER").expect("Missing JWT_ISSUER environment variable");

    // Extract authorization header
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| AuthError::from_type(AuthErrorType::InvalidToken))?;

    // Build validation
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 5;
    validation.set_audience(&[aud]);
    validation.set_issuer(&[iss]);

    // Decode token
    let token_data = decode::<T>(bearer.token(), &KEYS.decoding, &validation)
        .map_err(|_| AuthError::from_type(AuthErrorType::InvalidToken))?;
    Ok(token_data.claims)
}

// Authentication claims
#[derive(Debug, Deserialize, Iterable, Serialize)]
pub struct AuthClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: u64,
    pub role: Vec<String>,
    pub iat: usize,
}

impl JWTClaims for AuthClaims {
    // Create new claims from UUID
    async fn new(uuid: String) -> Result<Self, AuthError> {
        // Get environment variables
        let aud = env::var("JWT_AUDIENCE").expect("Missing JWT_AUDIENCE environment variable");
        let iss = env::var("JWT_ISSUER").expect("Missing JWT_ISSUER environment variable");

        // Build claims from database user
        match get_db_user_by_uuid(uuid).await {
            Ok(user) => Ok(Self {
                iss,
                sub: user.uuid,
                aud,
                exp: get_current_timestamp() + *TOKEN_LIFETIME,
                role: if user.is_admin {
                    vec!["admin".to_string(), "user".to_string()]
                } else {
                    vec!["user".to_string()]
                },
                iat: get_current_timestamp() as usize,
            }),
            Err(_) => Err(AuthError::from_type(AuthErrorType::TokenGeneration)),
        }
    }

    // Create default claims
    fn default() -> Self {
        // Get environment variables
        let aud = env::var("JWT_AUDIENCE").expect("Missing JWT_AUDIENCE environment variable");
        let iss = env::var("JWT_ISSUER").expect("Missing JWT_ISSUER environment variable");

        // Build claims
        Self {
            iss,
            sub: String::new(),
            aud,
            exp: get_current_timestamp() + *TOKEN_LIFETIME,
            role: vec!["user".to_string()],
            iat: get_current_timestamp() as usize,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthClaims
where
    S: Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        from_request_parts::<AuthClaims>(parts).await
    }
}

// Authentication request claims
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthRequestClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: u64,
    pub iat: usize,
}

impl JWTClaims for AuthRequestClaims {
    // Create new claims from UUID
    async fn new(uuid: String) -> Result<Self, AuthError> {
        // Get environment variables
        let aud = env::var("JWT_AUDIENCE").expect("Missing JWT_AUDIENCE environment variable");
        let iss = env::var("JWT_ISSUER").expect("Missing JWT_ISSUER environment variable");

        // Build claims
        Ok(Self {
            iss,
            sub: uuid,
            aud,
            exp: get_current_timestamp() + *TOKEN_LIFETIME,
            iat: get_current_timestamp() as usize,
        })
    }

    // Create default claims
    fn default() -> Self {
        // Get environment variables
        let aud = env::var("JWT_AUDIENCE").expect("Missing JWT_AUDIENCE environment variable");
        let iss = env::var("JWT_ISSUER").expect("Missing JWT_ISSUER environment variable");

        // Build claims
        Self {
            iss,
            sub: String::new(),
            aud,
            exp: get_current_timestamp() + *TOKEN_LIFETIME,
            iat: get_current_timestamp() as usize,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthRequestClaims
where
    S: Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        from_request_parts::<AuthRequestClaims>(parts).await
    }
}

#[derive(Debug)]
pub struct AuthError(crate::types::auth::AuthError);

impl AuthError {
    pub fn from_type(error_type: AuthErrorType) -> Self {
        Self { 0: crate::types::auth::AuthError::from_type(error_type) }
    }

    pub fn status(&self) -> StatusCode {
        self.0.status.to_owned()
    }

    pub fn body(&self) -> AuthErrorBody {
        self.0.body.to_owned()
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        (self.status(), Json(json!(self.body()))).into_response()
    }
}
