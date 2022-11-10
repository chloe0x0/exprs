use crate::lexer::*;
use crate::token::*;

#[derive(Clone, Debug)]
pub struct AstNode {
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
    pub fn eval_node(&self) -> f64 {
        match self.tok {
            Token::Num(k) => k,
            Token::Bin(op) => {
                match op {
                    Op::SUB => self.lhs.as_ref().unwrap().eval_node() - self.rhs.as_ref().unwrap().eval_node(),
                    Op::SUM => self.lhs.as_ref().unwrap().eval_node() + self.rhs.as_ref().unwrap().eval_node(),
                    Op::MUL => self.lhs.as_ref().unwrap().eval_node() * self.rhs.as_ref().unwrap().eval_node(),
                    Op::DIV => self.lhs.as_ref().unwrap().eval_node() / self.rhs.as_ref().unwrap().eval_node(),
                    Op::EXP => self.lhs.as_ref().unwrap().eval_node().powf(self.rhs.as_ref().unwrap().eval_node()),
                    _ => todo!()
                }
            },
            Token::Una(op) => {
                match op {
                    Op::NEG => -self.lhs.as_ref().unwrap().eval_node(),
                    _ => todo!()
                }
            },
            _ => todo!()
        }
    }
}

#[derive(Debug)]
pub struct AST {
    pub root: Option<Box<AstNode>>,
}

impl AST {
    #[inline]
    /// Create an empty AST
    pub fn new(root: AstNode) -> Self {
        AST { root: Some(Box::new(root)) }
    }
    pub fn eval(&self) -> Option<f64> {
        match self.root {
            None => None,
            Some(ref r) => {
                Some(r.eval_node())
            }
        }
    }
}

/// An implementation of the Shunting Yard Algorithm for parsing infix expressions into an AST
pub fn parse(expr: &String) -> AST {
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
            Token::Bin(op) | Token::Una(op) => {
                while operator_stack.len() != 0 {
                    let top = operator_stack.last().unwrap().to_owned();
                    if !top.is_op() {
                        break;
                    }

                    // get the precedence of the second operator
                    let o2_p = top.get_op().prec();
                    // get the precedence of the current operator
                    let o1_p = op.prec();

                    if (o1_p < o2_p && op.assoc() == Assoc::RIGHT)
                        || (o1_p == o2_p && op.assoc() == Assoc::LEFT)
                    {
                        break;
                    }

                    if token.is_bin() {
                        // binary operator needs two operands
                        assert!(output.len() >= 2);

                        let rhs = output.pop().unwrap();
                        let lhs = output.pop().unwrap();

                        let node = AstNode::new(
                            token.to_owned(),
                            Some(Box::new(lhs)),
                            Some(Box::new(rhs)),
                        );

                        output.push(node);
                    } else {
                        // unary operator only needs one operand
                        assert!(output.len() != 0);

                        let operand = output.pop().unwrap();

                        let node = AstNode::new(token.to_owned(), Some(Box::new(operand)), None);

                        output.push(node);
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

        assert!(output.len() >= 2);
        assert!(top.is_op());

        if top.is_bin() {
            let rhs = output.pop().unwrap();
            let lhs = output.pop().unwrap();

            output.push(AstNode::new(
                top.to_owned(),
                Some(Box::new(lhs)),
                Some(Box::new(rhs)),
            ));

        } else {
            assert!(output.len() != 0);
            let operand = output.pop().unwrap();

            output.push(AstNode::new(
                top.to_owned(),
                Some(Box::new(operand)),
                None
            ));
        }
    }

    assert!(output.len() != 0);

    AST::new(output.pop().unwrap().to_owned())
}
