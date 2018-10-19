extern crate clap;
extern crate semver;
extern crate toml_edit;

mod config;
mod version;

use semver::Version;
use toml_edit::Document;
//use tomllib::types::{ParseResult, Value};
//use tomllib::TOMLParser;

fn main() {
    let conf = config::get_config();
    let raw_data = std::fs::read_to_string(&conf.manifest).unwrap();

    let mut result = raw_data
        .parse::<Document>()
        .expect("could not parse Cargo.toml");

    let mut version = Version::parse(
        result["package"]["version"]
            .as_str()
            .expect("version is not a string"),
    )
    .unwrap();

    let old_version = version.clone();
    version::update_version(&mut version, conf.version, conf.pre_release, conf.metadata);
    println!("Version {} -> {}", old_version, version);

    *result["package"]["version"].as_value_mut().unwrap() = version.to_string().into();

    std::fs::write(&conf.manifest, result.to_string()).unwrap();
}
