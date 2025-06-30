use crate::handlers::{
    create_token, create_token_mint, keypair, send_sol, send_token, sign_message, verify_message,
};
use axum::{routing::post, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/keypair", post(keypair::generate_keypair))
        .route("/token/create", post(create_token::create_token))
        .route("/token/mint", post(create_token_mint::mint_token))
        .route("/message/sign", post(sign_message::sign_message))
        .route("/verify/message", post(verify_message::verify_message))
        .route("/send/sol", post(send_sol::send_sol))
        .route("/send/token", post(send_token::send_token))
}
