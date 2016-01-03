[![crates.io](https://img.shields.io/crates/v/cargo-bump.svg)](https://crates.io/crate/cargo-bump)
[![build status](https://travis-ci.org/wraithan/cargo-bump.svg?branch=master)](https://travis-ci.org/wraithan/cargo-bump)
# cargo-bump

**Note**: Until this package is 1.0.0 or higher, do not expect all of these
  features to work.

This adds the command `cargo bump` which bumps the current version in your
`Cargo.toml`. If there is a `.git` it will also attempt to commit the changes it
made and tag that commit with the new version.

This is meant to be a clone of `npm version` with the `pre*` version specifiers
omitted as I rarely see the prerelease versions on
[crates.io](https://crates.io/).

## warnings

This currently does a destructive overwrite of your Cargo.toml and may reorder
things and will lose comments. See
[#1](https://github.com/wraithan/cargo-bump/issues/1) for why and what the plan
is to fix this.

## installation

Install using cargo:

`cargo install cargo-bump`

## examples

Increment the patch version: `cargo bump` or `cargo bump patch`

Increment the minor version: `cargo bump minor`

Increment the minor version with custom note: `cargo bump minor -m "%s is the new best version!"`

Increment the major and don't commit: `cargo bump --no-git-tag-version major`

Set the version number directly: `cargo bump 13.3.7`

## usage

```
USAGE:
        cargo bump [FLAGS] [<version> | major | minor | patch]

FLAGS:
    -h, --help                  Prints help information
        --no-git-tag-version    Disables the git iteractions
    -v, --version               Prints version information

OPTIONS:
    -m, --message <message>    Commit message, %s will be replaced with new version number

ARGS:
    version    Version should be a semver (https://semver.org/) string or the
               position of the current version to increment: major, minor or patch.
```

## order of operations

1. Validate arguments. If a string is passed for version, make sure it is a
   valid [semver](http://semver.org/) string.
2. Check to make sure the git working directory is clean before we get started.
   Omitted if `--no-git-tag-version` is set or if `.git` is not detected.
3. Bump version in package.json as requested (`<newversion>`, `patch`, `minor`,
   `major`).
4. Commit and tag. Omitted if `--no-git-tag-version` is set or if `.git` is
   not detected.
