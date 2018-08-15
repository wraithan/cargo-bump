const VERSION: &'static str = env!("CARGO_PKG_VERSION");

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use clap::{App, AppSettings, Arg, ArgMatches};
use semver::{SemVerError, Version};

pub fn get_config() -> Config {
    let matches = build_cli_parser().get_matches();
    Config::from_matches(matches)
}

fn build_cli_parser<'a, 'b>() -> App<'a, 'b> {
    App::new("cargo-bump")
        .version(VERSION)
        .version_message("Prints version of the cargo-bump utility itself")
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
        .arg(Arg::with_name("version").index(2).help(
            "Version should be a semver (https://semver.org/) string or the \
             position of the current version to increment: major, minor or patch."))
        .arg(Arg::with_name("major")
            .long("major")
            .conflicts_with_all(&["minor", "patch"])
            .help("Increment major version"))
        .arg(Arg::with_name("minor")
            .long("minor")
            .conflicts_with_all(&["major", "patch"])
            .help("Increment minor version"))
        .arg(Arg::with_name("patch")
            .long("patch")
            .conflicts_with_all(&["minor", "major"])
            .help("Increment patch version"))
        .arg(Arg::with_name("print")
            .long("print")
            .help("Only print the crate version as the output"))
}

fn search_up_for(root: &Path, target: &str) -> Option<PathBuf> {
    let mut current = root;

    loop {
        let potential = current.join(target);

        if fs::metadata(&potential).is_ok() {
            return Some(potential);
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub version: Option<NewVersion>,
    pub print_version_only: bool,
    pub root: PathBuf,
    pub manifest: PathBuf,
}

impl Config {
    fn from_matches(matches: ArgMatches) -> Config {
        let cwd = env::current_dir().unwrap();
        let manifest = search_up_for(&cwd, "Cargo.toml").expect("couldn't find Cargo.toml");
        let mut root = manifest.clone();
        root.pop();

        let print_version_only = matches.is_present("print");

        let version = if matches.is_present("major") {
            assert!(!matches.is_present("version"), "Version can't be specified when using --major");
            Some(NewVersion::Major)
        } else if matches.is_present("minor") {
            assert!(!matches.is_present("version"), "Version can't be specified when using --minor");
            Some(NewVersion::Minor)
        } else if matches.is_present("patch") {
            assert!(!matches.is_present("version"), "Version can't be specified when using --patch");
            Some(NewVersion::Patch)
        } else if let Some(arg) = matches.value_of("version") {
            Some(NewVersion::from_str(arg)
                .expect("Invalid semver version, expected version or major, minor, patch"))
        } else if print_version_only {
            None
        } else {
            Some(NewVersion::Patch)
        };

        Config {
            version,
            print_version_only,
            root: root,
            manifest: manifest,
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum NewVersion {
    Replace(Version),
    Major,
    Minor,
    Patch,
}

impl FromStr for NewVersion {
    type Err = SemVerError;
    fn from_str(input: &str) -> Result<NewVersion, Self::Err> {
        Ok(match input {
            "major" => NewVersion::Major,
            "minor" => NewVersion::Minor,
            "patch" => NewVersion::Patch,
            _ => NewVersion::Replace(Version::parse(input)?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{build_cli_parser, Config, NewVersion};
    use std::env;
    use semver::Version;

    fn test_config(input: Vec<&str>, version: NewVersion) {
        let parser = build_cli_parser();
        let root = env::current_dir().unwrap();
        let mut manifest = root.clone();
        manifest.push("Cargo.toml");
        let matches = parser.get_matches_from_safe(input).unwrap();
        let config = Config::from_matches(matches);
        assert_eq!(config.version, Some(version));
        assert_eq!(config.root, root);
        assert_eq!(config.manifest, manifest);
    }

    #[test]
    fn bump_arg_only() {
        let input = vec!["cargo-bump", "bump"];
        test_config(input, NewVersion::Patch)
    }

    #[test]
    fn version_arg_minor() {
        let input = vec!["cargo-bump", "bump", "minor"];
        test_config(input, NewVersion::Minor)
    }

    #[test]
    fn version_arg_string_good() {
        let input = vec!["cargo-bump", "bump", "1.2.3"];
        test_config(input, NewVersion::Replace(Version::parse("1.2.3").unwrap()))
    }
}
