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
            version: NewVersion::from_str(matches.value_of("version").unwrap_or("patch")).expect(
                "Invalid semver version, expected version or major, minor, patch",
            ),
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
        test_config(
            input,
            NewVersion::Replace(Version::parse("1.2.3").unwrap()),
            true,
            "v%s",
        )
    }
}
