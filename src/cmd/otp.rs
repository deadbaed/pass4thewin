use crate::password::Password;
use crate::settings::Settings;
use anyhow::anyhow;
use std::time::SystemTime;
use totp_rs::{Algorithm, TOTP};

pub fn code(password_name: &str, clipboard: bool, settings: &Settings) -> anyhow::Result<()> {
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

    let contents = password.to_string()?;

    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, "secret_here");
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let url = totp.get_url("account_name", "issuer");
    println!("{}", url);
    let token = totp.generate(time);
    println!("{}", token);

    Ok(())
}
