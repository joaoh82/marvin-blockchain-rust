use crate::proto;

pub struct HeaderList {
    pub headers: Vec<proto::Header>,
}

impl HeaderList {
    pub fn new() -> Self {
        HeaderList {
            headers: Vec::new(),
        }
    }

    /// Add a header to the list
    pub fn add(&mut self, h: proto::Header) {
        self.headers.push(h);
    }

    /// Get a header from the list given an index. The index is 0-based and is also the height of the header.
    pub fn get(&self, index: usize) -> Option<&proto::Header> {
        self.headers.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.headers.is_empty()
    }

    /// Returns the last header in the list. The last header can be used to get the hash of the last block and create a new block.
    pub fn last(&self) -> Option<&proto::Header> {
        self.headers.last()
    }

    // Returns the height of the last header. The height is the index of the last header.
    pub fn height(&self) -> i64 {
        self.headers.len() as i64 - 1
    }

    /// Returns the length of the list. len() - 1 is the height of the last header.
    pub fn len(&self) -> i64 {
        self.headers.len() as i64
    }
}