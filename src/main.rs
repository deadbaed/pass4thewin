mod utils;

use utils::clipboard::set_clipboard;
use utils::notification::send_notification;
use utils::qrcode::export_to_qrcode;
use utils::tree::tree;

use std::env;

fn main() {
    let current_path = env::current_dir().unwrap();
    let current_path_str = current_path.to_str().unwrap();

    let path = "C:\\users\\x4m3\\.password-store\\";
    if let Some(tree_str) = tree(&current_path) {
        println!("{}", tree_str)
    }
    if set_clipboard(current_path_str) == true {
        // somehow notifications are not always sending
        send_notification("my-secure-password");
        export_to_qrcode(current_path_str);
    }
}
