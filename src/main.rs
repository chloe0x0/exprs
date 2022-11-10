use exprs::*;

fn main() {
    let expr = String::from("1 + 2");
    println!("{} = {}", expr, parse(&expr).eval().expect("Could not evaluate"));
    /* 
    println!("Tokenizing : {}", expr);
    println!("{:?}", tokenize(&expr));

    let tree = parse(&expr);
    println!("{:?}", tree);
    */

}
