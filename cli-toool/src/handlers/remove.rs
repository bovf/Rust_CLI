use std::fs;


pub fn handle_remove(full_path: &Option<String>) {
    if let Some(ref path) = full_path {
        println!("Remove flag is set");
        println!("Removing file...");
        fs::remove_file(path).expect("Unable to remove file");
    } else {
        println!("Cannot remove file because no valid input file was provided");
    }
}
