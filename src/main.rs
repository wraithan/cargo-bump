extern crate clap;
extern crate semver;
extern crate toml;

mod config;
mod version;

use std::fs::OpenOptions;
use std::io::Write;
use toml::Value;

fn main() {
    let conf = config::get_config();
    let (mut v, mut t) = version::get_current_version(&conf.manifest);
    let old_version = v.clone();
    version::update_version(&mut v, conf.version);
    println!("Version {} -> {}", old_version, v);

    match t {
        Value::Table(ref mut top) => {
            match top.get_mut("package").unwrap() {
                &mut Value::Table(ref mut package) => {
                    let version = package.entry("version".to_owned())
                        .or_insert_with(|| panic!("missing package"));
                    *version = toml::Value::String(v.to_string());
                }
                _ => panic!("package not a table")
            }
        }
        _ => panic!("top not a table")
    }

    let new_manifest = build_new_manifest(t).unwrap();
    let new_manifest_bytes = new_manifest.as_bytes();
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&conf.manifest)
        .unwrap();
    f.write_all(new_manifest_bytes).unwrap();
}

fn build_new_manifest(toml: Value) -> Option<String> {
    match toml {
        Value::Table(mut data) => {
            let package_value = data.remove("package").unwrap();
            Some(format!("[package]\n{}{}",
                         package_value,
                         toml::Value::Table(data)))
        }
        _ => None
    }
}
