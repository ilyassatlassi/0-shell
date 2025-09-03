#[derive(Debug, Clone, PartialEq)]

pub enum Token {
    Word(String),
    Pipe,
    RedirectOut,
    RedirectAppend,
    RedirectIn,
    And,
    Semicolon,
    EOF,
}

#[derive(Debug)]

pub struct TokenWithPos {
    pub token: Token,
    pub start: usize,
    pub end: usize,
}