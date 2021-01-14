use crate::password::Password;
use crate::settings::Settings;
use crate::sync::add_commit_update_password;
use anyhow::{anyhow, Context};
use git2::Repository;

pub fn edit(password_name: &str, settings: &Settings) -> anyhow::Result<()> {
    // Create empty password
    let mut password = Password::default();

    // Set path of password
    password.set_filepath(settings.get_password_store_path()?, &password_name);

    // Check if password exists
    if !password.exists() {
        return Err(anyhow!("Password `{}` does not exist", password_name));
    }

    // Attempt to open and decrypt password in file
    let key_path = settings.get_pgp_key_path()?;
    password.open_decrypt(key_path, None)?;

    // Open contents in text editor
    let original_password = password.to_string()?;
    let modified_password = edit::edit(&original_password)
        .context(format!("Failed to edit password {}", password_name))?;

    // Don't do anything if there are no changes
    if original_password == modified_password {
        println!("Password unchanged");
        return Ok(());
    }

    // Use updated password
    password.multi_line_input(&modified_password);

    // Encrypt password and write output to file
    password.encrypt_with_key(settings.get_pgp_key_path()?)?;

    // Git operations if git repo is present
    if let Ok(repo) = Repository::open(&settings.get_password_store_path()?) {
        add_commit_update_password(&repo, &password)?
    }

    println!("Password `{}` has been updated", password_name);

    Ok(())
}
