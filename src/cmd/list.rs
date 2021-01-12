use crate::settings::Settings;
use crate::tree::tree;
use anyhow::Context;

pub fn list(password: Option<String>, settings: &Settings) -> anyhow::Result<()> {
    let mut path = settings.get_password_store_path()?.to_path_buf();

    if let Some(p) = password {
        path.push(p);
    }

    let tree = tree(&path).context("Failed to construct tree")?;

    println!("{}", tree);

    Ok(())
}
