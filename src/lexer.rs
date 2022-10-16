use crate::token::*;
use std::iter::Peekable;

//. Parse a numeric literal from a peekable iterator
fn get_literal<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> f32 {
    let mut lexeme = String::new();

    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' | '.' => {
                lexeme.push(c);
            },
            _ => break
        } 

        it.next();
    }

    lexeme.parse::<f32>().expect("Could not parse!")
}

pub fn tokenize(expr: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut stream = expr.chars().peekable();

    while let Some(&c) = stream.peek() {
        match c {
            // must be a numeric literal
            '0'..='9' => {
                tokens.push(Token::Num(get_literal(&mut stream)));
            }
            '(' => {
                tokens.push(Token::LP);
                stream.next();
            }
            ')' => {
                tokens.push(Token::RP);
                stream.next();
            }
            // Binary Operator
            '-' | '+' | '*' | '/' | '^' => {
                let op = Op::from_char(c).expect("Could not parse operator");
                tokens.push(Token::Bin(op));

                stream.next();
            }
            _ => {
                stream.next();
            }
        }
    }

    tokens
}
