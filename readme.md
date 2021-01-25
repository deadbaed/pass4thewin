# pass4thewin
pass for the windows platform

Quick (and dirty) clone of [pass](https://passwordstore.org) written in Rust for Windows.
It should be compatible with pass (but no guarantees)

Requires `git` to sync with git repos, otherwise everything is in the binary.

## development

Tools required: `rust` and `cargo`. You can use [rustup](https://rustup.rs) to install them.

Run `cargo build` to compile and `cargo run` to run the binary.

## install

You can find binaries as development goes:

1. Pick the most recent result in the [list of events](https://github.com/x4m3/pass4thewin/actions?query=branch%3Amaster+is%3Asuccess)
2. Download the artifact, it's a zip file containing the binary (x64 only)
3. Place the binary in a folder in your PATH
4. You should be able to run `pass4thewin` in a terminal