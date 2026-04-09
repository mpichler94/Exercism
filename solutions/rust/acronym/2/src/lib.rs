use regex_lite::Regex;

pub fn abbreviate(phrase: &str) -> String {
    let pattern = Regex::new(r"(?:[\s\-_]+|^)([a-zA-Z])|(?:[a-z])([A-Z])").unwrap();
    pattern
        .captures_iter(phrase)
        .map(|c| {
            c.get(1)
                .unwrap_or_else(|| c.get(2).unwrap())
                .as_str()
                .trim()
                .to_uppercase()
        })
        .collect()
}
