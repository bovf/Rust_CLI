use crate::command::Command;
use std::fs::{self, metadata};
use std::path::Path;

pub struct DirCommand {
    full_path: Option<String>,
    dry_run: bool,
}

impl DirCommand {
    pub fn new(full_path: Option<String>, dry_run: bool) -> Self {
        DirCommand { full_path, dry_run }
    }
}

impl Command for DirCommand {
    fn execute(&self) {
        if let Some(ref path) = self.full_path {
            let metadata = metadata(path).unwrap();
            let is_dir = metadata.is_dir();
            if is_dir {
                if self.dry_run {
                    println!("Dry run: would list contents of directory {}", path);
                    list_dir_contents(path);
                } else {
                    println!("{} is a directory", path);
                }
            } else {
                println!("{} is a file", path);
            }
        } else {
            println!("No valid input provided");
        }
    }
}

fn list_dir_contents<P: AsRef<Path>>(path: P) {
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            list_dir_contents(path);
        } else {
            println!("{}", path.display());
        }
    }
}

