use crate::error::{Result, MarvinError};
use crate::proto;
use crate::types;

use core::hash;
use std::collections::HashMap;

/// Mempool struct is a pool of transactions that are not yet included in a block
pub struct Mempool {
    pub transactions: HashMap<String, proto::Transaction>,
}

impl Mempool {
    /// Create a new Mempool
    pub fn new() -> Self {
        Mempool {
            transactions: HashMap::new(),
        }
    }

    /// Flush the mempool by removing all transactions
    pub fn flush(&mut self) {
        self.transactions.clear();
    }

    /// Get the number of transactions in the mempool
    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    /// Check if a transaction is in the mempool
    pub fn has(&self, tx: &mut proto::Transaction) -> bool {
        let hash = types::transaction::hash_transaction(tx);
        let hash_str = hex::encode(hash);

        self.transactions.contains_key(&hash_str)
    }

    /// Add a transaction to the mempool
    pub fn add(&mut self, tx: &mut proto::Transaction) -> Result<()> {
        if self.has(tx) {
            return Err(MarvinError::General(String::from("Transaction already exists in the mempool")));
        }

        let hash = types::transaction::hash_transaction(tx);
        let hash_str = hex::encode(hash);

        self.transactions.insert(hash_str, tx.clone());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys;
    use crate::proto;

    #[test]
    fn test_new_mempool() {
        let mempool = Mempool::new();

        assert_eq!(mempool.len(), 0);
    }

    #[test]
    fn test_add_transaction() {
        let mut mempool = Mempool::new();

        let mnemonic = "all wild paddle pride wheat menu task funny sign profit blouse hockey";
        let mut private_key = keys::get_private_key_from_mnemonic(&mnemonic).unwrap();
        let public_key = private_key.public_key();

        let mut tx = proto::Transaction {
            from: public_key.to_bytes().to_vec(),
            to: public_key.to_bytes().to_vec(),
            value: 1000,
            data: b"Transaction data".to_vec(),
            signature: [0; 64].to_vec(),
            nonce: 123,
            hash: [0; 32].to_vec(),
        };
        let _ = types::transaction::sign_transaction(&mut private_key, &mut tx).unwrap();

        mempool.add(&mut tx).unwrap();
        assert_eq!(mempool.len(), 1);

        mempool.flush();
        assert_eq!(mempool.len(), 0);
    }
}