use git2::{Commit, Error, ObjectType, Oid, Repository, Tree};
use std::path::Path;

pub fn create_repo(path: &Path) -> Option<Repository> {
    match Repository::init(path) {
        Ok(repo) => Some(repo),
        Err(_) => None,
    }
}

pub fn open_repo(path: &Path) -> Option<Repository> {
    match Repository::open(path) {
        Ok(repo) => Some(repo),
        Err(_) => None,
    }
}

/// Create empty commit with no files
pub fn create_initial_commit(repo: &Repository) -> Result<Oid, Error> {
    let sig = repo.signature()?;

    // Create empty tree to commit
    let empty_oid = {
        // Get repo index
        let mut index = repo.index()?;

        // Write index to tree
        index.write_tree()?
    };
    let tree = repo.find_tree(empty_oid)?;

    // Make commit with no parent commits (since it's the first one)
    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
}

/// Get HEAD commit
fn get_head_commit(repo: &Repository) -> Result<Commit, Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| Error::from_str("failed to find commit"))
}

/// Make a commit with a tree and a message to current branch
pub fn create_commit(repo: &Repository, tree: &Tree, message: &str) -> Result<Oid, Error> {
    let sig = repo.signature()?;
    let parent_commit = get_head_commit(repo)?;

    repo.commit(Some("HEAD"), &sig, &sig, message, tree, &[&parent_commit])
}

/// Create tree
pub fn add_file<'a>(repo: &'a Repository, path: &Path) -> Result<Tree<'a>, Error> {
    // Get repo index
    let mut index = repo.index()?;

    // Add path to index
    index.add_path(path)?;

    // Write index as tree
    let oid = index.write_tree()?;

    // Get tree from newly written index
    repo.find_tree(oid)
}
