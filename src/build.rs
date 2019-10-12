use std::process::Command;

pub fn run() {
    let output = Command::new("cargo").args(&["build"]).output().unwrap();

    if !output.status.success() {
        panic!("Bumping the version caused cargo build to fail. Stopping any further actions.");
    }
}
