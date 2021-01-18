use crate::settings::Settings;
use crate::sync;
use anyhow::{anyhow, Context};
use git2::Repository;
use std::path::{Path, PathBuf};

/// Remove file `current_path` and folders if they are empty
fn remove_file(current_path: &mut PathBuf, base_path: &Path, path: &str) -> anyhow::Result<()> {
    // Add path to list of items to remove
    let mut vec = Vec::new();
    vec.push(current_path.clone());

    // If current folder has nothing left after deleting the `path`
    // Try to remove parent folders until root of password store
    while current_path.pop() {
        // Stop when we are at the root of password store
        if current_path.as_path() == base_path {
            break;
        }

        // Try to read contents of directory
        if let Ok(contents) = current_path.read_dir() {
            // If there's only one element left (the one we are deleting)
            if contents.count() == 1 {
                vec.push(current_path.clone());
            }
        }
    }

    // Delete all the elements
    fs_extra::remove_items(&vec)?;

    println!("Removed `{}` from password store", path);

    Ok(())
}

pub fn remove(path: &str, settings: &Settings) -> anyhow::Result<()> {
    let mut current_path = settings.get_password_store_path()?.join(path);

    // If old path does not exist, try with .gpg at the end
    if !current_path.exists() {
        current_path.set_extension("gpg");
        // If it still does not exist, then give up
        if !current_path.exists() {
            return Err(anyhow!("Could not locate `{}` in password store", path));
        }
    }

    // TODO: support deletion of folders directly
    // Sadly, only removing of password files are supported atm
    // Otherwise the implementation of git remove folder and it's contents is required
    if current_path.is_dir() {
        println!(
            "The entry `{}` is a directory, please try again with a password entry",
            path
        );
        println!("Delete operation canceled");
        return Ok(());
    }

    let base_path = settings.get_password_store_path()?;
    let orig_to_rm = current_path.clone();
    remove_file(&mut current_path, base_path, &path)?;

    // Git operations if git repo is present
    if let Ok(repo) = Repository::open(&settings.get_password_store_path()?) {
        let relative_path = sync::get_relative_path(&repo, &orig_to_rm)
            .context(format!("Failed to get relative path of `{}`", path))?;
        sync::rm_file(&repo, &relative_path)?;

        let commit_message = format!("Removed {} from password store", path);
        sync::create_commit(&repo, &commit_message)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::password::Password;
    use tempfile::tempdir;

    #[test]
    fn remove_file() -> anyhow::Result<()> {
        let tmp_dir = tempdir()?;

        let pgp_key = format!("{}\\tests\\secret-key.asc", env!("CARGO_MANIFEST_DIR"));
        let password_contents = "my_super_secure_password";
        let password_name = "fol1/fol2/password";
        let password_store = crate::cmd::insert::tests::create_password_store(&tmp_dir.path())?;

        // create password
        let mut password = Password::from_single_line(password_contents);
        password.set_filepath(&password_store, password_name);
        password.encrypt_with_key(pgp_key.as_ref())?;

        // make sure the password exists
        assert_eq!(password.exists(), true);

        // rm the file
        let orig_password_path = password.get_filepath().unwrap().to_path_buf();
        let mut password_path = orig_password_path.clone();
        super::remove_file(&mut password_path, &password_store, password_name)?;

        // the file should be gone now
        assert_eq!(orig_password_path.exists(), false);

        Ok(())
    }
}
