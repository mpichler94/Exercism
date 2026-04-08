pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 || digits.len() == 0 || len > digits.len() {
        return Vec::new();
    }
    if len == digits.len() {
        return vec![digits.to_string()];
    }

    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|chars| chars.iter().collect())
        .collect()
}
