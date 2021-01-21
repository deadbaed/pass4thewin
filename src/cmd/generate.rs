use crate::password::{set_to_clipboard, Password};
use crate::settings::Settings;
use crate::sync::add_commit_password;
use anyhow::anyhow;
use dialoguer::Confirm;
use git2::Repository;
use passwords::PasswordGenerator;

pub fn generate(
    password_name: &str,
    length: Option<usize>,
    force: bool,
    clipboard: bool,
    settings: &Settings,
) -> anyhow::Result<()> {
    // Create empty password
    let mut password = Password::default();

    // Set path of password
    password.set_filepath(settings.get_password_store_path()?, password_name);

    // If path is a folder
    if password.get_filepath().is_none() {
        return Err(anyhow!("`{}` is a folder in password store", password_name));
    }

    // Check if file exists, if so ask to overwrite if force flag is not passed
    if password.exists()
        && !force
        && !Confirm::new()
            .with_prompt(format!(
                "An entry already exists for `{}`. Overwrite it?",
                password_name
            ))
            .default(false)
            .show_default(true)
            .interact()?
    {
        // If user says no
        println!("Password generation canceled.");
        return Ok(());
    }

    // Generate password
    let generated = PasswordGenerator::new()
        .length(20)
        .spaces(true)
        .exclude_similar_characters(true);
    let generated = match length {
        Some(length) => generated.length(length),
        None => generated,
    };
    let output = match generated.generate_one() {
        Ok(output) => output,
        Err(e) => {
            return Err(anyhow!("Password generation failed: {}", e));
        }
    };
    password.single_line_input(&output);

    // Encrypt password and write output to file
    password.encrypt_with_key(settings.get_pgp_key_path()?)?;

    // Git operations if git repo is present
    if let Ok(repo) = Repository::open(&settings.get_password_store_path()?) {
        add_commit_password(&repo, &password)?
    }

    if clipboard {
        return set_to_clipboard(&output, &password_name);
    }

    println!("Inserted `{}` in password store", password_name);

    // Display password
    println!("====\n{}", password.to_string()?);

    Ok(())
}
