const VERSION: &str = env!("CARGO_PKG_VERSION");

use cargo_metadata::MetadataCommand;
use clap::{App, AppSettings, Arg, ArgMatches};
use semver::{Identifier, SemVerError, Version};
use std::path::PathBuf;
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
        .arg(
            Arg::with_name("manifest-path")
                .long("manifest-path")
                .value_name("PATH")
                .takes_value(true),
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

#[derive(Debug, PartialEq)]
pub struct Config {
    pub version: NewVersion,
    pub metadata: Option<Vec<Identifier>>,
    pub pre_release: Option<Vec<Identifier>>,

    pub manifest: PathBuf,
}

impl Config {
    fn from_matches(matches: ArgMatches) -> Config {
        let version = NewVersion::from_str(matches.value_of("version").unwrap_or("patch"))
            .expect("Invalid semver version, expected version or major, minor, patch");

        let mut metadata_cmd = MetadataCommand::new();
        if let Some(path) = matches.value_of("manifest-path") {
            metadata_cmd.manifest_path(path);
        }
        let metadata = metadata_cmd.exec().expect("get cargo metadata");
        if metadata.workspace_members.len() == 1 {
            Config {
                version,
                metadata: matches.value_of("metadata").map(parse_identifiers),
                pre_release: matches.value_of("pre-release").map(parse_identifiers),
                manifest: metadata[&metadata.workspace_members[0]]
                    .manifest_path
                    .clone(),
            }
        } else {
            panic!("Workspaces are not supported yet.");
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
