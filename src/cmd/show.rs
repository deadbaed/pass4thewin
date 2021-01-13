use crate::password::Password;
use crate::settings::Settings;
use anyhow::{anyhow, Context};

pub fn show(
    password_name: Option<String>,
    line: Option<usize>,
    clipboard: bool,
    settings: &Settings,
) -> anyhow::Result<()> {
    // Check whether to run the `list` command or not
    match &password_name {
        Some(password) => {
            let path = settings.get_password_store_path()?.join(&password);

            // If password is a folder run `list` command instead
            if let Ok(path) = std::fs::metadata(&path) {
                if path.is_dir() {
                    return crate::cmd::list(Some(password.to_owned()), settings);
                }
            }
        }
        None => return crate::cmd::list(password_name, settings),
    }

    // Create empty password
    let mut password = Password::default();

    // Set path of password
    let password_name =
        password_name.context("There is no password to see (this should not happen)")?;
    password.set_filepath(settings.get_password_store_path()?, &password_name);

    // Check if password exists
    if !password.exists() {
        return Err(anyhow!("Password `{}` does not exist", password_name));
    }

    // Attempt to open and decrypt password in file
    let key_path = settings.get_pgp_key_path()?;
    password.open_decrypt(key_path)?;

    /*

    3. load file and decrypt with key
    4. handle options and display to output

    */

    Ok(())
}
