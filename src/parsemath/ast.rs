#[derive(Debug, Clone)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Substract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Num(f64),
}
