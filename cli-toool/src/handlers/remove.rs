use crate::command::Command;
use dialoguer::Confirm;

pub struct RemoveCommand {
    full_path: Option<String>,
    dry_run: bool,
}

impl RemoveCommand {
    pub fn new(full_path: Option<String>, dry_run: bool) -> Self {
        RemoveCommand { full_path, dry_run }
    }
}

impl Command for RemoveCommand {
    fn execute(&self) {
        if self.dry_run {
            println!("Dry run: would remove {:?}", self.full_path);
            return;
        }

        if let Some(ref path) = self.full_path {
            if Confirm::new().with_prompt(format!("Are you sure you want to remove {}", path)).interact().unwrap() {
                println!("Removing file...");
                std::fs::remove_file(path).expect("Unable to remove file");
            } else {
                println!("Remove operation cancelled.");
            }
        } else {
            println!("Cannot remove file because no valid input file was provided");
        }
    }
}
