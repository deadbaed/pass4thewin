use crate::constants::ID_APPLICATION;
use crate::password::Password;
use anyhow::{anyhow, Context};
use git2::{Commit, Error, ObjectType, Oid, Repository, Signature, Tree};
use std::path::{Path, PathBuf};

/// Initiate repository
pub fn init_repo(path: &Path) -> anyhow::Result<Repository> {
    let repo = Repository::init(path)?;

    create_initial_commit(&repo)?;

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

/// Get current git signature or create one
fn get_signature(repo: &Repository) -> Result<Signature, Error> {
    // Try to get local signature
    if let Ok(sig) = repo.signature() {
        return Ok(sig);
    }

    // Else create a signature based of user's local information
    let username = std::env::var("%USERNAME%").unwrap_or_else(|_| ID_APPLICATION.into());
    let hostname = std::env::var("%COMPUTERNAME%").unwrap_or_else(|_| "windows".into());
    let email = format!("{}@{}.local", username, hostname);

    Signature::now(&username, &email)
}

/// Create initial commit with no files
fn create_initial_commit(repo: &Repository) -> Result<(), Error> {
    // Get user information
    let sig = get_signature(repo)?;

    let tree = write_index_to_tree(repo)?;

    // Make commit with no parent commits and with empty tree (since it's the first one)
    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;

    Ok(())
}

/// Get HEAD commit
pub fn get_head_commit(repo: &Repository) -> Result<Commit, Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| Error::from_str("Failed to find HEAD commit"))
}

/// Create a commit with a message
fn create_commit(repo: &Repository, message: &str) -> Result<Oid, Error> {
    // Get user information
    let sig = get_signature(repo)?;

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
pub fn get_relative_path(repo: &Repository, full_path: &Path) -> Option<PathBuf> {
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

pub fn add_commit_password(repo: &Repository, password: &Password) -> anyhow::Result<()> {
    let path = password
        .get_filepath()
        .context("Path of password is not set (this should not happen)")?;

    add_file_commit_with_message(repo, path, "Added password")
}

pub fn add_commit_file(repo: &Repository, path: &Path) -> anyhow::Result<()> {
    add_file_commit_with_message(repo, path, "Added file")
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn init_repo() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempdir()?;

        // create git repo
        let repo = super::init_repo(tmp_dir.path())?;
        println!("created git repo {}", repo.path().display());

        // make sure the initial commit is present
        assert_eq!(super::get_head_commit(&repo).is_ok(), true);

        Ok(())
    }

    #[test]
    fn add_file() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempdir()?;

        // create git repo
        let repo = super::init_repo(tmp_dir.path())?;
        println!("created git repo {}", repo.path().display());

        // create example file
        let relative_path = Path::new("example.txt");
        let file_path = tmp_dir.path().join(relative_path);
        {
            let mut file = File::create(&file_path).expect("Couldn't create file");
            file.write_all(b"example file\n")?;
            println!("created file {}", file_path.display());
        }

        // make sure the index is empty before
        assert_eq!(repo.index()?.is_empty(), true);

        // add file to index
        super::add_file(&repo, &relative_path)?;
        assert_eq!(repo.index()?.is_empty(), false);
        println!("added file to index {}", file_path.display());

        Ok(())
    }

    #[test]
    fn commit_staging() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempdir()?;

        // create git repo
        let repo = super::init_repo(tmp_dir.path())?;
        println!("created git repo {}", repo.path().display());

        // create example file
        let relative_path = Path::new("example.txt");
        let file_path = tmp_dir.path().join(relative_path);
        {
            let mut file = File::create(&file_path).expect("Couldn't create file");
            file.write_all(b"example file\n")?;
            println!("created file {}", file_path.display());
        }

        // add file to index
        super::add_file(&repo, &relative_path)?;
        println!("added file to index {}", file_path.display());

        // commit new file
        let commit = super::create_commit(&repo, "test commit")?;
        println!("commit has been created");

        // make sure id of new commit is the id of the HEAD commit
        let head_commit = super::get_head_commit(&repo)?.id();
        assert_eq!(commit, head_commit);

        Ok(())
    }
}
