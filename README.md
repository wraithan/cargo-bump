[![crates.io](https://img.shields.io/crates/v/cargo-bump.svg)](https://crates.io/crates/cargo-bump)
[![build status](https://travis-ci.org/wraithan/cargo-bump.svg?branch=master)](https://travis-ci.org/wraithan/cargo-bump)
# cargo-bump

This adds the command `cargo bump` which bumps the current version in your
`Cargo.toml`.

This is meant to be a clone of `npm version` with the `pre*` version specifiers
omitted as I rarely see the pre-release versions on [crates.io](https://crates.io/).

## installation

Install using cargo:

`cargo install cargo-bump`

## examples

Increment the patch version: `cargo bump` or `cargo bump patch`

Increment the minor version and create a git tag: `cargo bump minor --git-tag`

Set the version number directly: `cargo bump 13.3.7`

## usage

```
USAGE:
    cargo bump [<version> | major | minor | patch] [FLAGS]

    Version parts: ${MAJOR}.${MINOR}.${PATCH}-${PRE-RELEASE}+${BUILD}
    Example: 3.1.4-alpha+159

FLAGS:
    -g, --git-tag     Optional commit the updated version and create a git tag.
    -h, --help        Prints help information
    -r, --run-buid    Optional run `cargo build` before handling any git logic.
                                      This has the added benefit of fixing the Cargo.lock before the git commits are
                      made.
    -v, --version     Prints version information

OPTIONS:
    -b, --build <BUILD>                 Optional build metadata for this version.
        --manifest-path <PATH>          Optional path to Cargo.toml
    -p, --pre-release <RELEASE TYPE>    Optional pre-release information.

ARGS:
    <version>    Version should be a semver (https://semver.org/) string or the position of the current version to
                 increment: major, minor or patch.
```
