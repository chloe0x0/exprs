use exprs::*;

fn main() {
    let expr = String::from("1 + 2");
    println!("Tokenizing : {}", expr);
    println!("{:?}", tokenize(&expr));

    let tree = parse(&expr);
    println!("{:?}", tree);

    println!("Eval: ");
}
