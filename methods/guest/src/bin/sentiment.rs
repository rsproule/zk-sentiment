#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::serde::to_vec;
use risc0_zkvm::sha::{Impl, Sha256};
use sentiment::{analyze, Analysis};
use sentiment_core::{Input, parse_result};
risc0_zkvm::guest::entry!(main);

pub fn main() {
    let input: Input = env::read();
    // let msg_hashed = Impl::hash_words(&to_vec(&input.message).unwrap());
    let analysis: Analysis = analyze(input.message);
    let output = parse_result(analysis);
    // let output = parse_result(analysis, *msg_hashed);
    env::commit(&output);
}
