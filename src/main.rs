use std::io::{self, Write};

use shell_starter_rust::{self as util, handle_unknown_command};

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
        "exit" => util::handle_exit(tokens),
        "echo" => print!("{}\n", tokens[1..].join(" ")),
        "type" =>  util::get_type(tokens[1]),
        "pwd" => {
            let proc = util::run_system_command("pwd", Vec::new());
            util::handle_system_cmd_result(proc);
        },
        "cd" => {
            let args: Vec<&str>;
            if tokens.len() == 1 {
                args = Vec::new();
            }else{
                args = tokens[1..].to_vec();
            }

            let proc = util::run_system_command("cd",args);
            util::handle_system_cmd_result(proc);
        },
        &_ => handle_unknown_command(tokens)
    };
}
