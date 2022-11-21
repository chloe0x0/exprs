/// The Parser is a modified Shunting Yard algorithm
///
/// rather than converting infix expressions to postfix expressions it converts them to ASTs
///
/// These ASTs can then be evaluated via a Post Order traversal
use crate::lexer::*;
use crate::token::*;

#[derive(Clone, Debug)]
/// A node in the Abstract Syntax Tree
pub struct AstNode {
    pub tok: Token,
    pub lhs: Option<Box<AstNode>>,
    pub rhs: Option<Box<AstNode>>,
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

#[derive(Debug)]
pub struct AST {
    pub root: Option<Box<AstNode>>,
}

impl AST {
    #[inline]
    pub fn new(root: AstNode) -> Self {
        AST {
            root: Some(Box::new(root)),
        }
    }
}

/// Used in the Shunting Yard parser to handle popping operators
fn pop_operator(out: &mut Vec<AstNode>, tok: &Token) {
    if tok.is_bin() {
        // binary operator needs two operands
        assert!(out.len() >= 2);

        let rhs = out.pop().unwrap();
        let lhs = out.pop().unwrap();

        let node = AstNode::new(tok.to_owned(), Some(Box::new(lhs)), Some(Box::new(rhs)));

        out.push(node);
    } else {
        // unary operator only needs one operand
        assert!(out.len() != 0);

        let operand = out.pop().unwrap();

        let node = AstNode::new(tok.to_owned(), Some(Box::new(operand)), None);

        out.push(node);
    }
}

/// An implementation of the Shunting Yard Algorithm for parsing infix expressions into an AST
pub fn parse(expr: &str) -> AST {
    let tokens = tokenize(expr);

    let mut operator_stack: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut output: Vec<AstNode> = Vec::with_capacity(tokens.len());

    for token in tokens.into_iter() {
        match token {
            Token::LP => operator_stack.push(token.to_owned()),
            Token::RP => {
                while operator_stack.len() != 0 {
                    let t = operator_stack.last().unwrap();

                    match t {
                        Token::LP => break,
                        Token::Bin(_op) | Token::Una(_op) => {
                            let operator = operator_stack.pop().unwrap();
                            pop_operator(&mut output, &operator);
                        }
                        _ => output.push(AstNode::new(
                            operator_stack.pop().unwrap().to_owned(),
                            None,
                            None,
                        )),
                    }
                }

                assert!(matches!(operator_stack.last(), Some(Token::LP)));
                operator_stack.pop(); // pop off left paren
            }
            Token::Num(_k) => {
                // leaf node
                let node = AstNode::new(token.to_owned(), None, None);
                output.push(node);
            }
            Token::Var(id) => {
                let new_tok = Token::Var(id.clone());
                let node = AstNode::new(new_tok.to_owned(), None, None);
                output.push(node);
            }
            Token::Bin(op) | Token::Una(op) => {
                while operator_stack.len() != 0 {
                    let top = operator_stack.last().unwrap().to_owned();
                    if !top.is_op() {
                        break;
                    }

                    // get the precedence of the second operator
                    let o2_p = top.get_op().unwrap().prec();
                    // get the precedence of the current operator
                    let o1_p = op.prec();

                    if (o1_p < o2_p && op.assoc() == Assoc::RIGHT)
                        || (o1_p <= o2_p && op.assoc() == Assoc::LEFT)
                    {
                        pop_operator(&mut output, &operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }

                operator_stack.push(token.to_owned());
            }
            _ => (),
        }
    }

    // while there are tokens to be read on the stack
    while operator_stack.len() != 0 {
        let top = operator_stack.pop().unwrap();
        assert!(!matches!(top, Token::LP)); // mismatched paren check

        assert!(top.is_op());

        pop_operator(&mut output, &top);
    }

    assert!(output.len() != 0);

    AST::new(output.pop().unwrap().to_owned())
}
