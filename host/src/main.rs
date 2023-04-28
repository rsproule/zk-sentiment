use methods::{SENTIMENT_ELF, SENTIMENT_ID};
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::{Executor, ExecutorEnv};
use sentiment_core::Output;
use std::time::Instant;

fn main() {
    // TODO: read this message from the cli args
    let msg = "Wow! Risc Zero is so cool.";
    println!("Getting sentiment analysis for message: \"{msg}\"");
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&msg).unwrap())
        .build();
    let mut exec = Executor::from_elf(env, SENTIMENT_ELF).unwrap();
    let session = exec.run().unwrap();
    // Prove the session to produce a receipt.
    let now = Instant::now();
    let receipt = session.prove().unwrap();
    let output: Output = from_slice(&receipt.journal).expect("Failed to get result");
    println!(
        "Done: total proof generation time: {:?} seconds",
        now.elapsed().as_secs()
    );
    receipt
        .verify(SENTIMENT_ID)
        .expect("Proven code should verify");
    println!("Proof receipt: {receipt:?}");
    println!("Sentiment output: {output:?}");
}
