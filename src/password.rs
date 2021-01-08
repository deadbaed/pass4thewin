use std::io::{Error, ErrorKind, Read};
use std::path::PathBuf;

#[derive(Default)]
pub struct Password {
    // Path of file
    path: Option<PathBuf>,

    // Password contents
    contents: Option<Vec<String>>,
}

impl Password {
    pub fn read(&self) {
        // open from file, decrypt here
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
                // TODO: add retype password
                rpassword::read_password_from_tty(Some(
                    format!("Enter password for {}: ", password_name).as_str(),
                ))?
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
        self.contents = Some(output);

        Ok(())
    }

    pub fn output(&self) {
        // raw text, qr code, otp, multiple lines or not
    }
}
