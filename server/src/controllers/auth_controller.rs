use axum::routing::post;
use axum::{Json, Router};
use bcrypt::verify;
use http::header::AUTHORIZATION;
use http::{HeaderMap, HeaderValue, StatusCode};

use crate::strategies::auth_strategy::{AuthError, AuthRequestClaims, JWTClaims};
use crate::strategies::user_strategy;
use crate::types::auth::{AuthErrorType, AuthToken};
use crate::types::user::{UserInformation, UserLogin, UserRegister};

pub fn routes() -> Router {
    Router::new().route("/register", post(register)).route("/login", post(login))
}

// User register route
async fn register(
    Json(payload): Json<UserRegister>,
) -> Result<(StatusCode, HeaderMap, Json<UserInformation>), AuthError> {
    // Attempt to insert user into database
    let db_result = user_strategy::insert_db_user(payload).await;
    if let Err(error) = db_result {
        if error.to_string().contains("duplicate key") {
            return Err(AuthError::from_type(AuthErrorType::UserExists));
        }
        return Err(AuthError::from_type(AuthErrorType::ServerError));
    }

    // Create user information from user result
    let user = db_result.unwrap();
    let user_info = UserInformation::from_user(user);

    // Generate authentication token from UUID
    let auth_token: AuthToken;
    let request_result =
        AuthRequestClaims::new(user_info.uuid.clone()).await.unwrap().generate_token();
    match request_result {
        Ok(token) => auth_token = token,
        Err(error) => {
            println!("Error generating token from UUID {}: {:?}", user_info.uuid, error);
            return Err(AuthError::from_type(AuthErrorType::TokenGeneration));
        }
    }

    // Insert token into header map
    let mut header_map = HeaderMap::new();
    header_map.insert(AUTHORIZATION, HeaderValue::from_str(&auth_token.to_string()).unwrap());

    // Return success response
    Ok((StatusCode::CREATED, header_map, Json(user_info)))
}

// User login route
async fn login(
    Json(payload): Json<UserLogin>,
) -> Result<(StatusCode, HeaderMap, Json<UserInformation>), AuthError> {
    // Attempt to get user from database
    let db_result = user_strategy::get_db_user_by_identifier(payload.username).await;
    if let Err(_) = db_result {
        return Err(AuthError::from_type(AuthErrorType::UserNotExists));
    }

    // Verify user by password
    let user = db_result.unwrap();
    if verify(payload.password, &user.password).unwrap() {
        // Generate authentication token from UUID
        let user_info = UserInformation::from_user(user);
        let auth_token: AuthToken;
        let request_result =
            AuthRequestClaims::new(user_info.uuid.clone()).await.unwrap().generate_token();
        match request_result {
            Ok(token) => auth_token = token,
            Err(error) => {
                println!("Error generating token from UUID {}: {:?}", user_info.uuid, error);
                return Err(AuthError::from_type(AuthErrorType::TokenGeneration));
            }
        }

        // Insert token into header map
        let mut header_map = HeaderMap::new();
        header_map.insert(AUTHORIZATION, HeaderValue::from_str(&auth_token.to_string()).unwrap());

        // Return success response
        Ok((StatusCode::OK, header_map, Json(user_info)))
    } else {
        Err(AuthError::from_type(AuthErrorType::WrongCredentials))
    }
}
