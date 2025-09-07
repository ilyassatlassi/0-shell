use crate::utils::error::Result;

pub struct Echo;

impl super::Command for Echo {
    fn execute(&self, args: &[String]) -> Result<()> {
        if args.is_empty() {
            println!();
            return Ok(());
        }

        // Process each argument and handle escape sequences
        let mut output = String::new();
        let mut first = true;
        
        for arg in args {
            if !first {
                output.push(' ');
            }
            
            // Process escape sequences in each argument
            output.push_str(&self.process_escape_sequences(arg));
            first = false;
        }
        
        println!("{}", output);
        Ok(())
    }
}

impl Echo {
    fn process_escape_sequences(&self, input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        let mut escaped = false;
        
        while let Some(c) = chars.next() {
            if escaped {
                match c {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    '\'' => result.push('\''),
                    '0' => result.push('\0'),
                    _ => {
                        // Unknown escape sequence, treat literally
                        result.push('\\');
                        result.push(c);
                    }
                }
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else {
                result.push(c);
            }
        }
        
        // Handle trailing backslash
        if escaped {
            result.push('\\');
        }
        
        result
    }
}