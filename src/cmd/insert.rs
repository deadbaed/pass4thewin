use crate::password::Password;
use crate::settings::Settings;
use anyhow::anyhow;

pub fn insert(
    password_name: &str,
    multi_line: bool,
    echo: bool,
    force: bool,
    settings: &Settings,
) -> anyhow::Result<()> {
    // Make sure password store exists
    settings.get_password_store_path()?;

    // TODO: Check if file exists and ask for overwrite (ignore if flag force is passed)

    // Create empty password
    let mut password = Password::default();

    // Get password from terminal
    if let Err(e) = password.terminal_input(password_name, multi_line) {
        return Err(anyhow!("Password insertion aborted: {}", e));
    }

    /*

      1. check if file exists (unless force flag is passed)
      2. get password (single line: ask twice for confirmation, multiline open notepad or terminal?)
      3. put contents in tmp file
      4. encrypt tmp file
      5. create folders if needed before
      5. move it to path
      6. add commit
      7. if echo flag is on display password

    */

    Ok(())
}
