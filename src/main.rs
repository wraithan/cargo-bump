#![deny(clippy::all)]

extern crate cargo_metadata;
extern crate clap;
extern crate semver;
extern crate tomllib;

mod config;
mod version;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use semver::Version;
use tomllib::types::{ParseResult, Value};
use tomllib::TOMLParser;

fn main() {
    let conf = config::get_config();
    let raw_data = read_file(&conf.manifest);

    let output = update_toml_with_version(&raw_data, conf.version);

    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&conf.manifest)
        .unwrap();
    f.write_all(output.as_bytes()).unwrap();
}

fn read_file(file: &Path) -> String {
    let mut file = File::open(file).unwrap();
    let mut raw_data = String::new();
    file.read_to_string(&mut raw_data).unwrap();
    raw_data
}

fn update_toml_with_version(raw_data: &str, new_version: config::NewVersion) -> String {
    let parser = TOMLParser::new();
    let (mut parser, result) = parser.parse(raw_data);
    match result {
        ParseResult::Full => {}
        _ => panic!("couldn't parse Cargo.toml"),
    }

    let raw_value = parser
        .get_value("package.version")
        .expect("package.version missing");
    let mut version = match raw_value {
        Value::String(raw_version, _) => Version::parse(&raw_version).unwrap(),
        _ => panic!("version not a string"),
    };

    let old_version = version.clone();
    version::update_version(&mut version, new_version);
    println!("Version {} -> {}", old_version, version);

    parser.set_value(
        "package.version",
        Value::basic_string(version.to_string()).unwrap(),
    );
    format!("{}", parser)
}

#[cfg(test)]
mod test {
    use super::*;

    fn toml_test_wrapper(
        template: &str,
        version_modifier: config::NewVersion,
        start_version: &str,
        end_version: &str,
    ) {
        let input = template.replace("$VERSION", &format!("\"{}\"", start_version));
        let expected_output = template.replace("$VERSION", &format!("\"{}\"", end_version));
        let output = update_toml_with_version(&input, version_modifier);
        assert_eq!(
            output, expected_output,
            "toml output should be same with new version"
        );
    }

    #[test]
    fn toml_test_simple() {
        let input = "[package]
version = $VERSION";
        let version_modifier = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.0.0");
        let version_modifier = "patch".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.0.1");
        let version_modifier = "minor".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.1.0");
        let version_modifier = "major".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "2.0.0");
    }

    #[test]
    fn toml_test_formatting_preserved_spaces() {
        let input = "  [package]
    version = $VERSION";
        let version_modifier = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.1.0", "1.0.0");
        let version_modifier = "patch".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.1.0", "1.1.1");
        let version_modifier = "minor".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.1.0", "1.2.0");
        let version_modifier = "major".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.1.0", "2.0.0");

        let input = "  [  package   ]
    version= $VERSION";
        let version_modifier = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.1.1", "1.0.0");
        let version_modifier = "patch".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.1.1", "1.1.2");
        let version_modifier = "minor".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.1.1", "1.2.0");
        let version_modifier = "major".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.1.1", "2.0.0");

        let input = "  [  package   ]



    version= $VERSION
    
    ";
        let version_modifier = "4.0.0".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "2.0.0", "4.0.0");
        let version_modifier = "patch".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "2.0.0", "2.0.1");
        let version_modifier = "minor".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "2.0.0", "2.1.0");
        let version_modifier = "major".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "2.0.0", "3.0.0");
    }

    #[test]
    fn toml_test_formatting_preserved_comments() {
        let input = "#before header
[package]
version = $VERSION";
        let version_modifier = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.0.0");
        let version_modifier = "patch".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.0.1");
        let version_modifier = "minor".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.1.0");
        let version_modifier = "major".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "2.0.0");

        let input = "[package]# end of header
version= $VERSION";
        let version_modifier = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.0.0");
        let version_modifier = "patch".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.0.1");
        let version_modifier = "minor".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.1.0");
        let version_modifier = "major".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "2.0.0");

        let input = "[package]
# version = \"2.0.0\"
version= $VERSION";
        let version_modifier = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.0.0");
        let version_modifier = "patch".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.0.1");
        let version_modifier = "minor".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.1.0");
        let version_modifier = "major".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "2.0.0");
    }

    #[test]
    #[ignore = "tomllib can't handle having [a] and [a.b] in the same file"]
    fn toml_test_dotted_headers() {
        let input = "[package]
version = $VERSION

[a]
d = false

[a.b]
c = true";
        let version_modifier = "1.0.0".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.0.0");
        let version_modifier = "patch".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.0.1");
        let version_modifier = "minor".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "1.1.0");
        let version_modifier = "major".parse().expect("version modifier");
        toml_test_wrapper(input, version_modifier, "1.0.0", "2.0.0");
    }
}
