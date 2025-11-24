// Utils module
// This module contains utility functions

use ring::aead::{self, BoundKey, Nonce, NonceSequence, UnboundKey};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};
use std::fs;
use std::path::PathBuf;

/// Counter-based nonce sequence for AES-GCM
struct CounterNonceSequence(u32);

impl CounterNonceSequence {
    fn new() -> Self {
        CounterNonceSequence(0)
    }
}

impl NonceSequence for CounterNonceSequence {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        let mut nonce_bytes = [0u8; 12];
        let bytes = self.0.to_be_bytes();
        nonce_bytes[8..12].copy_from_slice(&bytes);
        self.0 += 1;
        Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}

/// Encryptor for sensitive data (passwords, API keys)
pub struct Encryptor {
    key_bytes: Vec<u8>,
}

impl Encryptor {
    /// Create a new encryptor with machine-specific key
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let key_path = Self::get_key_path()?;

        let key_bytes = if key_path.exists() {
            // Load existing key
            fs::read(&key_path)?
        } else {
            // Generate new key
            let rng = SystemRandom::new();
            let mut key_bytes = vec![0u8; 32]; // 256-bit key
            rng.fill(&mut key_bytes)
                .map_err(|_| "Failed to generate random key")?;

            // Save key with restricted permissions
            fs::create_dir_all(key_path.parent().unwrap())?;
            fs::write(&key_path, &key_bytes)?;

            // Set file permissions to 600 (owner read/write only) on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&key_path)?.permissions();
                perms.set_mode(0o600);
                fs::set_permissions(&key_path, perms)?;
            }

            key_bytes
        };

        Ok(Self { key_bytes })
    }

    /// Encrypt plaintext and return as HEX string
    pub fn encrypt(&self, plaintext: &str) -> Result<String, Box<dyn std::error::Error>> {
        let unbound_key = UnboundKey::new(&aead::AES_256_GCM, &self.key_bytes)
            .map_err(|_| "Failed to create encryption key")?;
        let nonce_sequence = CounterNonceSequence::new();
        let mut sealing_key = aead::SealingKey::new(unbound_key, nonce_sequence);

        let mut in_out = plaintext.as_bytes().to_vec();
        let tag = sealing_key
            .seal_in_place_separate_tag(aead::Aad::empty(), &mut in_out)
            .map_err(|_| "Encryption failed")?;

        // Append tag to ciphertext
        in_out.extend_from_slice(tag.as_ref());

        // Convert to HEX string
        Ok(hex::encode(in_out))
    }

    /// Decrypt HEX string to plaintext
    pub fn decrypt(&self, hex_ciphertext: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Decode HEX string
        let mut in_out = hex::decode(hex_ciphertext)?;

        if in_out.len() < aead::AES_256_GCM.tag_len() {
            return Err("Ciphertext too short".into());
        }

        let unbound_key = UnboundKey::new(&aead::AES_256_GCM, &self.key_bytes)
            .map_err(|_| "Failed to create decryption key")?;
        let nonce_sequence = CounterNonceSequence::new();
        let mut opening_key = aead::OpeningKey::new(unbound_key, nonce_sequence);

        let plaintext = opening_key
            .open_in_place(aead::Aad::empty(), &mut in_out)
            .map_err(|_| "Decryption failed")?;

        String::from_utf8(plaintext.to_vec())
            .map_err(|e| format!("Invalid UTF-8: {}", e).into())
    }

    /// Get path to encryption key file
    fn get_key_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let home_dir = dirs::home_dir()
            .ok_or("Failed to get home directory")?;
        Ok(home_dir.join(".es_client").join(".key"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let encryptor = Encryptor::new().unwrap();
        let plaintext = "my_secret_password";

        let encrypted = encryptor.encrypt(plaintext).unwrap();
        assert_ne!(encrypted, plaintext);
        assert!(!encrypted.is_empty());

        let decrypted = encryptor.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_decrypt_invalid() {
        let encryptor = Encryptor::new().unwrap();
        let result = encryptor.decrypt("invalid_hex");
        assert!(result.is_err());
    }
}
