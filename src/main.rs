mod ast;
mod tokenizer;

use std::fs::read_to_string;

use crate::ast::ASTParser;
use crate::tokenizer::Token;
use crate::tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Didn't provide source file");
        return;
    }

    let src = read_to_string(&args[1]).expect("Couldn't read provided source file");
    let tokenizer = Tokenizer::new(src);
    let parser = ASTParser::new(tokenizer).parse();
    println!("{:#?}", parser);
}
