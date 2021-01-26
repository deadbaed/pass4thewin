use crate::settings::Settings;
use anyhow::{anyhow, Context};
use directories_next::BaseDirs;
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::policy::StandardPolicy;
use sequoia_openpgp::{Cert, Fingerprint};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Extract fingerprint of first private key found in file
fn extract_fingerprint(key: &Path) -> anyhow::Result<Fingerprint> {
    let policy = &mut StandardPolicy::new();
    let cert = Cert::from_file(key).context("Failed to load key from file")?;

    // let secret_keys: Vec<Key<key::SecretParts, key::UnspecifiedRole>> = cert // <- for Rust on CLion
    let secret_keys = cert
        .keys()
        .with_policy(policy, None)
        .for_transport_encryption()
        .for_storage_encryption()
        .secret()
        .alive()
        .revoked(false)
        .into_iter()
        .map(|key| key.key().clone())
        .collect::<Vec<_>>();

    let secret_key = match secret_keys.first() {
        Some(key) => key,
        None => return Err(anyhow!("Failed to find a secret key in file")),
    };
    Ok(secret_key.fingerprint())
}

/// Get path of a potential password store based off provided path
fn get_password_store_path(path: &Option<PathBuf>) -> anyhow::Result<PathBuf> {
    let mut new_path = match path {
        Some(path) => dunce::canonicalize(&path)?,
        None => match BaseDirs::new() {
            Some(home_dir) => PathBuf::from(home_dir.home_dir()),
            None => return Err(anyhow!("Failed to get home directory path")),
        },
    };
    new_path.push(".password-store");

    Ok(new_path)
}

/// Create new password store on path with provided pgp key
///
/// Returns path of new password store
pub fn new_password_store(pgp_key: &Path, path: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    // Extract fingerprint from file
    let fingerprint = extract_fingerprint(pgp_key)?;

    // Create password store folder
    let password_store_path = get_password_store_path(&path)?;
    std::fs::create_dir_all(&password_store_path)?;

    // Display fingerprint in hex
    let content = format!("0x{:X}\n", fingerprint);

    // Construct path of `.gpg-id`
    let mut gpg_id_path = password_store_path.clone();
    gpg_id_path.push(".gpg-id");

    // Write file
    let mut file = File::create(&gpg_id_path)?;
    file.write_all(content.as_bytes())?;

    // sign `.gpg-id` with key ??

    println!(
        "Created new password store at location {}",
        password_store_path.display()
    );
    Ok(password_store_path)
}

pub fn init(pgp_key: &Path, path: Option<PathBuf>, settings: &mut Settings) -> anyhow::Result<()> {
    // Check if a password store exists at provided location
    let mut password_store_path = get_password_store_path(&path)?;
    password_store_path.push(".gpg-id");

    let pgp_key = dunce::canonicalize(pgp_key)?;

    // Create new password store if there is not
    if !password_store_path.is_file() {
        password_store_path = new_password_store(&pgp_key, path)?;
    } else {
        password_store_path.pop();
        println!(
            "Detected password store at location {}",
            password_store_path.display()
        );
    }

    // Set and write settings
    settings.set_pgp_key_path(&pgp_key);
    settings.set_password_store_path(&password_store_path);
    settings.write()?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::cmd::init::new_password_store;
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn create_password_store() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempdir()?;

        let secret_key_path = format!("{}\\tests\\secret-key.asc", env!("CARGO_MANIFEST_DIR"));

        let password_store_path = new_password_store(
            secret_key_path.as_ref(),
            Some(PathBuf::from(tmp_dir.path())),
        )?;

        let gpg_id_path = password_store_path.join(".gpg-id");
        assert_eq!(gpg_id_path.is_file(), true);

        // Print contents of pgp-id
        let mut gpg_id = File::open(&gpg_id_path)?;
        let mut contents = String::new();
        gpg_id.read_to_string(&mut contents)?;
        println!("Contents: {:?}", contents);

        Ok(())
    }
}
