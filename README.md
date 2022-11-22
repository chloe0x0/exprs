# exprs 
A 0 dependency math expression parser and evaluator in Rust. It aims to be simple to use and is more of an exercise than anything. 

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

## Installation
Simply add the current version to your ```Cargo.toml``` dependency list
```toml
[dependencies]
exprs = "0.1"
```
in your crate root add
```rust
extern crate exprs;
```


## TODO
* publish to crates.io
* More robust parse errors rather than assert macros