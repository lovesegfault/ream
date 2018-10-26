use ream_parser::scheme;

use std::io::prelude::*;
use std::io;

fn main() {
    let mut input = String::new();
    let parser = scheme::NumParser::new();

    loop {
        // Print prompt
        print!(">>> ");
        let _ = io::stdout().flush();

        // Read input
        io::stdin().read_line(&mut input).ok().expect("Failed to read input line");
        // Quit on exit
        match input.trim() {
            "exit" => break,
            line => {
                println!("{}", parser.parse(&line).expect("failed to parse expression"));
            }
        }
        // Clear input buffer
        input.clear();
    }

    println!("Thank you for using ream!");
}
