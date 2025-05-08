use ring::aead::{self, Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::error::Error;
use std::sync::Arc;
use std::fmt;

#[derive(Debug)]
pub enum CryptoError {
    EncryptionError(String),
    DecryptionError(String),
    EncodingError(String),
    InvalidData(String),
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::EncryptionError(e) => write!(f, "Encryption error: {}", e),
            CryptoError::DecryptionError(e) => write!(f, "Decryption error: {}", e),
            CryptoError::EncodingError(e) => write!(f, "Encoding error: {}", e),
            CryptoError::InvalidData(e) => write!(f, "Invalid data: {}", e),
        }
    }
}

impl Error for CryptoError {}

pub struct Encryption {
    key: Arc<aead::LessSafeKey>,
    rng: Arc<SystemRandom>,
}

impl Clone for Encryption {
    fn clone(&self) -> Self {
        Self {
            key: Arc::clone(&self.key),
            rng: Arc::clone(&self.rng),
        }
    }
}

impl Encryption {
  pub fn new() -> Result<Self, CryptoError> {
      let rng = Arc::new(SystemRandom::new());
      let mut key_bytes = [0u8; 32];
      rng.fill(&mut key_bytes)
          .map_err(|e| CryptoError::EncryptionError(e.to_string()))?;
      let unbound = UnboundKey::new(&AES_256_GCM, &key_bytes)
          .map_err(|e| CryptoError::EncryptionError(e.to_string()))?;
      Ok(Self {
          key: Arc::new(LessSafeKey::new(unbound)),
          rng,
      })
  }

  pub fn encrypt(&self, message: &str) -> Result<String, CryptoError> {
      // Génération nonce
      let mut nonce_bytes = [0u8; 12];
      self.rng.fill(&mut nonce_bytes)
          .map_err(|e| CryptoError::EncryptionError(e.to_string()))?;
      let nonce = Nonce::assume_unique_for_key(nonce_bytes);

      // Préparer buffer plaintext + place pour tag
      let mut in_out = message.as_bytes().to_vec();
      in_out.reserve(AES_256_GCM.tag_len());
      // Encrypt & append tag
      self.key
          .seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
          .map_err(|e| CryptoError::EncryptionError(e.to_string()))?;

      // Résultat = nonce || ciphertext || tag
      let mut res = nonce_bytes.to_vec();
      res.extend_from_slice(&in_out);
      Ok(BASE64.encode(&res))
  }

  pub fn decrypt(&self, encrypted: &str) -> Result<String, CryptoError> {
      // Base64 → bytes
      let data = BASE64
          .decode(encrypted)
          .map_err(|e| CryptoError::InvalidData(e.to_string()))?;
      if data.len() < 12 + AES_256_GCM.tag_len() {
          return Err(CryptoError::InvalidData("Données trop courtes".into()));
      }

      // Séparer nonce et ciphertext||tag
      let nonce = Nonce::try_assume_unique_for_key(&data[..12])
          .map_err(|e| CryptoError::DecryptionError(e.to_string()))?;
      let mut buf = data[12..].to_vec(); // contient ciphertext + tag

      // Décrypter in‑place et récupérer slice plaintext
      let plaintext = self
          .key
          .open_in_place(nonce, Aad::empty(), &mut buf)
          .map_err(|e| CryptoError::DecryptionError(e.to_string()))?;

      // UTF‑8 → String
      String::from_utf8(plaintext.to_vec())
          .map_err(|e| CryptoError::EncodingError(e.to_string()))
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let crypto = Encryption::new().unwrap();
        let message = "Hello, World!";
        
        let encrypted = crypto.encrypt(message).unwrap();
        let decrypted = crypto.decrypt(&encrypted).unwrap();
        
        assert_eq!(message, decrypted);
    }

    #[test]
    fn test_invalid_base64() {
        let crypto = Encryption::new().unwrap();
        let result = crypto.decrypt("invalid-base64!");
        assert!(matches!(result, Err(CryptoError::InvalidData(_))));
    }
} 