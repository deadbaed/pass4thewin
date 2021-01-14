use anyhow::anyhow;
use anyhow::Context;
use sequoia_openpgp::crypto::{Password, SessionKey};
use sequoia_openpgp::packet::{key, Key, PKESK, SKESK};
use sequoia_openpgp::parse::stream::{
    DecryptionHelper, DecryptorBuilder, MessageStructure, VerificationHelper,
};
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::policy::{Policy, StandardPolicy};
use sequoia_openpgp::types::SymmetricAlgorithm;
use sequoia_openpgp::Result;
use sequoia_openpgp::{crypto, KeyID};
use sequoia_openpgp::{Cert, Fingerprint, KeyHandle};
use std::path::Path;

struct Helper {
    cert: Cert,
    secret_keys: Vec<Key<key::SecretParts, key::UnspecifiedRole>>,
    password: Option<String>,
}

impl VerificationHelper for Helper {
    fn get_certs(&mut self, _ids: &[KeyHandle]) -> Result<Vec<Cert>> {
        Ok(Vec::new())
    }

    fn check(&mut self, _structure: MessageStructure) -> Result<()> {
        Ok(())
    }
}

impl Helper {
    fn new(policy: &dyn Policy, cert: Cert, password: Option<String>) -> Self {
        // Import all secrets keys found in cert
        let secret_keys = cert
            .keys()
            .with_policy(policy, None)
            .for_transport_encryption()
            .for_storage_encryption()
            .secret()
            .into_iter()
            .map(|key| key.key().clone())
            .collect::<Vec<_>>();

        Self {
            cert,
            secret_keys,
            password,
        }
    }

    /// Try to decrypt PKESK packet with `keypair`
    /// And try to decrypt packet parser with `decrypt`
    fn try_decrypt<D>(
        &self,
        pkesk: &PKESK,
        sym_algo: Option<SymmetricAlgorithm>,
        keypair: &mut dyn crypto::Decryptor,
        decrypt: &mut D,
    ) -> Option<Fingerprint>
    where
        D: FnMut(SymmetricAlgorithm, &SessionKey) -> bool,
    {
        match pkesk
            .decrypt(keypair, sym_algo)
            .and_then(|(algo, session_key)| {
                if decrypt(algo, &session_key) {
                    Some(session_key)
                } else {
                    None
                }
            }) {
            Some(_session_key) => Some(self.cert.fingerprint()),
            None => None,
        }
    }
}

/// Go through secret keys loaded from file
/// and return the Key whose KeyID
/// matches with the KeyID of the recipient.
///
/// Return `None` if no key matches
fn get_secret_key_for_recipient(
    secret_keys: &[Key<key::SecretParts, key::UnspecifiedRole>],
    recipient: KeyID,
) -> Option<Key<key::SecretParts, key::UnspecifiedRole>> {
    for key in secret_keys {
        if KeyID::from(key.fingerprint()) == recipient {
            return Some(key.clone());
        }
    }
    None
}

impl DecryptionHelper for Helper {
    fn decrypt<D>(
        &mut self,
        pkesks: &[PKESK],
        _skesks: &[SKESK],
        sym_algo: Option<SymmetricAlgorithm>,
        mut decrypt: D,
    ) -> Result<Option<Fingerprint>>
    where
        D: FnMut(SymmetricAlgorithm, &SessionKey) -> bool,
    {
        // PKESK is the session key
        let session_key = &pkesks[0];
        let recipient_keyid = session_key.recipient().clone();

        // Get secret key to use to decrypt file
        let mut secret_key = get_secret_key_for_recipient(&self.secret_keys, recipient_keyid)
            .context("Could not find key to decrypt file")?;

        // Try to use secret key without prompting for a password
        if !secret_key.secret().is_encrypted() {
            if let Some(fingerprint) =
                secret_key
                    .clone()
                    .into_keypair()
                    .ok()
                    .and_then(|mut keypair| {
                        self.try_decrypt(session_key, sym_algo, &mut keypair, &mut decrypt)
                    })
            {
                return Ok(Some(fingerprint));
            }
        }

        let keypair = {
            match &self.password {
                Some(password) => {
                    // If we already have a password to try against
                    let algo = secret_key.pk_algo();
                    let password = Password::from(password.clone());
                    if let Ok(()) = secret_key.secret_mut().decrypt_in_place(algo, &password) {
                        Some(secret_key.clone().into_keypair()?)
                    } else {
                        None
                    }
                }
                None => loop {
                    // Loop and ask for a password until we get a good one or user aborts decryption
                    let password =
                        rpassword::read_password_from_tty(Some("Enter password to decrypt key: "))?
                            .into();

                    let algo = secret_key.pk_algo();
                    if let Ok(()) = secret_key.secret_mut().decrypt_in_place(algo, &password) {
                        break Some(secret_key.clone().into_keypair()?);
                    } else {
                        eprintln!("Bad password. Please try again (press Ctrl+C to cancel)")
                    }
                },
            }
        };

        if keypair.is_none() {
            return Err(anyhow!("Invalid password for key"));
        }

        if let Some(fingerprint) =
            self.try_decrypt(session_key, sym_algo, &mut keypair.unwrap(), &mut decrypt)
        {
            return Ok(Some(fingerprint));
        }

        Ok(None)
    }
}

pub fn decrypt(encrypted_path: &Path, key_path: &Path, password: Option<String>) -> Result<String> {
    let policy = &mut StandardPolicy::new();
    let cert = Cert::from_file(key_path).context("Failed to load key from file")?;

    let helper = Helper::new(policy, cert, password);

    let decryptor = DecryptorBuilder::from_file(encrypted_path)
        .context(format!("Failed to open file {}", encrypted_path.display()))?;

    let mut decryptor = decryptor.with_policy(policy, None, helper)?;

    let mut writer: Vec<u8> = vec![];
    std::io::copy(&mut decryptor, &mut writer)?;

    Ok(String::from_utf8(writer)?)
}
