

pub fn check_in_list(uri: &str, whitelist: Vec<String>) -> bool {
    for entry in whitelist {
        if uri.starts_with(entry.as_ref()) {
            return true
        }
    }
    return false
}