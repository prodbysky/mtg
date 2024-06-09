use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
enum Token {
    Mut,
    TypeSpecifier(String),
    NumberLiteral(String),
    Equals,
    Semicolon,
    Symbol(String),
}

#[derive(Debug)]
struct InvalidTokenError;

impl FromStr for Token {
    type Err = InvalidTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_string();
        match s.as_str() {
            "mut" => Ok(Self::Mut),
            "=" => Ok(Self::Equals),
            ";" => Ok(Self::Semicolon),
            "i32" => Ok(Self::TypeSpecifier(s)),
            _ => {
                if let Ok(_) = s.parse::<i32>() {
                    Ok(Self::NumberLiteral(s))
                } else {
                    Ok(Self::Symbol(s))
                }
            }
        }
    }
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

fn tokenize(words: Vec<String>) -> Vec<Token> {
    return words.iter().map(|w| Token::from_str(w).unwrap()).collect();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Didn't provide source file");
        return;
    }

    let src = read_to_string(&args[1]).expect("Couldn't read provided source file");

    let tokens = tokenize(split_words(&src));

    println!("{:?}", tokens);
}
