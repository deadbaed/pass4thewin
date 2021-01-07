use notify_rust::Notification;
use crate::constants::ID_APPLICATION;

/// Send notification
///
/// Note: using `unwrap` here because there is no way to catch errors at the moment
/// -phil november 2nd, 2020
pub fn send_notification(password_name: &str) {
    let body = format!("copied password {} to clipboard", password_name);

    Notification::new()
        .summary(ID_APPLICATION)
        .body(&body)
        .show()
        .unwrap();
}
