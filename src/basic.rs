use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::str;

/// A minimal Byte Pair Encoding tokenizer.
pub struct BasicTokenizer {
    merges: HashMap<(usize, usize), usize>,
    vocab: HashMap<usize, Vec<u8>>,
}

impl BasicTokenizer {
    /// Constructs a new `BasicTokenizer`.
    pub fn new() -> Self {
        BasicTokenizer {
            merges: HashMap::new(),
            vocab: (0..=255).map(|i| (i as usize, vec![i])).collect(),
        }
    }

    /// Trains the tokenizer on the given text to generate a vocabulary.
    pub fn train(&mut self, text: &str, vocab_size: usize) {
        assert!(vocab_size >= 256, "Vocab size must be at least 256.");
        let num_merges: usize = vocab_size - 256;

        let text_bytes = text.as_bytes();
        let mut ids: Vec<usize> = text_bytes.iter().map(|&b| b as usize).collect();

        for i in 0..num_merges {
            let stats = get_stats(&ids);
            if let Some(&pair) = stats
                .iter()
                .max_by_key(|&(_, &count)| count)
                .map(|(pair, _)| pair)
            {
                let idx = 256 + i;
                ids = merge(&ids, pair, idx);
                self.merges.insert((pair.0 as usize, pair.1 as usize), idx);

                let token1 = self.vocab.get(&(pair.0 as usize)).unwrap();
                let token2 = self.vocab.get(&(pair.1 as usize)).unwrap();
                let new_token = [token1.clone(), token2.clone()].concat();

                self.vocab.insert(idx, new_token);
            }
        }
    }

    // fn save(&self, file_prefix: &str) {
    //     let model_filename = format!("{}.model", file_prefix);
    //     let vocab_filename = format!("{}.vocab", file_prefix);

    //     // Save the model file
    //     let model_file = File::create(&model_filename).expect("Unable to create model file");
    //     let mut model_writer = BufWriter::new(model_file);

    //     writeln!(model_writer, "minbpe v1").expect("Unable to write to model file");
    //     writeln!(model_writer, "{}", self.pattern).expect("Unable to write pattern to model file");
    //     for ((idx1, idx2), idx) in &self.merges {
    //         writeln!(model_writer, "{} {}", idx1, idx2)
    //             .expect("Unable to write merges to model file");
    //     }

    //     // Save the vocab file
    //     let vocab_file = File::create(&vocab_filename).expect("Unable to create vocab file");
    //     let mut vocab_writer = BufWriter::new(vocab_file);

    //     for (idx, token) in &self.vocab {
    //         let token_str = render_token(token);
    //         writeln!(vocab_writer, "[{}] {}", token_str, idx)
    //             .expect("Unable to write vocab to file");
    //     }
    // }

    pub fn encode(&self, text: &str) -> Vec<usize> {
        let mut ids: Vec<usize> = text.as_bytes().iter().map(|&b| b as usize).collect();

        while ids.len() >= 2 {
            let stats: HashMap<(usize, usize), usize> = get_stats(&ids);
            if let Some(&pair) = stats
                .keys()
                .min_by_key(|&&pair| self.merges.get(&pair).unwrap_or(&usize::MAX))
            {
                if let Some(&idx) = self.merges.get(&pair) {
                    ids = merge(&ids, pair, idx);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        ids
    }

    pub fn decode(&self, ids: &[usize]) -> String {
        let bytes: Vec<u8> = ids
            .iter()
            .flat_map(|&id| self.vocab.get(&id).unwrap().clone())
            .collect();
        String::from_utf8(bytes).unwrap_or_default()
    }
}

fn get_stats(ids: &[usize]) -> HashMap<(usize, usize), usize> {
    let mut stats = HashMap::new();
    for window in ids.windows(2) {
        if let [a, b] = *window {
            *stats.entry((a as usize, b as usize)).or_insert(0) += 1;
        }
    }
    stats
}

fn merge(ids: &[usize], pair: (usize, usize), new_id: usize) -> Vec<usize> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < ids.len() {
        if i < ids.len() - 1 && ids[i] == pair.0 as usize && ids[i + 1] == pair.1 as usize {
            result.push(new_id);
            i += 2;
        } else {
            result.push(ids[i]);
            i += 1;
        }
    }
    result
}
