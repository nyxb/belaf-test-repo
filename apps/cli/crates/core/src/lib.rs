//! CLI core library

/// Truncates a string to a maximum length, adding ellipsis if needed
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        s.chars().take(max_len).collect()
    } else {
        format!("{}...", s.chars().take(max_len - 3).collect::<String>())
    }
}

pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Pads a string to a minimum length with a given character
pub fn pad_start(s: &str, min_len: usize, pad_char: char) -> String {
    if s.len() >= min_len {
        s.to_string()
    } else {
        let padding: String = std::iter::repeat(pad_char).take(min_len - s.len()).collect();
        format!("{}{}", padding, s)
    }
}

/// Pads a string to a minimum length with a given character (end)
pub fn pad_end(s: &str, min_len: usize, pad_char: char) -> String {
    if s.len() >= min_len {
        s.to_string()
    } else {
        let padding: String = std::iter::repeat(pad_char).take(min_len - s.len()).collect();
        format!("{}{}", s, padding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("Hello World", 5), "He...");
        assert_eq!(truncate("Hi", 5), "Hi");
    }

    #[test]
    fn test_pad_start() {
        assert_eq!(pad_start("42", 5, '0'), "00042");
        assert_eq!(pad_start("hello", 3, ' '), "hello");
    }

    #[test]
    fn test_pad_end() {
        assert_eq!(pad_end("hi", 5, '.'), "hi...");
    }
}
