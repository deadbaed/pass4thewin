mod utils;

use utils::clipboard::set_clipboard;
use utils::notification::send_notification;
use utils::tree::tree;

fn main() {
    let path = "C:\\users\\x4m3\\.password-store\\";
    if let Some(tree_str) = tree(path) {
        println!("{}", tree_str)
    }
    if set_clipboard(path) == true {
        // somehow notifications are not always sending
        send_notification("my-secure-password");
    }
}
