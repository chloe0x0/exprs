use crate::lexer::*;
use crate::token::*;

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

    let mut operator_stack: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut output: Vec<AstNode> = Vec::with_capacity(tokens.len());

    for token in tokens.into_iter() {
        match token {
            Token::LP => operator_stack.push(token.to_owned()),
            Token::RP => {
                while let t = operator_stack.last() {
                    match t {
                        Some(token) => {
                            match token {
                                Token::LP => break,
                                _ => output.push(AstNode::new(token.to_owned(), None, None))
                            }
                        },
                        None => panic!("Unbalanced parentheses!")
                    }
                }

                assert!(matches!(operator_stack.last(), Some(Token::LP)));
                operator_stack.pop();
            }
            Token::Num(_k) => operator_stack.push(token.to_owned()),
            Token::Bin(op) => {
                while operator_stack.len() != 0 {
                    let top = operator_stack.last().unwrap();
                    if !top.is_bin() {
                        break;
                    }

                    // get the precedence of the second operator
                    let o2_p = top.get_op().prec();
                    // get the precedence of the current operator
                    let o1_p = op.prec();

                    if (o1_p < o2_p && op.assoc() == Assoc::RIGHT) || (o1_p == o2_p && op.assoc() == Assoc::LEFT) {
                        break;
                    }

                    output.push(AstNode::new(operator_stack.pop().unwrap().to_owned(), None, None));
                }

                operator_stack.push(token.to_owned());
            }
            _ => todo!()
        }
    }

    todo!()
}
