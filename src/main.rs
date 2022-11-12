use exprs::*;

fn main() {
    let expr = String::from("(1 + -1) * (2 + 3)");

    let tree: AST = parse(&expr);

    let computation = tree.eval().expect("Could not parse expression");

    println!("{} = {}", expr, computation);
}
