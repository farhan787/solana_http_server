use crate::response::ApiResponse;
use crate::utils::crypto;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    pub message: Option<String>,
    pub signature: Option<String>,
    pub pubkey: Option<String>,
}

#[derive(Serialize)]
pub struct VerifyMessageResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

pub async fn verify_message(
    Json(payload): Json<VerifyMessageRequest>,
) -> (StatusCode, Json<ApiResponse<VerifyMessageResponse>>) {
    let message = match payload.message {
        Some(m) if !m.is_empty() => m,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Missing required fields: message")),
            )
        }
    };

    let signature = match payload.signature {
        Some(sig) if !sig.is_empty() => sig,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Missing required fields: signature")),
            )
        }
    };

    let pubkey = match payload.pubkey {
        Some(pk) if !pk.is_empty() => pk,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Missing required fields: pubkey")),
            )
        }
    };

    match crypto::verify_message(&message, &signature, &pubkey) {
        Ok(valid) => {
            let response = VerifyMessageResponse {
                valid,
                message,
                pubkey,
            };
            (StatusCode::OK, Json(ApiResponse::success(response)))
        }
        Err(err) => (StatusCode::BAD_REQUEST, Json(ApiResponse::error(err))),
    }
}
