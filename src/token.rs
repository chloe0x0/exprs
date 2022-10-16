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
}

#[derive(Debug)]
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

impl Token {
    #[inline]
    pub fn is_bin(&self) -> bool {
        matches!(*self, Token::Bin(_))
    }
    #[inline]
    pub fn is_una(&self) -> bool {
        matches!(*self, Token::Una(_))
    }
    #[inline]
    pub fn is_fn(&self) -> bool {
        matches!(*self, Token::Fn(_))
    }
    #[inline]
    pub fn is_var(&self) -> bool {
        matches!(*self, Token::Var(_))
    }
    #[inline]
    pub fn is_num(&self) -> bool {
        matches!(*self, Token::Num(_))
    }
    #[inline]
    pub fn is_lp(&self) -> bool {
        matches!(*self, Token::LP)
    }
    #[inline]
    pub fn is_rp(&self) -> bool {
        matches!(*self, Token::RP)
    }
}