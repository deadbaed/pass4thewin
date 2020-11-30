use clipboard::{get_clipboard, set_clipboard};
use decrypt::decrypt;
use encrypt::encrypt_file;
use git2::Repository;
use notification::send_notification;
use qrcode::export_to_qrcode;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use sync::{add_commit_file, init_repo};
use tree::tree;

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
    assert_eq!(get_clipboard().unwrap(), current_path_str);

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

    let path_plaintext_file = "C:\\Users\\x4m3\\Desktop\\openpgp-testing\\file.txt";
    let path_privkey = "C:\\Users\\x4m3\\Desktop\\openpgp-testing\\private.gpg";

    let path_encrypted_file = encrypt_file(path_plaintext_file.as_ref(), path_privkey.as_ref())
        .expect("failed to encrypt file");

    match decrypt(path_encrypted_file.as_ref(), path_privkey.as_ref()) {
        Ok(e) => println!("ok {:?}", e),
        Err(e) => eprintln!("err {:?}", e),
    }
}
