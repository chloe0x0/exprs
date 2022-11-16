use exprs::*;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read stdin!");

    // trim off the whitespace from read_line
    input = input.trim().to_string();

    // Create the Abstract Syntax Tree
    let tree = parse(&input);

    // Evaluate the abstract syntax tree
    let result = tree.eval().unwrap();

    println!("{} = {}", input, result);
}
