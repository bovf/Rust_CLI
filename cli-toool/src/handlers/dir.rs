use std::fs::metadata;

pub fn handle_dir(full_path: &Option<String>) -> Option<bool> {
    if let Some(ref path) = full_path {
        let metadata = metadata(path).unwrap();
        let is_dir = metadata.is_dir();
        if metadata.is_dir() {
            println!("{} is a directory", path);
        } else {
            println!("{} is a file", path);
        }
        return Some(is_dir);
    } else {
        println!("No valid input provided");
        return Some(false);
    }
}
