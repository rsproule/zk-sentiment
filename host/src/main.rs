use colored::Colorize;
use methods::{SENTIMENT_ELF, SENTIMENT_ID};
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::{Executor, ExecutorEnv};
use sentiment_core::Output;
use std::time::Instant;

fn main() {
    // TODO: read this message from the cli args
    let msg = "Wow risc zero is so cool. So easy to use and exciting! Writing circuits by hand is terrible!";
    println!("Getting sentiment analysis for message: \"{}\"", &msg);
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
    println!("Sentiment output: {output:?}");
    println!("Proof: {:?}", receipt.journal);
    display_result(msg.to_owned(), output);
}

pub fn display_result(message: String, output: Output) {
    for word in message.split_whitespace() {
        let word_remove_non_alpha_chars = word.replace(|c: char| !c.is_alphabetic(), "");
        let mut polarizing = false;
        if output
            .positive_words
            .contains(&word_remove_non_alpha_chars.to_owned())
        {
            print!("{} ", word.green());
            polarizing = true;
        }
        if output
            .negative_words
            .contains(&word_remove_non_alpha_chars.to_owned())
        {
            println!("{} ", word.red());
            polarizing = true;
        }
        if !polarizing {
            print!("{word} ");
        }
    }
}
