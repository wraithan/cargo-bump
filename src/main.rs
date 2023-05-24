#![deny(clippy::all)]

extern crate cargo_metadata;
extern crate clap;
extern crate semver;
extern crate toml_edit;

mod config;
mod git;
mod version;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use toml_edit::Document;
use std::process::Command;

use semver::Version;

fn main() {
    let conf = config::get_config();
    let raw_data = read_file(&conf.manifest);
    let use_git = conf.git_tag;

    if use_git {
        git::git_check();
    }

    let output = update_toml_with_version(&raw_data, conf.version_modifier);
    let version = output["package"]["version"].as_str().unwrap();

    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&conf.manifest)
        .unwrap();
    f.write_all(output.to_string().as_bytes()).unwrap();

    if !&conf.ignore_lockfile {
        Command::new("cargo")
        .args(&["generate-lockfile", "--offline", "--manifest-path", &conf.manifest.to_string_lossy()])
        .status()
        .expect("Failed to generate lockfile");
    }

    if use_git {
        git::git_commit_and_tag(version);
    }
}

fn read_file(file: &Path) -> String {
    let mut file = File::open(file).unwrap();
    let mut raw_data = String::new();
    file.read_to_string(&mut raw_data).unwrap();
    raw_data
}

fn update_toml_with_version(raw_data: &str, version_modifier: config::VersionModifier) -> Document {
    let mut value = raw_data
        .parse::<toml_edit::Document>()
        .expect("parsed toml");
    let version = {
        let version_string = value["package"]["version"]
            .as_str()
            .expect("toml has version");
        let mut version = version_string
            .parse::<Version>()
            .expect("version is semver");
        version::update_version(&mut version, version_modifier);
        version
    };
    value["package"]["version"] = toml_edit::value(version.to_string());

    value
}

#[cfg(test)]
mod test {
    use super::*;
    use config::{ModifierType, VersionModifier};

    fn toml_test_wrapper(
        template: &str,
        version_modifier: VersionModifier,
        start_version: &str,
        end_version: &str,
    ) {
        let input = template.replace("$VERSION", &format!("\"{}\"", start_version));
        let expected_output = template.replace("$VERSION", &format!("\"{}\"", end_version));
        let output = update_toml_with_version(&input, version_modifier);
        assert_eq!(
            expected_output,
            output.to_string().trim_end(),
            "toml output should be same with new version"
        );
    }

    #[test]
    fn toml_test_simple() {
        let input = "[package]
version = $VERSION";
        let mod_type = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.0.0",
        );
        let mod_type = ModifierType::Patch;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.0.1",
        );
        let mod_type = ModifierType::Minor;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.1.0",
        );
        let mod_type = ModifierType::Major;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "2.0.0",
        );
        let version_mod = VersionModifier::new(ModifierType::Major, Some("RC"), None);
        toml_test_wrapper(input, version_mod, "1.0.0", "2.0.0-RC");
        let version_mod = VersionModifier::new(
            ModifierType::Major,
            None,
            Some("ac44f1f8f31acf4728bd2055d716776b"),
        );
        toml_test_wrapper(
            input,
            version_mod,
            "1.0.0",
            "2.0.0+ac44f1f8f31acf4728bd2055d716776b",
        );
        let version_mod = VersionModifier::new(ModifierType::Major, Some("alpha"), Some("2230"));
        toml_test_wrapper(input, version_mod, "1.0.0", "2.0.0-alpha+2230");
    }

    #[test]
    fn toml_test_formatting_preserved_spaces() {
        let input = "  [package]
    version = $VERSION";
        let mod_type = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.0.0",
        );
        let mod_type = ModifierType::Patch;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.1.1",
        );
        let mod_type = ModifierType::Minor;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.2.0",
        );
        let mod_type = ModifierType::Major;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "2.0.0",
        );

        let input = "  [package]
version= $VERSION";
        let mod_type = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.0.0",
        );
        let mod_type = ModifierType::Patch;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.1.1",
        );
        let mod_type = ModifierType::Minor;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.2.0",
        );
        let mod_type = ModifierType::Major;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "2.0.0",
        );

        let input = "  [package]
