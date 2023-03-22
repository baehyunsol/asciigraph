use crate::merge::Alignment;
use crate::utils::{into_lines, from_lines};

/// It doesn't break lines automatically; you have to break them using '\n' characters.
/// If there's a line who's longer than `width`, it wouldn't work. You have to take care of that.
pub fn format_lines(string: &String, width: usize, alignment: Alignment) -> String {
    let mut lines = into_lines(string);

    for line in lines.iter_mut() {

        if width < line.len() {
            continue;
        }

        let margin = width - line.len();

        let (left_padding, right_padding) = match alignment {
            Alignment::Center => (
                margin / 2,
                margin / 2 + margin % 2
            ),
            Alignment::Left => (
                0,
                margin
            ),
            Alignment::Right => (
                margin,
                0
            )
        };

        *line = vec![
            vec![' ' as u16; left_padding],
            line.clone(),
            vec![' ' as u16; right_padding]
        ].concat();
    }

    from_lines(&lines)
}

pub fn sns_int(n: i64) -> String {

    if n < 0 {
        format!("-{}", sns_int(-n))
    }

    else {

        if n < 100_000 {
            n.to_string()
        }

        else if n < 1_000_000 {
            format!("{}K", n / 1000)
        }

        else if n < 10_000_000 {
            let m =  n / 1_000_000;
            let sm = n / 10_000 % 100;

            format!("{}.{:02}M", m, sm)
        }

        else if n < 100_000_000 {
            let m =  n / 1_000_000;
            let sm = n / 100_000 % 10;

            format!("{}.{}M", m, sm)
        }

        else if n < 1_000_000_000 {
            format!("{}M", n / 1_000_000)
        }

        else if n < 10_000_000_000 {
            let b =  n / 1_000_000_000;
            let sb = n / 10_000_000 % 100;

            format!("{}.{:02}B", b, sb)
        }

        else if n < 100_000_000_000 {
            let b =  n / 1_000_000_000;
            let sb = n / 100_000_000 % 10;

            format!("{}.{}B", b, sb)
        }

        else if n < 1_000_000_000_000 {
            format!("{}B", n / 1_000_000_000)
        }

        else if n < 10_000_000_000_000 {
            let t =  n / 1_000_000_000_000;
            let st = n / 10_000_000_000 % 100;

            format!("{}.{:02}T", t, st)
        }

        else if n < 100_000_000_000_000 {
            let t =  n / 1_000_000_000_000;
            let st = n / 100_000_000_000 % 10;

            format!("{}.{}T", t, st)
        }

        else if n < 1_000_000_000_000_000 {
            format!("{}T", n / 1_000_000_000_000)
        }

        else if n < 10_000_000_000_000_000 {
            let q =  n / 1_000_000_000_000_000;
            let sq = n / 10_000_000_000_000 % 100;
            format!("{}.{:02}Q", q, sq)
        }

        else if n < 100_000_000_000_000_000 {
            let q =  n / 1_000_000_000_000_000;
            let sq = n / 100_000_000_000_000 % 10;
            format!("{}.{}Q", q, sq)
        }

        else {
            format!("{}Q", n / 1_000_000_000_000_000)
        }

    }

}

// .9999
// 9.999
// 99.99
// 999.9
// 9999
pub fn fractional_number(n: i64) -> String {

    if n < 0 {
        format!("-{}", fractional_number(-n))
    }

    else {

        if n >= 16384_000 {
            sns_int(n / 16384)
        }

        else if n == 0 {
            "0".to_string()
        }

        else {
            let integer = n / 16384;
            let mut fraction = n % 16384 * 1000 / 16384;  // 0 ~ 999

            if integer >= 100 {
                fraction /= 100;  // 0 ~ 9

                format!("{}.{}", integer, fraction)
            }

            else if integer >= 10 {
                fraction /= 10;  // 0 ~ 99

                format!("{}.{:02}", integer, fraction)
            }

            else {
                format!("{}.{:03}", integer, fraction)
            }

        }

    }

}

pub fn calc_sns_int_max_len(y_min: i64, y_max: i64) -> usize {
    #[cfg(test)] assert!(y_min <= y_max);

    // y_min < 0, y_max < 0
    if y_max < 0 {
        1 + calc_sns_int_max_len(-y_max, -y_min)
    }

    // y_min < 0, y_max >= 0
    else if y_min < 0 {
        (1 + calc_sns_int_max_len(0, -y_min)).max(calc_sns_int_max_len(0, y_max))
    }

    // y_min >= 0, y_max >= 0
    else {

        if y_max < 10 {
            1
        }

        else if y_max < 100 {
            2
        }

        else if y_max < 1000 {
            3
        }

        else if y_max < 10000 {
            4
        }

        else {
            5
        }

    }

}

pub fn calc_fractional_number_max_len(y_min: i64, y_max: i64) -> usize {
    #[cfg(test)] assert!(y_min <= y_max);

    // y_min < 0, y_max < 0
    if y_max < 0 {
        1 + calc_fractional_number_max_len(-y_max, -y_min)
    }

    // y_min < 0, y_max >= 0
    else if y_min < 0 {
        (1 + calc_fractional_number_max_len(0, -y_min)).max(calc_fractional_number_max_len(0, y_max))
    }

    // y_min >= 0, y_max >= 0
    else {

        if y_max == 0 {
            1
        }

        else if y_max < 16384_000 {
            5
        }

        else if y_min >= 16384_000 {
            calc_sns_int_max_len(y_min / 16384, y_max / 16384)
        }

        else {
            5
        }

    }

}