use config::{ModifierType, VersionModifier};
use git;
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
        ModifierType::Auto => {
            let commit_message = git::log();
            if commit_message.contains("[major]") {
                old.increment_major();
            } else if commit_message.contains("[minor]") {
                old.increment_minor();
            } else {
                old.increment_patch();
            }
        }
    }

    if let Some(pre) = by.pre_release {
        old.pre = pre;
    }
    if let Some(build) = by.build_metadata {
        old.build = build;
    }
}
