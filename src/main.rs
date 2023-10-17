use std::process::Command;
use std::os::unix::process::parent_id;

fn main() {
    Command::new("kill").args(["-s", "kill", &parent_id().to_string()]).output().expect("failed to execute process in bash");
}
