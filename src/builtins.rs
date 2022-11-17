

/// factorial of an f64=
pub fn fct(x: f64) -> f64 {
    if x.floor() - x != 0.0 {
        panic!("Can't take factorial of non-integer argument {}!", x)
    }

    let mut factorial: f64 = 1.0;
    for i in 2..=x as i64 {
        factorial *= i as f64;
    }

    factorial
}
