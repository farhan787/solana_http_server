use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::response::ApiResponse;

#[derive(Deserialize)]
pub struct TokenCreateRequest {
    pub mintAuthority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Serialize)]
pub struct AccountInfo {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct TokenCreateResponse {
    pub program_id: String,
    pub accounts: Vec<AccountInfo>,
    pub instruction_data: String,
}

pub async fn create_token(
    Json(payload): Json<TokenCreateRequest>,
) -> (StatusCode, Json<ApiResponse<TokenCreateResponse>>) {
    let instruction_info = format!("CreateTokenMint: decimals={}", payload.decimals);

    let response = TokenCreateResponse {
        program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
        accounts: vec![
            AccountInfo {
                pubkey: payload.mint.clone(),
                is_signer: false,
                is_writable: true,
            },
            AccountInfo {
                pubkey: payload.mintAuthority.clone(),
                is_signer: true,
                is_writable: false,
            },
        ],
        instruction_data: base64::encode(instruction_info),
    };

    (StatusCode::OK, Json(ApiResponse::success(response)))
}
