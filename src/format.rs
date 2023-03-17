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