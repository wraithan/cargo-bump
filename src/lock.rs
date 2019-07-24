use cargo::core::Workspace;
use cargo::ops::{pkgid, update_lockfile, UpdateOptions};
use cargo::util::config::Config;
use std::path::Path;

pub fn update_lock(manifest: &Path) {
    let config =
        Config::default().expect("Could not create default Cargo config when updating Cargo.lock.");
    let workspace = Workspace::new(manifest, &config)
        .expect("Invalid workspace created from Cargo.toml when updating Cargo.lock");
    let to_update = pkgid(&workspace, None)
        .expect("Could not find a valid package id when updating Cargo.lock.");
    let update_options = UpdateOptions {
        config: &config,
        to_update: vec![to_update.name().as_str().into()],
        precise: None,
        aggressive: false,
        dry_run: false,
    };
    update_lockfile(&workspace, &update_options).expect("Failed updating Cargo.lock file");
}
