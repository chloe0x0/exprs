use exprs::*;

fn main() {
    let tree: AST = parse("1 + 2");

    let computation = tree.eval().expect("Could not evaluate expression");

    println!("1 + 2 = {}", computation);

    println!("{:?}", tree);
}
