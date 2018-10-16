use config::NewVersion;
use semver::{Identifier, Version};

pub fn update_version(
    old: &mut Version,
    by: NewVersion,
    pre_release: Option<Vec<Identifier>>,
    metadata: Option<Vec<Identifier>>,
) {
    match by {
        NewVersion::Replace(v) => {
            *old = v;
        }
        NewVersion::Major => {
            old.increment_major();
        }
        NewVersion::Minor => {
            old.increment_minor();
        }
        NewVersion::Patch => {
            old.increment_patch();
        }
    }

    if let Some(pre) = pre_release {
        old.pre = pre;
    }
    if let Some(build) = metadata {
        old.build = build;
    }
}
