use crate::command::Command;

pub struct RemoveCommand {
    full_path: Option<String>,
}

impl RemoveCommand {
    pub fn new(full_path: Option<String>) -> Self {
        RemoveCommand { full_path }
    }
}

impl Command for RemoveCommand {
    fn execute(&self) {
        if let Some(ref path) = self.full_path {
            println!("Remove flag is set");
            println!("Removing file...");
            std::fs::remove_file(path).expect("Unable to remove file");
        } else {
            println!("Cannot remove file because no valid input file was provided");
        }
    }
}

