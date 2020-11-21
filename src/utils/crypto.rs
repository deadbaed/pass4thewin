use anyhow::Context;
use sequoia_openpgp::crypto::SessionKey;
use sequoia_openpgp::packet::{PKESK, SKESK};
use sequoia_openpgp::parse::stream::{
    DecryptionHelper, DecryptorBuilder, MessageStructure, VerificationHelper,
};
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::policy::{Policy, StandardPolicy};
use sequoia_openpgp::types::SymmetricAlgorithm;
use sequoia_openpgp::KeyID;
use sequoia_openpgp::Result;
use sequoia_openpgp::{Cert, Fingerprint, KeyHandle};
use std::path::Path;

struct Helper<'a> {
    policy: &'a dyn Policy,
    secret: &'a Cert,
}

impl<'a> VerificationHelper for Helper<'a> {
    fn get_certs(&mut self, _ids: &[KeyHandle]) -> Result<Vec<Cert>> {
        println!("getting certs");
        Ok(Vec::new())
    }

    fn check(&mut self, _structure: MessageStructure) -> Result<()> {
        println!("checking");
        Ok(())
    }
}

impl<'a> DecryptionHelper for Helper<'a> {
    fn decrypt<D>(
        &mut self,
        pkesks: &[PKESK],
        skesks: &[SKESK],
        sym_algo: Option<SymmetricAlgorithm>,
        mut decrypt: D,
    ) -> Result<Option<Fingerprint>>
    where
        D: FnMut(SymmetricAlgorithm, &SessionKey) -> bool,
    {
        println!("decrypting");

        // PKESK is the session key
        let session_key = &pkesks[0];
        let recipient_keyid = session_key.recipient();
        println!("recipient {}", recipient_keyid);

        // TODO: go through all the keys that can be used to decrypt stuff
        for ka in self
            .secret
            .keys()
            .with_policy(self.policy, None)
            .for_transport_encryption()
            .for_storage_encryption()
            .secret()
        {
            let key_keyid: KeyID = ka.key().fingerprint().into();
            if recipient_keyid == &key_keyid {
                println!("key match!");
            }
        }

        // TODO: collect when key match, otherwise if empty at end return error

        // TODO: prompt password to decrypt session key
        //
        // let password = rpassword::read_password_from_tty(Some(
        //     &"Enter password to decrypt key: ".to_string(),
        // ))?;

        Ok(None)
    }
}

pub fn decrypt(encrypted_path: &Path, key_path: &Path) -> Result<String> {
    let policy = &mut StandardPolicy::new();
    let cert = Cert::from_file(key_path).context("Failed to load key from file")?;

    let helper = Helper {
        policy,
        secret: &cert,
    };

    let decryptor = DecryptorBuilder::from_file(encrypted_path)
        .context(format!("Failed to open file {}", encrypted_path.display()))?;

    let mut decryptor = decryptor.with_policy(policy, None, helper)?;

    let mut writer: Vec<u8> = vec![];
    std::io::copy(&mut decryptor, &mut writer)?;

    Ok(String::from_utf8(writer)?)
}
