use crate::settings::Settings;
use crate::sync;
use anyhow::anyhow;
use dialoguer::Confirm;
use git2::Repository;

pub fn m0ve(
    old_path: &str,
    new_path: &str,
    force: bool,
    settings: &Settings,
) -> anyhow::Result<()> {
    let mut old = settings.get_password_store_path()?.join(old_path);
    let new = {
        let p = settings.get_password_store_path()?;

        // To be able to move stuff to the root of the password store
        // Works for `/`, `\`, `\\` and `.`
        if new_path == "/" || new_path == "\\" || new_path == "\\\\" || new_path == "." {
            p.to_path_buf()
        } else {
            p.join(new_path)
        }
    };

    // If old path does not exist, try with .gpg at the end
    if !old.exists() {
        old.set_extension("gpg");
        // If it still does not exist, then give up
        if !old.exists() {
            return Err(anyhow!("Could not locate {} in password store", old_path));
        }
    }

    // If `new_path` exists
    if new.exists() {
        // If it's a folder, move `old_path` inside folder `new_path`
        if new.is_dir() {
            // Construct list of paths to move
            let mut from_paths = Vec::new();
            from_paths.push(&old);

            let options = fs_extra::dir::CopyOptions::new();
            fs_extra::move_items(&from_paths, &new, &options)?;
        } else {
            // (It's a file)

            // File already exists and will be overwritten
            if !force
                && !Confirm::new()
                    .with_prompt(format!(
                        "The entry {} already exists. Overwrite it?",
                        new_path
                    ))
                    .default(false)
                    .show_default(true)
                    .interact()?
            {
                // If user says no
                println!("Moving operation canceled");
                return Ok(());
            }

            // Rename old_path to new_path, overwrite file at new_path
            std::fs::rename(&old, &new)?;
        }
    } else {
        // Rename old_path to new_path
        std::fs::rename(&old, &new)?;
    }

    println!("Moved `{}` to `{}`", old_path, new_path);

    // Git operations if git repo is present
    if let Ok(repo) = Repository::open(&settings.get_password_store_path()?) {
        sync::rm_path(&repo, &old)?;
        sync::add_path(&repo, &new)?;
        let commit_message = format!("Moved {} to {}", old_path, new_path);
        sync::create_commit(&repo, &commit_message)?;
    }

    Ok(())
}
