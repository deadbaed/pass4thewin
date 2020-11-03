use qr2term::print_qr;

pub fn export_to_qrcode(str: &str) -> bool {
    match print_qr(str) {
        Ok(_) => true,
        Err(_) => false,
    }
}
