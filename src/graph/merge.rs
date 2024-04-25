use crate::Alignment;
use crate::lines::Lines;

/// merge 2 graphs with this function
pub fn merge_horiz(str1: &str, str2: &str, alignment: Alignment, margin: usize) -> String {
    let mut l1 = Lines::from_string(str1, Alignment::First);
    l1 = l1.add_padding([0, 0, 0, margin]);

    let l2 = Lines::from_string(str2, Alignment::First);

    l1.merge_horizontally(&l2, alignment).to_string()
}

/// merge 2 graphs with this function
pub fn merge_vert(str1: &str, str2: &str, alignment: Alignment, margin: usize) -> String {
    let mut l1 = Lines::from_string(str1, Alignment::First);
    l1 = l1.add_padding([0, margin, 0, 0]);

    let l2 = Lines::from_string(str2, Alignment::First);

    l1.merge_vertically(&l2, alignment).to_string()
}
