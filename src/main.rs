use exprs::*;

fn main() {
    let expr = String::from("((1 + 1) * (3 + 2)) / 2");

    println!("{:?}", tokenize(&expr));

    let tree: AST = parse(&expr);

    let computation = tree.eval().expect("Could not evaluate expression");

    println!("{} = {}", expr, computation);

    println!("{:?}", tree);
}
