use risc0_zkvm::sha::Digest;
use sentiment::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Input {
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Output {
    pub positive_score: u32,
    pub negative_score: u32,
    pub score: u32,
    pub positive_comparative: u32,
    pub negative_comparative: u32,
    pub comparative: u32,
    pub positive_words: Vec<String>,
    pub negative_words: Vec<String>,
    pub message_hash: Digest,
}

pub fn parse_result(analysis: Analysis, message_hash: Digest) -> Output {
    // r0 env::commit seems to freak out when i try to serialize a float. Just multiply by 100 and cast to u32.
    let pos_score = (analysis.positive.score * 100.0) as u32;
    let neg_score = (analysis.negative.score * 100.0) as u32;
    let score = (analysis.score * 100.0) as u32;
    let pos_comparative = (analysis.positive.comparative * 100.0) as u32;
    let neg_comparative = (analysis.negative.comparative * 100.0) as u32;
    let comparative = (analysis.comparative * 100.0) as u32;

    Output {
        positive_score: pos_score,
        negative_score: neg_score,
        score,
        positive_comparative: pos_comparative,
        negative_comparative: neg_comparative,
        positive_words: analysis.positive.words,
        negative_words: analysis.negative.words,
        comparative,
        message_hash,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sentiment() {
        let message = "wow risc zero is so cool. so easy to use and exciting! Writing circuits by hand is terrible!".to_owned();
        let result = super::analyze(message.clone());
        let output = super::parse_result(result, [0; 32].into());
        println!("result: {output:?}");
        assert_eq!(output.positive_score, 900);
        assert_eq!(output.negative_score, 300);
        assert_eq!(output.score, 600);
        assert_eq!(output.positive_comparative, 47);
        assert_eq!(output.negative_comparative, 15);
        assert_eq!(output.comparative, 31);
    }
}
