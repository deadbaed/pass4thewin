use crate::settings::Settings;
use crate::sync::{add_commit_file, init_repo};
use git2::Repository;

pub fn init(settings: &Settings) -> anyhow::Result<()> {
    let password_store_path = settings.get_password_store_path()?;

    // try to open repo
    if Repository::open(&password_store_path).is_ok() {
        // if ok, go out
        println!(
            "Git repository is already initialized for password store {}",
            password_store_path.display()
        );
        return Ok(());
    }

    // else create one
    let repo = init_repo(&password_store_path)?;

    // add .gpg-id file
    add_commit_file(&repo, &password_store_path.join(".gpg-id"))?;

    println!(
        "Initiated git repository for password store {}",
        password_store_path.display()
    );

    Ok(())
}
