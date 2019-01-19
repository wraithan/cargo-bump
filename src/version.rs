use config::NewVersion;
use semver::Version;

pub fn update_version(old: &mut Version, by: NewVersion) {
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
}
