use crate::token::*;
use std::iter::Peekable;

/// Parse a numeric literal from a peekable iterator
fn get_literal<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> f64 {
    let mut lexeme = String::new();

    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' | '.' => {
                lexeme.push(c);
            }
            _ => break,
        }

        it.next();
    }

    lexeme
        .parse::<f64>()
        .expect(&format!("Could not parse '{}' as an f64!", lexeme))
}

// get identifier (function or variable) from the iterator
fn get_ident<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> Token {
    todo!()
}

pub fn tokenize(expr: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut stream = expr.chars().peekable();

    let mut is_sub: bool = false;

    while let Some(&c) = stream.peek() {
        match c {
            // must be a numeric literal
            '0'..='9' => {
                tokens.push(Token::Num(get_literal(&mut stream)));
                is_sub = true; // next - will be a subtraction
            }
            'a'..='z' | 'A'..='Z' => {
                tokens.push(get_ident(&mut stream));
                is_sub = true;
            }
            '(' => {
                tokens.push(Token::LP);
                stream.next();
            }
            ')' => {
                tokens.push(Token::RP);
                stream.next();
            }
            // - should be handled seperate from other operators
            // can be a binary operator
            // or a unary one (negation)
            '-' => {
                if is_sub {
                    tokens.push(Token::Bin(Op::SUB));
                    is_sub = false;
                } else {
                    tokens.push(Token::Una(Op::NEG));
                }

                stream.next();
            }
            // Binary Operator
            '+' | '*' | '/' | '^' => {
                let op = Op::from_char(c).expect("Could not parse operator");
                tokens.push(Token::Bin(op));

                stream.next();
            }
            '!' => {
                tokens.push(Token::Una(Op::FAC));
                stream.next();
            }
            _ => {
                stream.next();
            }
        }
    }

    tokens
}
