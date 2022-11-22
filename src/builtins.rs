/// factorial of an f64, panics if the input is not a whole number, or is negative
///
/// ```rust
/// use exprs::fct;
/// assert_eq!(fct(5.0), 120.0);
/// ```
pub fn fct(x: f64) -> f64 {
    if x.floor() - x != 0.0 {
        panic!("Can't take factorial of non-integer argument {}!", x)
    } else if x < 0.0 {
        panic!("Cannot take the factorial of a negative number!")
    }

    let mut factorial: f64 = 1.0;
    for i in 2..=x as i64 {
        factorial *= i as f64;
    }

    factorial
}
