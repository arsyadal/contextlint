pub fn estimate_tokens(text: &str) -> usize {
    text.chars().count().div_ceil(4)
}
