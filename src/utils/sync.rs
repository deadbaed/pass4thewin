use git2::{Commit, Error, ObjectType, Oid, Repository, Tree};
use std::path::{Path, PathBuf};

/// Initiate repository with empty commit
pub fn init_repo(path: &Path) -> Result<Repository, Error> {
    let repo = Repository::init(path)?;
    create_initial_commit(&repo)?;

    Ok(repo)
}

/// Create initial commit with no files
fn create_initial_commit(repo: &Repository) -> Result<Oid, Error> {
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
fn create_commit(repo: &Repository, tree: &Tree, message: &str) -> Result<Oid, Error> {
    let sig = repo.signature()?;
    let parent_commit = get_head_commit(repo)?;

    repo.commit(Some("HEAD"), &sig, &sig, message, tree, &[&parent_commit])
}

/// Create tree
fn add_file<'a>(repo: &'a Repository, path: &Path) -> Result<Tree<'a>, Error> {
    // Get repo index
    let mut index = repo.index()?;

    // Add path to index
    index.add_path(path)?;

    // Write index as tree
    let oid = index.write_tree()?;

    // Get tree from newly written index
    repo.find_tree(oid)
}

/// Gets relative path of a file based of the root of the git repo
///
/// repo: git repository (will be used as base path
/// relative_path: full path of file to extract relative path
fn get_relative_path(repo: &Repository, relative_path: &Path) -> Option<PathBuf> {
    // We want the root of the repo, *without* the git folder
    let path_repo = repo.path().parent()?;

    // Remove the common base path, and return the rest
    match relative_path.strip_prefix(path_repo) {
        Ok(path) => Some(PathBuf::from(path)),
        Err(_) => None,
    }
}

/// Add a file and commit it
///
/// repo: git repository
/// path: full path of the file to add
pub fn add_commit_file(repo: &Repository, path: &Path) -> bool {
    let relative_path_file = match get_relative_path(&repo, &path) {
        Some(file_path) => file_path,
        None => return false,
    };

    let tree_file_added = match add_file(&repo, &relative_path_file) {
        Ok(tree) => tree,
        Err(_) => return false,
    };

    let commit_msg = format!("Added password {}\n", relative_path_file.display());
    create_commit(&repo, &tree_file_added, &commit_msg).is_ok()
}
