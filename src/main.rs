mod utils;

use utils::clipboard::set_clipboard;
use utils::notification::send_notification;
use utils::qrcode::export_to_qrcode;
use utils::sync::{create_repo, open_repo};
use utils::tree::tree;

use crate::utils::sync::{add_file, create_commit, create_initial_commit};
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
    let repo = match open_repo(&repo_test_path) {
        Some(repo) => {
            println!("repo opened");
            repo
        }
        None => match create_repo(&repo_test_path) {
            Some(repo) => {
                println!("repo created");
                match create_initial_commit(&repo) {
                    Ok(_) => println!("first commit created"),
                    Err(e) => eprintln!("failed to create first commit {}", e),
                }
                repo
            }
            None => panic!("could not create repo"),
        },
    };

    let relative_path = Path::new("example3.txt");
    {
        let file_path = repo_test_path.join(relative_path);
        let mut file = File::create(file_path).expect("Couldn't create file");
        file.write_all(b"Hello git2").unwrap();
    }

    let tree_file_added = add_file(&repo, &relative_path).unwrap();
    println!("added file: {:?}", tree_file_added);

    let commit_id = create_commit(&repo, &tree_file_added, "added file").unwrap();
    println!("New commit: {}", commit_id);
}
