mod basic;
use basic::BasicTokenizer;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the user has provided the required argument
    if args.len() < 2 {
        eprintln!("Usage: {} <text>", args[0]);
        std::process::exit(1);
    }

    let text = &args[1]; // Get the input text from the command-line arguments

    let mut tokenizer = BasicTokenizer::new();

    let vocab_size = 256 + 3; // 256 byte tokens plus 3 merges

    // Train the tokenizer on the given text with a specified number of merges
    tokenizer.train(text, vocab_size);

    // Encode a piece of text using the trained tokenizer
    let encoded = tokenizer.encode(text);
    println!("Encoded: {:?}", encoded);

    // Decode the encoded tokens back into a string
    let decoded = tokenizer.decode(&encoded);
    println!("Decoded: {:?}", decoded);

    // If you implement a save functionality
    // tokenizer.save("basic_tokenizer");
}
