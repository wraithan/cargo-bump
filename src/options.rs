const VERSION: &'static str = env!("CARGO_PKG_VERSION");

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use cargo::util::important_paths;
use clap::{App, AppSettings, Arg, ArgMatches};

pub fn get_options() -> Options {
    let matches = build_cli_parser().get_matches();
    Options::from_matches(matches)
}

// Has crazy lifetimes due to how clap works. The app return value is never
// exposed outside of this module, so I'm not very concerned.
fn build_cli_parser<'a>() -> App<'a, 'a, 'a, 'a, 'a, 'a> {
    App::new("cargo-bump")
        .version(VERSION)
        .author("Wraithan McCarroll <xwraithanx@gmail.com>")
        .usage("cargo bump [FLAGS] [<version> | major | minor | patch]")
        .about("Increments the version number in Cargo.toml as specified.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version_short("v")
        .arg(Arg::with_name("bump")
             .possible_value("bump")
             .index(1)
             .required(true)
             .hidden(true))
        .arg(Arg::with_name("no_git")
             .long("no-git-tag-version")
             .help("Disables the git iteractions"))
        .arg(Arg::with_name("message")
             .short("m")
             .long("message")
             .takes_value(true)
             .help("Commit message, %s will be replaced with new version number"))
        .arg(Arg::with_name("version")
             .index(2)
             .help("Version should be a semver (https://semver.org/) string or the
               position of the current version to increment: major, minor or patch."))
}

fn find_git_dir(root: &Path) -> bool {
    let mut current = root;

    loop {
        let potential = current.join(".git");

        if fs::metadata(&potential).is_ok() {
            return true
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => return false
        }
    }
}

fn has_git_on_path() -> bool {
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            let potential = path.join("git");
            if fs::metadata(&potential).is_ok() {
                return true
            }
        }
    }
    false
}

#[derive(Debug, PartialEq)]
pub struct Options {
    version: NewVersion,
    git_tag: bool,
    message: String,
    root: PathBuf,
    manifest: PathBuf
}

impl Options {
    fn from_matches(matches: ArgMatches) -> Options {
        let cwd = env::current_dir().unwrap();
        let manifest = important_paths::find_project_manifest(&cwd, "Cargo.toml").unwrap();
        let mut root = manifest.clone();
        root.pop();
        let has_git = has_git_on_path();
        let has_git_dir = has_git && find_git_dir(&root);
        Options{
            version: NewVersion::from_str(matches.value_of("version").unwrap_or("patch")),
            git_tag: has_git_dir && !matches.is_present("no_git"),
            message: matches.value_of("message").unwrap_or("tagging version %s").to_owned(),
            root: root,
            manifest: manifest
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum NewVersion {
    String(String),
    Major,
    Minor,
    Patch
}

impl NewVersion {
    fn from_str(input: &str) -> NewVersion {
        match input {
            "major" => NewVersion::Major,
            "minor" => NewVersion::Minor,
            "patch" => NewVersion::Patch,
            _ => NewVersion::String(input.to_owned())

        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NewVersion, Options, build_cli_parser};
    use std::env;

    fn test_options(input: Vec<&str>, version: NewVersion, git_tag: bool, message: &str) {
        let parser = build_cli_parser();
        let root = env::current_dir().unwrap();
        let mut manifest = root.clone();
        manifest.push("Cargo.toml");
        let matches = parser.get_matches_from_safe(input).unwrap();
        let options = Options::from_matches(matches);
        assert_eq!(options.version, version);
        assert_eq!(options.git_tag, git_tag);
        assert_eq!(&options.message, message);
        assert_eq!(options.root, root);
        assert_eq!(options.manifest, manifest);
    }

    #[test]
    fn bump_arg_only() {
        let input = vec!["cargo-bump", "bump"];
        test_options(input, NewVersion::Patch, true, "tagging version %s")
    }

    #[test]
    fn version_arg_minor() {
        let input = vec!["cargo-bump", "bump", "minor"];
        test_options(input, NewVersion::Minor, true, "tagging version %s")
    }

    #[test]
    fn git_tag_version_set() {
        let input = vec!["cargo-bump", "bump", "--no-git-tag-version"];
        test_options(input, NewVersion::Patch, false, "tagging version %s")
    }

    #[test]
    fn commit_message_set() {
        let input = vec!["cargo-bump", "bump", "-m", "stuff and things"];
        test_options(input, NewVersion::Patch, true, "stuff and things")
    }
}
