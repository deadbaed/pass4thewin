use clipboard_win::{get_clipboard_string, set_clipboard_string};

pub fn get_clipboard() -> Option<String> {
    match get_clipboard_string() {
        Ok(str) => Some(str),
        Err(_) => None,
    }
}

pub fn set_clipboard(text: &str) -> bool {
    set_clipboard_string(text).is_ok()
}

#[cfg(test)]
mod tests {
    use crate::clipboard::{get_clipboard, set_clipboard};

    #[test]
    fn test_clipboard() {
        let to_clipboard = "this is a test string";

        assert_eq!(set_clipboard(to_clipboard), true);

        let from_clipboard = get_clipboard().unwrap();
        assert_eq!(to_clipboard, from_clipboard);
    }
}
