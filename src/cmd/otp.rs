use crate::password::{set_to_clipboard, Password};
use crate::settings::Settings;
use anyhow::{anyhow, Context};
use std::time::SystemTime;
use totp_rs::TOTP;
use url::Url;

fn extract_secret_key(password: &Password) -> anyhow::Result<Vec<u8>> {
    let contents = password
        .line(1)
        .context("Failed to get first line of password")?;

    // Parse secret key from URI
    // Keep first instance of "secret"
    let secret_key = Url::parse(contents)?
        .query_pairs()
        .filter(|(key, _)| key == "secret")
        .map(|(_, value)| value)
        .collect::<Vec<_>>()
        .get(0)
        .context("Failed to get secret key")?
        .to_string();

    // Decode secret key
    base32::decode(base32::Alphabet::RFC4648 { padding: false }, &secret_key)
        .context("Failed to decode secret key")
}

fn generate_otp_token(secret_key: &[u8]) -> anyhow::Result<String> {
    let totp = TOTP::new(totp_rs::Algorithm::SHA1, 6, 1, 30, secret_key);
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    Ok(totp.generate(time))
}

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

    // Get OTP secret key
    let secret_key = extract_secret_key(&password)?;

    // Attempt to generate otp token
    let token = generate_otp_token(&secret_key)?;

    if clipboard {
        return set_to_clipboard(&token, password_name);
    }

    println!("{}", token);

    Ok(())
}
