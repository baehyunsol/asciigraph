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

pub fn sns_int(n: i64) -> String {

    if n < 0 {
        format!("-{}", sns_int(-n))
    }

    else {

        if n < 1_000 {
            n.to_string()
        }

        else if n < 10_000 {
            let k =  n / 1000;
            let sk = n / 10 % 100;

            format!("{}.{:02}K", k, sk)
        }

        else if n < 1_000_000 {
            format!("{}K", n / 1000)
        }

        else if n < 10_000_000 {
            let m =  n / 1_000_000;
            let sm = n / 10_000 % 100;

            format!("{}.{:02}M", m, sm)
        }

        else if n < 1_000_000_000 {
            format!("{}M", n / 1_000_000)
        }

        else if n < 10_000_000_000 {
            let b =  n / 1_000_000_000;
            let sb = n / 10_000_000 % 100;

            format!("{}.{:02}B", b, sb)
        }

        else if n < 1_000_000_000_000 {
            format!("{}B", n / 1_000_000_000)
        }

        else if n < 10_000_000_000_000 {
            let t =  n / 1_000_000_000_000;
            let st = n / 10_000_000_000 % 100;

            format!("{}.{:02}T", t, st)
        }

        else if n < 1_000_000_000_000_000 {
            format!("{}T", n / 1_000_000_000_000)
        }

        else if n < 10_000_000_000_000_000 {
            let q =  n / 1_000_000_000_000_000;
            let sq = n / 10_000_000_000_000 % 100;
            format!("{}.{:02}Q", q, sq)
        }

        else {
            format!("{}Q", n / 1_000_000_000_000_000)
        }

    }

}

// 9.999
// 99.99
// 999
pub fn fractional_number(n: i64) -> String {

    if n < 0 {
        format!("-{}", fractional_number(-n))
    }

    else {

        if n >= 409600 {
            sns_int(n / 4096)
        }

        else {
            let integer = n / 4096;
            let mut fraction = n % 4096 * 1000 / 4096;  // 0 ~ 999

            if integer >= 10 {
                fraction /= 10;  // 0 ~ 99

                format!("{}.{:02}", integer, fraction)
            }

            else {
                format!("{}.{:03}", integer, fraction)
            }

        }

    }

}