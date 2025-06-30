use crate::response::ApiResponse;
use crate::utils::crypto;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignMessageRequest {
    pub message: Option<String>,
    pub secret: Option<String>,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

pub async fn sign_message(
    Json(payload): Json<SignMessageRequest>,
) -> (StatusCode, Json<ApiResponse<SignMessageResponse>>) {
    let message = match payload.message {
        Some(m) if !m.is_empty() => m,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Missing 'message' field")),
            );
        }
    };

    let secret = match payload.secret {
        Some(s) if !s.is_empty() => s,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Missing 'secret' field")),
            );
        }
    };

    match crypto::sign_message(&secret, &message) {
        Ok((signature, public_key)) => {
            let response = SignMessageResponse {
                signature,
                public_key,
                message,
            };
            (StatusCode::OK, Json(ApiResponse::success(response)))
        }
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(format!("Signing failed: {}", err))),
        ),
    }
}
