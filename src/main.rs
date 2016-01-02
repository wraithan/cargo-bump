extern crate clap;
extern crate semver;
extern crate toml;

mod config;
mod version;

fn main() {
    let conf = config::get_config();
    println!("{:#?}", conf);
    let (mut v, t) = version::get_current_version(&conf.manifest);
    println!("Before: {:#?}", v);
    version::update_version(&mut v, conf.version);
    println!("After: {:#?}", v);
    println!("{}", toml::encode_str(&t));
}
