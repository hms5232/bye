extern crate ferris_says;

use std::process::Command;
use std::os::unix::process::parent_id;
use ferris_says::say;
use std::io::{ stdout, BufWriter };

fn main() {
    let out = "Bye bye!\nI will pinch the connection.";
    let width = 80;
    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();

    Command::new("kill").args(["-s", "kill", &parent_id().to_string()]).output().expect("failed to execute process in bash");
}
