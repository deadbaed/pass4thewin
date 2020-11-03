mod utils;

use utils::clipboard::set_clipboard;
use utils::notification::send_notification;
use utils::qrcode::export_to_qrcode;
use utils::sync::{create_repo, open_repo};
use utils::tree::tree;

use std::env;
use std::path::PathBuf;

fn main() {
    let current_path = env::current_dir().unwrap();
    let current_path_str = current_path.to_str().unwrap();

    if let Some(tree_str) = tree(&current_path) {
        println!("{}", tree_str)
    }
    if set_clipboard(current_path_str) == true {
        // somehow notifications are not always sending
        send_notification("my-secure-password");
        export_to_qrcode(current_path_str);
    }

    // Try to open git repository, create one on failure
    let repo_test_path = current_path_str.to_owned() + "\\test-repo";
    let repo_test_path = PathBuf::from(repo_test_path);
    let repo = match open_repo(&repo_test_path) {
        Some(repo) => {
            println!("repo opened");
            repo
        }
        None => match create_repo(&repo_test_path) {
            Some(repo) => {
                println!("repo created");
                repo
            }
            None => panic!("could not create repo"),
        },
    };
}
