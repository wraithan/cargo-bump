use semver::Version;
use config::NewVersion;

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
