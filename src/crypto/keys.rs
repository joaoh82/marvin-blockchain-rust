use ed25519_dalek::{ed25519::signature::SignerMut, Signature, SigningKey, VerifyingKey};
use rand::rngs::OsRng;

const PRIVATE_KEY_SIZE: usize = ed25519_dalek::SECRET_KEY_LENGTH;
const PUBLIC_KEY_SIZE: usize = ed25519_dalek::PUBLIC_KEY_LENGTH;
const SIGNATURE_SIZE: usize = ed25519_dalek::SIGNATURE_LENGTH;

pub struct PrivateKey {
    pub key: SigningKey,
}

impl PrivateKey {
    pub fn generate_private_key() -> PrivateKey {
        let mut csprng = OsRng;
        let key: SigningKey = SigningKey::generate(&mut csprng);
        PrivateKey { key }
    }

    pub fn sign(&mut self, data: &[u8]) -> Signature {
        self.key.sign(data)
    }

    /// Sign a message with the private key
    pub fn bytes(&self) -> [u8; PRIVATE_KEY_SIZE] {
        self.key.to_bytes()
    }

    /// Sign a message with the private key
    pub fn public_key(&self) -> PublicKey {
        PublicKey {
            key: self.key.verifying_key(),
        }
    }
}

pub struct PublicKey {
    pub key: VerifyingKey,
}

impl PublicKey {
    pub fn bytes(&self) -> [u8; PUBLIC_KEY_SIZE] {
        self.key.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_private_key() {
        let private_key = PrivateKey::generate_private_key();
        assert_eq!(PRIVATE_KEY_SIZE, private_key.bytes().len());

        let public_key = private_key.public_key();
        assert_eq!(PUBLIC_KEY_SIZE, public_key.bytes().len());
    }

    #[test]
    fn test_private_key_sign() {
        let mut private_key = PrivateKey::generate_private_key();
        let data = b"hello world";
        let signature = private_key.sign(data);
        assert_eq!(SIGNATURE_SIZE, signature.to_bytes().len());
    }
}
