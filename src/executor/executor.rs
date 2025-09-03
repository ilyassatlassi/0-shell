use crate::utils::error::{ShellError, Result};
use crate::types::tokens::Token;
use super::commands;

pub struct Executor {
    // Shell state can be added here
}

impl Executor {
    pub fn new() -> Self {
        Executor {}
    }

    pub fn execute_tokens(&mut self, tokens: Vec<Token>) -> Result<()> {
        let mut command_args = Vec::new();
        let mut current_command = Vec::new();
        
        // Simple command parsing (no pipes/redirection yet)
        for token in tokens {
            match token {
                Token::Word(cmd) => {
                    current_command.push(cmd);
                }
                Token::Semicolon => {
                    if !current_command.is_empty() {
                        command_args.push(current_command);
                        current_command = Vec::new();
                    }
                }
                _ => {
                    // TODO: Handle other tokens later
                    eprintln!("Warning: Token not implemented yet: {:?}", token);
                }
            }
        }
        
        if !current_command.is_empty() {
            command_args.push(current_command);
        }

        // Execute each command
        for args in command_args {
            if let Some(command) = args.get(0) {
                self.execute_single_command(command, &args[1..])?;
            }
        }

        Ok(())
    }

    fn execute_single_command(&self, command: &str, args: &[String]) -> Result<()> {
        if let Some(cmd) = commands::get_command(command) {
            cmd.execute(args)
        } else {
            Err(ShellError::command_not_found(command))
        }
    }
}