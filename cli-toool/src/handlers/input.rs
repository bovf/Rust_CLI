use std::fs;


pub fn handle_input(input: &str) -> Option<String> {
    if file_exists(input) {
        println!("Input file: {}", input);
        let full_path = get_file_full_path(input);
        println!("Full path: {}", full_path);
        Some(full_path)
    } else {
        println!("File does not exist");
        None
    }
}

fn file_exists(filepath: &str) -> bool {
    fs::metadata(filepath).is_ok()
}

fn get_file_full_path(filepath: &str) -> String {
    let full_path = fs::canonicalize(filepath).unwrap();
    full_path.to_str().unwrap().to_string()
}
