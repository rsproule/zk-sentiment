use methods::{SENTIMENT_ELF, SENTIMENT_ID};
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::{Executor, ExecutorEnv};
use sentiment_core::{Input, Output};

fn main() {
    let msg = "wow risc zero is so cool";
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&msg).unwrap())
        .build();
    let mut exec = Executor::from_elf(env, SENTIMENT_ELF).unwrap();
    println!("host - run executor");
    let session = exec.run().unwrap();
    // Prove the session to produce a receipt.
    println!("host - prove session and build receipt");
    let receipt = session.prove().unwrap();
    let output: Output = from_slice(&receipt.journal).expect("Failed to get result");
    println!("{:?}", output);
    receipt
        .verify(SENTIMENT_ID)
        .expect("Proven code should verify");
}
