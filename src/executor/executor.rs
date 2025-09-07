use std::process::{Command, Stdio};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use crate::ast::nodes::AstNode;
use crate::utils::error::{ShellError, Result};
use super::commands;

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Executor
    }

    pub fn execute_ast(&self, ast: &AstNode) -> Result<()> {
        match ast {
            AstNode::Command { name, flags, args } => {
                self.execute_command(name, flags, args)
            }
            AstNode::Redirect { command, operator, file } => {
                self.execute_redirect(command, operator, file)
            }
            AstNode::Pipeline { left, right } => {
                self.execute_pipeline(left, right)
            }
            AstNode::Sequence { left, right } => {
                self.execute_sequence(left, right)
            }
        }
    }

    fn execute_command(&self, name: &str, flags: &[String], args: &[String]) -> Result<()> {
        // Check if it's a built-in command first
        if let Some(builtin_cmd) = commands::get_command(name) {
            // Combine flags and args for built-in commands
            let all_args: Vec<String> = flags.iter()
                .chain(args.iter())
                .cloned()
                .collect();
            return builtin_cmd.execute(&all_args);
        }

        // // For external commands, use std::process::Command
        // let mut cmd = Command::new(name);
        
        // // Add flags and arguments
        // cmd.args(flags);
        // cmd.args(args);

        // // Execute the command
        // let status = cmd.status()
        //     .map_err(|e| {
        //         if e.kind() == io::ErrorKind::NotFound {
        //             ShellError::command_not_found(name)
        //         } else {
        //             ShellError::execution(&format!("Failed to execute '{}': {}", name, e))
        //         }
        //     })?;

        // if !status.success() {
        //     return Err(ShellError::execution(&format!(
        //         "Command '{}' failed with exit code: {}",
        //         name,
        //         status.code().unwrap_or(-1)
        //     )));
        // }

        Ok(())
    }

    fn execute_redirect(&self, command: &AstNode, operator: &crate::types::tokens::Token, file: &str) -> Result<()> {
        match operator {
            crate::types::tokens::Token::RedirectOut => {
                // > - overwrite file
                let output_file = File::create(file)
                    .map_err(|e| ShellError::execution(&format!("Cannot create file '{}': {}", file, e)))?;
                
                if let AstNode::Command { name, flags, args } = command {
                    let mut cmd = Command::new(name);
                    cmd.args(flags);
                    cmd.args(args);
                    cmd.stdout(Stdio::from(output_file));
                    
                    let status = cmd.status()
                        .map_err(|e| ShellError::execution(&format!("Failed to execute '{}': {}", name, e)))?;
                    
                    if !status.success() {
                        return Err(ShellError::execution(&format!(
                            "Command '{}' failed with exit code: {}",
                            name,
                            status.code().unwrap_or(-1)
                        )));
                    }
                } else {
                    // Handle nested redirects or other AST nodes
                    return self.execute_ast(command);
                }
            }
            crate::types::tokens::Token::RedirectAppend => {
                // >> - append to file
                let output_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file)
                    .map_err(|e| ShellError::execution(&format!("Cannot open file '{}': {}", file, e)))?;
                
                if let AstNode::Command { name, flags, args } = command {
                    let mut cmd = Command::new(name);
                    cmd.args(flags);
                    cmd.args(args);
                    cmd.stdout(Stdio::from(output_file));
                    
                    let status = cmd.status()
                        .map_err(|e| ShellError::execution(&format!("Failed to execute '{}': {}", name, e)))?;
                    
                    if !status.success() {
                        return Err(ShellError::execution(&format!(
                            "Command '{}' failed with exit code: {}",
                            name,
                            status.code().unwrap_or(-1)
                        )));
                    }
                } else {
                    return self.execute_ast(command);
                }
            }
            crate::types::tokens::Token::RedirectIn => {
                // < - read from file
                let input_file = File::open(file)
                    .map_err(|e| ShellError::execution(&format!("Cannot open file '{}': {}", file, e)))?;
                
                if let AstNode::Command { name, flags, args } = command {
                    let mut cmd = Command::new(name);
                    cmd.args(flags);
                    cmd.args(args);
                    cmd.stdin(Stdio::from(input_file));
                    
                    let status = cmd.status()
                        .map_err(|e| ShellError::execution(&format!("Failed to execute '{}': {}", name, e)))?;
                    
                    if !status.success() {
                        return Err(ShellError::execution(&format!(
                            "Command '{}' failed with exit code: {}",
                            name,
                            status.code().unwrap_or(-1)
                        )));
                    }
                } else {
                    return self.execute_ast(command);
                }
            }
            _ => {
                return Err(ShellError::execution(&format!(
                    "Unsupported redirection operator: {:?}",
                    operator
                )));
            }
        }
        
        Ok(())
    }

    fn execute_pipeline(&self, left: &AstNode, right: &AstNode) -> Result<()> {
        match (left, right) {
            (AstNode::Command { name: left_name, flags: left_flags, args: left_args }, 
             AstNode::Command { name: right_name, flags: right_flags, args: right_args }) => {
                
                // Create the first command
                let mut left_cmd = Command::new(left_name);
                left_cmd.args(left_flags);
                left_cmd.args(left_args);
                
                // Create the second command
                let mut right_cmd = Command::new(right_name);
                right_cmd.args(right_flags);
                right_cmd.args(right_args);
                
                // Set up the pipe
                let left_output = left_cmd.stdout(Stdio::piped())
                    .spawn()
                    .map_err(|e| ShellError::execution(&format!("Failed to execute '{}': {}", left_name, e)))?;
                
                right_cmd.stdin(left_output.stdout.unwrap());
                
                // Execute the right command and wait
                let status = right_cmd.status()
                    .map_err(|e| ShellError::execution(&format!("Failed to execute '{}': {}", right_name, e)))?;
                
                if !status.success() {
                    return Err(ShellError::execution(&format!(
                        "Pipeline failed at '{}' with exit code: {}",
                        right_name,
                        status.code().unwrap_or(-1)
                    )));
                }
            }
            _ => {
                // For more complex pipelines, use recursive execution
                // This handles cases like: cmd1 | (cmd2 > file) or other nested structures
                let left_output = self.execute_and_capture(left)?;
                
                if let AstNode::Command { name, flags, args } = right {
                    let mut cmd = Command::new(name);
                    cmd.args(flags);
                    cmd.args(args);
                    cmd.stdin(Stdio::piped());
                    
                    let mut child = cmd.spawn()
                        .map_err(|e| ShellError::execution(&format!("Failed to execute '{}': {}", name, e)))?;
                    
                    if let Some(mut stdin) = child.stdin.take() {
                        stdin.write_all(&left_output)
                            .map_err(|e| ShellError::execution(&format!("Failed to write to pipe: {}", e)))?;
                    }
                    
                    let status = child.wait()
                        .map_err(|e| ShellError::execution(&format!("Failed to wait for '{}': {}", name, e)))?;
                    
                    if !status.success() {
                        return Err(ShellError::execution(&format!(
                            "Command '{}' failed with exit code: {}",
                            name,
                            status.code().unwrap_or(-1)
                        )));
                    }
                } else {
                    // Handle other AST nodes on the right side
                    return self.execute_ast(right);
                }
            }
        }
        
        Ok(())
    }

    fn execute_sequence(&self, left: &AstNode, right: &AstNode) -> Result<()> {
        // Execute left command
        self.execute_ast(left)?;
        
        // Execute right command
        self.execute_ast(right)?;
        
        Ok(())
    }

    // Helper function to execute a command and capture its output
    fn execute_and_capture(&self, ast: &AstNode) -> Result<Vec<u8>> {
        if let AstNode::Command { name, flags, args } = ast {
            let output = Command::new(name)
                .args(flags)
                .args(args)
                .output()
                .map_err(|e| ShellError::execution(&format!("Failed to execute '{}': {}", name, e)))?;
            
            if !output.status.success() {
                return Err(ShellError::execution(&format!(
                    "Command '{}' failed with exit code: {}",
                    name,
                    output.status.code().unwrap_or(-1)
                )));
            }
            
            Ok(output.stdout)
        } else {
            // For non-command nodes, execute normally (output will go to stdout)
            self.execute_ast(ast)?;
            Ok(Vec::new()) // Return empty output for non-command nodes
        }
    }
}