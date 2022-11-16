# exprs 
A math expression parser in Rust

# Quick Start

Parsing an expression is simple
```rust
use exprs::*;

fn main() {
    let expr = Expr::new("1 + 2");
 
    let computation = expr.eval();

    assert_eq!(computation, 3.0);
}
```

## TODO
* publish to crates.io
* More robust parse errors rather than assert macros