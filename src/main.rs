use std::fs::read_to_string;

enum Token<'a> {
    Mut,
    TypeSpecifier(&'a str),
    NumberLiteral(i32),
    Equals,
    Semicolon,
    Symbol(&'a str),
}

fn split_words(source: &str) -> Vec<String> {
    let mut words = vec![];
    let mut iter = source.chars().peekable();
    while let Some(mut c) = iter.next() {
        if c.is_whitespace() {
            continue;
        }

        let mut buf = String::new();
        if c == ';' {
            words.push(";".to_string());
            continue;
        }

        buf.push(c);
        c = iter.next().unwrap();

        while !c.is_whitespace() {
            if c == ';' {
                words.push(buf.clone());
                words.push(c.to_string());
                c = iter.next().unwrap();
                buf.clear();
                continue;
            }
            buf.push(c);
            c = iter.next().unwrap();
        }

        if !buf.is_empty() {
            words.push(buf);
        }
    }

    words
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Didn't provide source file");
        return;
    }

    let src = read_to_string(&args[1]).expect("Couldn't read provided source file");

    let tokens = split_words(&src);

    println!("{:?}", tokens);
}
