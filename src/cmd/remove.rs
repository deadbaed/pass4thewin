use crate::settings::Settings;
use crate::sync;
use anyhow::anyhow;
use git2::Repository;

pub fn remove(path: &str, recursive: bool, force: bool, settings: &Settings) -> anyhow::Result<()> {
    let mut current_path = settings.get_password_store_path()?.join(path);

    // If old path does not exist, try with .gpg at the end
    if !current_path.exists() {
        current_path.set_extension("gpg");
        // If it still does not exist, then give up
        if !current_path.exists() {
            return Err(anyhow!("Could not locate `{}` in password store", path));
        }
    }

    // Add path to list of items to remove
    let mut vec = Vec::new();
    let orig_to_rm = current_path.clone();
    vec.push(orig_to_rm.clone());

    // If current folder has nothing left
    // Try to remove parent folders until root of password store
    let base_path = settings.get_password_store_path()?;
    while current_path.pop() {
        if current_path.as_path() == base_path {
            break;
        }
        if current_path.read_dir().is_err() {
            vec.push(current_path.clone());
        }
    }

    // Delete all the elements
    fs_extra::remove_items(&vec)?;

    println!("Removed `{}` from password store", path);

    // Git operations if git repo is present
    if let Ok(repo) = Repository::open(&settings.get_password_store_path()?) {
        sync::rm_path(&repo, &orig_to_rm)?;
        let commit_message = format!("Removed {} from password store", path);
        sync::create_commit(&repo, &commit_message)?;
    }

    Ok(())
}
