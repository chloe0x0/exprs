/// Associativity of an operator
#[derive(PartialEq)]
pub enum Assoc {
    LEFT,
    NONE,
    RIGHT,
}

#[derive(Debug, PartialEq, Clone)]
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
    NEG,
}

impl Op {
    pub fn from_char(s: char) -> Result<Op, String> {
        match s {
            '-' => Ok(Op::SUB),
            '+' => Ok(Op::SUM),
            '*' => Ok(Op::MUL),
            '/' => Ok(Op::DIV),
            '^' => Ok(Op::EXP),
            '!' => Ok(Op::FAC),

            _ => Err(format!("Could not parse {} as operator", s)),
        }
    }
    /// Get the operator's associativity
    pub fn assoc(&self) -> Assoc {
        match *self {
            Op::SUM | Op::MUL | Op::SUB | Op::DIV => Assoc::LEFT,
            Op::EXP => Assoc::RIGHT,
            _ => Assoc::NONE,
        }
    }
    /// Get the operator's precedence
    pub fn prec(&self) -> u32 {
        match *self {
            Op::SUB | Op::SUM => 2,
            Op::MUL | Op::DIV => 3,
            Op::EXP => 4,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
/// Token type
pub enum Token {
    /// Numeric Literal
    Num(f64),
    /// Binary Operator
    Bin(Op),
    /// Unary Operator
    Una(Op),
    /// Function identifier
    Fn(String),
    /// Variable identifier
    Var(String),
    /// Left Paren
    LP,
    /// Right Paren
    RP,
}
