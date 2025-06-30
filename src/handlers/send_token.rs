use axum::{http::StatusCode, Json};
use base64::{engine::general_purpose, Engine as _};
use bs58;
use serde::{Deserialize, Serialize};

use crate::response::ApiResponse;

#[derive(Deserialize)]
pub struct SendTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct TokenTransferAccount {
    pub pubkey: String,
    pub isSigner: bool,
}

#[derive(Serialize)]
pub struct SendTokenResponse {
    pub program_id: String,
    pub accounts: Vec<TokenTransferAccount>,
    pub instruction_data: String,
}

pub async fn send_token(
    Json(payload): Json<SendTokenRequest>,
) -> (StatusCode, Json<ApiResponse<SendTokenResponse>>) {
    // Basic validation
    for field in [&payload.destination, &payload.mint, &payload.owner] {
        if bs58::decode(field).into_vec().is_err() {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Invalid base58 public key")),
            );
        }
    }

    if payload.amount == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Amount must be greater than 0")),
        );
    }

    let mock_instruction = format!(
        "transfer {} tokens from {} to {} (mint: {})",
        payload.amount, payload.owner, payload.destination, payload.mint
    );

    let response = SendTokenResponse {
        program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
        accounts: vec![
            TokenTransferAccount {
                pubkey: payload.owner.clone(),
                isSigner: true,
            },
            TokenTransferAccount {
                pubkey: payload.destination.clone(),
                isSigner: false,
            },
        ],
        instruction_data: general_purpose::STANDARD.encode(mock_instruction),
    };

    (StatusCode::OK, Json(ApiResponse::success(response)))
}
