use std::process::Command;

pub fn check() {
    let output = Command::new("git")
        .args(&["status", "--porcelain"])
        .output()
        .expect("This tool requires git. Please install git and try again.");
    if !output.stdout.is_empty() {
        panic!("Working directory is not clean. Please commit changes before trying to update the version.");
    }
}

pub fn tag(version: &str) {
    Command::new("git")
        .args(&["tag", "-am", version, version])
        .status()
        .expect("Something went wrong when creating a git tag.");
}

pub fn commit(version: &str) {
    Command::new("git")
        .args(&["commit", "-am", version])
        .status()
        .expect("Something went wrong trying to commit the new version.");
}

pub fn commit_and_tag(version: &str) {
    commit(version);
    tag(version);
}

pub fn log() -> String {
    let output = Command::new("git")
        .args(&["log", "-1", "--pretty=%B"])
        .output()
        .expect("Something went wrong trying to read the most recent commit message.")
        .stdout;
    String::from_utf8(output).expect("Commit message was not valid UTF-8")
}
