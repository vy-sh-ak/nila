use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::{rngs::OsRng, RngCore};

use crate::error::AppError;

const PREFIX: &str = "enc.v1:";
const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12;

pub const MASTER_KEY_ID: &str = "security.master_key";

/// AES-256-GCM helper for encrypting secrets at rest (e.g. model API keys).
///
/// Encrypted values are stored as `enc.v1:<nonce_b64>:<ciphertext_b64>` so the
/// scheme can be versioned later. Values without the prefix pass through
/// untouched, keeping reads tolerant of plaintext rows written before
/// encryption existed.
#[derive(Clone)]
pub struct Crypto {
    cipher: Aes256Gcm,
}

impl Crypto {
    /// Generate a new random 256-bit master key, base64 encoded for storage.
    pub fn generate_key() -> String {
        let mut key = [0u8; KEY_LEN];
        OsRng.fill_bytes(&mut key);
        BASE64.encode(key)
    }

    pub fn new(master_key_b64: &str) -> Result<Self, AppError> {
        let key = BASE64.decode(master_key_b64)?;
        if key.len() != KEY_LEN {
            return Err(AppError::Crypto(format!(
                "invalid master key length: expected {KEY_LEN} bytes, got {}",
                key.len()
            )));
        }
        let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| AppError::Crypto(e.to_string()))?;
        Ok(Self { cipher })
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, AppError> {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        OsRng.fill_bytes(&mut nonce_bytes);
        let ciphertext = self.cipher.encrypt(Nonce::from_slice(&nonce_bytes), plaintext.as_bytes())?;
        Ok(format!(
            "{PREFIX}{}:{}",
            BASE64.encode(nonce_bytes),
            BASE64.encode(ciphertext)
        ))
    }

    pub fn decrypt(&self, stored: &str) -> Result<String, AppError> {
        let Some(payload) = stored.strip_prefix(PREFIX) else {
            return Ok(stored.to_owned());
        };
        let (nonce_b64, ciphertext_b64) = payload
            .split_once(':')
            .ok_or_else(|| AppError::Crypto("malformed encrypted value".into()))?;
        let nonce_bytes = BASE64.decode(nonce_b64)?;
        let ciphertext = BASE64.decode(ciphertext_b64)?;
        let plaintext =
            self.cipher
                .decrypt(Nonce::from_slice(&nonce_bytes), ciphertext.as_ref())?;
        String::from_utf8(plaintext).map_err(|e| AppError::Crypto(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrips() {
        let crypto = Crypto::new(&Crypto::generate_key()).unwrap();
        let secret = "sk-test-1234567890";
        let stored = crypto.encrypt(secret).unwrap();
        assert_ne!(stored, secret);
        assert!(stored.starts_with(PREFIX));
        assert_eq!(crypto.decrypt(&stored).unwrap(), secret);
    }

    #[test]
    fn passes_through_plaintext() {
        let crypto = Crypto::new(&Crypto::generate_key()).unwrap();
        assert_eq!(crypto.decrypt("plain-key").unwrap(), "plain-key");
    }

    #[test]
    fn rejects_tampered_ciphertext() {
        let crypto = Crypto::new(&Crypto::generate_key()).unwrap();
        let mut stored = crypto.encrypt("secret").unwrap();
        stored.push('x');
        assert!(crypto.decrypt(&stored).is_err());
    }
}
