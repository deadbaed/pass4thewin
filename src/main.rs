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

fn clean_corner(str: &mut String) {
    for _ in 0..4 {
        str.pop();
    }
}

fn tree(path: &str, string: &mut String, corner: &mut String) {
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
        add_corner_current_line(corner, cur_elem, num_elements);

        // put corner and current name
        string.push_str(corner);
        string.push_str(current_name);
        string.push('\n');

        // clean corner
        clean_corner(corner);

        if current_path.is_dir() {
            // if next element is the last
            add_corner_other_line(corner, cur_elem, num_elements);

            // pass through new folder
            tree(current_path.to_str().unwrap(), string, corner);

            // clean corner
            clean_corner(corner);
        }
        cur_elem += 1;
    }
}

fn main() {
    let path = "C:\\users\\x4m3\\.password-store\\";

    let mut corner = String::new();
    let mut string = String::new();
    println!("{}", path);
    tree(path, &mut string, &mut corner);

    println!("{}", string);
}
