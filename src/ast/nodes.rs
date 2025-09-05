#[derive(Debug)]
pub enum AstNode {
    // Simple command: command [flags...] [args...]
    Command {
        name: String,
        flags: Vec<String>,
        args: Vec<String>,
    },
    
    // Redirection: command > file, command < file, command >> file
    Redirect {
        command: Box<AstNode>,
        operator: Token,  // RedirectOut, RedirectAppend, RedirectIn
        file: String,
    },
    
    // Pipeline: command1 | command2
    Pipeline {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    
    // Sequence: command1 ; command2
    Sequence {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
}