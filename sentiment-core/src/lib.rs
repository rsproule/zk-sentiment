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
    // pub message_hash: Digest,
}

pub fn parse_result(analysis: Analysis) -> Output {
    // seems to freak out when i try to serialize a float. Just multiply by 100 and cast to u32.
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
        negative_words: analysis.positive.words,
        positive_words: analysis.negative.words,
        comparative,
        // message_hash,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sentiment() {
        let result = super::analyze("wow risc zero is so cool. If only there was more people using it! Im so mad about that.".to_owned());
        let output = super::parse_result(result);
        println!("result: {output:?}");
    }
}
