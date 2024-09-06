const CHAIN_MNEMONIC: &str = "velvet echo quill jungle nimbus crescent whisk anchor harbor tangle mosaic horizon";

use crate::core::header_list::HeaderList;
use crate::core::storage::Storage;
use crate::error::{Result, MarvinError};
use crate::proto;
use crate::utils::log::make_json_logger;
use crate::types;

pub struct Blockchain {
    pub headers: HeaderList,
    pub store: Box<dyn Storage>,
    pub logger: slog::Logger,
}

impl Blockchain {
    pub fn new(store: Box<dyn Storage>) -> Self {
        let mut bc = Blockchain {
            headers: HeaderList::new(),
            store,
            logger: make_json_logger(),
        };

        // Create the genesis block
        let genesis_block = Blockchain::create_genesis_block().unwrap();

        // Add the genesis block to the blockchain
        bc.add_block_without_validation(genesis_block).unwrap();

        bc
    }

    // Adds a block to the blockchain
    pub fn add_block(&mut self, block: proto::Block) -> Result<()> {
        // Validate the block before adding to the blockchain
        self.validate_block(&block)?;

        self.add_block_without_validation(block)
    }

    // Adds a block to the blockchain without validation (private method)
    fn add_block_without_validation(&mut self, block: proto::Block) -> Result<()> {
        if let Some(header) = block.header.as_ref() {
            // Add the header to the header list
            self.headers.add(header.clone());
        } else {
            return Err(MarvinError::General(String::from("Block header is missing")));
        }

        // TODO: Log block added to the blockchain
        info!(self.logger, "Block added to the blockchain"; 
            "height" => self.height(), 
            "hash" => hex::encode(crate::types::block::hash_block(&block))
        );

        // Store the block in the storage
        self.store.put(&block)?;

        Ok(())
    }

    // Creates the genesis block of the blockchain
    pub fn create_genesis_block() -> Result<proto::Block> {
        let mut private_key = crate::crypto::keys::get_private_key_from_mnemonic(CHAIN_MNEMONIC)?;

        // Create the genesis block
        let mut block = proto::Block::default();
        let mut header = proto::Header::default();
        header.height = 0;
        header.version = 1;
        header.timestamp = Blockchain::get_current_timestamp_as_unix_nano() as i64;
        header.prev_block_hash = vec![0; 32];

        block.header = Some(header);

        // Signs the block
        crate::types::block::sign_block(&mut private_key, &mut block)?;

        Ok(block)
    }

    // Returns the current timestamp as a Unix timestamp in nanoseconds
    fn get_current_timestamp_as_unix_nano() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        since_the_epoch.as_nanos() as u64
    }

    // Returns the height of the blockchain
    pub fn height(&self) -> usize {
        // [0, 1, 2 ,3] => 4 len
	    // [0, 1, 2 ,3] => 3 height
        self.headers.height() as usize
    }

    // Checks if the blockchain has a block at a given height
    pub fn has_block(&self, height: usize) -> bool {
        return height <= self.height();
    }

    // Checks if a block is valid to be added to the blockchain
    pub fn validate_block(&self, block: &proto::Block) -> Result<()> {
        // Check if the block is already in the blockchain
        if self.has_block(block.header.as_ref().unwrap().height as usize) {
            return Err(MarvinError::General(
                String::from(format!("Block already exists at height {}", block.header.as_ref().unwrap().height)))
            );
        }

        // Check if the block height is the next height in the blockchain
        if block.header.as_ref().unwrap().height as usize != self.height() + 1 {
            return Err(MarvinError::General(
                String::from(format!("Block height is not the next height in the blockchain. Expected height: {}", self.height() + 1)))
            );
        }

        // Check if the block is valid
        crate::types::block::verify_block(block).unwrap();

        // Retrieve the last header in the blockchain, calculate the hash of the last block and compare it with the previous hash in the new block
        let last_header = self.headers.last().unwrap();
        let last_hash = crate::types::block::hash_header(last_header);
        // Check if the previous hash in the new block is the hash of the last block
        if last_hash != block.header.as_ref().unwrap().prev_block_hash {
            return Err(MarvinError::General(
                String::from("Previous hash in the new block is not the hash of the last block"))
            );
        }
    
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::core::storage::MemoryStore;

    #[test]
    fn test_new_blockchain() {
        let store = Box::new(MemoryStore::new());
        let mut blockchain = Blockchain::new(store);

        assert_eq!(blockchain.height(), 0);
        assert_eq!(blockchain.has_block(0), true);
    }

    #[test]
    fn test_add_block() {
        let store = Box::new(MemoryStore::new());
        let mut blockchain = Blockchain::new(store);

        let num_blocks = 10;
        for i in 0..num_blocks {
            let prev_block_hash = crate::types::block::hash_header(blockchain.headers.last().unwrap());
            let block = generate_random_block((i+1) as i64, prev_block_hash);
            let result = blockchain.add_block(block);

            assert_eq!(result.is_ok(), true);
        }

        assert_eq!(blockchain.height() as i64, num_blocks);

        let existing_block = generate_random_block(1, vec![0; 32]);
        let result = blockchain.add_block(existing_block);
        assert_eq!(result.is_err(), true);
    }

    fn generate_random_block(height: i64, prev_block_hash: Vec<u8>) -> proto::Block {
        let mnemonic = "all wild paddle pride wheat menu task funny sign profit blouse hockey";
        let mut private_key = crate::crypto::keys::get_private_key_from_mnemonic(mnemonic).unwrap();

        let mut block = proto::Block::default();
        let mut header = proto::Header::default();
        header.height = height as u64;
        header.version = 1;
        header.timestamp = Blockchain::get_current_timestamp_as_unix_nano() as i64;
        header.prev_block_hash = prev_block_hash;
        header.nonce = 1;
        header.difficulty = 1;


        block.header = Some(header);

        // Signs the block
        crate::types::block::sign_block(&mut private_key, &mut block).unwrap();

        block
    }
}