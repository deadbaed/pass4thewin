use anyhow::anyhow;
use git2::{Commit, Error, ObjectType, Oid, Repository, Tree};
use std::path::{Path, PathBuf};

/// Initiate repository
pub fn init_repo(path: &Path) -> anyhow::Result<Repository> {
    let repo = Repository::init(path)?;
    create_initial_commit(&repo)?;

    // add .gpg-id file
    add_commit_file(&repo, &path.join(".gpg-id"))?;

    Ok(repo)
}

/// Write index to a git tree
fn write_index_to_tree(repo: &Repository) -> Result<Tree, Error> {
    // Create tree from current index
    let tree = {
        // Get repo index
        let mut index = repo.index()?;

        // Write index to a tree
        index.write_tree()?
    };

    // Return tree object
    repo.find_tree(tree)
}

/// Create initial commit with no files
fn create_initial_commit(repo: &Repository) -> Result<(), Error> {
    // Get user information
    let sig = repo.signature()?;

    let tree = write_index_to_tree(repo)?;

    // Make commit with no parent commits and with empty tree (since it's the first one)
    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;

    Ok(())
}

/// Get HEAD commit
fn get_head_commit(repo: &Repository) -> Result<Commit, Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| Error::from_str("failed to find commit"))
}

/// Create a commit with a message
fn create_commit(repo: &Repository, message: &str) -> Result<Oid, Error> {
    // Get user information
    let sig = repo.signature()?;

    let tree = write_index_to_tree(repo)?;

    // Get parent commit
    let parent_commit = get_head_commit(repo)?;

    // Create new commit
    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent_commit])
}

/// Add file to staging index
fn add_file(repo: &Repository, path: &Path) -> Result<(), Error> {
    // Get repo index
    let mut index = repo.index()?;

    // Add path to index
    index.add_path(path)?;

    // Write index
    index.write()
}

/// Gets relative path of a file based of the root of the git repo
///
/// repo: git repository (will be used as base path
/// full_path: full path of file to extract relative path
fn get_relative_path(repo: &Repository, full_path: &Path) -> Option<PathBuf> {
    // We want the root of the repo, *without* the git folder
    let path_repo = repo.path().parent()?;

    // Remove the common base path, and return the rest
    match full_path.strip_prefix(path_repo) {
        Ok(path) => Some(PathBuf::from(path)),
        Err(_) => None,
    }
}

/// Add a file and commit it
///
/// repo: git repository
/// path: full path of the file to add
/// message: commit prefix before filename
fn add_file_commit_with_message(
    repo: &Repository,
    path: &Path,
    message: &str,
) -> Result<(), anyhow::Error> {
    let relative_path_file = match get_relative_path(&repo, &path) {
        Some(file_path) => file_path,
        None => {
            return Err(anyhow!(
                "Failed to get relative path of file {}",
                path.display(),
            ))
        }
    };

    add_file(&repo, &relative_path_file)?;
    let commit_msg = format!("{} {}", message, relative_path_file.display());
    create_commit(&repo, &commit_msg)?;

    Ok(())
}

pub fn add_commit_password(repo: &Repository, path: &Path) -> anyhow::Result<()> {
    add_file_commit_with_message(repo, path, "Added password")
}

pub fn add_commit_file(repo: &Repository, path: &Path) -> anyhow::Result<()> {
    add_file_commit_with_message(repo, path, "Added file")
}
