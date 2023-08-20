use sha2::{Digest, Sha256};

/// Uses SHA256 for hashing
/// recives bytes `Vec<u8>`
#[derive(Clone)]
pub struct HashBuilder<'a> {
    /// Target data
    source: Option<&'a Vec<u8>>,

    /// Final Generated Token
    result: Option<String>,
}

impl<'a> HashBuilder<'a> {
    /// Creates a new Token object
    pub fn new() -> Self {
        Self {
            source: None,
            result: None,
        }
    }

    /// Change the source
    pub fn set_source(mut self, new_source: &'a Vec<u8>) -> Self {
        self.source = Some(new_source);

        self
    }

    /// Generates the final hash and set to result
    pub fn generate(mut self) -> Self {
        let mut hasher = Sha256::new();

        match self.source {
            Some(s) => hasher.update(s),

            None => hasher.update(vec![]),
        }

        self.result = Some(format!("{:x}", hasher.finalize()));

        self
    }

    /// Returns the copy of result
    pub fn get_result(&self) -> Option<String> {
        self.result.clone()
    }
}
