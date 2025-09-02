use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ShellError {
    Io(io::Error),
    Lexer(String, usize),  // (message, position)
    Parser(String),
    Execution(String),
    CommandNotFound(String),
    InvalidArguments(String),
}

// Implement Display for pretty printing
impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::Io(e) => write!(f, "I/O error: {}", e),
            ShellError::Lexer(msg, pos) => write!(f, "Lexer error at position {}: {}", pos, msg),
            ShellError::Parser(msg) => write!(f, "Parser error: {}", msg),
            ShellError::Execution(msg) => write!(f, "Execution error: {}", msg),
            ShellError::CommandNotFound(cmd) => write!(f, "Command not found: {}", cmd),
            ShellError::InvalidArguments(msg) => write!(f, "Invalid arguments: {}", msg),
        }
    }
}

// Convert std::io::Error to our ShellError
impl From<io::Error> for ShellError {
    fn from(error: io::Error) -> Self {
        ShellError::Io(error)
    }
}

// Our Result type alias
pub type Result<T> = std::result::Result<T, ShellError>;

// Helper functions for creating errors
impl ShellError {
    pub fn lexer(message: &str, position: usize) -> Self {
        ShellError::Lexer(message.to_string(), position)
    }
    
    pub fn parser(message: &str) -> Self {
        ShellError::Parser(message.to_string())
    }
    
    pub fn execution(message: &str) -> Self {
        ShellError::Execution(message.to_string())
    }
    
    pub fn command_not_found(command: &str) -> Self {
        ShellError::CommandNotFound(command.to_string())
    }
    
    pub fn invalid_arguments(message: &str) -> Self {
        ShellError::InvalidArguments(message.to_string())
    }
}