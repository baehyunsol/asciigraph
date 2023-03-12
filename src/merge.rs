use crate::utils::{from_v16, into_lines};

/// merge 2 graphs with this function
pub fn merge_vert(str1: &String, str2: &String, margin: usize, alignment: Alignment) -> String {

    if str1.len() == 0 {
        return str2.to_string();
    }

    else if str2.len() == 0 {
        return str1.to_string();
    }

    let mut lines1 = into_lines(str1);
    let mut lines2 = into_lines(str2);

    let line_width = lines1[0].len().max(lines2[0].len());
    let mut line1_left_padding = 0;
    let mut line1_right_padding = 0;
    let mut line2_left_padding = 0;
    let mut line2_right_padding = 0;

    match alignment {
        Alignment::Center => {

            if lines1[0].len() < line_width {
                let diff = line_width - lines1[0].len();
                line1_left_padding = diff / 2;
                line1_right_padding = diff / 2 + diff % 2;
            }

            else {
                let diff = line_width - lines2[0].len();
                line2_left_padding = diff / 2;
                line2_right_padding = diff / 2 + diff % 2;
            }

        }
        Alignment::Left => {
            line1_right_padding = line_width - lines1[0].len();
            line2_right_padding = line_width - lines2[0].len();
        }
        Alignment::Right => {
            line1_left_padding = line_width - lines1[0].len();
            line2_left_padding = line_width - lines2[0].len();
        }
    }

    for line in lines1.iter_mut() {
        *line = vec![
            vec![' ' as u16; line1_left_padding],
            line.to_vec(),
            vec![' ' as u16; line1_right_padding]
        ].concat();
    }

    for line in lines2.iter_mut() {
        *line = vec![
            vec![' ' as u16; line2_left_padding],
            line.to_vec(),
            vec![' ' as u16; line2_right_padding]
        ].concat();
    }

    let v16 = vec![
        lines1,
        vec![vec![' ' as u16; line_width]; margin],
        lines2
    ].concat().join(&['\n' as u16][..]);

    from_v16(&v16)
}

/// merge 2 graphs with this function
pub fn merge_horiz(str1: &String, str2: &String, margin: usize) -> String {

    if str1.len() == 0 {
        return str2.to_string();
    }

    else if str2.len() == 0 {
        return str1.to_string();
    }

    let lines1 = into_lines(str1);
    let lines2 = into_lines(str2);

    if lines1.len() == lines2.len() {
        from_v16(&(0..lines1.len()).map(
            |i|
            vec![lines1[i].clone(), vec![' ' as u16; margin], lines2[i].clone()].concat()
        ).collect::<Vec<Vec<u16>>>().join(&['\n' as u16][..]))
    }

    else if lines1.len() < lines2.len() {
        let line1_width = lines1[0].len();
        let height_diff = lines2.len() - lines1.len();
        let result = vec![
            (0..height_diff).map(
                |i|
                vec![vec![' ' as u16; margin + line1_width], lines2[i].clone()].concat()
            ).collect::<Vec<Vec<u16>>>(),
            (height_diff..lines2.len()).map(
                |i|
                vec![lines1[i - height_diff].clone(), vec![' ' as u16; margin], lines2[i].clone()].concat()
            ).collect::<Vec<Vec<u16>>>()
        ].concat().join(&['\n' as u16][..]);

        from_v16(&result)
    }

    else {
        let line2_width = lines2[0].len();
        let height_diff = lines1.len() - lines2.len();
        let result = vec![
            (0..height_diff).map(
                |i|
                vec![lines1[i].clone(), vec![' ' as u16; margin + line2_width]].concat()
            ).collect::<Vec<Vec<u16>>>(),
            (height_diff..lines1.len()).map(
                |i|
                vec![lines1[i].clone(), vec![' ' as u16; margin], lines2[i - height_diff].clone()].concat()
            ).collect::<Vec<Vec<u16>>>()
        ].concat().join(&['\n' as u16][..]);

        from_v16(&result)
    }

}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Alignment {
    Left, Right, Center
}