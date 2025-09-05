#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Commands and arguments
    Command(String),  // First word in a simple command: "ls", "echo"
    Flag(String),     // Words starting with '-': "-l", "-a", "--all"
    Argument(String), // Other words: "file.txt", "/home/user"

    // Operators
    Pipe,           // "|"
    RedirectOut,    // ">"
    RedirectAppend, // ">>"
    RedirectIn,     // "<"
    Semicolon,      // ";"
    EOF,
}

#[derive(Debug)]
pub struct TokenWithPos {
    pub token: Token,
    pub start: usize,
    pub end: usize,
}

// Helper methods
impl Token {
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Token::Pipe
                | Token::RedirectOut
                | Token::RedirectAppend
                | Token::RedirectIn
                | Token::Semicolon
        )
    }

    pub fn is_word(&self) -> bool {
        matches!(
            self,
            Token::Command(_) | Token::Flag(_) | Token::Argument(_)
        )
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Token::Command(s) | Token::Flag(s) | Token::Argument(s) => Some(s),
            _ => None,
        }
    }
}
