use std::io::{self};

use repl::start;

pub mod lexer;
pub mod repl;
pub mod token;
pub mod ast;

fn main() {
    println!("Hello, world!");
    println!("Please type in the code");
    start(io::stdin(), io::stdout())
}
