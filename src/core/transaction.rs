use crate::crypto::keys::{PublicKey, SignatureWrapper};
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
    pub fn hash(&self) -> Option<hash::Hash> {
        if let Some(hash) = &self.hash {
            return Some(hash.clone());
        }

        let bytes = self.to_bytes();
        let mut hasher = Sha256::new();
        hasher.input(&bytes);
        let mut hash = [0; 32];
        hasher.result(&mut hash);

        Some(hash::Hash { hash })
    }

    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn decode(data: &[u8]) -> Self {
        bincode::deserialize::<Transaction>(data).unwrap()
    }
}