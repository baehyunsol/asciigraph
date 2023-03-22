const DUMMY_KOREAN: u16 = 13;

#[inline]
pub fn into_v16(s: &str) -> Vec<u16> {
    let mut result = Vec::with_capacity(s.len() * 5 / 4);

    for c in String::from(s).encode_utf16().filter(|c| *c != 13) {
        result.push(c);

        if '가' as u16 <= c && c <= '힣' as u16 {
            result.push(DUMMY_KOREAN);
        }

    }

    result
}

#[inline]
pub fn from_v16(v: &[u16]) -> String {
    String::from_utf16(&v.iter().filter(|c| c != &&DUMMY_KOREAN).map(|c| *c).collect::<Vec<u16>>()).unwrap()
}

pub fn right_align(s: String, length: usize) -> String {

    if s.len() < length {
        vec![
            " ".repeat(length - s.len()),
            s
        ].concat()
    }

    else {
        s
    }

}

pub fn into_lines(string: &String) -> Vec<Vec<u16>> {
    into_v16(string).split(
        |c| *c == '\n' as u16
    ).map(
        |line| line.to_vec()
    ).collect()
}

pub fn from_lines(lines: &Vec<Vec<u16>>) -> String {
    from_v16(&lines.join(&['\n' as u16][..]))
}