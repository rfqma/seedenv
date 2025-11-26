use base64::{Engine as _, engine::general_purpose};

pub fn encrypt_secret(
    plaintext: &str,
    public_key_base64: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    sodiumoxide::init().map_err(|_| "failed to initialize sodiumoxide")?;

    let public_key_bytes = general_purpose::STANDARD.decode(public_key_base64)?;

    let public_key = sodiumoxide::crypto::box_::PublicKey::from_slice(&public_key_bytes)
        .ok_or("invalid public key")?;

    let ciphertext = sodiumoxide::crypto::sealedbox::seal(plaintext.as_bytes(), &public_key);

    let encrypted_base64 = general_purpose::STANDARD.encode(&ciphertext);

    Ok(encrypted_base64)
}
