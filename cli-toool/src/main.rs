extern crate clap;
mod arguments;
mod handlers; 


use handlers::input::handle_input;
use handlers::remove::handle_remove;
use handlers::dir::handle_dir;

fn main() {
    let matches = arguments::build_app().get_matches();
    
    let full_path = if let Some(input) = matches.value_of("input") {
        handle_input(input)
    } else {
        println!("No input file provided");
        None
    };

    if matches.is_present("remove") {
        handle_remove(&full_path);
    }
    if matches.is_present("dir") {
        println!("Dir flag is set");
        handle_dir(&full_path);
    }

}
