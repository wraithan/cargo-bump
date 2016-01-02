use config::NewVersion;
use semver::Version;
use std::fs;
use std::io::Read;
use std::path::Path;
use toml::{Parser, Value, Table};

pub fn get_current_version(file: &Path) -> (Version, Table) {
    let mut file = fs::File::open(file).unwrap();
    let mut raw_data = String::new();
    file.read_to_string(&mut raw_data).unwrap();
    let mut parser = Parser::new(&raw_data);
    let parsed_data: Table = parser.parse()
        .unwrap_or_else(|| panic!("couldn't parse Cargo.toml, {:?}", parser.errors));
    let copy = parsed_data.clone();
    let raw_version = parsed_data.get("package")
        .unwrap_or_else(|| panic!("Cargo.toml is missing package section"))
        .as_table()
        .unwrap_or_else(|| panic!("package section ewas not a table"))
        .get("version")
        .unwrap_or_else(|| panic!("Cargo.toml is missing version field"))
        .as_str()
        .unwrap_or_else(|| panic!("version field was not a string"));
    (Version::parse(raw_version).unwrap(), copy)
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
