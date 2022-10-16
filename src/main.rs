use exprs::*;

fn main() {
    let expr = String::from("-1.5-20001.255 * (2 - -4)");
    println!("Tokenizing : {}", expr);
    println!("{:?}", tokenize(expr));
}
