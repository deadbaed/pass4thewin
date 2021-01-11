use anyhow::Context;
use qr2term::print_qr;
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

    pub fn read(&self) {
        // open from file, decrypt here
        // store file line by line: https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings
    }

    pub fn write(&self) {
        // write to file, encrypt here, force writing or not
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

        let mut output = Vec::new();

        // Iterator over all lines from input
        let mut iter = raw_input.lines();

        // Get first line
        let mut first = match iter.next() {
            Some(first) => first.to_string(),
            None => return Err(Error::new(ErrorKind::Other, "Empty password")),
        };

        // Try to get second line (if it's a multiline input)
        match iter.next() {
            Some(second) => {
                // If input is multiline, add a unix newline for first line
                first.push('\n');
                output.push(first);

                // Do the same for the second line (otherwise it will be lost)
                let mut second = second.to_string();
                second.push('\n');
                output.push(second);
            }
            // If there is no second line return first line without newline
            None => output.push(first),
        }

        // Iterate over the rest of the multiline input
        for line in iter {
            // Add unix newline character at the end of each line
            let mut line = line.to_string();
            line.push('\n');

            output.push(line);
        }

        // Save final vector
        self.password = Some(output);

        Ok(())
    }

    pub fn output(&self) {
        // raw text, otp, multiple lines or not
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
    pub fn line(&self, line: usize) -> Option<&String> {
        let line = {
            // Try to prevent from accessing line `-1`
            if line == 0 {
                line
            } else {
                // We start counting from 0, remember?
                line - 1
            }
        };

        self.password.as_ref()?.get(line)
    }

    /// Format password as a qr code, written directly to [stdout]
    ///
    /// [stdout]: std::io::stdout
    pub fn to_qrcode(&self) -> anyhow::Result<()> {
        match print_qr(self.to_string()?) {
            Ok(()) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
