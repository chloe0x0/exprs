//! # exprs
//! 
//! A 0 dependency math expression parser and evaluator in Rust. It aims to be simple to use and is more of an exercise than anything. 
//! 
//! # Evaluating Expressions
//! 
//! ```rust
//! use exprs::*;
//! 
//! fn main() {
//!     let expr = Expr::new("1 + 2");
//!     
//!     assert_eq!(expr.eval(), 3.0);
//! }
//! ```
//! ## Installation
//! Simply add the current version to your ```Cargo.toml``` dependency list
//! ```toml
//! [dependencies]
//! exprs = "0.1.1"
//! ```
//! in your crate root add
//! ```rust
//! extern crate exprs;
//! ```
//! 
//! 

// Token type
pub mod token;
pub use token::*;

// Bring in the Lexer
pub mod lexer;
pub use lexer::*;

// Bring in the Parser
pub mod parser;
pub use parser::*;

// Bring in the Expr type
pub mod expr;
pub use expr::*;

// Builtin functions (currently only Factorial)
pub mod builtins;
pub use builtins::*;

#[cfg(test)]
mod tests {
    use crate::{Expr, fct};
    #[test]
    fn test_ops() {
        let mut e = Expr::new("1  + 2");
        assert_eq!(e.eval(), 3.0);

        e = Expr::new(" 1 - 2");
        assert_eq!(e.eval(), -1.0);

        e = Expr::new("2^2^2");
        assert_eq!(e.eval(), 16.0);

        e = Expr::new("2^(1+1)^(1+1+1-1)");
        assert_eq!(e.eval(), 16.0);

        e = Expr::new("2^(3^1)");
        assert_eq!(e.eval(), 8.0);
    }
    #[test]
    fn test_builtins() {
        // Test factorials
        assert_eq!(fct(5.0), 120.0);
        assert_eq!(fct(1.0), 1.0);
        assert_eq!(fct(10.0), 3628800.0);
    }
}