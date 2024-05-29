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
        &_ => {
            let command = command_exists(tokens[0]);
            match command {
                Some(cmd) => {
                    let mut child = process::Command::new(cmd)
                    .args(tokens[1..].into_iter())
                    .stdout(std::io::stdout())
                    .spawn()
                    .unwrap();
                child.wait().unwrap();
                },
                None => println!("{}: command not found", tokens[0])
            }
        }
    }
}

fn get_type(command: &str){
    match command {
        "exit" | "echo" | "type" => println!("{} is a shell builtin", command),
        &_ => {
            let path = command_exists(command);

            match path {
                Some(exists) => println!("{} is {}", command,exists),
                None => println!("{} not found", command)
            }
        }
    }
}

fn command_exists(command: &str) -> Option<String>{
    let path_vect = get_paths();

    for path in path_vect {
        let mut full_path = Path::new(&path).join(command);
        full_path.set_extension("");

        if full_path.exists(){
            return Some(String::from(full_path.to_str().unwrap()))
        }
    }

    None
}

fn get_paths() -> Vec<String>{
    let path_var = env::var("PATH").unwrap_or_default();

    //for windows use ;
    path_var.split(":").map(String::from).collect()
}
