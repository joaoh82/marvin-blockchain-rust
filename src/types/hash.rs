use bincode;
use serde::{Deserialize, Serialize};

const HASH_SIZE: usize = 32;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Hash {
    pub hash: [u8; HASH_SIZE],
}

impl Hash {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Serialization failed")
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.hash)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).expect("Deserialization failed")
    }

    pub fn is_zero(&self) -> bool {
        for byte in self.hash.iter() {
            if *byte != 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_is_zero() {
        let hash = Hash {
            hash: [0; HASH_SIZE],
        };
        assert!(hash.is_zero());

        let hash = Hash {
            hash: [1; HASH_SIZE],
        };
        assert!(!hash.is_zero());
    }
}
