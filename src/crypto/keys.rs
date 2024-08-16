use bip39::Mnemonic;
use ed25519_dalek::{ed25519::signature::SignerMut, Signature, SigningKey, Verifier, VerifyingKey};
use pbkdf2::pbkdf2_hmac;
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;

const PRIVATE_KEY_SIZE: usize = ed25519_dalek::SECRET_KEY_LENGTH;
const PUBLIC_KEY_SIZE: usize = ed25519_dalek::PUBLIC_KEY_LENGTH;
const SIGNATURE_SIZE: usize = ed25519_dalek::SIGNATURE_LENGTH;
const ADDRESS_SIZE: usize = 20;
const SEED_SIZE: usize = 32;

pub struct PrivateKey {
    pub key: SigningKey,
}

/// new_entropy generates a new 16 byte entropy
pub fn new_entropy() -> [u8; 16] {
    let mut csprng = OsRng;
    let mut entropy = [0u8; 16];
    csprng.fill_bytes(&mut entropy);
    entropy
}

/// get_mnemonic_from_entropy generates a new mnemonic from the given entropy
pub fn get_mnemonic_from_entropy(entropy: &[u8; 16]) -> String {
    let mnemonic = Mnemonic::from_entropy(entropy).unwrap();
    mnemonic.to_string()
}

/// get_seed_from_mnemonic generates a new seed from the given mnemonic
pub fn get_seed_from_mnemonic(mnemonic: &str) -> [u8; SEED_SIZE] {
    let mut seed = [0u8; SEED_SIZE];
    let salt = b"mnemonic+some password";
    // number of iterations
    let n = 1024_000;

    pbkdf2_hmac::<Sha256>(mnemonic.as_bytes(), salt, n, &mut seed);

    seed
}

/// get_private_key_from_mnemonic generates a new private key from the given mnemonic
pub fn get_private_key_from_mnemonic(mnemonic: &str) -> PrivateKey {
    let seed = get_seed_from_mnemonic(mnemonic);
    new_private_key_from_seed(&seed)
}

/// new_private_key_from_string generates a new private key from the given private key string
pub fn new_private_key_from_string(private_key: &str) -> PrivateKey {
    let decoded_private_key = hex::decode(private_key).unwrap();

    new_private_key_from_seed(&decoded_private_key[..SEED_SIZE].try_into().unwrap())
}

/// new_private_key_from_seed generates a new private key from the given seed
pub fn new_private_key_from_seed(seed: &[u8; 32]) -> PrivateKey {
    if seed.len() != SEED_SIZE {
        panic!("Invalid seed size, expected 32 bytes.");
    }

    let key = SigningKey::from_bytes(seed);
    PrivateKey { key }
}

/// generate_private_key generates a new private key with a random seed (Good for testing purposes)
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
    /// Convert the signature to bytes
    pub fn to_bytes(&self) -> [u8; SIGNATURE_SIZE] {
        self.signature.to_bytes()
    }

    /// Verify a message with the public key
    pub fn verify(&self, data: &[u8], public_key: &PublicKey) -> bool {
        public_key.key.verify(data, &self.signature).is_ok()
    }
}

pub struct Address {
    pub value: [u8; ADDRESS_SIZE],
}

impl Address {
    /// Convert the address to a string in hex format
    pub fn to_string(&self) -> String {
        hex::encode(self.value)
    }

    /// Convert the address to bytes
    pub fn to_bytes(&self) -> [u8; ADDRESS_SIZE] {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_private_key_from_mnemonic_deterministic() {
        let entropy = new_entropy();
        let mnemonic = get_mnemonic_from_entropy(&entropy);
        let private_key = get_private_key_from_mnemonic(&mnemonic);

        assert_eq!(
            private_key.to_bytes().len(),
            PRIVATE_KEY_SIZE,
            "Private key should be 64 bytes long"
        );
    }

    #[test]
    fn test_new_private_key_from_mnemonic() {
        let mnemonic = "reason oil range swear outdoor letter pair city axis expire tower dumb";
        let address_string = "fddf13d119646c53622a70adba80081a3ef13522";
        let private_key = get_private_key_from_mnemonic(&mnemonic);

        assert_eq!(
            private_key.to_bytes().len(),
            PRIVATE_KEY_SIZE,
            "Private key should be 64 bytes long"
        );

        assert_eq!(
            private_key.public_key().address().to_string(),
            address_string,
            "Address should be equal to the expected value"
        );
    }

    #[test]
    fn test_get_seed_from_mnemonic_length() {
        let mnemonic = "test mnemonic";
        let seed = get_seed_from_mnemonic(mnemonic);
        assert_eq!(seed.len(), SEED_SIZE, "Seed should be 32 bytes long");
    }

    #[test]
    fn test_get_seed_from_mnemonic_deterministic() {
        let mnemonic = "test mnemonic";
        let seed1 = get_seed_from_mnemonic(mnemonic);
        let seed2 = get_seed_from_mnemonic(mnemonic);
        assert_eq!(
            seed1, seed2,
            "Seed should be the same for the same mnemonic"
        );
    }

    #[test]
    fn test_get_seed_from_mnemonic_different_mnemonics() {
        let mnemonic1 = "test mnemonic one";
        let mnemonic2 = "test mnemonic two";
        let seed1 = get_seed_from_mnemonic(mnemonic1);
        let seed2 = get_seed_from_mnemonic(mnemonic2);
        assert_ne!(
            seed1, seed2,
            "Seed should be different for different mnemonics"
        );
    }

    #[test]
    fn test_new_entropy_length() {
        let entropy = new_entropy();
        assert_eq!(entropy.len(), 16, "Entropy should be 16 bytes long");
    }

    #[test]
    fn test_new_entropy_randomness() {
        let entropy1 = new_entropy();
        let entropy2 = new_entropy();

        // It's extremely unlikely that two random 128-bit values will be the same
        assert_ne!(entropy1, entropy2, "Entropy values should be different");
    }

    #[test]
    fn test_get_mnemonic_from_entropy() {
        let entropy = new_entropy();
        let mnemonic = get_mnemonic_from_entropy(&entropy);
        println!("Mnemonic: {}", mnemonic);
        assert_eq!(12, mnemonic.split_whitespace().count());
    }

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
        assert_eq!(address_string, address.to_string());
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
        let invalid_private_key = generate_private_key();
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
        // println!("{}", address.string());
        assert_eq!(ADDRESS_SIZE, address.value.len());
    }
}
