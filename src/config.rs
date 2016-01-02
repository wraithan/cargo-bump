const VERSION: &'static str = env!("CARGO_PKG_VERSION");

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use clap::{App, AppSettings, Arg, ArgMatches};
use semver::Version;

pub fn get_config() -> Config {
    let matches = build_cli_parser().get_matches();
    Config::from_matches(matches)
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

fn search_up_for(root: &Path, target: &str) -> Option<PathBuf> {
    let mut current = root;

    loop {
        let potential = current.join(target);

        if fs::metadata(&potential).is_ok() {
            return Some(potential)
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => return None
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
pub struct Config {
    pub version: NewVersion,
    pub git_tag: bool,
    pub message: String,
    pub root: PathBuf,
    pub manifest: PathBuf
}

impl Config {
    fn from_matches(matches: ArgMatches) -> Config {
        let cwd = env::current_dir().unwrap();
        let manifest = search_up_for(&cwd, "Cargo.toml").unwrap_or_else(|| panic!("couldn't find Cargo.toml"));
        let mut root = manifest.clone();
        root.pop();
        let has_git = has_git_on_path();
        let has_git_dir = has_git && search_up_for(&root, ".git").is_some();
        Config{
            version: NewVersion::from_str(matches.value_of("version").unwrap_or("patch")),
            git_tag: has_git_dir && !matches.is_present("no_git"),
            message: matches.value_of("message").unwrap_or("v%s").to_owned(),
            root: root,
            manifest: manifest
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum NewVersion {
    String(Version),
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
            _ => {
                NewVersion::String(Version::parse(input).unwrap())
            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NewVersion, Config, build_cli_parser};
    use std::env;
    use semver::Version;

    fn test_config(input: Vec<&str>, version: NewVersion, git_tag: bool, message: &str) {
        let parser = build_cli_parser();
        let root = env::current_dir().unwrap();
        let mut manifest = root.clone();
        manifest.push("Cargo.toml");
        let matches = parser.get_matches_from_safe(input).unwrap();
        let config = Config::from_matches(matches);
        assert_eq!(config.version, version);
        assert_eq!(config.git_tag, git_tag);
        assert_eq!(&config.message, message);
        assert_eq!(config.root, root);
        assert_eq!(config.manifest, manifest);
    }

    #[test]
    fn bump_arg_only() {
        let input = vec!["cargo-bump", "bump"];
        test_config(input, NewVersion::Patch, true, "v%s")
    }

    #[test]
    fn version_arg_minor() {
        let input = vec!["cargo-bump", "bump", "minor"];
        test_config(input, NewVersion::Minor, true, "v%s")
    }

    #[test]
    fn version_arg_string_good() {
        let input = vec!["cargo-bump", "bump", "1.2.3"];
        test_config(input, NewVersion::String(Version::parse("1.2.3").unwrap()), true, "v%s")
    }

    #[test]
    fn git_tag_version_set() {
        let input = vec!["cargo-bump", "bump", "--no-git-tag-version"];
        test_config(input, NewVersion::Patch, false, "v%s")
    }

    #[test]
    fn commit_message_set() {
        let input = vec!["cargo-bump", "bump", "-m", "releasing version %s"];
        test_config(input, NewVersion::Patch, true, "releasing version %s")
    }

}
