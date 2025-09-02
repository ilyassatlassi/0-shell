mod utils;

use std::io::{self, Write};
use utils::error::ShellError;

fn main() -> Result<(), ShellError> {
    // let mut executor = Executor::new();
    
    loop {
        print!("$ ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if input.trim().is_empty() {
            continue;
        }
        
        if input.trim() == "exit" {
            println!("Goodbye!");
            break;
        }
        
        // Execute the command
        // match execute_command(&input, &mut executor) {
        //     Ok(_) => {}
        //     Err(e) => eprintln!("Error: {}", e),
        // }
    }
    
    Ok(())
}

// fn execute_command(input: &str, executor: &mut Executor) -> Result<(), ShellError> {
//     // Lexing (tokenization)
//     let mut lexer = Lexer::new(input.to_string());
//     let tokens = lexer.tokenize()?;
    
//     println!("Tokens: {:?}", tokens);
    
//     // TODO: Add parsing and execution in next steps
//     // let mut parser = Parser::new(tokens);
//     // let ast = parser.parse()?;
//     // executor.execute(&ast)?;
    
//     Ok(())
// }