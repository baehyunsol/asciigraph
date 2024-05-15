// TODO: WIP

use crate::color::{Color, ColorMode};

pub struct Table {
    width: usize,    // number of columns
    height: usize,   // number of rows
    cells: Vec<Cell>,
    col_widths: Vec<usize>,
    row_heights: Vec<usize>,
    selected: Option<(usize, usize)>,  // (col, row)
    paddings: [usize; 4],  // [top, bottom, left, right]

    // TODO: draw borders independently
    draw_border: bool,

    color_mode: ColorMode,
    selection_color: Option<Color>,
    primary_color: Option<Color>,
}

impl Table {
    // it's a bit inefficient, but I cannot think of any better implementation...
    pub fn draw(&self) -> String {
        debug_assert_eq!(self.col_widths.len(), self.width);
        debug_assert_eq!(self.row_heights.len(), self.height);

        let [
            padding_top,
            padding_bottom,
            padding_left,
            padding_right,
        ] = self.paddings;

        let mut line_width = 0;
        let mut line_count = 0;

        for w in self.col_widths.iter() {
            line_width += *w + 1;
        }

        line_width += 2 + padding_left + padding_right;  // including '\n'

        for h in self.row_heights.iter() {
            line_count += *h + 1;
        }

        line_count += 1 + padding_top + padding_bottom;

        let mut foreground: Vec<Option<Color>> = vec![None; line_width * line_count];
        let mut background: Vec<Option<Color>> = vec![None; line_width * line_count];
        let mut buffer = vec![' ' as u32; line_width * line_count];

        for i in 0..line_count {
            buffer[i * line_width + line_width - 1] = '\n' as u32;
        }

        if self.draw_border {
            let mut curr_y = padding_top;

            for h in self.row_heights.iter() {
                for x in padding_left..(line_width - 2 - padding_right) {
                    buffer[curr_y * line_width + x] = '─' as u32;
                }

                buffer[curr_y * line_width + padding_left] = '├' as u32;
                buffer[curr_y * line_width + line_width - 2 - padding_right] = '┤' as u32;

                curr_y += *h + 1;
            }

            for x in (padding_left + 1)..(line_width - 2 - padding_right) {
                buffer[curr_y * line_width + x] = '─' as u32;
            }

            let mut curr_x = padding_left;

            for w in self.col_widths.iter() {
                for y in padding_top..(line_count - 1 - padding_bottom) {
                    if buffer[y * line_width + curr_x] == ' ' as u32 {
                        buffer[y * line_width + curr_x] = '│' as u32;
                    }

                    else if curr_x == padding_left {
                        buffer[y * line_width + curr_x] = '├' as u32;
                    }

                    else {
                        buffer[y * line_width + curr_x] = '┼' as u32;
                    }
                }

                buffer[padding_top * line_width + curr_x] = '┬' as u32;
                buffer[(line_count - 1 - padding_bottom) * line_width + curr_x] = '┴' as u32;

                curr_x += *w + 1;
            }

            for y in padding_top..(line_count - 1 - padding_bottom) {
                if buffer[y * line_width + curr_x] == ' ' as u32 {
                    buffer[y * line_width + curr_x] = '│' as u32;
                }
            }

            buffer[curr_y * line_width + padding_left] = '╰' as u32;
            buffer[curr_y * line_width + line_width -  2 - padding_right] = '╯' as u32;
            buffer[padding_top * line_width + padding_left] = '╭' as u32;
            buffer[padding_top * line_width + line_width - 2 - padding_right] = '╮' as u32;
        }

        for col in 0..self.width {
            for row in 0..self.height {
                let curr_cell = &self.cells[row * self.width + col];
                let (mut x, mut y, mut w, mut h) = self.get_rect(col, row);
                let [
                    padding_top,
                    padding_bottom,
                    padding_left,
                    padding_right,
                ] = curr_cell.paddings;

                if padding_top + padding_bottom < h {
                    y += padding_top;
                    h -= padding_top + padding_bottom;
                }

                else {
                    // the cell is too small to apply the paddings
                    todo!();
                }

                if padding_left + padding_right < w {
                    x += padding_left;
                    w -= padding_left + padding_right;
                }

                else {
                    // the cell is too small to apply the paddings
                    todo!();
                }

                let chars = curr_cell.content.chars().collect::<Vec<_>>();
                let mut ch_index = 0;
                let line_break_at = w * 3 / 4;

                for yy in y..(y + h) {
                    for xx in x..(x + w) {
                        if ch_index == chars.len() {
                            break;
                        }

                        if chars[ch_index] == '\n' || (xx - x) >= line_break_at && chars[ch_index] == ' ' {
                            ch_index += 1;
                            break;
                        }

                        buffer[yy * line_width + xx] = chars[ch_index] as u32;
                        ch_index += 1;
                    }
                }

                // TODO: line alignment? horizontally and vertically
                // how about postprocessing?
                // just leave the above code as it is, so that the default alignment rule is left&top
                // then the engine counts how many whitespaces there are
                // if the alignment is center, for example, the engine moves the line horizontally based on the number of whitespaces at left and right
            }
        }

        if let Some((col, row)) = self.selected {
            let [
                padding_top, _,
                padding_left, _,
            ] = self.paddings;

            if padding_top > 0 && padding_left > 0 {
                let (x, y, w, h) = self.get_rect(col, row);

                for xx in x..(x + w) {
                    buffer[(padding_top - 1) * line_width + xx] = '▼' as u32;
                    foreground[(padding_top - 1) * line_width + xx] = self.primary_color.clone();
                }

                for yy in y..(y + h) {
                    buffer[yy * line_width + padding_left - 1] = '▶' as u32;
                    foreground[yy * line_width + padding_left - 1] = self.primary_color.clone();
                }

                for yy in y..(y + h) {
                    for xx in x..(x + w) {
                        background[yy * line_width + xx] = self.selection_color.clone();
                    }
                }
            }
        }

        // TODO: apply colors
        buffer.into_iter().map(
            |c| char::from_u32(c).unwrap()
        ).collect()
    }

    // it does respect self.paddings,
    // but does not respect cell.paddings
    // TODO: make it configurable whether or not to respect the cell's paddings
    fn get_rect(&self, col: usize, row: usize) -> (usize, usize, usize, usize) {  // (x, y, w, h)
        let mut curr_x = 1;

        for w in self.col_widths[..col].iter() {
            curr_x += *w + 1;
        }

        let mut curr_y = 1;

        for h in self.row_heights[..row].iter() {
            curr_y += *h + 1;
        }

        (curr_x + self.paddings[2], curr_y + self.paddings[0], self.col_widths[col], self.row_heights[row])
    }
}

#[derive(Clone)]
pub struct Cell {
    content: String,
    paddings: [usize; 4],  // [top, bottom, left, right]
}
