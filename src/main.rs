
mod utils;
mod executor;
mod lexer;
mod types;

use std::io::{self, Write};
use crate::types::tokens::Token;
use crate::utils::error::ShellError;
use crate::lexer::Lexer;
use crate::executor::executor::Executor;

fn main() {
    if let Err(e) = run_shell() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_shell() -> Result<(), ShellError> {
    let mut executor = Executor::new();
    
    loop {
        print!("$ ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // Ctrl+D pressed (EOF)
                println!();
                break;
            }
            Ok(_) => {
                // Process input
                if input.trim().is_empty() {
                    continue;
                }
                
                if input.trim() == "exit" {
                    println!("Goodbye!");
                    break;
                }
                
                // Tokenize and execute
                let mut lexer = Lexer::new(input.to_string());
                let tokens_with_pos = lexer.tokenize()?;
                
                let tokens: Vec<Token> = tokens_with_pos.into_iter().map(|twp| twp.token).collect();
                
                match executor.execute_tokens(tokens) {
                    Ok(_) => {}
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Err(e) => {
                return Err(ShellError::Io(e));
            }
        }
    }
    
    Ok(())
}