use base64::{engine::general_purpose, Engine as _};
use bs58;
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};

pub fn sign_message(secret_base58: &str, message: &str) -> Result<(String, String), String> {
    let secret_bytes = bs58::decode(secret_base58)
        .into_vec()
        .map_err(|_| "Invalid base58 secret key")?;

    if secret_bytes.len() != 64 {
        return Err("Secret key must be 64 bytes".into());
    }

    let keypair = Keypair::from_bytes(&secret_bytes).map_err(|_| "Invalid keypair bytes")?;
    let signature: Signature = keypair.sign(message.as_bytes());

    let signature_base64 = general_purpose::STANDARD.encode(signature.to_bytes());
    let public_key_base58 = bs58::encode(keypair.public.to_bytes()).into_string();

    Ok((signature_base64, public_key_base58))
}

pub fn verify_message(
    message: &str,
    signature_base64: &str,
    pubkey_base58: &str,
) -> Result<bool, String> {
    let signature_bytes = general_purpose::STANDARD
        .decode(signature_base64)
        .map_err(|_| "Invalid base64 signature".to_string())?;

    let signature = Signature::from_bytes(&signature_bytes)
        .map_err(|_| "Invalid signature format".to_string())?;

    let pubkey_bytes = bs58::decode(pubkey_base58)
        .into_vec()
        .map_err(|_| "Invalid base58 public key".to_string())?;

    let public_key = PublicKey::from_bytes(&pubkey_bytes)
        .map_err(|_| "Invalid public key format".to_string())?;

    public_key
        .verify(message.as_bytes(), &signature)
        .map(|_| true)
        .map_err(|_| "Signature verification failed".to_string())
}
