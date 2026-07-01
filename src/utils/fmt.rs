pub fn number(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::with_capacity(s.len() + (s.len() - 1) / 3);
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(c);
    }
    result
}

pub fn downloads(n: i64) -> String {
    let mut result = number(n);
    result.push_str(" dl");
    result
}

pub fn title(s: &str, max: usize) -> String {
    if s.len() <= max {
        format!("{s:<max$}")
    } else {
        let mut trimmed = String::with_capacity(max + 3);
        trimmed.push_str(&s[..max - 3]);
        trimmed.push_str("...");
        trimmed
    }
}

pub fn date(iso: &str) -> &str {
    iso.split('T').next().unwrap_or(iso)
}

pub fn loaders(slice: &[String]) -> String {
    if slice.is_empty() {
        return "\u{2014}".to_string();
    }
    slice.join(", ")
}

pub fn versions(slice: &[String], max: usize) -> String {
    if slice.is_empty() {
        return "\u{2014}".to_string();
    }
    let shown: Vec<&str> = slice.iter().take(max).map(|s| s.as_str()).collect();
    let mut result = shown.join(", ");
    if slice.len() > max {
        result.push_str(&format!(" (+{} more)", slice.len() - max));
    }
    result
}
