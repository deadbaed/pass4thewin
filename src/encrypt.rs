use sequoia_openpgp::policy::StandardPolicy;
use sequoia_openpgp::serialize::stream::{Encryptor, LiteralWriter, Message};
use sequoia_openpgp::Cert;
use std::io::Write;

pub fn encrypt(
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
