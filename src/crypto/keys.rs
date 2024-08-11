use ed25519_dalek::{ed25519::signature::SignerMut, Signature, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

const PRIVATE_KEY_SIZE: usize = ed25519_dalek::SECRET_KEY_LENGTH;
const PUBLIC_KEY_SIZE: usize = ed25519_dalek::PUBLIC_KEY_LENGTH;
const SIGNATURE_SIZE: usize = ed25519_dalek::SIGNATURE_LENGTH;
const ADDRESS_SIZE: usize = 20;
const SEED_SIZE: usize = 32;

pub struct PrivateKey {
    pub key: SigningKey,
}

pub fn new_private_key_from_string(private_key: &str) -> PrivateKey {
    let decoded_private_key = hex::decode(private_key).unwrap();

    new_private_key_from_seed(&decoded_private_key[..SEED_SIZE].try_into().unwrap())
}

pub fn new_private_key_from_seed(seed: &[u8; 32]) -> PrivateKey {
    if seed.len() != SEED_SIZE {
        panic!("Invalid seed size, expected 32 bytes.");
    }

    let key = SigningKey::from_bytes(seed);
    PrivateKey { key }
}

pub fn generate_private_key() -> PrivateKey {
    let mut csprng = OsRng;
    let key: SigningKey = SigningKey::generate(&mut csprng);
    PrivateKey { key }
}

impl PrivateKey {
    pub fn sign(&mut self, data: &[u8]) -> SignatureWrapper {
        SignatureWrapper {
            signature: self.key.sign(data),
        }
    }

    /// Sign a message with the private key
    pub fn to_bytes(&self) -> [u8; PRIVATE_KEY_SIZE] {
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
    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_SIZE] {
        self.key.to_bytes()
    }

    pub fn address(&self) -> Address {
        Address {
            value: self.key.to_bytes()[..ADDRESS_SIZE].try_into().unwrap(),
        }
    }
}

pub struct SignatureWrapper {
    pub signature: Signature,
}

impl SignatureWrapper {
    pub fn to_bytes(&self) -> [u8; SIGNATURE_SIZE] {
        self.signature.to_bytes()
    }

    pub fn verify(&self, data: &[u8], public_key: &PublicKey) -> bool {
        public_key.key.verify(data, &self.signature).is_ok()
    }
}

pub struct Address {
    pub value: [u8; ADDRESS_SIZE],
}

impl Address {
    pub fn string(&self) -> String {
        hex::encode(self.value)
    }

    pub fn to_bytes(&self) -> [u8; ADDRESS_SIZE] {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_private_key_from_seed() {
        let seed = [0u8; 32];
        let private_key = new_private_key_from_seed(&seed);
        assert_eq!(PRIVATE_KEY_SIZE, private_key.to_bytes().len());
    }

    #[test]
    fn test_new_private_key_from_string() {
        let seed = "753bfa924576a230736e83589933ccb7aad8fd3934d7e9637df4912b58ac95d6";
        let address_string = "339f9690596b35d909a8c47fe26c5e8697af034c";
        let private_key = new_private_key_from_string(&seed);

        assert_eq!(PRIVATE_KEY_SIZE, private_key.to_bytes().len());

        let address = private_key.public_key().address();
        assert_eq!(address_string, address.string());
    }

    #[test]
    fn test_generate_private_key() {
        let private_key = generate_private_key();
        assert_eq!(PRIVATE_KEY_SIZE, private_key.to_bytes().len());

        let public_key = private_key.public_key();
        assert_eq!(PUBLIC_KEY_SIZE, public_key.to_bytes().len());
    }

    #[test]
    fn test_private_key_sign() {
        let mut private_key = generate_private_key();
        let public_key: PublicKey = private_key.public_key();
        let mut invalid_private_key = generate_private_key();
        let invalid_public_key: PublicKey = invalid_private_key.public_key();

        let data = b"hello world";
        let signature = private_key.sign(data);

        // Verify the signature size
        assert_eq!(SIGNATURE_SIZE, signature.signature.to_bytes().len());

        // Verify the signature
        assert_eq!(true, signature.verify(data, &public_key));

        // Verify the signature
        assert_eq!(true, signature.verify(data, &public_key));

        // Verify the signature is invalid
        assert_eq!(false, signature.verify(data, &invalid_public_key));
    }

    #[test]
    fn test_public_jey_to_address() {
        let private_key = generate_private_key();
        let public_key = private_key.public_key();
        let address = public_key.address();
        println!("{}", address.string());
        assert_eq!(ADDRESS_SIZE, address.value.len());
    }
}
