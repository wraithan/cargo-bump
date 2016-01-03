use config::NewVersion;
use semver::Version;
use std::fs;
use std::io::Read;
use std::path::Path;
use toml::{Parser, Value};

pub fn get_current_version(file: &Path) -> (Version, Value) {
    let mut file = fs::File::open(file).unwrap();
    let mut raw_data = String::new();
    file.read_to_string(&mut raw_data).unwrap();
    let mut parser = Parser::new(&raw_data);
    let parsed_data = Value::Table(parser.parse()
        .unwrap_or_else(|| panic!("couldn't parse Cargo.toml, {:?}", parser.errors)));
    let raw_version = parsed_data.lookup("package.version")
        .unwrap_or_else(|| panic!("package.version missing"))
        .as_str()
        .unwrap_or_else(|| panic!("version not a string"));

    (Version::parse(raw_version).unwrap(), parsed_data.clone())
}

pub fn update_version(old: &mut Version, by: NewVersion) {
    match by {
        NewVersion::String(v) => {
            *old = v;
        },
        NewVersion::Major => {
            old.increment_major();
        },
        NewVersion::Minor => {
            old.increment_minor();
        },
        NewVersion::Patch => {
            old.increment_patch();
        }
    }
}
