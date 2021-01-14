use crate::decrypt::decrypt;
use crate::encrypt::encrypt;
use anyhow::{anyhow, Context};
use clipboard_win::set_clipboard_string;
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::Cert;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct Password {
    // Path of file
    path_file: Option<PathBuf>,

    // Password in plain text
    password: Option<Vec<String>>,
}

impl Password {
    pub fn get_filepath(&self) -> Option<&PathBuf> {
        self.path_file.as_ref()
    }

    pub fn set_filepath(&mut self, base_path: &Path, password_name: &str) {
        self.path_file = Some(base_path.join(format!("{}.gpg", password_name)));
    }

    pub fn exists(&self) -> bool {
        match &self.path_file {
            Some(path) => path.exists(),
            None => false,
        }
    }

    /// Open password in file and decrypt it with `key`
    pub fn open_decrypt(
        &mut self,
        key_path: &Path,
        password: Option<String>,
    ) -> anyhow::Result<()> {
        let file_path = self
            .get_filepath()
            .context("Path of password is not set (this should not happen)")?;

        // Attempt to decrypt password, result will be a one-line string
        let raw_file = decrypt(file_path, key_path, password)?;

        // Store lines in vector
        let vec = string_to_vec(&raw_file).context("Failed to parse password")?;

        // Save vector
        self.password = Some(vec);

        Ok(())
    }

    /// Encrypt the password using a key contained in the file `key`
    pub fn encrypt_with_key(&self, key: &Path) -> anyhow::Result<()> {
        // Create output file
        let path = self
            .get_filepath()
            .context("Path of password is not set (this should not happen)")?;

        let parent_path = path.parent().context("Get parent folder")?;
        std::fs::create_dir_all(parent_path).context("Creating folders for password")?;

        let mut output = File::create(path)?;

        // Get pgp key
        let cert = Cert::from_file(key).context("Failed to load key from file")?;

        // Get password contents
        let contents = self.to_string()?;

        encrypt(&contents, &mut output, &cert)
    }

    #[cfg(test)]
    pub fn from_single_line(s: &str) -> Self {
        let mut vec = Vec::new();
        vec.push(s.to_string());

        Self {
            password: Some(vec),
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub fn from_multi_line(v: &Vec<String>) -> Self {
        Self {
            password: Some(v.clone()),
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub fn clean_password(&mut self) {
        self.password = None;
    }

    /// Set password from a single line input
    pub fn single_line_input(&mut self, s: &str) {
        let mut vec = Vec::new();
        vec.push(s.into());

        self.password = Some(vec);
    }

    /// Set password from a multi line input
    pub fn multi_line_input(&mut self, s: &str) {
        self.password = string_to_vec(s);
    }

    /// Get password from terminal
    pub fn terminal_input(&mut self, password_name: &str, multi_line: bool) -> Result<(), Error> {
        // Get input
        let raw_input = {
            if multi_line {
                let mut buffer = String::new();
                println!(
                    "Enter contents of {} and press Ctrl+Z when finished:",
                    password_name
                );
                std::io::stdin().read_to_string(&mut buffer)?;

                buffer
            } else {
                let password1 = rpassword::read_password_from_tty(Some(
                    format!("Enter password for {}: ", password_name).as_str(),
                ))?;

                // Make sure first entry is not empty to move forward
                if password1.is_empty() {
                    return Err(Error::new(ErrorKind::Other, "Empty password"));
                }
                let password2 = rpassword::read_password_from_tty(Some(
                    format!("Retype password for {}: ", password_name).as_str(),
                ))?;

                if password1 != password2 {
                    return Err(Error::new(ErrorKind::Other, "Passwords do not match"));
                }

                password1
            }
        };

        // Convert one-line password string to vector
        let output = match string_to_vec(&raw_input) {
            Some(output) => output,
            None => return Err(Error::new(ErrorKind::Other, "Empty password")),
        };

        // Save final vector
        self.password = Some(output);

        Ok(())
    }

    /// Format password as a single block
    pub fn to_string(&self) -> anyhow::Result<String> {
        let mut string = String::new();

        let password = self
            .password
            .as_ref()
            .context("There is no password, this should never happen")?;

        for line in password {
            string.push_str(&line);
        }

        Ok(string)
    }

    /// Get specific line of password
    pub fn line(&self, line: usize) -> Option<&str> {
        let line = {
            // Try to prevent from accessing line `-1`
            if line == 0 {
                line
            } else {
                // We start counting from 0, remember?
                line - 1
            }
        };

        Some(self.password.as_ref()?.get(line)?.as_str())
    }
}

/// Convert a one-line password to a vector
fn string_to_vec(input: &str) -> Option<Vec<String>> {
    let mut vec = Vec::new();

    // Iterator over all lines from input
    let mut iter = input.lines();

    // Get first line
    let mut first = match iter.next() {
        Some(first) => first.to_string(),
        None => return None,
    };

    // Try to get second line (if it's a multiline input)
    match iter.next() {
        Some(second) => {
            // If input is multiline, add a unix newline for first line
            first.push('\n');
            vec.push(first);

            // Do the same for the second line (otherwise it will be lost)
            let mut second = second.to_string();
            second.push('\n');
            vec.push(second);
        }
        // If there is no second line return first line without newline
        None => vec.push(first),
    }

    // Iterate over the rest of the multiline input
    for line in iter {
        // Add unix newline character at the end of each line
        let mut line = line.to_string();
        line.push('\n');

        vec.push(line);
    }

    Some(vec)
}

/// Set a password to the clipboard
pub fn set_to_clipboard(s: &str, name: &str) -> anyhow::Result<()> {
    match set_clipboard_string(s) {
        Ok(()) => {
            println!("Password `{}` has been put on the clipboard", name);
            Ok(())
        }
        Err(e) => Err(anyhow!(
            "Failed to put password `{}` to the clipboard\n{:?}",
            name,
            e
        )),
    }
}
