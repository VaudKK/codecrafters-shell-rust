use std::{env, io::Error, path::Path, process::{self, Child}};

pub fn get_type(command: &str){
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

pub fn command_exists(command: &str) -> Option<String>{
    let path_vect = get_paths();

    for path in path_vect {
        let mut full_path = Path::new(&path).join(command);

        //for windows set extension to exe
        //for unix set extension to ""
        full_path.set_extension("");

        if full_path.exists(){
            return Some(String::from(full_path.to_str().unwrap()))
        }
    }

    None
}

pub fn get_paths() -> Vec<String>{
    let path_var = env::var("PATH").unwrap_or_default();

    //for windows use ;

    path_var.split(":").map(String::from).collect()
}


pub fn run_system_command(command: &str, args: Vec<&str>) -> Result<Child,Error>{
    process::Command::new(command)
        .args(args.into_iter())
        .stdout(std::io::stdout())
        .spawn()
}

pub fn handle_system_cmd_result(rslt: Result<Child,Error>){
    let mut result = rslt.unwrap_or_else(|err| {
        eprintln!("Error executing command: {err}");
        process::exit(0);
    });

    result.wait().unwrap_or_else(|err|{
        eprintln!("Error executing child process: {err}");
        process::exit(0);
    });
}

pub fn handle_exit(commands: Vec<&str>){
    if commands.len() > 1 {
        process::exit(commands[1].parse::<i32>().unwrap());
    }else{
        process::exit(0);
    }
}

pub fn handle_unknown_command(tokens: Vec<&str>){
    let command = command_exists(tokens[0]);
    match command {
        Some(cmd) => {
            let child = run_system_command(&cmd, tokens[1..].to_vec());
            handle_system_cmd_result(child);
        },
        None => println!("{}: command not found", tokens[0])
    }
}

pub fn handle_cd_command(tokens: Vec<&str>){
    let _args: Vec<&str>;
    let _rslt = match tokens[1..] {
        ["~"] => run_system_command("cd", tokens[1..2].to_vec()),
        _ => run_system_command("cd", tokens[1..].to_vec())
    };

    if tokens.len() > 1 {
        if std::env::set_current_dir(Path::new(tokens[1])).is_err() {
            println!("{}: No such file or directory", tokens[1]);
        }
    }  
}