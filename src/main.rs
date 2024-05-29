#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin =  io::stdin();
        let mut input = String::new();

        stdin.read_line(&mut input).unwrap();
        
        let input = input.trim();
        run(input.to_string());
    }
}

fn run(command: String){
    let tokens: Vec<&str> = command.split_whitespace().collect();

    match tokens[0] {
        "exit" => process::exit(tokens[1].parse::<i32>().unwrap()),
        "echo" => print!("{}\n", tokens[1..].join(" ")),
        "type" =>  get_type(tokens[1]),
        &_ => println!("{}: command not found",tokens[0])
    }
}

fn get_type(command: &str){
    match command {
        "exit" | "echo" => println!("{} is a shell builtin", command),
        &_ => println!("{} not found",command)
    }
}
