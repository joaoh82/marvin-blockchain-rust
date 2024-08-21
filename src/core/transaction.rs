use crate::crypto::keys::{PrivateKey, PublicKey, SignatureWrapper};
use crate::types::hash;

use crypto::digest::Digest;
use serde::{Deserialize, Serialize};
use crypto::sha2::Sha256;


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Transaction {
    pub from: Option<PublicKey>,
    pub to: Option<PublicKey>,
    pub value: u64,
    pub data: Vec<u8>,
    pub signature: Option<SignatureWrapper>,
    pub nonce: u64,

    // Cached version of the transaction hash
    pub hash: Option<hash::Hash>,
}

impl Transaction {
    pub fn new(data: Vec<u8>) -> Self {
        Transaction {
            from: None,
            to: None,
            value: 0,
            data,
            signature: None,
            nonce: rand::random::<u64>(),
            hash: None,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Serialization failed")
    }

    /// Calculate the hash of the transaction
    pub fn hash(&mut self) -> Option<hash::Hash> {
        if let Some(hash) = &self.hash {
            return Some(hash.clone());
        }

        let bytes = self.to_bytes();
        let mut hasher = Sha256::new();
        hasher.input(&bytes);
        let mut hash = [0; 32];
        hasher.result(&mut hash);

        self.hash = Some(hash::Hash { hash });

        Some(hash::Hash { hash })
    }

    fn sign(&mut self, private_key: &mut PrivateKey) -> Result<(), String> {
        let hash = self.hash().unwrap();
        let signature = private_key.sign(&hash.hash);
        self.signature = Some(signature.unwrap());
        self.from = Some(private_key.public_key());

        Ok(())
    }

    fn verify(&mut self) -> bool {
        if self.from.is_none() || self.signature.is_none() {
            return false;
        }

        let public_key = self.from.unwrap();
        let signature = self.signature.unwrap();
        let hash = self.hash().unwrap();

        let is_verified = signature.verify(&hash.hash, &public_key);

        is_verified
    }

    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn decode(data: &[u8]) -> Self {
        bincode::deserialize::<Transaction>(data).unwrap()
    }
}

fn generate_random_transaction_with_signature() -> Transaction {
    let mut tx = Transaction::new(String::from("marvin!").into_bytes());
    let mut private_key = crate::crypto::keys::generate_private_key();
    tx.sign(&mut private_key).unwrap();
    tx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_hash() {
        let mut tx = Transaction::new(String::from("Hello, World!").into_bytes());
        let hash = tx.hash().unwrap();
        assert!(!hash.is_zero());
    }

    #[test]
    fn test_transaction_signing() {
        let mut tx = Transaction::new(String::from("Hello, World!").into_bytes());
        let mut private_key = crate::crypto::keys::generate_private_key();
        tx.sign(&mut private_key).unwrap();
        assert!(tx.signature.is_some());
    }

    #[test]
    fn test_transaction_verification() {
        let mut tx = Transaction::new(String::from("Hello, World!").into_bytes());
        let mut private_key = crate::crypto::keys::generate_private_key();
        tx.sign(&mut private_key).unwrap();
        assert!(tx.verify());
    }

    #[test]
    fn test_transaction_encode_decode() {
        let tx = generate_random_transaction_with_signature();
        let encoded = tx.encode();
        let decoded = Transaction::decode(&encoded);

        assert_eq!(tx.from, decoded.from);
        assert_eq!(tx.to, decoded.to);
        assert_eq!(tx.value, decoded.value);
        assert_eq!(tx.data, decoded.data);
        assert_eq!(tx.nonce, decoded.nonce);
        assert_eq!(tx.hash, decoded.hash);
        assert_eq!(tx.signature, decoded.signature);
    }
}