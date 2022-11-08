use exprs::*;

fn main() {
    let expr = String::from("()");
    println!("Tokenizing : {}", expr);
    println!("{:?}", tokenize(&expr));

    parse(&expr);
}
