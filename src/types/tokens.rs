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
// enum Token {
//     Command(String),    // e.g., "ls", "cd", "echo"
//     Flag(String),       // e.g., "-l", "-a", "-la"  
//     Argument(String),   // e.g., "/home/user", "file.txt"
//     Pipe,               // "|"
//     RedirectOut,        // ">"
//     RedirectIn,         // "<"
//     And,                // "&&"
//     Or,                 // "||"
//     Semicolon,          // ";"
// }
#[derive(Debug)]

pub struct TokenWithPos {
    pub token: Token,
    pub start: usize,
    pub end: usize,
}