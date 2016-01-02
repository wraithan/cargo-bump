# cargo-bump

**Note**: Until this package is 1.0.0 or higher, do not expect all of these
  features to work.

This adds the command `cargo bump` which bumps the current version in your
`Cargo.toml`. If there is a `.git` it will also attempt to commit the changes it
made and tag that commit with the new version.

This is meant to be a clone of `npm version` with the `pre*` version specifiers
omitted as I rarely see the prerelease versions on
[crates.io](https://crates.io/).

## examples

Increment the patch version: `cargo bump` or `cargo bump patch`

Increment the minor version: `cargo bump minor`

Increment the minor version with custom note: `cargo bump minor -m "%s is the new best version!"`

Increment the major and don't commit: `cargo bump --no-git-tag-version major`

Set the version number directly: `cargo bump 13.3.7`

## usage

`cargo bump [options] [<newversion> | major | minor | patch]`

Options:

* `--no-git-tag-version`: disables the git commit and tag. No effect if `.git`
  is not detected.
* `-m, --message`: sets the commit message replacing `%s` with the new version
  number. No effect if `.git` is not detected.

Version is either a [semver](http://semver.org/) string containing the new
version, or the part of the current version to increment: `major`, `minor`,
`patch`.

## order of operations

1. Validate arguments. If a string is passed for version, make sure it is a
   valid [semver](http://semver.org/) string.
2. Check to make sure the git working directory is clean before we get started.
   Omitted if `--no-git-tag-version` is set or if `.git` is not detected.
3. Bump version in package.json as requested (`<newversion>`, `patch`, `minor`,
   `major`).
4. Commit and tag. Omitted if `--no-git-tag-version` is set or if `.git` is
   not detected.
