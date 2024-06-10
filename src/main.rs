use crate::tokenizer::Tokenizer;
use std::fs::read_to_string;
mod tokenizer;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Didn't provide source file");
        return;
    }

    let src = read_to_string(&args[1]).expect("Couldn't read provided source file");
    let tokenizer = Tokenizer::new(src);

    for token in tokenizer {
        println!("{:?}", token);
    }
}
