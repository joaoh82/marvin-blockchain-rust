use crate::error::{Result, MarvinError};

use crate::proto;


// Store is a trait that defines the methods that a store must implement.
pub trait Storage {
    fn put(&mut self, block: &proto::Block) -> Result<()>;
    fn get(&self, hash: String) -> Result<proto::Block>; 
}

pub struct MemoryStore {}

impl MemoryStore {
    pub fn new() -> Self {
        MemoryStore {}
    }
}

impl Storage for MemoryStore {
    fn put(&mut self, block: &proto::Block) -> Result<()> {
        Ok(())
    }

    fn get(&self, hash: String) -> Result<proto::Block> {
        println!("MemoryStore::get: {}", hash);

        // empty block
        Ok(proto::Block::default())

        // Err(MarvinError::NotImplemented("MemoryStore::get".to_string()))
    }
}