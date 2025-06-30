use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::response::ApiResponse;

#[derive(Deserialize)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct AccountInfo {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct MintTokenResponse {
    pub program_id: String,
    pub accounts: Vec<AccountInfo>,
    pub instruction_data: String,
}

pub async fn mint_token(
    Json(payload): Json<MintTokenRequest>,
) -> (StatusCode, Json<ApiResponse<MintTokenResponse>>) {
    let program_id = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string();

    let accounts = vec![
        AccountInfo {
            pubkey: payload.mint.clone(),
            is_signer: false,
            is_writable: true,
        },
        AccountInfo {
            pubkey: payload.destination.clone(),
            is_signer: false,
            is_writable: true,
        },
        AccountInfo {
            pubkey: payload.authority.clone(),
            is_signer: true,
            is_writable: false,
        },
    ];

    // Encode a mock instruction data string with amount info
    let instruction_info = format!("MintTo: amount={}", payload.amount);
    let instruction_data = base64::encode(instruction_info);

    let response = MintTokenResponse {
        program_id,
        accounts,
        instruction_data,
    };

    (StatusCode::OK, Json(ApiResponse::success(response)))
}
