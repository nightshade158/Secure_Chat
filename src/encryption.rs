use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;
use rand::rngs::OsRng;
use base64::{engine::general_purpose, Engine};
use sha2::{Sha256, Digest};

fn derive_key(secret: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(secret.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result[..32]); // Ensure it's exactly 32 bytes
    key
}

pub fn encrypt_message(secret: &str, message: &str) -> Result<(String, String), String> {
    let key_bytes = derive_key(secret);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12]; // 12-byte nonce
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    match cipher.encrypt(nonce, message.as_bytes()) {
        Ok(ciphertext) => Ok((
            general_purpose::STANDARD.encode(nonce_bytes),
            general_purpose::STANDARD.encode(ciphertext),
        )),
        Err(_) => Err("Encryption failed".to_string()),
    }
}

pub fn decrypt_message(secret: &str, nonce_b64: &str, ciphertext_b64: &str) -> Result<String, String> {
    let key_bytes = derive_key(secret);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let nonce_bytes = general_purpose::STANDARD.decode(nonce_b64).map_err(|_| "Invalid nonce")?;
    if nonce_bytes.len() != 12 {
        return Err("Nonce must be exactly 12 bytes".to_string());
    }
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = general_purpose::STANDARD.decode(ciphertext_b64).map_err(|_| "Invalid ciphertext")?;

    cipher.decrypt(nonce, ciphertext.as_ref())
        .map(|plaintext| String::from_utf8(plaintext).map_err(|_| "Invalid UTF-8".to_string()))
        .map_err(|_| "Decryption failed".to_string())?
}
