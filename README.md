[![crates.io](https://img.shields.io/crates/v/cargo-bump.svg)](https://crates.io/crates/cargo-bump)
[![build status](https://travis-ci.org/wraithan/cargo-bump.svg?branch=master)](https://travis-ci.org/wraithan/cargo-bump)
# cargo-bump

This adds the command `cargo bump` which bumps the current version in your
`Cargo.toml`.

This is meant to be a clone of `npm version` with the `pre*` version specifiers
omitted as I rarely see the pre-release versions on [crates.io](https://crates.io/).
It does not do the git operations but is still useful.

## installation

Install using cargo:

`cargo install cargo-bump`

## examples

Increment the patch version: `cargo bump` or `cargo bump patch`

Increment the minor version: `cargo bump minor`

Set the version number directly: `cargo bump 13.3.7`

## usage

```
USAGE:
    cargo bump [FLAGS] [<version> | major | minor | patch]

FLAGS:
    -h, --help       Prints help information
    -v, --version    Prints version information

ARGS:
    <version>    Version should be a semver (https://semver.org/) string or the
                 position of the current version to increment: major, minor or patch.
```
