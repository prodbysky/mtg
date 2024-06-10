use crate::{Token, Tokenizer};
use std::iter::Peekable;
use std::mem::discriminant;

#[derive(Debug)]
pub struct ASTParser {
    tokenizer: Peekable<Tokenizer>,
}

#[derive(Debug)]
pub enum ASTNode {
    AST {
        token: Token,
        children: Vec<ASTNode>,
    },
    Operator(Token),
    BinaryOperation {
        left: Box<ASTNode>,
        op: Box<ASTNode>,
        right: Box<ASTNode>,
    },

    UnaryOperation {
        op: Box<ASTNode>,
        expression: Box<ASTNode>,
    },

    NumberLiteral {
        raw: String,
        parsed: i32,
    },
}

impl ASTParser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self {
            tokenizer: tokenizer.peekable(),
        }
    }

    fn consume(&mut self, token: Token) {
        if discriminant(self.tokenizer.peek().unwrap()) == discriminant(&token) {
            self.tokenizer.next();
        }
    }

    fn factor(&mut self) -> Option<ASTNode> {
        if let Some(Token::Plus) = self.tokenizer.peek() {
            self.consume(Token::Plus);
            return Some(ASTNode::UnaryOperation {
                op: Box::new(ASTNode::Operator(Token::Plus)),
                expression: Box::new(self.factor()?),
            });
        } else if let Some(Token::Minus) = self.tokenizer.peek() {
            self.consume(Token::Minus);
            return Some(ASTNode::UnaryOperation {
                op: Box::new(ASTNode::Operator(Token::Minus)),
                expression: Box::new(self.factor()?),
            });
        } else if let Some(Token::NumberLiteral(value)) = self.tokenizer.peek() {
            let val = value.clone();
            self.consume(Token::NumberLiteral("BLANK".to_string()));
            return Some(ASTNode::NumberLiteral {
                raw: val.to_string(),
                parsed: val.parse().unwrap(),
            });
        } else if let Some(Token::LParen) = self.tokenizer.peek() {
            self.consume(Token::LParen);
            let expr = self.expression();
            self.consume(Token::RParen);
            return Some(expr?);
        }

        None
    }

    fn term(&mut self) -> Option<ASTNode> {
        let mut node = self.factor();

        while self.tokenizer.peek() == Some(&Token::Star)
            || self.tokenizer.peek() == Some(&Token::FSlash)
        {
            match self.tokenizer.peek() {
                Some(Token::Star) => {
                    self.consume(Token::Star);
                    let children = vec![node?, self.factor()?];
                    node = Some(ASTNode::AST {
                        token: Token::Star,
                        children,
                    });
                }
                Some(Token::FSlash) => {
                    self.consume(Token::FSlash);
                    let children = vec![node?, self.factor()?];
                    node = Some(ASTNode::AST {
                        token: Token::FSlash,
                        children,
                    })
                }
                _ => {}
            }
        }

        node
    }

    fn expression(&mut self) -> Option<ASTNode> {
        let mut node = self.term();

        while self.tokenizer.peek() == Some(&Token::Plus)
            || self.tokenizer.peek() == Some(&Token::Minus)
        {
            match self.tokenizer.peek() {
                Some(Token::Plus) => {
                    self.consume(Token::Plus);
                    let children = vec![node?, self.term()?];
                    node = Some(ASTNode::AST {
                        token: Token::Plus,
                        children,
                    });
                }
                Some(Token::Minus) => {
                    self.consume(Token::Minus);
                    let children = vec![node?, self.term()?];
                    node = Some(ASTNode::AST {
                        token: Token::Minus,
                        children,
                    });
                }
                _ => return None,
            }
        }

        node
    }

    pub fn parse(&mut self) -> Option<Vec<ASTNode>> {
        let mut program = vec![];
        while let Some(_) = self.tokenizer.peek() {
            program.push(self.expression()?);
        }

        Some(program)
    }
}
