use crate::settings::Settings;
use crate::sync;
use dialoguer::Confirm;
use git2::Repository;

pub fn m0ve(
    old_path: &str,
    new_path: &str,
    force: bool,
    settings: &Settings,
) -> anyhow::Result<()> {
    let mut old = settings.get_password_store_path()?.join(old_path);
    let new = settings.get_password_store_path()?.join(new_path);

    // Check if new path already exists, if so ask to overwrite if force flag is not passed
    if new.exists()
        && !force
        && !Confirm::new()
            .with_prompt(format!(
                "The entry {} already exists. Overwrite it?",
                new.display()
            ))
            .default(false)
            .show_default(true)
            .interact()?
    {
        // If user says no
        println!("Moving operation canceled");
        return Ok(());
    }

    // If old path does not exist, try with .gpg at the end
    if !old.exists() {
        old = settings
            .get_password_store_path()?
            .join(format!("{}.gpg", old_path));
    }

    // TODO: Move paths here
    // let options = fs_extra::dir::CopyOptions::new();
    // let mut from_paths = Vec::new();
    // from_paths.push(&old);
    //
    // match fs_extra::move_items(&from_paths, &new, &options) {
    //     Ok(result) => println!("ok {}", result),
    //     Err(e) => eprintln!("error {}", e),
    // }

    // Git operations if git repo is present
    if let Ok(repo) = Repository::open(&settings.get_password_store_path()?) {
        sync::rm_path(&repo, &old)?;
        sync::add_path(&repo, &new)?;
        let commit_message = format!("Moved {} to {}", old_path, new_path);
        sync::create_commit(&repo, &commit_message)?;
    }

    Ok(())
}
