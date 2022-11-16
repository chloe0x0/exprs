use exprs::*;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read stdin!");

    // trim off the whitespace from read_line
    input = input.trim().to_string();

    // create an expr type
    let expr = Expr::new(&input);

    // Evaluate
    let computation = expr.eval();

    println!("{} = {}", expr, computation);
}
