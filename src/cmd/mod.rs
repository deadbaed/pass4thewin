pub mod git;

pub fn show(password: &str, line: Option<usize>, clipboard: bool) {
    println!(
        "cmd show: password {} line {:?} clipboard {:?}",
        password, line, clipboard
    );
    println!("TODO: if it's a folder forget about line and clipboard");
}

pub fn list(password: Option<String>) {
    println!("cmd list: password {:?}", password);
}

pub fn init(gpg_id: &str) {
    println!("cmd init: gpg_id {:?}", gpg_id);
}

pub fn find(search: &str) {
    println!("cmd find: search {:?}", search);
}

pub fn insert(password: &str, multi_line: Option<usize>, echo: bool, force: bool) {
    println!(
        "cmd insert: password {:?} multi_line {:?}, echo {} force {}",
        password, multi_line, echo, force
    );
}

pub fn edit(password: &str) {
    println!("cmd edit: password {}", password);
}

pub fn generate(password: &str, length: Option<usize>) {
    println!("cmd generate: password {} length {:?}", password, length);
}

pub fn remove(path: &str, recursive: bool, force: bool) {
    println!(
        "cmd generate: path {} recursive {} force {}",
        path, recursive, force
    );
}

pub fn m0ve(old_path: &str, new_path: &str, force: bool) {
    println!(
        "cmd move: old_path {} new_path {} force {}",
        old_path, new_path, force
    );
}

pub fn copy(old_path: &str, new_path: &str, force: bool) {
    println!(
        "cmd copy: old_path {} new_path {} force {}",
        old_path, new_path, force
    );
}
