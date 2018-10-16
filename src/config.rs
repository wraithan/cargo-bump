const VERSION: &'static str = env!("CARGO_PKG_VERSION");

use clap::{App, AppSettings, Arg, ArgMatches};
use semver::{Identifier, SemVerError, Version};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn get_config() -> Config {
    let matches = build_cli_parser().get_matches();
    Config::from_matches(matches)
}

fn build_cli_parser<'a, 'b>() -> App<'a, 'b> {
    App::new("cargo-bump")
        .version(VERSION)
        .author("Wraithan McCarroll <xwraithanx@gmail.com>")
        .usage("cargo bump [FLAGS] [<version> | major | minor | patch]")
        .about("Increments the version number in Cargo.toml as specified.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version_short("v")
        .arg(
            Arg::with_name("bump")
                .possible_value("bump")
                .index(1)
                .required(true)
                .hidden(true),
        )
        .arg(Arg::with_name("version").index(2).help(
            "Version should be a semver (https://semver.org/) string or the \
             position of the current version to increment: major, minor or patch.",
        ))
        .arg(
            Arg::with_name("pre-release")
                .short("p")
                .long("pre-release")
                .takes_value(true)
                .help("Optional pre-release information."),
        )
        .arg(
            Arg::with_name("metadata")
                .short("m")
                .long("metadata")
                .takes_value(true)
                .help("Optional metadata for this version."),
        )
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
    pub version: NewVersion,
    pub metadata: Option<Vec<Identifier>>,
    pub pre_release: Option<Vec<Identifier>>,

    pub root: PathBuf,
    pub manifest: PathBuf,
}

impl Config {
    fn from_matches(matches: ArgMatches) -> Config {
        let cwd = env::current_dir().unwrap();
        let manifest =
            search_up_for(&cwd, "Cargo.toml").unwrap_or_else(|| panic!("couldn't find Cargo.toml"));
        let mut root = manifest.clone();
        root.pop();
        Config {
            version: NewVersion::from_str(matches.value_of("version").unwrap_or("patch"))
                .expect("Invalid semver version, expected version or major, minor, patch"),
            metadata: matches.value_of("metadata").map(parse_identifiers),
            pre_release: matches.value_of("pre-release").map(parse_identifiers),
            root: root,
            manifest: manifest,
        }
    }
}

fn parse_identifiers(value: &str) -> Vec<Identifier> {
    value
        .split('.')
        .map(|identifier| {
            if let Ok(i) = identifier.parse() {
                Identifier::Numeric(i)
            } else {
                Identifier::AlphaNumeric(identifier.to_string())
            }
        })
        .collect()
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
    use semver::Version;
    use std::env;

    fn test_config(input: Vec<&str>, version: NewVersion) {
        let parser = build_cli_parser();
        let root = env::current_dir().unwrap();
        let mut manifest = root.clone();
        manifest.push("Cargo.toml");
        let matches = parser.get_matches_from_safe(input).unwrap();
        let config = Config::from_matches(matches);
        assert_eq!(config.version, version);
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
