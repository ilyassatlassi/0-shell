use crate::types::tokens::{Token, TokenWithPos};
use crate::ast::nodes::AstNode;
use crate::utils::error::{ShellError, Result};

pub struct Parser {
    tokens: Vec<TokenWithPos>,
    position: usize,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithPos>) -> Self {
        let mut parser = Parser {
            tokens,
            position: 0,
            current_token: None,
        };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.current_token = Some(self.tokens[self.position].token.clone());
            self.position += 1;
        } else {
            self.current_token = None;
        }
    }

    fn peek(&self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            Some(&self.tokens[self.position].token)
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Result<AstNode> {
        self.parse_line()
    }

    // line : pipeline (';' pipeline)*
    fn parse_line(&mut self) -> Result<AstNode> {
        let mut left = self.parse_pipeline()?;

        // Handle sequence: cmd1 ; cmd2 ; cmd3
        while let Some(Token::Semicolon) = &self.current_token {
            self.advance();
            let right = self.parse_pipeline()?;
            left = AstNode::Sequence {
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // pipeline : command ('|' command)*
    fn parse_pipeline(&mut self) -> Result<AstNode> {
        let mut left = self.parse_command()?;

        // Handle pipes: cmd1 | cmd2 | cmd3
        while let Some(Token::Pipe) = &self.current_token {
            self.advance();
            let right = self.parse_command()?;
            left = AstNode::Pipeline {
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // command : simple_command (redirection)*
    fn parse_command(&mut self) -> Result<AstNode> {
        let mut command = self.parse_simple_command()?;

        // Parse redirections: cmd > file, cmd < file, cmd >> file
        while let Some(token) = &self.current_token {
            match token {
                Token::RedirectOut | Token::RedirectAppend | Token::RedirectIn => {
                    let operator = token.clone();
                    self.advance();
                    let file = self.parse_redirect_file()?;
                    command = AstNode::Redirect {
                        command: Box::new(command),
                        operator,
                        file,
                    };
                }
                _ => break,
            }
        }

        Ok(command)
    }

    fn parse_redirect_file(&mut self) -> Result<String> {
        if let Some(Token::Argument(file)) = &self.current_token {
            let file_clone = file.clone();
            self.advance();
            Ok(file_clone)
        } else {
            Err(ShellError::parser("Expected filename after redirection operator"))
        }
    }

    // simple_command : command (flag | argument)*
    fn parse_simple_command(&mut self) -> Result<AstNode> {
        // Parse command name (must be first token)
        let name = if let Some(Token::Command(cmd)) = &self.current_token {
            let cmd_clone = cmd.clone();
            self.advance();
            cmd_clone
        } else {
            return Err(ShellError::parser("Expected command name"));
        };

        let mut flags = Vec::new();
        let mut args = Vec::new();

        // Parse flags and arguments until operator or end
        while let Some(token) = &self.current_token {
            match token {
                Token::Flag(flag) => {
                    flags.push(flag.clone());
                    self.advance();
                }
                Token::Argument(arg) => {
                    args.push(arg.clone());
                    self.advance();
                }
                _ => break, // Stop at operators: |, ;, >, <, >>
            }
        }

        Ok(AstNode::Command {
            name,
            flags,
            args,
        })
    }
}