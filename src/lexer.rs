use crate::token::*;
use std::iter::Peekable;

/// Parse a numeric literal from a peekable iterator
fn get_literal<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> Token {
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

    Token::Num(
        lexeme
            .parse::<f64>()
            .expect(&format!("Could not parse '{}' as an f64!", lexeme)),
    )
}

// get identifier (function or variable) from the iterator
// no support for functions yet
fn get_ident<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> Token {
    let mut lexeme = String::new();

    while let Some(&c) = it.peek() {
        match c {
            'a'..='z' | 'A'..='Z' => {
                lexeme.push(c);
            }
            _ => break,
        }
    }

    Token::Var(lexeme)
}

/// The scanner/ lexer/ tokenizer
/// Converts a character stream into a vector of lexical tokens
pub fn tokenize(expr: &String) -> Vec<Token> {
    let mut tokens = Vec::with_capacity(expr.len()); // by setting capacity to num of chars in string we should get less reallocs

    let mut stream = expr.chars().peekable();

    let mut is_sub: bool = false;

    while let Some(&c) = stream.peek() {
        match c {
            // must be a numeric literal
            '0'..='9' => {
                tokens.push(get_literal(&mut stream));
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

                is_sub = false;

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
