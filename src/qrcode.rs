use qr2term::print_qr;

/// Export string to qrcode, written directly to [stdout]
///
/// [stdout]: std::io::stdout
pub fn export_to_qrcode(str: &str) -> bool {
    print_qr(str).is_ok()
}
