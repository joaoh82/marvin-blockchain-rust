use crate::core::transaction::Transaction;
use crate::crypto::keys::{PrivateKey, PublicKey, SignatureWrapper};
use crate::types::hash;

use bincode;
use crypto::digest::Digest;
use serde::{Deserialize, Serialize};

use crypto::sha2::Sha256;

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub previous_block_hash: [u8; 32],

    pub tx_hash: hash::Hash,
    pub version: u32,
    pub height: u32,
    pub timestamp: u32,

    pub nonce: u32,
    pub difficulty: u8,
}

impl Header {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Serialization failed")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    header: Header,
    transactions: Vec<Transaction>,
    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]
    public_key: Option<PublicKey>,
    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]
    signature: Option<SignatureWrapper>,

    // Cached version of the header hash
    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]
    hash: Option<hash::Hash>,
}

impl Block {
    pub fn new(header: Header, transactions: Vec<Transaction>) -> Self {
        Block {
            header,
            transactions,
            public_key: None,
            signature: None,
            hash: None,
        }
    }

    /// Serialize the block to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Serialization failed")
    }


    /// Add a transaction to the block
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
        let mut txHash = self.calculate_tx_hash();
        self.header.tx_hash = txHash;
    }


    /// Calculates the hash of all the transactions in the block
    pub fn calculate_tx_hash(&self) -> hash::Hash {
        let mut hasher = Sha256::new();

        for tx in &self.transactions {
            hasher.input(&tx.to_bytes());
        }

        let mut hash = [0; 32];
        hasher.result(&mut hash);

        hash::Hash { hash }
    }


    /// Sign the block with the given private key
    pub fn sign(&mut self, mut private_key: PrivateKey) -> Result<(), String> {
        let public_key = private_key.public_key();
        let signature = private_key.sign(&self.header.to_bytes());

        self.public_key = Some(public_key);
        self.signature = Some(signature);

        Ok(())
    }

    /// Verify the signature of the block
    pub fn verify(&self) -> Result<bool, String> {
        if self.public_key.is_none() || self.signature.is_none() {
            return Err("Block is not signed".to_string());
        }

        let public_key = self.public_key.as_ref().unwrap();
        let signature = self.signature.as_ref().unwrap();

        let is_valid = signature.verify(&self.header.to_bytes(), public_key);
        if !is_valid {
            return Err("Invalid signature".to_string());
        }

        let mut tx_hash = self.calculate_tx_hash();
        if tx_hash != self.header.tx_hash {
            return Err("Invalid transaction hash".to_string());
        }

        Ok(is_valid)
    }

    /// Calculate the hash of the block
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
        bincode::deserialize::<Block>(data).unwrap()
    }

}

/// Generate a random block for testing purposes
fn generate_random_block() -> Block {
    let mut private_key = crate::crypto::keys::generate_private_key();

    let mut header = Header {
        previous_block_hash: [0; 32],
        tx_hash: hash::Hash { hash: [0; 32] },
        version: 1,
        height: 1,
        timestamp: 1,
        nonce: 1,
        difficulty: 1,
    };

    let transactions = vec![];
    let mut block = Block::new(header, transactions);
    let tx_hash = block.calculate_tx_hash();
    block.header.tx_hash = tx_hash;

    block.sign(private_key).unwrap();

    block
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto;

    #[test]
    fn test_block_encode_decode() {
        let block = generate_random_block();
        let encoded = block.encode();
        let decoded = Block::decode(&encoded);

        assert_eq!(block.header.previous_block_hash, decoded.header.previous_block_hash);
        assert_eq!(block.header.tx_hash, decoded.header.tx_hash);
        assert_eq!(block.header.version, decoded.header.version);
        assert_eq!(block.header.height, decoded.header.height);
        assert_eq!(block.header.timestamp, decoded.header.timestamp);
        assert_eq!(block.header.nonce, decoded.header.nonce);
        assert_eq!(block.header.difficulty, decoded.header.difficulty);
        assert_eq!(block.transactions.len(), decoded.transactions.len());
        // assert_eq!(block.public_key, decoded.public_key);
        // assert_eq!(block.signature, decoded.signature);
    }

    #[test]
    fn test_block_verify() {
        let mut block = generate_random_block();
        let private_key = crypto::keys::generate_private_key();
        block.sign(private_key).unwrap();

        let is_valid = block.verify().unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_block_sign() {
        let mut block = generate_random_block();
        let private_key = crypto::keys::generate_private_key();
        block.sign(private_key).unwrap();

        assert!(block.public_key.is_some());
        assert!(block.signature.is_some());
    }

    #[test]
    fn test_block_hash() {
        let block = generate_random_block();
        let hash = block.hash().unwrap();
        assert!(!hash.is_zero());
    }

    #[test]
    fn test_block_new() {
        let header = Header {
            previous_block_hash: [0; 32],
            tx_hash: hash::Hash { hash: [0; 32] },
            version: 1,
            height: 1,
            timestamp: 1,
            nonce: 1,
            difficulty: 1,
        };

        let transactions = vec![];
        let block = Block::new(header, transactions);

        assert_eq!(block.header.previous_block_hash, [0; 32]);
        assert_eq!(block.header.tx_hash.hash, [0; 32]);
        assert_eq!(block.header.version, 1);
        assert_eq!(block.header.height, 1);
        assert_eq!(block.header.timestamp, 1);
        assert_eq!(block.header.nonce, 1);
        assert_eq!(block.header.difficulty, 1);
        assert_eq!(block.transactions.len(), 0);
    }

    #[test]
    fn test_header_serialization() {
        let header = Header {
            previous_block_hash: [0; 32],
            tx_hash: hash::Hash { hash: [0; 32] },
            version: 1,
            height: 1,
            timestamp: 1,
            nonce: 1,
            difficulty: 1,
        };

        let bytes = header.to_bytes();
        let deserialized_header: Header =
            bincode::deserialize(&bytes).expect("Deserialization failed");

        assert_eq!(
            header.previous_block_hash,
            deserialized_header.previous_block_hash
        );
        assert_eq!(header.tx_hash, deserialized_header.tx_hash);
        assert_eq!(header.version, deserialized_header.version);
        assert_eq!(header.height, deserialized_header.height);
        assert_eq!(header.timestamp, deserialized_header.timestamp);
        assert_eq!(header.nonce, deserialized_header.nonce);
        assert_eq!(header.difficulty, deserialized_header.difficulty);
    }
}
