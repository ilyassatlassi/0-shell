use crate::types::tokens::{Token, TokenWithPos};
use crate::utils::error::{Result, ShellError};

pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input.trim().to_string(),
            position: 0,
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.current_char = Some(self.input.chars().nth(self.position).unwrap());
            self.position += 1;
        } else {
            self.current_char = None;
        }
    }

    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input.chars().nth(self.position).unwrap())
        } else {
            None
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<TokenWithPos>> {
        let mut tokens = Vec::new();
        let mut is_start_of_command = true;

        while self.current_char.is_some() {
            let start = self.position - 1;

            // Skip whitespace
            if self.current_char.unwrap().is_whitespace() {
                self.advance();
                continue;
            }

            let token = match self.current_char.unwrap() {
                '|' => {
                    self.advance();
                    Token::Pipe
                }
                '>' => {
                    self.advance();
                    if self.current_char == Some('>') {
                        self.advance();
                        Token::RedirectAppend
                    } else {
                        Token::RedirectOut
                    }
                }
                '<' => {
                    self.advance();
                    Token::RedirectIn
                }
                ';' => {
                    self.advance();
                    Token::Semicolon
                }
                '\'' | '"' => {
                    let word = self.parse_quoted_string()?;
                    self.classify_word(word, is_start_of_command)
                }
                _ => {
                    let word = self.parse_word()?;
                    self.classify_word(word, is_start_of_command)
                }
            };

            // Update state for next token
            is_start_of_command = match &token {
                Token::Semicolon | Token::Pipe => true,
                _ => false,
            };

            let end = self.position - 1;
            tokens.push(TokenWithPos { token, start, end });
        }

        Ok(tokens)
    }

    fn classify_word(&self, word: String, is_start_of_command: bool) -> Token {
        if is_start_of_command {
            Token::Command(word)
        } else if word.starts_with('-') {
            Token::Flag(word)
        } else {
            Token::Argument(word)
        }
    }

    fn parse_quoted_string(&mut self) -> Result<String> {
        let quote_char = self.current_char.unwrap();
        let quote_start = self.position - 1;
        self.advance(); // Skip opening quote

        let mut content = String::new();
        let mut escaped = false;

        while let Some(c) = self.current_char {
            if escaped {
                content.push(c);
                escaped = false;
                self.advance();
            } else if c == '\\' {
                escaped = true;
                self.advance();
            } else if c == quote_char {
                self.advance(); // Skip closing quote
                return Ok(content);
            } else {
                content.push(c);
                self.advance();
            }
        }

        // If we get here, we reached EOF without closing quote
        Err(ShellError::lexer("Unclosed quote", quote_start))
    }

    fn parse_word(&mut self) -> Result<String> {
        let mut word = String::new();
        let mut escaped = false;

        while let Some(c) = self.current_char {
            if escaped {
                word.push(c);
                escaped = false;
                self.advance();
            } else if c == '\\' {
                escaped = true;
                self.advance();
            } else if c.is_whitespace() || matches!(c, '|' | '>' | '<' | ';') {
                break;
            } else {
                word.push(c);
                self.advance();
            }
        }
        println!("{}", word);

        Ok(word)
    }
}
