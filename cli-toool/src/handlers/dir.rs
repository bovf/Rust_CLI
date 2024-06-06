use crate::command::Command;
use std::fs::metadata;

pub struct DirCommand {
    full_path: Option<String>,
}

impl DirCommand {
    pub fn new(full_path: Option<String>) -> Self {
        DirCommand { full_path }
    }
}

impl Command for DirCommand {
    fn execute(&self) {
        if let Some(ref path) = self.full_path {
            let metadata = metadata(path).unwrap();
            let is_dir = metadata.is_dir();
            if is_dir {
                println!("{} is a directory", path);
            } else {
                println!("{} is a file", path);
            }
        } else {
            println!("No valid input provided");
        }
    }
}

