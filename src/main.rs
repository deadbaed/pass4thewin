use std::fs;

fn add_corner_current_line(str: &mut String, cur_elem: usize, num_elements: usize) {
    if cur_elem + 1 == num_elements {
        str.push_str("└── ");
    } else {
        str.push_str("├── ");
    }
}

fn add_corner_other_line(str: &mut String, cur_elem: usize, num_elements: usize) {
    if cur_elem + 1 == num_elements {
        str.push(' ');
    } else {
        str.push('|');
    }
    str.push_str("   ");
}

fn tree(path: &str, string: &mut String) {
    let mut cur_elem: usize = 0;
    let num_elements = fs::read_dir(path).unwrap().count();

    for entry in fs::read_dir(path).unwrap() {
        let dir = entry.unwrap();
        let current_path = dir.path();
        let current_name = dir.file_name();
        let current_name = current_name.to_str().unwrap();

        // if current path is a hidden folder / file, skip it
        if current_name.starts_with(".") {
            cur_elem += 1;
            continue;
        }

        // if next element is the last
        add_corner_current_line(string, cur_elem, num_elements);

        println!("{}{}", string, current_name);
        for _ in 0..4 {
            string.pop();
        }
        if current_path.is_dir() {
            // if next element is the last
            add_corner_other_line(string, cur_elem, num_elements);

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
