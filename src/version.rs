use config::{ModifierType, VersionModifier};
use semver::Version;

pub fn update_version(old: &mut Version, by: VersionModifier) {
    match by.mod_type {
        ModifierType::Replace(v) => {
            *old = v;
        }
        ModifierType::Major => {
            old.increment_major();
        }
        ModifierType::Minor => {
            old.increment_minor();
        }
        ModifierType::Patch => {
            old.increment_patch();
        }
    }

    if let Some(pre) = by.pre_release {
        old.pre = pre;
    }
    if let Some(build) = by.build_metadata {
        old.build = build;
    }
}
