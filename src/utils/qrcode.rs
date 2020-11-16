use qr2term::print_qr;

pub fn export_to_qrcode(str: &str) -> bool {
    print_qr(str).is_ok()
}
