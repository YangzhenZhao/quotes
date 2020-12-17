pub fn exchange_prefix(code: &str) -> &str {
    if code.starts_with("6") {
        "sh"
    } else {
        "sz"
    }
}
