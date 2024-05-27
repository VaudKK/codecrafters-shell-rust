#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();
        print!("{command}: command not found\n");

        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
