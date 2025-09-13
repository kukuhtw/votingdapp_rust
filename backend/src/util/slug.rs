/*
backend/src/util/slug.rs
*/


pub fn slugify(title: &str) -> String {
    let s = title
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() { c }
            else if c.is_whitespace() || "-_".contains(c) { '-' }
            else { '-' }
        })
        .collect::<String>();
    let s = s.trim_matches('-').to_string();
    // collapse multiple dashes
    let mut out = String::with_capacity(s.len());
    let mut last_dash = false;
    for ch in s.chars() {
        if ch == '-' {
            if !last_dash { out.push('-'); }
            last_dash = true;
        } else {
            out.push(ch);
            last_dash = false;
        }
    }
    if out.is_empty() { "poll".into() } else { out }
}
