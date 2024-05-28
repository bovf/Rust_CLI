extern crate clap;
use std::fs;
use clap::{Arg, App};

fn main() {
    let matches = App::new("cli-tool")
        .arg(Arg::with_name("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Sets the input file to use")
            .takes_value(true))
        .arg(Arg::with_name("remove")
            .short('r')
            .long("remove")
            .help("If set will delete the input file")
            .takes_value(false))
        .get_matches();
    
    let mut full_path: Option<String> = None;

    if let Some(input) = matches.value_of("input") {
        if file_exists(input) {
            println!("Input file: {}", input);
            full_path = Some(get_file_full_path(input));
            println!("Full path: {}", full_path.as_ref().unwrap());
        } else {
            println!("File does not exist");
        }
    } else {
        println!("No input file provided");
    }

    if matches.is_present("remove") {
        if let Some(ref path) = full_path {
            println!("Remove flag is set");
            println!("Removing file...");
            fs::remove_file(path).expect("Unable to remove file");
        } else {
            println!("Cannot remove file because no valid input file was provided");
        }
    }
}

fn file_exists(filepath: &str) -> bool {
    fs::metadata(filepath).is_ok()
}

fn get_file_full_path(filepath: &str) -> String {
    let full_path = fs::canonicalize(filepath).unwrap();
    full_path.to_str().unwrap().to_string()
}
