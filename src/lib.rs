//! # exprs
//! 
//!  A simple math expression parser in Rust
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