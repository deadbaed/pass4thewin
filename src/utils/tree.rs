use std::fs;
use std::io;

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

fn tree_folder(path: &str, string: &mut String, corner: &mut String) -> io::Result<()> {
    let mut cur_elem: usize = 0;
    let num_elements = fs::read_dir(path)?.count();

    for entry in fs::read_dir(path)? {
        let dir = entry?;
        let current_path = dir.path();
        let current_path_str = current_path
            .to_str()
            .expect("can't convert path to a utf8 string");
        let current_name = dir.file_name();
        let current_name = current_name
            .to_str()
            .expect("can't convert name to a utf8 string");

        // if current path is a hidden folder / file, skip it
        if current_name.starts_with('.') {
            cur_elem += 1;
            continue;
        }

        // add corner for current line
        add_corner_current_line(corner, cur_elem, num_elements);

        // put corner and current name
        string.push_str(corner);
        string.push_str(current_name);
        string.push('\n');

        // clean corner
        clean_corner(corner);

        if current_path.is_dir() {
            // add padding corner
            add_corner_other_line(corner, cur_elem, num_elements);

            // pass through new folder
            tree_folder(current_path_str, string, corner)?;

            // clean corner
            clean_corner(corner);
        }
        cur_elem += 1;
    }
    Ok(())
}

/// Pass through a path
pub fn tree(path: &str) -> Option<String> {
    let mut corner = String::new();
    let mut string = String::new();

    string.push_str(path);
    string.push('\n');

    tree_folder(path, &mut string, &mut corner).ok();

    Some(string)
}
