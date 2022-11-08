use crate::lexer::*;
use crate::token::*;
use std::collections::VecDeque;

/// An implementation of the Shunting Yard Algorithm for parsing infix expressions into an AST
#[derive(Clone)]
struct AstNode {
    tok: Token,
    lhs: Option<Box<AstNode>>,
    rhs: Option<Box<AstNode>>,
}

impl AstNode {
    #[inline]
    pub fn new(tok: Token, lhs: Option<Box<AstNode>>, rhs: Option<Box<AstNode>>) -> Self {
        AstNode {
            tok: tok,
            lhs: lhs,
            rhs: rhs,
        }
    }
}

pub struct AST {
    root: Option<Box<AstNode>>,
}

impl AST {
    #[inline]
    /// Create an empty AST
    pub fn new() -> Self {
        AST { root: None }
    }
}

pub fn parse(expr: &String) -> AST {
    let tokens = tokenize(expr);

    let mut op_stack: Vec<AstNode> = Vec::with_capacity(tokens.len()); // Operator Stack
    let mut out: VecDeque<AstNode> = VecDeque::with_capacity(tokens.len()); // Output Queue

    for token in tokens.into_iter() {
        match token {
            Token::LP => op_stack.push(AstNode::new(token, None, None)),
            Token::RP => {
                let mut found_left_paren: bool = false;
                while op_stack.len() != 0 {
                    let mut popped = op_stack.pop().unwrap();
                    if popped.tok == Token::LP {
                        found_left_paren = true;
                        break;
                    } else {
                        assert!(op_stack.len() >= 2);

                        let rhs = op_stack.pop().unwrap();
                        let lhs = op_stack.pop().unwrap();

                        popped.rhs = Some(Box::new(rhs));
                        popped.lhs = Some(Box::new(lhs));
                    }
                }

                if !found_left_paren {
                    panic!("Unbalanced parentheses!!!");
                }
            }
            Token::Num(_k) => out.push_front(AstNode::new(token, None, None)),
            Token::Bin(op) => {
                assert!(op_stack.len() != 0);

                while op_stack.len() != 0 {
                    let top = op_stack.first().unwrap();

                    match top.tok {
                        Token::Bin(o2) => {
                            if 
                        }
                        _ => break,
                    }
                }
            }
            _ => (),
        }
    }

    let mut tree = AST::new();

    todo!()
}
