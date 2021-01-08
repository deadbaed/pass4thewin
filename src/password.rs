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

        // Insert line by line to final vector
        for line in raw_input.lines() {
            output.push(line.to_string());
        }

        if output.is_empty() {
            return Err(Error::new(ErrorKind::Other, "Empty password"));
        }

        // Save final vector
        self.password = Some(output);

        Ok(())
    }

    pub fn output(&self) {
        // raw text, qr code, otp, multiple lines or not
    }

    /// Format password as a single block
    pub fn to_string(&self) -> Option<String> {
        let mut string = String::new();

        for line in self.password.as_ref()? {
            string.push_str(&line);
        }

        Some(string)
    }
}
