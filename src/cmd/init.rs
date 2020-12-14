use std::path::{Path, PathBuf};

pub fn init(pgp_key: &Path, path: Option<PathBuf>) {
    println!("cmd init: pgp_key {:?} path {:?}", pgp_key, path);

    // extract keyid from file

    // create password store repository

    // put keyid in `.gpg-id`

    // sign `.gpg-id` with key ??
}
