use git2::Repository;
use std::path::PathBuf;

pub fn create_repo(path: &PathBuf) -> Option<Repository> {
    match Repository::init(path) {
        Ok(repo) => Some(repo),
        Err(_) => None,
    }
}

pub fn open_repo(path: &PathBuf) -> Option<Repository> {
    match Repository::open(path) {
        Ok(repo) => Some(repo),
        Err(_) => None,
    }
}
