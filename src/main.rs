#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path::Path, process};

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
        "exit" | "echo" | "type" => println!("{} is a shell builtin", command),
        &_ => get_system_command(command)
    }
}

fn get_system_command(command: &str){
    let path_var = env::var("PATH").unwrap_or_default();
    
    let path_vect: Vec<&str> = path_var.split(':').collect();

    let mut found = false;
    for path in path_vect {
        let mut full_path = Path::new(path).join(command);
        full_path.set_extension("");

        if full_path.exists(){
            println!("{} is {}", command, full_path.display());
            found = true;
            break;
        }
    }

    if !found {
        println!("{}: not found", command);
    }

}
