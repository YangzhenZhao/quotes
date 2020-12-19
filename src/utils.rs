pub fn exchange_prefix(code: &str) -> &str {
    if code.starts_with('6') {
        "sh"
    } else {
        "sz"
    }
}

pub fn format_stock_code(codes: Vec<&str>) -> Vec<String> {
    if !codes.is_empty() && codes[0].starts_with('s') {
        return codes.iter().map(|x| x.to_string()).collect();
    }
    let mut res = Vec::with_capacity(codes.len());
    for code in codes.iter() {
        res.push(format!("{}{}", exchange_prefix(&code), code));
    }
    res
}
