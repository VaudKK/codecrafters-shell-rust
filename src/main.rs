use std::io::{self, Write};

use shell_starter_rust as util;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        
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
        "cd" => util::handle_cd_command(tokens),
        &_ => util::handle_unknown_command(tokens)
    };
}
