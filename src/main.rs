use exprs::*;

fn main() {
    let expr = String::from("1 - 200.5 * (2 + 4)");
    println!("Tokenizing : {}", expr);
    println!("{:?}", tokenize(expr));
}
