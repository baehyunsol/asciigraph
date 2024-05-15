use crate::alignment::Alignment;
use crate::color::{Color, ColorMode};
use crate::utils::into_v16;

#[derive(Clone, Debug)]
pub struct Lines {
    lines: Vec<Vec<u16>>,
    colors: Vec<Vec<Option<Color>>>,
    width: usize,
    height: usize,
}

impl Lines {
    pub fn new(width: usize, height: usize) -> Self {
        Lines {
            lines: vec![vec![' ' as u16; width]; height],
            colors: vec![vec![None; width]; height],
            width, height,
        }
    }

    pub fn empty() -> Self {
        Lines {
            lines: vec![],
            colors: vec![],
            width: 0,
            height: 0,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> u16 {
        self.lines[y][x]
    }

    pub fn get_color(&self, x: usize, y: usize) -> Option<Color> {
        self.colors[y][x].clone()
    }

    pub fn set(&mut self, x: usize, y: usize, c: u16) {
        self.lines[y][x] = c;
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: Option<Color>) {
        self.colors[y][x] = color;
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn crop(&self, x: usize, y: usize, w: usize, h: usize) -> Lines {
        let w = w.min(self.width - x);
        let h = h.min(self.height - y);

        let new_lines: Vec<Vec<u16>> = (y..(y + h)).map(|line_no| self.lines[line_no][x..(x + w)].to_vec()).collect();
        let new_colors: Vec<Vec<Option<Color>>> = (y..(y + h)).map(|line_no| self.colors[line_no][x..(x + w)].to_vec()).collect();

        Lines {
            lines: new_lines,
            colors: new_colors,
            width: w,
            height: h,
        }
    }

    // if you want to blit a fraction of `other`, crop it first
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn blit(&self, other: &Lines, x: usize, y: usize, transparent_char: Option<char>) -> Lines {
        let mut result = self.clone();

        if x >= self.width || y >= self.height {
            return result;
        }

        let transparent_char = transparent_char.map(|c| c as u16);

        for x_ in x..self.width.min(x + other.width) {
            for y_ in y..self.height.min(y + other.height) {
                let ch = other.get(x_ - x, y_ - y);
                let color = other.get_color(x_ - x, y_ - y);

                if Some(ch) == transparent_char {
                    continue;
                }

                result.set(x_, y_, ch);
                result.set_color(x_, y_, color);
            }
        }

        result
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn merge_vertically(&self, other: &Lines, alignment: Alignment) -> Lines {
        let (padding1, padding2) = if self.width >= other.width {
            (0, 0)
        } else {
            match alignment {
                Alignment::Center => (
                    (other.width - self.width) / 2 + (other.width - self.width) % 2,
                    (other.width - self.width) / 2,
                ),
                Alignment::First => (
                    0,
                    other.width - self.width,
                ),
                Alignment::Last => (
                    other.width - self.width,
                    0,
                ),
                _ => todo!(),
            }
        };

        let (padding3, padding4) = if other.width >= self.width {
            (0, 0)
        } else {
            match alignment {
                Alignment::Center => (
                    (self.width - other.width) / 2 + (self.width - other.width) % 2,
                    (self.width - other.width) / 2,
                ),
                Alignment::First => (
                    0,
                    self.width - other.width,
                ),
                Alignment::Last => (
                    self.width - other.width,
                    0
                ),
                _ => todo!(),
            }
        };

        let new_lines = vec![
            self.lines.iter().map(
                |line| vec![
                    vec![' ' as u16; padding1],
                    line.to_vec(),
                    vec![' ' as u16; padding2],
                ].concat()
            ).collect::<Vec<Vec<u16>>>(),
            other.lines.iter().map(
                |line| vec![
                    vec![' ' as u16; padding3],
                    line.to_vec(),
                    vec![' ' as u16; padding4],
                ].concat()
            ).collect(),
        ].concat();

        let new_colors = vec![
            self.colors.iter().map(
                |color| vec![
                    vec![None; padding1],
                    color.to_vec(),
                    vec![None; padding2],
                ].concat()
            ).collect::<Vec<Vec<Option<Color>>>>(),
            other.colors.iter().map(
                |color| vec![
                    vec![None; padding3],
                    color.to_vec(),
                    vec![None; padding4],
                ].concat()
            ).collect(),
        ].concat();

        Lines {
            lines: new_lines,
            colors: new_colors,
            width: self.width.max(other.width),
            height: self.height + other.height,
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn merge_horizontally(&self, other: &Lines, alignment: Alignment) -> Lines {
        if self.height < other.height {
            let (padding1, padding2) = match alignment {
                Alignment::First => (0, other.height - self.height),
                Alignment::Last => (other.height - self.height, 0),
                Alignment::Center => (
                    (other.height - self.height) / 2 + (other.height - self.height) % 2,
                    (other.height - self.height) / 2,
                ),
                _ => todo!(),
            };
            let mut new_lines = Vec::with_capacity(other.height);
            let mut new_colors = Vec::with_capacity(other.height);
            let mut index = 0;

            for _ in 0..padding1 {
                new_lines.push(vec![
                    vec![' ' as u16; self.width],
                    other.lines[index].clone(),
                ].concat());
                new_colors.push(vec![
                    vec![None; self.width],
                    other.colors[index].clone(),
                ].concat());

                index += 1;
            }

            for _ in 0..self.height {
                new_lines.push(vec![
                    self.lines[index - padding1].clone(),
                    other.lines[index].clone(),
                ].concat());
                new_colors.push(vec![
                    self.colors[index - padding1].clone(),
                    other.colors[index].clone(),
                ].concat());

                index += 1;
            }

            for _ in 0..padding2 {
                new_lines.push(vec![
                    vec![' ' as u16; self.width],
                    other.lines[index].clone(),
                ].concat());
                new_colors.push(vec![
                    vec![None; self.width],
                    other.colors[index].clone(),
                ].concat());

                index += 1;
            }

            Lines {
                lines: new_lines,
                colors: new_colors,
                width: self.width + other.width,
                height: other.height,
            }
        }

        else if self.height == other.height {
            let new_lines = (0..self.height).map(
                |i| vec![
                    self.lines[i].clone(),
                    other.lines[i].clone(),
                ].concat()
            ).collect();
            let new_colors = (0..self.height).map(
                |i| vec![
                    self.colors[i].clone(),
                    other.colors[i].clone(),
                ].concat()
            ).collect();

            Lines {
                lines: new_lines,
                colors: new_colors,
                width: self.width + other.width,
                height: self.height,
            }
        }

        else {
            let (padding1, padding2) = match alignment {
                Alignment::First => (0, self.height - other.height),
                Alignment::Last => (self.height - other.height, 0),
                Alignment::Center => (
                    (self.height - other.height) / 2 + (self.height - other.height) % 2,
                    (self.height - other.height) / 2,
                ),
                _ => todo!(),
            };
            let mut new_lines = Vec::with_capacity(self.height);
            let mut new_colors = Vec::with_capacity(self.height);
            let mut index = 0;

            for _ in 0..padding1 {
                new_lines.push(vec![
                    self.lines[index].clone(),
                    vec![' ' as u16; other.width],
                ].concat());
                new_colors.push(vec![
                    self.colors[index].clone(),
                    vec![None; other.width],
                ].concat());

                index += 1;
            }

            for _ in 0..other.height {
                new_lines.push(vec![
                    self.lines[index].clone(),
                    other.lines[index - padding1].clone(),
                ].concat());
                new_colors.push(vec![
                    self.colors[index].clone(),
                    other.colors[index - padding1].clone(),
                ].concat());

                index += 1;
            }

            for _ in 0..padding2 {
                new_lines.push(vec![
                    self.lines[index].clone(),
                    vec![' ' as u16; other.width],
                ].concat());
                new_colors.push(vec![
                    self.colors[index].clone(),
                    vec![None; other.width],
                ].concat());

                index += 1;
            }

            Lines {
                lines: new_lines,
                colors: new_colors,
                width: self.width + other.width,
                height: self.height,
            }
        }
    }

    /// top, bottom, left, right
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add_padding(&self, paddings: [usize; 4]) -> Lines {
        let new_width = self.width + paddings[2] + paddings[3];

        let new_lines = vec![
            vec![vec![' ' as u16; new_width]; paddings[0]],
            self.lines.iter().map(
                |line|
                vec![
                    vec![' ' as u16; paddings[2]],
                    line.to_vec(),
                    vec![' ' as u16; paddings[3]],
                ].concat()
            ).collect::<Vec<Vec<u16>>>(),
            vec![vec![' ' as u16; new_width]; paddings[1]],
        ].concat();

        let new_colors = vec![
            vec![vec![None; new_width]; paddings[0]],
            self.colors.iter().map(
                |color|
                vec![
                    vec![None; paddings[2]],
                    color.to_vec(),
                    vec![None; paddings[3]],
                ].concat()
            ).collect::<Vec<Vec<Option<Color>>>>(),
            vec![vec![None; new_width]; paddings[1]],
        ].concat();

        Lines {
            lines: new_lines,
            colors: new_colors,
            width: new_width,
            height: self.height + paddings[0] + paddings[1],
        }
    }

    /// top, bottom, left, right
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add_border(&self, borders: [bool; 4]) -> Lines {
        let mut with_padding = self.add_padding([
            borders[0] as usize,
            borders[1] as usize,
            borders[2] as usize,
            borders[3] as usize,
        ]);

        if borders[0] {
            for x in 0..with_padding.width {
                with_padding.set(x, 0, '─' as u16);
            }
        }

        if borders[1] {
            for x in 0..with_padding.width {
                with_padding.set(x, with_padding.height - 1, '─' as u16);
            }
        }

        if borders[2] {
            for y in 0..with_padding.height {
                with_padding.set(0, y, '│' as u16);
            }
        }

        if borders[3] {
            for y in 0..with_padding.height {
                with_padding.set(with_padding.width - 1, y, '│' as u16);
            }
        }

        if borders[0] && borders[2] {
            with_padding.set(0, 0, '╭' as u16);
        }

        if borders[0] && borders[3] {
            with_padding.set(with_padding.width - 1, 0, '╮' as u16);
        }

        if borders[1] && borders[2] {
            with_padding.set(0, with_padding.height - 1, '╰' as u16);
        }

        if borders[1] && borders[3] {
            with_padding.set(with_padding.width - 1, with_padding.height - 1, '╯' as u16);
        }

        with_padding
    }

    /// If `s` is generated by this library with a specific ColorMode, set `color_mode` value to that ColorMode.
    /// If `s` is from elsewhere, just set it to `ColorMode::None`.
    pub fn from_string(
        s: &str,
        alignment: Alignment,
        color_mode: &ColorMode,
    ) -> Self {

        // it seems like s.split() when s is empty returns a non-empty vector
        if s.len() == 0 {
            return Lines::empty();
        }

        let mut max_width = 0;
        let raw_lines: Vec<Vec<u16>> = s.split("\n").map(
            |raw_line| {
                let result = into_v16(&raw_line);
                let line_len = count_chars(&result, color_mode);

                if line_len > max_width {
                    max_width = line_len;
                }

                result
            }
        ).collect();
        let mut result = Vec::with_capacity(raw_lines.len());

        for raw_line in raw_lines.into_iter() {
            let line_len = count_chars(&raw_line, color_mode);
            let (padding1, padding2) = match alignment {
                Alignment::Center => (
                    (max_width - line_len) / 2 + (max_width - line_len) % 2,
                    (max_width - line_len) / 2,
                ),
                Alignment::First => (
                    0,
                    max_width - line_len,
                ),
                Alignment::Last => (
                    max_width - line_len,
                    0,
                ),
                _ => todo!(),
            };

            result.push(
                vec![
                    vec![' ' as u16; padding1],
                    raw_line,
                    vec![' ' as u16; padding2],
                ].concat()
            );
        }

        Lines {
            width: if result.len() > 0 { count_chars(&result[0], color_mode) } else { 0 },
            height: result.len(),
            colors: result.iter().map(
                |line| vec![None; count_chars(line, color_mode)]
            ).collect(),
            lines: result,
        }
    }

    pub fn to_string(&self, color_mode: &ColorMode) -> String {
        let string = self.lines.iter().map(|line| String::from_utf16_lossy(line)).collect::<Vec<String>>().join("\n");

        if let ColorMode::None = color_mode {
            string
        }

        else {
            color_mode.apply_colors(
                string,
                self.colors.join(&[None][..]),  // none color variants for '\n'
            )
        }
    }

    pub fn set_color_all(&mut self, color: Option<Color>) {
        for cc in self.colors.iter_mut() {
            for c in cc.iter_mut() {
                *c = color.clone();
            }
        }
    }

    #[cfg(test)]
    pub fn is_valid(&self) -> bool {
        self.lines.len() == self.height && self.lines.iter().all(|line| line.len() == self.width)
    }
}

impl std::fmt::Display for Lines {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.to_string(&ColorMode::None))
    }
}

enum AnsiEscapeParseState {
    None,
    Esc0,  // encountered `\x1b', expecting '['
    Esc1,  // encountered '[', expecting '3'
    Esc2,  // encountered '3', expecting 'm'
}

fn count_chars(line: &[u16], color_mode: &ColorMode) -> usize {
    match color_mode {
        // `count_chars` for ColorMode::Html is not implemented yet!!
        // that means you cannot merge 2 graphs whose color mode is html
        ColorMode::None
        | ColorMode::Html { .. } => line.len(),

        ColorMode::Terminal => {
            let mut curr_state = AnsiEscapeParseState::None;
            let mut char_count = 0;

            for c in line.iter() {
                match curr_state {
                    AnsiEscapeParseState::None => {
                        if *c == 27 {
                            curr_state = AnsiEscapeParseState::Esc0;
                        }

                        else {
                            char_count += 1;
                        }
                    },
                    AnsiEscapeParseState::Esc0 => {
                        if *c == '[' as u16 {
                            curr_state = AnsiEscapeParseState::Esc1;
                        }

                        else if *c == 27 {
                            char_count += 1;
                        }

                        else {
                            curr_state = AnsiEscapeParseState::None;
                            char_count += 2;
                        }
                    },
                    AnsiEscapeParseState::Esc1 => {
                        if *c == '3' as u16 {
                            curr_state = AnsiEscapeParseState::Esc2;
                        }

                        else if *c == 27 {
                            curr_state = AnsiEscapeParseState::Esc0;
                            char_count += 2;
                        }

                        else {
                            curr_state = AnsiEscapeParseState::None;
                            char_count += 3;
                        }
                    },
                    AnsiEscapeParseState::Esc2 => {
                        if *c == 'm' as u16 {
                            curr_state = AnsiEscapeParseState::None;
                        }
                    },
                }
            }

            char_count
        },
    }
}
