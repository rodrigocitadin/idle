pub fn length_of_last_word(s: String) -> i32 {
    s.trim_end()
        .split_whitespace()
        .last()
        .map(|word| word.len() as i32)
        .unwrap_or(0)
}
