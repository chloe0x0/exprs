use exprs::*;

fn main() {
    let expr = String::from("(1 + 2)");
    println!("Tokenizing : {}", expr);
    println!("{:?}", tokenize(&expr));

    parse(&expr);
}
