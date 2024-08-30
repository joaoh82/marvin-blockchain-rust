use core::hash;

use crate::crypto::keys::{PrivateKey, PublicKey, SignatureWrapper};
use crate::crypto::keys::{SIGNATURE_SIZE, PUBLIC_KEY_SIZE};
use crate::crypto::keys::*;
use crate::proto;

use prost;
use prost::Message;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use super::transaction::hash_transaction;


/// Serialize a header
pub fn serialize_header(h : proto::Header) -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    h.encode(&mut buf).map_err(|e| e.to_string())?;
    Ok(buf)
}

/// Deserialize a header
pub fn deserialize_header(data: &[u8]) -> Result<proto::Header, String> {
    proto::Header::decode(data).map_err(|e| e.to_string())
}

/// Serialize a block
pub fn serialize_block(b : proto::Block) -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    b.encode(&mut buf).map_err(|e| e.to_string())?;
    Ok(buf)
}

/// Deserialize a block
pub fn deserialize_block(data: &[u8]) -> Result<proto::Block, String> {
    proto::Block::decode(data).map_err(|e| e.to_string())
}

/// Sign a block
pub fn sign_block(private_key: &mut PrivateKey, b: &mut proto::Block) -> Result<(SignatureWrapper), String> {
    let hash = hash_block(b);
    let signature = private_key.sign(&hash).map_err(|e| e.to_string())?;

    b.signature = signature.to_bytes().to_vec();
    b.public_key = private_key.public_key().to_bytes().to_vec();
    b.hash = hash;

    Ok(signature)
}

/// Verify a block
pub fn verify_block(b: &proto::Block) -> Result<bool, String> {
    if b.signature.is_empty() || b.public_key.is_empty() {
        return Err("Block is not signed".to_string());
    }

    if b.signature.len() != SIGNATURE_SIZE {
        return Err("Invalid signature size".to_string());
    }

    if b.public_key.len() != PUBLIC_KEY_SIZE {
        return Err("Invalid public key size".to_string());
    }

    let signature = SignatureWrapper::from_bytes(&b.signature);
    let public_key = PublicKey::from_bytes(&b.public_key);
    let hash = hash_block(b);
    let is_valid = signature.verify(&hash, &public_key);

    Ok(is_valid)
}

/// Calculate the hash of a header
pub fn hash_header(h: &proto::Header) -> Vec<u8> {
    let data = serialize_header(h.clone()).unwrap();
    let mut hasher = Sha256::new();
    hasher.input(&data);

    let mut hash = [0; 32]; 
    hasher.result(&mut hash);

    hash.to_vec()
}

/// Calculate the hash of a block
pub fn hash_block(b: &proto::Block) -> Vec<u8> {
    hash_header(b.header.as_ref().unwrap())
}

/// Add a transaction to a block
pub fn add_transaction(b: &mut proto::Block, tx: proto::Transaction) {
    b.transactions.push(tx);
    let hash = calculate_tx_hash(&b.transactions);
    
    b.header.as_mut().unwrap().tx_hash = hash;
}

/// Calculate the hash of a list of transactions
pub fn calculate_tx_hash(txs : &Vec<proto::Transaction>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    for tx in txs {
        let data = hash_transaction(tx);
        hasher.input(&data);
    }

    let mut hash = [0; 32];
    hasher.result(&mut hash);

    hash.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{crypto::keys, proto};
    // use prost;
    // use prost::{Enumeration, Message};

    #[test]
    fn test_serialize_header() {
        let header = proto::Header {
            prev_block_hash: [0; 32].to_vec(),
            tx_hash: [0; 32].to_vec(),
            version: 1,
            height: 1,
            timestamp: 1627483623,
            nonce: 12345,
            difficulty: 10,
        };

        let data = serialize_header(header.clone()).unwrap();
        let h2 = deserialize_header(&data).unwrap();

        assert_eq!(header, h2);
    }

    #[test]
    fn test_serialize_header_fail() {
        let header = proto::Header {
            prev_block_hash: [0; 32].to_vec(),
            tx_hash: [0; 32].to_vec(),
            version: 1,
            height: 1,
            timestamp: 1627483623,
            nonce: 12345,
            difficulty: 10,
        };

        let data = serialize_header(header.clone()).unwrap();
        let h2 = deserialize_header(&data[1..]);

        assert!(h2.is_err());
    }

    #[test]
    fn test_serialize_block() {
        let header = proto::Header {
            prev_block_hash: [0; 32].to_vec(),
            tx_hash: [0; 32].to_vec(),
            version: 1,
            height: 1,
            timestamp: 1627483623,
            nonce: 12345,
            difficulty: 10,
        };

        let tx = proto::Transaction {
            from: [0; 32].to_vec(),
            to: [0; 32].to_vec(),
            value: 1000,
            data: b"Transaction data".to_vec(),
            signature: [0; 64].to_vec(),
            nonce: 123,
            hash: [0; 32].to_vec(),
        };

        let block = proto::Block {
            header: Some(header),
            transactions: vec![tx],
            public_key: [0; 32].to_vec(),
            signature: vec![],
            hash: vec![],
        };

        let data = serialize_block(block.clone()).unwrap();
        let b2 = deserialize_block(&data).unwrap();

        assert_eq!(block, b2);
    }

    #[test]
    fn test_sign_block() {
        let mnemonic = "all wild paddle pride wheat menu task funny sign profit blouse hockey";
	    let address_string = "e15af3cd7d9c09ebaf20d1f97ea396c218b66037";

        let mut private_key = keys::get_private_key_from_mnemonic(mnemonic);
        let public_key = private_key.public_key();
        let address = public_key.address();
        assert_eq!(address.to_string(), address_string);

        // Create an instance of Header
        let header = proto::Header {
            prev_block_hash: [0; 32].to_vec(),
            tx_hash: [0; 32].to_vec(),
            version: 1,
            height: 1,
            timestamp: 1627483623,
            nonce: 12345,
            difficulty: 10,
        };

        // Create an instance of Block
        let mut block = proto::Block {
            header: Some(header),
            transactions: vec![],
            public_key: public_key.to_bytes().to_vec(),
            signature: vec![],
            hash: vec![],
        };

        let public_key_to = keys::generate_private_key().public_key();

        let tx = proto::Transaction {
            from: public_key.to_bytes().to_vec(),
            to: public_key_to.to_bytes().to_vec(),
            value: 1000,
            data: b"Transaction data".to_vec(),
            signature: [0; 64].to_vec(),
            nonce: 123,
            hash: [0; 32].to_vec(),
        };

        add_transaction(&mut block, tx);

        assert_eq!(block.transactions.len(), 1);

        let signature = sign_block(&mut private_key, &mut block).unwrap();

        assert_eq!(signature.to_bytes().len(), SIGNATURE_SIZE);

        let is_valid = verify_block(&block).unwrap();
        assert!(is_valid);

    }
    
}