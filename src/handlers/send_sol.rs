use axum::{http::StatusCode, Json};
use base64::{engine::general_purpose, Engine as _};
use bs58;
use serde::{Deserialize, Serialize};

use crate::response::ApiResponse;

#[derive(Deserialize)]
pub struct SendSolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Serialize)]
pub struct SendSolResponse {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

pub async fn send_sol(
    Json(payload): Json<SendSolRequest>,
) -> (StatusCode, Json<ApiResponse<SendSolResponse>>) {
    if bs58::decode(&payload.from).into_vec().is_err()
        || bs58::decode(&payload.to).into_vec().is_err()
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Invalid base58 address")),
        );
    }

    if payload.lamports == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Lamports must be greater than 0")),
        );
    }

    let mock_instruction = format!(
        "transfer {} lamports from {} to {}",
        payload.lamports, payload.from, payload.to
    );

    let response = SendSolResponse {
        program_id: "11111111111111111111111111111111".to_string(),
        accounts: vec![payload.from, payload.to],
        instruction_data: general_purpose::STANDARD.encode(mock_instruction),
    };

    (StatusCode::OK, Json(ApiResponse::success(response)))
}
