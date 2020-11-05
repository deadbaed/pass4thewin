mod utils;

use utils::clipboard::set_clipboard;
use utils::notification::send_notification;
use utils::qrcode::export_to_qrcode;
use utils::sync::{add_commit_file, init_repo};
use utils::tree::tree;

use git2::Repository;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let current_path = env::current_dir().unwrap();
    let current_path_str = current_path.to_str().unwrap();

    if let Some(tree_str) = tree(&current_path) {
        println!("{}", tree_str)
    }
    if set_clipboard(current_path_str) {
        // somehow notifications are not always sending
        send_notification("my-secure-password");
        export_to_qrcode(current_path_str);
    }

    // Try to open git repository, create one on failure
    // let repo_test_path = current_path_str.to_owned() + "\\test-repo";
    let repo_test_path = "C:\\Users\\x4m3\\Desktop\\test-repo";
    let repo_test_path = PathBuf::from(repo_test_path);
    let repo = match Repository::open(&repo_test_path) {
        Ok(repo) => {
            println!("repo opened");
            repo
        }
        Err(_) => match init_repo(&repo_test_path) {
            Ok(repo) => {
                println!("repo created");
                repo
            }
            Err(e) => panic!("{}", e),
        },
    };

    let relative_path = Path::new("example3.txt");
    let file_path = repo_test_path.join(relative_path);
    {
        let mut file = File::create(&file_path).expect("Couldn't create file");
        file.write_all(b"Hello git2").unwrap();
    }

    add_commit_file(&repo, &file_path);
}
