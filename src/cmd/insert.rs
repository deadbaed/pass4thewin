pub fn insert(password: &str, multi_line: Option<usize>, echo: bool, force: bool) {
    println!(
        "cmd insert: password {:?} multi_line {:?}, echo {} force {}",
        password, multi_line, echo, force
    );
}
