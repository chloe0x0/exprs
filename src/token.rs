#[derive(PartialEq, Debug)]
/// # Associativity of an operator
///
/// Given a binary operator ~
///
/// And the expression: x ~ y ~ q
///
/// If ~ is left associative it is evaluated as: (x ~ y) ~ q
///
/// If it is right associative: x ~ (y ~ q)
///
pub enum Assoc {
    LEFT,
    NONE,
    RIGHT,
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// # Operators
///
/// Current Operators:
///
/// Binary Operations: +, -, *, /, ^
///
/// Unary Operations: - (negation), ! (factorial)
///
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
    /// Used to parse an Operator from a character
    ///
    /// If a valid operator was not found, it returns an err type stating the character that couldn't be parsed
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
    ///
    /// ```rust
    /// use exprs::{Op, Assoc};
    /// assert_eq!(Op::EXP.assoc(), Assoc::RIGHT);
    /// ```
    pub fn assoc(&self) -> Assoc {
        match *self {
            Op::SUM | Op::MUL | Op::SUB | Op::DIV => Assoc::LEFT,
            Op::EXP | Op::NEG | Op::FAC => Assoc::RIGHT,
            _ => Assoc::NONE,
        }
    }
    /// Get the operator's precedence
    ///
    /// Higher precedence operators are evaluated before lower precedence operators
    pub fn prec(&self) -> u8 {
        match *self {
            Op::NEG => 5,
            Op::EXP | Op::FAC => 4,
            Op::SUB | Op::SUM => 2,
            Op::MUL | Op::DIV => 3,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
/// # Token type
///
/// Encodes a lexical token
///
/// ## Variants
///
/// * Num(f64): A numeric literal
///     * Token::Num(3.5); the token representing the numeric literal 3.5
/// * Bin(op): A binary operator
/// * Una(op): A unary operator
/// * Fn(String): A function identifier (not yet implemented)
/// * Var(String): A variable identifier
/// * LP, RP: Left and Right parantheses
///
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

impl Token {
    /// check if the token is a binary operator
    #[inline]
    pub fn is_bin(&self) -> bool {
        matches!(self, Token::Bin(_x))
    }
    /// check if the token is an operator
    #[inline]
    pub fn is_op(&self) -> bool {
        matches!(self, Token::Bin(_x)) || matches!(self, Token::Una(_x))
    }
    /// get the operator's token
    #[inline]
    pub fn get_op(&self) -> Option<Op> {
        if let Token::Bin(op) = self {
            Some(op.clone())
        } else if let Token::Una(op) = self {
            Some(op.clone())
        } else {
            None
        }
    }
}
