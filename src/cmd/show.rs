pub fn show(password: &str, line: Option<usize>, clipboard: bool) {
    println!(
        "cmd show: password {} line {:?} clipboard {:?}",
        password, line, clipboard
    );
    println!("TODO: if it's a folder forget about line and clipboard");
}
