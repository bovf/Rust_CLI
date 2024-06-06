extern crate clap;
mod arguments;
mod command;
mod handlers;

use command::Command;
use handlers::input::InputCommand;
use handlers::remove::RemoveCommand;
use handlers::dir::DirCommand;

fn main() {
    let matches = arguments::build_app().get_matches();
    
    let input_command = matches.value_of("input").map(|input| InputCommand::new(input.to_string()));
    if let Some(command) = input_command {
        command.execute();
    } else {
        println!("No input file provided");
    }

    let full_path = matches.value_of("input").map(|input| get_file_full_path(input));

    if matches.is_present("remove") {
        let command = RemoveCommand::new(full_path.clone());
        command.execute();
    }
    if matches.is_present("dir") {
        println!("Dir flag is set");
        let command = DirCommand::new(full_path.clone());
        command.execute();
    }
}

fn get_file_full_path(filepath: &str) -> String {
    let full_path = std::fs::canonicalize(filepath).unwrap();
    full_path.to_str().unwrap().to_string()
}
