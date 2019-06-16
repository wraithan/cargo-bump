use std::process::Command;

pub fn git_check() {
    let output = Command::new("git")
        .args(&["status", "--porcelain"])
        .output()
        .expect("This tool requires git. Please install git and try again.");
    if !output.stdout.is_empty() {
        panic!("Working directory is not clean. Please commit changes before trying to update the version.");
    }
}

pub fn git_tag(version: &str) {
    Command::new("git")
        .args(&["tag", "-am", version, version])
        .status()
        .expect("Something went wrong when creating a git tag.");
}

pub fn git_commit(version: &str) {
    Command::new("git")
        .args(&["commit", "-am", version])
        .status()
        .expect("Something went wrong trying to commit the new version.");
}

pub fn git_commit_and_tag(version: &str) {
    git_commit(version);
    git_tag(version);
}
