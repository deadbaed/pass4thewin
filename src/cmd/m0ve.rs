use crate::settings::Settings;
use crate::sync;
use anyhow::anyhow;
use dialoguer::Confirm;
use git2::Repository;
use std::path::PathBuf;

fn move_action(old: &PathBuf, new: &PathBuf, force: bool, new_path: &str) -> anyhow::Result<()> {
    // If `new_path` exists
    if new.exists() {
        // If it's a folder, move `old_path` inside folder `new_path`
        if new.is_dir() {
            // Construct list of paths to move
            let mut from_paths = Vec::new();
            from_paths.push(&old);

            let options = fs_extra::dir::CopyOptions::new();
            fs_extra::move_items(&from_paths, &new, &options)?;
        } else {
            // (It's a file)

            // File already exists and will be overwritten
            if !force
                && !Confirm::new()
                    .with_prompt(format!(
                        "The entry {} already exists. Overwrite it?",
                        new_path
                    ))
                    .default(false)
                    .show_default(true)
                    .interact()?
            {
                // If user says no
                println!("Moving operation canceled");
                return Ok(());
            }

            // Rename old_path to new_path, overwrite file at new_path
            std::fs::rename(&old, &new)?;
        }
    } else {
        // Rename old_path to new_path
        std::fs::rename(&old, &new)?;
    }
    Ok(())
}

pub fn m0ve(
    old_path: &str,
    new_path: &str,
    force: bool,
    settings: &Settings,
) -> anyhow::Result<()> {
    let mut old = settings.get_password_store_path()?.join(old_path);
    let new = {
        let p = settings.get_password_store_path()?;

        // To be able to move stuff to the root of the password store
        // Works for `/`, `\`, `\\` and `.`
        if new_path == "/" || new_path == "\\" || new_path == "\\\\" || new_path == "." {
            p.to_path_buf()
        } else {
            p.join(new_path)
        }
    };

    // If old path does not exist, try with .gpg at the end
    if !old.exists() {
        old.set_extension("gpg");
        // If it still does not exist, then give up
        if !old.exists() {
            return Err(anyhow!("Could not locate {} in password store", old_path));
        }
    }

    move_action(&old, &new, force, new_path)?;

    println!("Moved `{}` to `{}`", old_path, new_path);

    // Git operations if git repo is present
    if let Ok(repo) = Repository::open(&settings.get_password_store_path()?) {
        sync::rm_path(&repo, &old)?;
        sync::add_path(&repo, &new)?;
        let commit_message = format!("Moved {} to {}", old_path, new_path);
        sync::create_commit(&repo, &commit_message)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::password::Password;
    use crate::tree::tree;
    use tempfile::tempdir;

    #[test]
    fn move_file_to_folder() -> anyhow::Result<()> {
        let tmp_dir = tempdir()?;

        let pgp_key = format!("{}\\tests\\secret-key.asc", env!("CARGO_MANIFEST_DIR"));
        let password_contents = "my_super_secure_password";
        let password_name = "password";
        let password_store = crate::cmd::insert::tests::create_password_store(&tmp_dir.path())?;

        // create password
        let mut password = Password::from_single_line(password_contents);
        password.set_filepath(&password_store, password_name);
        password.encrypt_with_key(pgp_key.as_ref())?;

        // make sure the password exists
        assert_eq!(password.exists(), true);

        // create folder
        let folder = "folder";
        let mut folder_path = password_store.join(folder);
        std::fs::create_dir(&folder_path)?;

        // do the action
        let old = password_store.join(format!("{}.gpg", password_name));
        super::move_action(&old, &folder_path, true, folder)?;
        folder_path.push(format!("{}.gpg", password_name));

        // the old file should not exist anymore
        assert_eq!(old.exists(), false);

        // the new file should be here
        assert_eq!(folder_path.exists(), true);

        Ok(())
    }

    #[test]
    fn move_folder_to_folder() -> anyhow::Result<()> {
        let tmp_dir = tempdir()?;

        let pgp_key = format!("{}\\tests\\secret-key.asc", env!("CARGO_MANIFEST_DIR"));
        let password_contents = "my_super_secure_password";
        let password_name = "folder/password";
        let password_store = crate::cmd::insert::tests::create_password_store(&tmp_dir.path())?;

        // create password
        let mut password = Password::from_single_line(password_contents);
        password.set_filepath(&password_store, password_name);
        password.encrypt_with_key(pgp_key.as_ref())?;

        // make sure the password exists
        assert_eq!(password.exists(), true);

        println!("{}", tree(&password_store).unwrap());

        // create folder
        let folder = "new_folder";
        let mut folder_path = password_store.join(folder);
        std::fs::create_dir(&folder_path)?;

        println!("{}", tree(&password_store).unwrap());

        // do the action
        let old = password_store.join(password_name);
        println!("old {}", old.display());
        println!("folder path {}", folder_path.display());
        super::move_action(&old, &folder_path, true, folder)?;
        folder_path.push(format!("{}.gpg", password_name));

        println!("{}", tree(&password_store).unwrap());

        // the old folder should not exist anymore
        assert_eq!(old.exists(), false);

        // the new folder should be here
        assert_eq!(folder_path.is_file(), true);

        Ok(())
    }

    #[test]
    fn move_file_to_root() -> anyhow::Result<()> {
        Ok(())
    }

    #[test]
    fn move_folder_to_root() -> anyhow::Result<()> {
        Ok(())
    }

    #[test]
    fn rename_file() -> anyhow::Result<()> {
        Ok(())
    }

    #[test]
    fn rename_folder() -> anyhow::Result<()> {
        Ok(())
    }
}
