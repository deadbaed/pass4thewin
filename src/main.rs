use std::fs;

fn tree(path: &str, string: &mut String) {
    let mut cur_elem: usize = 0;
    let num_elements = fs::read_dir(path).unwrap().count();

    for entry in fs::read_dir(path).unwrap() {
        let dir = entry.unwrap();
        let current_path = dir.path();
        let current_filename = dir.file_name();
        let current_filename = current_filename.to_str().unwrap();

        // if current path is a hidden folder / file, skip it
        if current_filename.starts_with(".") {
            cur_elem += 1;
            continue;
        }

        // if next element is the last
        if cur_elem + 1 == num_elements {
            string.push_str("└── ");
        } else {
            string.push_str("├── ");
        }

        println!("{}{}", string, current_filename);
        for _ in 0..4 {
            string.pop();
        }
        if current_path.is_dir() {
            // if next element is the last
            if cur_elem + 1 == num_elements {
                string.push(' ');
            } else {
                string.push('|');
            }

            string.push_str("   ");
            tree(current_path.to_str().unwrap(), string);
            for _ in 0..4 {
                string.pop();
            }
        }
        cur_elem += 1;
    }
}

fn main() {
    let path = "C:\\users\\x4m3\\.password-store\\";

    let mut string = String::new();
    println!("{}", path);
    tree(path, &mut string);
}
