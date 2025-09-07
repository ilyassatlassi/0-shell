use super::commands;
use crate::ast::nodes::AstNode;
use crate::utils::error::{Result, ShellError};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

pub struct Executor;

impl Executor {
    pub fn execute_ast(&self, ast: &AstNode) -> Result<()> {
        self.execute_ast_with_streams(
            ast, 
            &mut io::stdin(),   // Use mutable references
            &mut io::stdout(),  // Use mutable references
            &mut io::stderr(),  // Use mutable references
        )
    }

    fn execute_ast_with_streams(
        &self,
        ast: &AstNode,
        stdin: &mut dyn Read,
        stdout: &mut dyn Write,
        stderr: &mut dyn Write,
    ) -> Result<()> {
        match ast {
            AstNode::Command { name, flags, args } => {
                self.execute_command(name, flags, args, stdin, stdout, stderr)
            }
            AstNode::Redirect { command, operator, file } => {
                self.execute_redirect(command, operator, file, stdin, stdout, stderr)
            }
            AstNode::Pipeline { left, right } => {
                self.execute_pipeline(left, right, stdin, stdout, stderr)
            }
            AstNode::Sequence { left, right } => {
                self.execute_sequence(left, right, stdin, stdout, stderr)
            }
        }
    }

    fn execute_command(
        &self,
        name: &str,
        flags: &[String],
        args: &[String],
        stdin: &mut dyn Read,
        stdout: &mut dyn Write,
        stderr: &mut dyn Write,
    ) -> Result<()> {
        // Check if it's a built-in command first
        if let Some(builtin_cmd) = commands::get_command(name) {
            // Combine flags and args for built-in commands
            let all_args: Vec<String> = flags.iter()
                .chain(args.iter())
                .cloned()
                .collect();
            
            // For built-in commands, just execute normally
            return builtin_cmd.execute(&all_args);
        }

        // For external commands, handle manually with the provided streams
        self.execute_external_with_streams(name, flags, args, stdin, stdout, stderr)
    }

    fn execute_external_with_streams(
        &self,
        name: &str,
        flags: &[String],
        args: &[String],
        stdin: &mut dyn Read,
        stdout: &mut dyn Write,
        stderr: &mut dyn Write,
    ) -> Result<()> {
        let mut cmd = Command::new(name);
        cmd.args(flags);
        cmd.args(args);

        // Set up I/O streams using the provided stream objects
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn()
            .map_err(|e| {
                if e.kind() == io::ErrorKind::NotFound {
                    ShellError::command_not_found(name)
                } else {
                    ShellError::execution(&format!("Failed to execute '{}': {}", name, e))
                }
            })?;

        // Write to child's stdin from our stdin
        if let Some(mut child_stdin) = child.stdin.take() {
            let mut buffer = Vec::new();
            stdin.read_to_end(&mut buffer)
                .map_err(|e| ShellError::execution(&format!("Failed to read from stdin: {}", e)))?;
            
            child_stdin.write_all(&buffer)
                .map_err(|e| ShellError::execution(&format!("Failed to write to process stdin: {}", e)))?;
        }

        let output = child.wait_with_output()
            .map_err(|e| ShellError::execution(&format!("Failed to get process output: {}", e)))?;

        // Write child's output to our provided streams
        stdout.write_all(&output.stdout)
            .map_err(|e| ShellError::execution(&format!("Failed to write stdout: {}", e)))?;
        
        stderr.write_all(&output.stderr)
            .map_err(|e| ShellError::execution(&format!("Failed to write stderr: {}", e)))?;

        if !output.status.success() {
            return Err(ShellError::execution(&format!(
                "Command '{}' failed with exit code: {}",
                name,
                output.status.code().unwrap_or(-1)
            )));
        }

        Ok(())
    }

    fn execute_redirect(
        &self,
        command: &AstNode,
        operator: &crate::types::tokens::Token,
        file: &str,
        stdin: &mut dyn Read,
        stdout: &mut dyn Write,
        stderr: &mut dyn Write,
    ) -> Result<()> {
        match operator {
            crate::types::tokens::Token::RedirectOut => {
                let mut file_handle = File::create(file)
                    .map_err(|e| ShellError::execution(&format!("Cannot create file '{}': {}", file, e)))?;
                
                self.execute_ast_with_streams(
                    command,
                    stdin,
                    &mut file_handle,  // Use file as stdout
                    stderr,
                )
            }
            crate::types::tokens::Token::RedirectAppend => {
                let mut file_handle = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file)
                    .map_err(|e| ShellError::execution(&format!("Cannot open file '{}': {}", file, e)))?;
                
                self.execute_ast_with_streams(
                    command,
                    stdin,
                    &mut file_handle,  // Use file as stdout
                    stderr,
                )
            }
            crate::types::tokens::Token::RedirectIn => {
                let mut file_handle = File::open(file)
                    .map_err(|e| ShellError::execution(&format!("Cannot open file '{}': {}", file, e)))?;
                
                self.execute_ast_with_streams(
                    command,
                    &mut file_handle,  // Use file as stdin
                    stdout,
                    stderr,
                )
            }
            _ => {
                Err(ShellError::execution(&format!(
                    "Unsupported redirection operator: {:?}",
                    operator
                )))
            }
        }
    }

    fn execute_pipeline(
        &self,
        left: &AstNode,
        right: &AstNode,
        stdin: &mut dyn Read,
        stdout: &mut dyn Write,
        stderr: &mut dyn Write,
    ) -> Result<()> {
        // Use a memory buffer for the pipe
        let mut buffer = Vec::new();
        
        // Execute left command to buffer
        {
            let mut buffer_writer = io::Cursor::new(&mut buffer);
            self.execute_ast_with_streams(
                left,
                stdin,
                &mut buffer_writer,
                stderr,
            )?;
        }

        // Execute right command from buffer
        {
            let mut buffer_reader = io::Cursor::new(&buffer);
            self.execute_ast_with_streams(
                right,
                &mut buffer_reader,
                stdout,
                stderr,
            )?;
        }

        Ok(())
    }

    fn execute_sequence(
        &self,
        left: &AstNode,
        right: &AstNode,
        stdin: &mut dyn Read,
        stdout: &mut dyn Write,
        stderr: &mut dyn Write,
    ) -> Result<()> {
        self.execute_ast_with_streams(left, stdin, stdout, stderr)?;
        self.execute_ast_with_streams(right, stdin, stdout, stderr)
    }
}