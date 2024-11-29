use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AuthToken {
    pub access_token: String,
    pub token_type: String,
}

impl AuthToken {
    pub fn new(access_token: String) -> Self {
        Self { access_token, token_type: "Bearer".to_string() }
    }

    pub fn from_string(string: String) -> Self {
        Self { access_token: string, token_type: "Bearer".to_string() }
    }

    pub fn to_string(self: AuthToken) -> String {
        self.access_token
    }

    pub fn default() -> Self {
        Self { access_token: String::new(), token_type: "Bearer".to_string() }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AuthErrorType {
    InvalidToken,
    ServerError,
    TokenGeneration,
    UserExists,
    UserNotExists,
    WrongCredentials,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthErrorBody {
    pub error_type: AuthErrorType,
    pub error_message: String,
}

#[derive(Clone, Debug)]
pub struct AuthError {
    pub status: StatusCode,
    pub body: AuthErrorBody,
}

impl AuthError {
    pub fn default() -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            body: AuthErrorBody {
                error_type: AuthErrorType::ServerError,
                error_message: "Uncategorized authentication error".to_string(),
            },
        }
    }

    pub fn from_type(error_type: AuthErrorType) -> Self {
        let (status, error_message) = match error_type {
            AuthErrorType::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
            }
            AuthErrorType::ServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Server error".to_string())
            }
            AuthErrorType::TokenGeneration => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Error generating token".to_string())
            }
            AuthErrorType::UserExists => (StatusCode::CONFLICT, "User already exists".to_string()),
            AuthErrorType::UserNotExists => {
                (StatusCode::NOT_FOUND, "User does not exist".to_string())
            }
            AuthErrorType::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, "Incorrect password".to_string())
            }
        };

        Self { status, body: AuthErrorBody { error_type, error_message } }
    }

    pub fn status(&self) -> StatusCode {
        self.status.to_owned()
    }

    pub fn body(&self) -> AuthErrorBody {
        self.body.to_owned()
    }
}
