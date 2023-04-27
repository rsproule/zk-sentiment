#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::serde::to_vec;
use risc0_zkvm::sha::{Impl, Sha256};
use sentiment::{positivity, Sentiment};
use sentiment_core::{Input, Output};
risc0_zkvm::guest::entry!(main);

pub fn main() {
    let input: Input = env::read();
    let msg_hashed = Impl::hash_words(&to_vec(&input.message).unwrap());
    let analysis: Sentiment = positivity(input.message);
    let score = (analysis.score  * 100.0) as u32; // seems to freak out when i try to serialize a float. Just multiply by 100 and cast to u32.
    env::commit(&Output {
        score,
        message_hash: *msg_hashed,
    });
}
