#[derive(Debug)]
/// Operations
pub enum Op {
    /// Binary Operations
    SUM,
    SUB,
    EXP,
    DIV,
    MUL,
    /// Unary Operations
    FAC,
}

impl Op {
    pub fn from_char(s: char) -> Result<Op, String> {
        match s {
            '-' => Ok(Op::SUB),
            '+' => Ok(Op::SUM),
            '*' => Ok(Op::MUL),
            '/' => Ok(Op::DIV),
            '^' => Ok(Op::EXP),

            _ => Err(format!("Could not parse {} as operator", s)),
        }
    }
}

#[derive(Debug)]
/// Token type
pub enum Token {
    Num(f32),    // Numeric literal
    Bin(Op),     // Binary Operator
    Fn(String),  // function identifier
    Var(String), // variable type
    LP,
    RP, // Parens
}
