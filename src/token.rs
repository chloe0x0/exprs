
#[derive(Debug)]
pub enum Type {
    SUM, SUB, EXP, DIV, MUL,    // Binary Operations

    NUM,                        // Numeric Literal
    VAR,                        // Variable type
    FUN,                        // Function
    RP, LP,                     // Parens
}

impl Type {
    pub fn from_str(&self, s: &str) -> Self {
        match s {
            "+" => Type::SUM,
            "-" => Type::SUB,
            "/" => Type::DIV,
            "*" => Type::MUL,
            "(" => Type::LP,
            ")" => Type::RP,
            _ => Type::NUM
        }
    }
}

#[derive(Debug)]
pub struct Token {
    flag: Type,
    lexeme: String,
    val: i32,
}

impl Token {
    pub fn new(lex: &str, flag: Type, val: i32) -> Self {
        Token { flag: flag, lexeme: lex.to_string(), val: val }
    }
}
