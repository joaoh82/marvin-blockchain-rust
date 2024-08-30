use crate::crypto::keys::{PrivateKey, PublicKey, SignatureWrapper};
use crate::crypto::keys::{SIGNATURE_SIZE, PUBLIC_KEY_SIZE};
use crate::crypto::keys::*;
use crate::proto;

use prost;
use prost::Message;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

/// Serialize a transaction
pub fn serialize_transaction(t : proto::Transaction) -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    t.encode(&mut buf).map_err(|e| e.to_string())?;
    Ok(buf)
}

/// Deserialize a transaction
pub fn deserialize_transaction(data: &[u8]) -> Result<proto::Transaction, String> {
    proto::Transaction::decode(data).map_err(|e| e.to_string())
}

/// Hash a transaction
pub fn hash_transaction(t: &mut proto::Transaction) -> Vec<u8> {
    let data = serialize_transaction(t.clone()).unwrap();
    let mut hasher = Sha256::new();
    hasher.input(&data);

    let mut hash = [0; 32];
    hasher.result(&mut hash);

    t.hash = hash.to_vec();

    hash.to_vec()
}

/// Sign a transaction
pub fn sign_transaction(private_key: &mut PrivateKey, t: &mut proto::Transaction) -> Result<(SignatureWrapper), String> {
    let hash = hash_transaction(t);
    let signature = private_key.sign(&hash).map_err(|e| e.to_string())?;

    t.signature = signature.to_bytes().to_vec();
    t.hash = hash;

    Ok(signature)
}

/// Verify a transaction
pub fn verify_transaction(t: &mut proto::Transaction) -> Result<bool, String> {
    if t.signature.is_empty() {
        return Err("Transaction is not signed".to_string());
    }

    if t.signature.len() != SIGNATURE_SIZE {
        return Err("Invalid signature size".to_string());
    }

    let temp_sig = t.signature.clone();
    let temp_hash = t.hash.clone();

    t.signature = [0; 64].to_vec();
    t.hash = [0; 32].to_vec();

    let hash = hash_transaction(t);

    t.signature = temp_sig;
    t.hash = temp_hash;

    let signature = SignatureWrapper::from_bytes(&t.signature);
    let public_key = PublicKey::from_bytes(&t.from);
    
    let is_valid = signature.verify(&hash, &public_key);
    if !is_valid {
        return Err("Invalid signature".to_string());
    }

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{crypto::keys, proto};

    #[test]
    fn test_serialize_transaction() {
        let mnemonic_to = "all wild paddle pride wheat menu task funny sign profit blouse hockey";
        let private_key_to = keys::get_private_key_from_mnemonic(&mnemonic_to);
        let public_key_to = private_key_to.public_key();
        // let address_to = public_key_to.address();

        let mnemonic_from = "hello wild paddle pride wheat menu task funny sign profit blouse hockey";
        let private_key_from = keys::get_private_key_from_mnemonic(&mnemonic_from);
        let public_key_from = private_key_from.public_key();
        // let address_from = public_key_from.address();

        let tx = proto::Transaction {
            from: public_key_from.to_bytes().to_vec(),
            to: public_key_to.to_bytes().to_vec(),
            value: 1000,
            data: b"Transaction data".to_vec(),
            signature: [0; 64].to_vec(),
            nonce: 123,
            hash: [0; 32].to_vec(),
        };

        let data = serialize_transaction(tx.clone()).unwrap();
        let tx2 = deserialize_transaction(&data).unwrap();

        assert_eq!(tx, tx2);
    }

    #[test]
    fn test_sign_transaction() {
        let mut private_key_from = keys::generate_private_key();
        let public_key_from = private_key_from.public_key();
        let mut private_key_to = keys::generate_private_key();
        let public_key_to = private_key_to.public_key();

        let mut tx = proto::Transaction {
            from: public_key_from.to_bytes().to_vec(),
            to: public_key_to.to_bytes().to_vec(),
            value: 1000,
            data: b"Transaction data".to_vec(),
            signature: [0; 64].to_vec(),
            nonce: 123,
            hash: [0; 32].to_vec(),
        }; 

        let signature = sign_transaction(&mut private_key_from, &mut tx).unwrap();
        assert_eq!(signature.to_bytes().len(), SIGNATURE_SIZE);

        let is_valid = verify_transaction(&mut tx).unwrap();
        assert!(is_valid);
    }
}