use crate::command::Command;

pub struct InputCommand {
    input: String,
}

impl InputCommand {
    pub fn new(input: String) -> Self {
        InputCommand { input }
    }
}

impl Command for InputCommand {
    fn execute(&self) {
        if file_exists(&self.input) {
            println!("Input file: {}", self.input);
            let full_path = get_file_full_path(&self.input);
            println!("Full path: {}", full_path);
        } else {
            println!("File does not exist");
        }
    }
}

fn file_exists(filepath: &str) -> bool {
    std::fs::metadata(filepath).is_ok()
}

fn get_file_full_path(filepath: &str) -> String {
    let full_path = std::fs::canonicalize(filepath).unwrap();
    full_path.to_str().unwrap().to_string()
}

