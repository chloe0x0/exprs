use crate::*;
use std::collections::HashMap;

/// Higher level API for handling Expressions
pub struct Expr {
    /// Abstract Syntax Tree of the Expression
    tree: AST,
    /// Context. A hashmap which stores the values of variables, used during eval
    context: HashMap<String, f64>,
    /// Store the Expression String so it can be printed without traversing the AST
    expr_string: String,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expr_string)
    }
}

impl Expr {
    /// Create an Expr type from an Expression string
    pub fn new(expression: &str) -> Self {
        Expr { tree: parse(expression), context: HashMap::new(), expr_string: expression.to_string() }
    }
    /// Evaluate an AST node
    fn eval_node(&self, node_ref: &Option<Box<AstNode>>) -> f64 {
        let node = node_ref.as_ref().unwrap();

        match &node.tok {
            Token::Num(k) => *k,
            Token::Var(id) => {
                match self.context.get(id) {
                    None => panic!("{} was not found in the Expr context", id),
                    Some(x) => *x
                }
            },
            Token::Una(op) | Token::Bin(op) => match op {
                Op::SUM => self.eval_node(&node.lhs) + self.eval_node(&node.rhs),
                Op::SUB => self.eval_node(&node.lhs) - self.eval_node(&node.rhs),
                Op::MUL => self.eval_node(&node.lhs) * self.eval_node(&node.rhs),
                Op::DIV => self.eval_node(&node.lhs) / self.eval_node(&node.rhs),
                Op::EXP => self.eval_node(&node.lhs).powf(self.eval_node(&node.rhs)),

                Op::NEG => -self.eval_node(&node.lhs),
                _ => todo!()
            },
            _ => panic!("{:?} should not have been encountered", node.tok)
        }
    }
    /// Evaluate the AST
    pub fn eval(&self) -> f64 {
        self.eval_node(&self.tree.root)
    }
    /// Set a variable value
    pub fn set_var(&mut self, id: &str, val: f64) {
        *self.context.entry(id.to_string()).or_insert(val) = val;
    }
}
