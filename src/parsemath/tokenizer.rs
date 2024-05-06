use std::str::Chars;
use std::str::Peekable;

pub struct Tokenizer {
    expr: Peekable<Chars>
}