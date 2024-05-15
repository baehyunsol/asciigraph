use crate::Alignment;
use crate::color::ColorMode;
use crate::lines::Lines;

/// It merges 2 graphs horizontally.
///
/// If `str1` and `str2` are both from `Graph::draw()`, make sure that they both use the same color_mode.
/// this argument tells which color_mode is used by the graphs. if you are just merging 2 random strings,
/// use `ColorMode::None`.
pub fn merge_horiz(
    str1: &str,
    str2: &str,
    color_mode: ColorMode,
    alignment: Alignment,
    margin: usize,
) -> String {
    let mut l1 = Lines::from_string(str1, Alignment::First, &color_mode);
    l1 = l1.add_padding([0, 0, 0, margin]);

    let l2 = Lines::from_string(str2, Alignment::First, &color_mode);

    l1.merge_horizontally(&l2, alignment).to_string(&ColorMode::None)  // the strings must already be colored; there's no need for an extra coloring
}

/// It merges 2 graphs vertically.
///
/// If `str1` and `str2` are both from `Graph::draw()`, make sure that they both use the same color_mode.
/// this argument tells which color_mode is used by the graphs. if you are just merging 2 random strings,
/// use `ColorMode::None`.
pub fn merge_vert(
    str1: &str,
    str2: &str,
    color_mode: ColorMode,
    alignment: Alignment,
    margin: usize,
) -> String {
    let mut l1 = Lines::from_string(str1, Alignment::First, &color_mode);
    l1 = l1.add_padding([0, margin, 0, 0]);

    let l2 = Lines::from_string(str2, Alignment::First, &color_mode);

    l1.merge_vertically(&l2, alignment).to_string(&ColorMode::None)  // the strings must already be colored; there's no need for an extra coloring
}
