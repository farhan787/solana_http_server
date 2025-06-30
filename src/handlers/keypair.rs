use crate::response::ApiResponse;
use axum::{http::StatusCode, Json};
use bs58;
use solana_sdk::signature::{Keypair, Signer};

pub async fn generate_keypair() -> (StatusCode, Json<ApiResponse<KeypairResponse>>) {
    let keypair = Keypair::new();

    let pubkey_bs58 = keypair.pubkey().to_string();

    let secret_bytes = keypair.to_bytes();
    let secret_bs58 = bs58::encode(secret_bytes).into_string();

    let data: KeypairResponse = KeypairResponse {
        pubkey: pubkey_bs58,
        secret: secret_bs58,
    };
    let response = ApiResponse::success(data);

    (StatusCode::OK, Json(response))
}

#[derive(serde::Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}
