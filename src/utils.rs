pub fn into_v16(s: &str) -> Vec<u16> {
    String::from(s).encode_utf16().filter(|c| *c != 13).collect()
}