version       = $VERSION";
        let mod_type = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.0.0",
        );
        let mod_type = ModifierType::Patch;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.1.1",
        );
        let mod_type = ModifierType::Minor;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.2.0",
        );
        let mod_type = ModifierType::Major;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "2.0.0",
        );
    }

    #[test]
    #[ignore = "toml_edit doesn't expose enough to preserve whitespace around replaced string"]
    fn toml_test_formatting_preserved_space_around_replaced_value() {
        let input = "  [package]
version =$VERSION";
        let mod_type = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.0.0",
        );
        let mod_type = ModifierType::Patch;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.1.1",
        );
        let mod_type = ModifierType::Minor;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.2.0",
        );
        let mod_type = ModifierType::Major;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "2.0.0",
        );

        let input = "  [package]
version =    $VERSION";
        let mod_type = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.0.0",
        );
        let mod_type = ModifierType::Patch;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.1.1",
        );
        let mod_type = ModifierType::Minor;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.2.0",
        );
        let mod_type = ModifierType::Major;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "2.0.0",
        );

        let input = "  [package]
version = $VERSION      ";
        let mod_type = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.0.0",
        );
        let mod_type = ModifierType::Patch;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.1.1",
        );
        let mod_type = ModifierType::Minor;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.2.0",
        );
        let mod_type = ModifierType::Major;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "2.0.0",
        );
    }

    #[test]
    #[ignore = "toml_edit doesn't handle preserving space in headers save test for later"]
    fn toml_test_formatting_preserved_header_spaces() {
        let input = "  [package]
    version = $VERSION
[     other]
a = true";
        let mod_type = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.0.0",
        );
        let mod_type = ModifierType::Patch;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.1.1",
        );
        let mod_type = ModifierType::Minor;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "1.2.0",
        );
        let mod_type = ModifierType::Major;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.0",
            "2.0.0",
        );

        let input = "  [  package   ]
    version= $VERSION";
        let mod_type = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.1",
            "1.0.0",
        );
        let mod_type = ModifierType::Patch;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.1",
            "1.1.2",
        );
        let mod_type = ModifierType::Minor;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.1",
            "1.2.0",
        );
        let mod_type = ModifierType::Major;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.1.1",
            "2.0.0",
        );

        let input = "  [  package   ]



    version= $VERSION
    
    ";
        let mod_type = "4.0.0".parse().expect("version modifier");
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "2.0.0",
            "4.0.0",
        );
        let mod_type = ModifierType::Patch;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "2.0.0",
            "2.0.1",
        );
        let mod_type = ModifierType::Minor;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "2.0.0",
            "2.1.0",
        );
        let mod_type = ModifierType::Major;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "2.0.0",
            "3.0.0",
        );
    }

    #[test]
    fn toml_test_formatting_preserved_comments() {
        let input = "#before header
[package]
version = $VERSION";
        let mod_type = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.0.0",
        );
        let mod_type = ModifierType::Patch;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.0.1",
        );
        let mod_type = ModifierType::Minor;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.1.0",
        );
        let mod_type = ModifierType::Major;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "2.0.0",
        );

        let input = "[package]# end of header
version = $VERSION";
        let mod_type = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.0.0",
        );
        let mod_type = ModifierType::Patch;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.0.1",
        );
        let mod_type = ModifierType::Minor;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.1.0",
        );
        let mod_type = ModifierType::Major;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "2.0.0",
        );

        let input = "[package]
# version = \"2.0.0\"
version = $VERSION";
        let mod_type = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.0.0",
        );
        let mod_type = ModifierType::Patch;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.0.1",
        );
        let mod_type = ModifierType::Minor;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.1.0",
        );
        let mod_type = ModifierType::Major;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "2.0.0",
        );
    }

    #[test]
    fn toml_test_dotted_headers() {
        let input = "[package]
version = $VERSION

[a]
d = false

[a.b]
c = true";
        let mod_type = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.0.0",
        );
        let mod_type = ModifierType::Patch;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.0.1",
        );
        let mod_type = ModifierType::Minor;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "1.1.0",
        );
        let mod_type = ModifierType::Major;
        toml_test_wrapper(
            input,
            VersionModifier::from_mod_type(mod_type),
            "1.0.0",
            "2.0.0",
        );
    }
}
