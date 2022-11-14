# exprs 
A math expression parser in Rust

# Quick Start

Parsing an expression is simple
```rust
use exprs::*;

fn main() {
    let tree: AST = parse("1 + 2");

    let computation: f64 = tree.eval().expect("Failed to evaluate expression");

    assert_eq!(computation, 3.0);
}
```

## TODO
* publish to crates.io
* Add support for functions and variables
* Add tests
* More robust parse errors rather than assert macros


## How it works
Expression String -> Lexer -> Tokens -> Parser -> AST -> Interpreter -> Output
