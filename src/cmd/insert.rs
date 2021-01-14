use crate::password::Password;
use crate::settings::Settings;
use crate::sync::add_commit_password;
use anyhow::anyhow;
use dialoguer::Confirm;
use git2::Repository;

pub fn insert(
    password_name: &str,
    multi_line: bool,
    echo: bool,
    force: bool,
    settings: &Settings,
) -> anyhow::Result<()> {
    // Create empty password
    let mut password = Password::default();

    // Set path of password
    password.set_filepath(settings.get_password_store_path()?, password_name);

    // Check if file exists, if so ask to overwrite if force flag is not passed
    if password.exists()
        && !force
        && !Confirm::new()
            .with_prompt(format!(
                "An entry already exists for {}. Overwrite it?",
                password_name
            ))
            .default(false)
            .show_default(true)
            .interact()?
    {
        // If user says no
        println!("Password insertion canceled.");
        return Ok(());
    }

    // Get password from terminal
    if let Err(e) = password.terminal_input(password_name, multi_line) {
        return Err(anyhow!("Password insertion aborted: {}", e));
    }

    // Encrypt password and write output to file
    password.encrypt_with_key(settings.get_pgp_key_path()?)?;

    // Git operations if git repo is present
    if let Ok(repo) = Repository::open(&settings.get_password_store_path()?) {
        add_commit_password(&repo, &password)?
    }

    // Display password if echo flag is passed
    if echo {
        println!("====\n{}", password.to_string()?);
    }

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use crate::cmd::init::new_password_store;
    use crate::password::Password;
    use crate::sync::{add_commit_password, get_head_commit, get_relative_path};
    use git2::Repository;
    use std::path::{Path, PathBuf};
    use tempfile::tempdir;

    pub fn create_password_store(tmp_dir: &Path) -> anyhow::Result<PathBuf> {
        let secret_key_path = format!("{}\\tests\\secret-key.asc", env!("CARGO_MANIFEST_DIR"));

        new_password_store(secret_key_path.as_ref(), Some(PathBuf::from(tmp_dir)))
    }

    #[test]
    fn insert_single_line() -> anyhow::Result<()> {
        let tmp_dir = tempdir()?;

        let password_contents = "my_super_secure_password";
        let password_name = "folder/password";
        let password_store = create_password_store(&tmp_dir.path())?;
        crate::cmd::git::init(&password_store)?;

        let mut password = Password::from_single_line(password_contents);
        password.set_filepath(&password_store, password_name);

        // Make sure the file does not exist
        assert_eq!(password.get_filepath().unwrap().exists(), false);

        password.encrypt_with_key(
            format!("{}\\tests\\secret-key.asc", env!("CARGO_MANIFEST_DIR")).as_ref(),
        )?;

        // File should exist now
        assert_eq!(password.get_filepath().unwrap().exists(), true);

        git_operation(&password_store, &password)?;

        Ok(())
    }

    #[test]
    fn insert_multi_line() -> anyhow::Result<()> {
        let tmp_dir = tempdir()?;

        let mut password_contents = Vec::new();
        password_contents.push("multi\n".to_string());
        password_contents.push("line\n".to_string());
        password_contents.push("password\n".to_string());

        let password_name = "folder/password";
        let password_store = create_password_store(tmp_dir.path())?;
        crate::cmd::git::init(&password_store)?;

        let mut password = Password::from_multi_line(&password_contents);
        password.set_filepath(&password_store, password_name);

        // Make sure the file does not exist
        assert_eq!(password.get_filepath().unwrap().exists(), false);

        password.encrypt_with_key(
            format!("{}\\tests\\secret-key.asc", env!("CARGO_MANIFEST_DIR")).as_ref(),
        )?;

        // File should exist now
        assert_eq!(password.get_filepath().unwrap().exists(), true);

        git_operation(&password_store, &password)?;

        Ok(())
    }

    fn git_operation(password_store_path: &Path, password: &Password) -> anyhow::Result<()> {
        let repo = Repository::open(password_store_path)?;
        add_commit_password(&repo, &password)?;

        let commit = String::from(get_head_commit(&repo).unwrap().message().unwrap());
        let path_file = get_relative_path(&repo, password.get_filepath().unwrap()).unwrap();
        let commit_msg_to_check = format!("Added password {}", path_file.display());

        assert_eq!(commit, commit_msg_to_check);

        Ok(())
    }
}
