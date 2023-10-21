extern crate ferris_says;

use std::process::Command;
#[cfg(not(target_os="windows"))]
use std::os::unix::process::parent_id;
use ferris_says::say;
use std::io::{ stdout, BufWriter };

fn main() {
    let text = if cfg!(target_os = "windows") {
        "Sorry!\nI can't pinch the connection on Windows currently."
    } else {
        "Bye bye!\nI will pinch the connection."
    };
    fsay(text, 80);
    pinch();
}

/// Call ferris-says.
///
/// # Arguments
///
/// * `out` - What would says by Ferris
/// * `width` - Max length of line
fn fsay(out: &str, width: usize) {
    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}

/// Close current connection
fn pinch() {
    #[cfg(not(target_os="windows"))]
    Command::new("kill").args(["-s", "kill", &parent_id().to_string()]).output().expect("failed to pinch");
}
