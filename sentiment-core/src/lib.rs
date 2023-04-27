use risc0_zkvm::sha::Digest;
use sentiment::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Input {
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Output {
    pub score: u32,
    pub message_hash: Digest,
}
pub fn sent() {
    positivity("wow risc zero is so cool".to_owned());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sentiment() {
        let result = super::positivity("wow risc zero is so cool".to_owned());
        println!("result: {:?}", result.score);
    }
}
