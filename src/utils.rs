use std::iter::FromIterator;

pub fn snakeify(s: &str) -> String {
    let mut buffer: Vec<char> = Vec::new();
    s.chars().for_each(|c| {
        if c.is_uppercase() {
            if !buffer.is_empty() {
                buffer.push('-');
            }
            buffer.push(c.to_ascii_lowercase());
        } else {
            buffer.push(c);
        }
    });

    String::from_iter(buffer)
}
