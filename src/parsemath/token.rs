use super::parser::OperPrec;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Add,
    Substract,
    Multiply,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    Num(f64),
    EOF
}

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        use self::OperPrec::*;
        use self::Token::*;
        match *self {
            Add | Substract => AddSub,
            Multiply | Divide => MulDiv,
            Caret => Power,
            _ => DefaultZero
        }
    }
}