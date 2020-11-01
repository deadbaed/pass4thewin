use std::fs;

fn tree(path: &str, string: &mut String) {
    let mut cur_len: usize = 0;
    let len_folders = fs::read_dir(path).unwrap().count();

    for entry in fs::read_dir(path).unwrap() {
        let dir = entry.unwrap();
        let current_path = dir.path();

        // if current path is a hidden folder / file, skip it
        if dir.file_name().to_str().unwrap().starts_with(".") {
            cur_len += 1;
            continue;
        }

        if cur_len + 1 == len_folders {
            string.push_str("└── ");
        } else {
            string.push_str("├── ");
        }
        println!("{}{}", string, dir.file_name().to_str().unwrap());
        for _ in 0..4 {
            string.pop();
        }
        if current_path.is_dir() {
            if cur_len + 1 != len_folders {
                string.push('|');
            } else {
                string.push(' ');
            }
            string.push_str("   ");
            tree(current_path.to_str().unwrap(), string);
            for _ in 0..4 {
                string.pop();
            }
        }
        cur_len += 1;
    }
}

fn main() {
    let path = "C:\\users\\x4m3\\.password-store\\";

    let mut string = String::new();
    println!("{}", path);
    tree(path, &mut string);
}
