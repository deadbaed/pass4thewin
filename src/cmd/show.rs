use crate::password::Password;
use crate::settings::Settings;
use anyhow::{anyhow, Context};
use clipboard_win::set_clipboard_string;
use qr2term::print_qr;

pub fn show(
    password_name: Option<String>,
    line: Option<usize>,
    clipboard: bool,
    qr_code: bool,
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
    password.open_decrypt(key_path, None)?;

    // Get specific line if asked
    let output = match line {
        Some(line) => password
            .line(line)
            .context(format!("Failed to get line {} of {}", line, password_name))?
            .into(),
        None => password.to_string().unwrap(),
    };

    if qr_code {
        return match print_qr(output) {
            Ok(()) => Ok(()),
            Err(e) => Err(e.into()),
        };
    }

    if clipboard {
        return match set_clipboard_string(&output) {
            Ok(()) => Ok(()),
            Err(e) => Err(anyhow!(
                "Failed to put password `{}` to the clipboard\n{:?}",
                password_name,
                e
            )),
        };
    }

    println!("{}", output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::password::Password;
    use tempfile::tempdir;

    #[test]
    fn full_password() -> anyhow::Result<()> {
        let tmp_dir = tempdir()?;

        let pgp_key = format!("{}\\tests\\secret-key.asc", env!("CARGO_MANIFEST_DIR"));
        let password_contents = "my_super_secure_password";
        let password_name = "password";
        let password_store = crate::cmd::insert::tests::create_password_store(&tmp_dir.path())?;
        crate::cmd::git::init(&password_store)?;

        // create password
        let mut password = Password::from_single_line(password_contents);
        password.set_filepath(&password_store, password_name);
        password.encrypt_with_key(pgp_key.as_ref())?;

        // clean password struct
        password.clean_password();

        // load from encrypted file with password
        password.open_decrypt(pgp_key.as_ref(), Some("password".into()))?;

        // make sure the content is the same as input
        assert_eq!(password.to_string()?, password_contents);

        Ok(())
    }

    #[test]
    fn specific_line() -> anyhow::Result<()> {
        let tmp_dir = tempdir()?;

        let pgp_key = format!("{}\\tests\\secret-key.asc", env!("CARGO_MANIFEST_DIR"));
        let mut password_contents = Vec::new();
        password_contents.push("multi\n".to_string());
        password_contents.push("line\n".to_string());
        password_contents.push("password\n".to_string());
        let password_name = "password";
        let password_store = crate::cmd::insert::tests::create_password_store(&tmp_dir.path())?;
        crate::cmd::git::init(&password_store)?;

        // create password
        let mut password = Password::from_multi_line(&password_contents);
        password.set_filepath(&password_store, password_name);
        password.encrypt_with_key(pgp_key.as_ref())?;

        // clean password struct
        password.clean_password();

        // load from encrypted file with password
        password.open_decrypt(pgp_key.as_ref(), Some("password".into()))?;

        // make sure the content is the same as input
        assert_eq!(password.line(2).unwrap(), password_contents.get(1).unwrap());

        Ok(())
    }
}
