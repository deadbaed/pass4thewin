use std::path::PathBuf;

pub struct Password {
    // Path of file
    path: Option<PathBuf>,

    // Contents: vector?
    contents: Option<Vec<String>>,
}

impl Password {
    fn read() {
        // open from file, decrypt here
    }

    fn write() {
        // write to file, encrypt here, force writing or not
    }

    fn input() {
        // terminal or notepad? multiple lines or not
    }

    fn output() {
        // raw text, qr code, otp, multiple lines or not
    }
}
