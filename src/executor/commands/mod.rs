pub mod echo;
pub mod cd;
pub mod ls;
pub mod pwd;
pub mod cat;
pub mod cp;
pub mod rm;
pub mod mv;
pub mod mkdir;
pub mod exit;

use crate::utils::error::Result;
use std::io::{Read, Write};

pub trait Command {
    fn execute(
        &self, 
        args: &[String], 
        stdin: &mut dyn Read,
        stdout: &mut dyn Write, 
        stderr: &mut dyn Write
    ) -> Result<()>;
}


pub fn get_command(name: &str) -> Option<Box<dyn Command>> {
    match name {
        "echo" => Some(Box::new(echo::Echo)),
        // "cd" => Some(Box::new(cd::Cd)),
        // "ls" => Some(Box::new(ls::Ls)),
        // "pwd" => Some(Box::new(pwd::Pwd)),
        // "cat" => Some(Box::new(cat::Cat)),
        // "cp" => Some(Box::new(cp::Cp)),
        // "rm" => Some(Box::new(rm::Rm)),
        // "mv" => Some(Box::new(mv::Mv)),
        // "mkdir" => Some(Box::new(mkdir::Mkdir)),
        // "exit" => Some(Box::new(exit::Exit)),
        _ => None,
    }
}