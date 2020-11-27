use anyhow::Context;
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::policy::StandardPolicy;
use sequoia_openpgp::serialize::stream::{Encryptor, LiteralWriter, Message};
use sequoia_openpgp::Cert;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn encrypt(
    plaintext: &str,
    ciphertext: &mut dyn Write,
    recipient: &Cert,
) -> sequoia_openpgp::Result<()> {
    let policy = &mut StandardPolicy::new();

    // Get recipient key
    let recipients = recipient
        .keys()
        .with_policy(policy, None)
        .for_transport_encryption()
        .for_storage_encryption()
        .secret()
        .alive()
        .revoked(false);

    // Start OpenPGP message
    let message = Message::new(ciphertext);

    // Define recipients of the message
    let message = Encryptor::for_recipients(message, recipients).build()?;

    // Emit literal data packet
    let mut message = LiteralWriter::new(message).build()?;

    // Encrypt data
    message.write_all(plaintext.as_bytes())?;

    // Finish OpenPGP message
    message.finalize()?;

    Ok(())
}

/// Encrypt the file `path` using a key contained in the file `key`
///
/// Return path of encrypted file
pub fn encrypt_file(path: &Path, key: &Path) -> anyhow::Result<PathBuf> {
    let file_contents = std::fs::read_to_string(path)?;

    // Use same filename as original file, only change the extension
    let output_path = path.with_extension("gpg");
    let mut output = File::create(&output_path)?;
    let cert = Cert::from_file(key).context("Failed to load key from file")?;

    encrypt(&file_contents, &mut output, &cert)?;

    Ok(output_path)
}
