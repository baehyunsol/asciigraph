use crate::alignment::Alignment;

#[derive(Clone, Debug)]
pub struct Lines {
    lines: Vec<Vec<u16>>,
    width: usize,
    height: usize
}

impl Lines {

    pub fn new(width: usize, height: usize) -> Self {
        Lines {
            lines: vec![vec![32; width]; height],
            width, height
        }
    }

    pub fn empty() -> Self {
        Lines {
            lines: vec![], width: 0, height: 0
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

    pub fn set(&mut self, x: usize, y: usize, c: u16) {
        self.lines[y][x] = c;
    }

    pub fn crop(&self, x: usize, y: usize, w: usize, h: usize) -> Lines {
        let w = w.min(self.width - x);
        let h = h.min(self.height - y);

        let new_lines: Vec<Vec<u16>> = (y..(y + h)).map(|line_no| self.lines[line_no][x..(x + w)].to_vec()).collect();

        Lines {
            lines: new_lines,
            width: w,
            height: h
        }
    }

    // if you want to blit a fraction of `other`, crop it first
    pub fn blit(&self, other: &Lines, x: usize, y: usize, transparent_char: Option<char>) -> Lines {
        let mut result = self.clone();

        if x >= self.width || y >= self.height {
            return result;
        }

        let transparent_char = transparent_char.map(|c| c as u16);

        for x_ in x..self.width.min(x + other.width) {

            for y_ in y..self.height.min(y + other.height) {
                let c = other.get(x_ - x, y_ - y);

                if Some(c) == transparent_char {
                    continue;
                }

                result.set(x_, y_, c);
            }

        }


        result
    }

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
                    other.width - self.width
                ),
                Alignment::Last => (
                    other.width - self.width,
                    0
                ),
                _ => todo!()
            }
        };

        let (padding3, padding4) = if other.width >= self.width {
            (0, 0)
        } else {
            match alignment {
                Alignment::Center => (
                    (self.width - other.width) / 2 + (self.width - other.width) % 2,
                    (self.width - other.width) / 2
                ),
                Alignment::First => (
                    0,
                    self.width - other.width
                ),
                Alignment::Last => (
                    self.width - other.width,
                    0
                ),
                _ => todo!()
            }
        };

        let new_lines = vec![
            self.lines.iter().map(
                |line| vec![
                    vec![32; padding1],
                    line.to_vec(),
                    vec![32; padding2]
                ].concat()
            ).collect::<Vec<Vec<u16>>>(),
            other.lines.iter().map(
                |line| vec![
                    vec![32; padding3],
                    line.to_vec(),
                    vec![32; padding4]
                ].concat()
            ).collect(),
        ].concat();

        Lines {
            lines: new_lines,
            width: self.width.max(other.width),
            height: self.height + other.height
        }
    }

    pub fn merge_horizontally(&self, other: &Lines, alignment: Alignment) -> Lines {

        if self.height < other.height {
            let (padding1, padding2) = match alignment {
                Alignment::First => (0, other.height - self.height),
                Alignment::Last => (other.height - self.height, 0),
                Alignment::Center => (
                    (other.height - self.height) / 2 + (other.height - self.height) % 2,
                    (other.height - self.height) / 2,
                ),
                _ => todo!()
            };
            let mut new_lines = Vec::with_capacity(other.height);
            let mut index = 0;

            for _ in 0..padding1 {
                new_lines.push(vec![
                    vec![32; self.width],
                    other.lines[index].clone()
                ].concat());

                index += 1;
            }

            for _ in 0..self.height {
                new_lines.push(vec![
                    self.lines[index - padding1].clone(),
                    other.lines[index].clone()
                ].concat());

                index += 1;
            }

            for _ in 0..padding2 {
                new_lines.push(vec![
                    vec![32; self.width],
                    other.lines[index].clone()
                ].concat());

                index += 1;
            }

            Lines {
                lines: new_lines,
                width: self.width + other.width,
                height: other.height
            }
        }

        else if self.height == other.height {
            let new_lines = (0..self.height).map(
                |i| vec![
                    self.lines[i].clone(),
                    other.lines[i].clone()
                ].concat()
            ).collect();

            Lines {
                lines: new_lines,
                width: self.width + other.width,
                height: self.height
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
                _ => todo!()
            };
            let mut new_lines = Vec::with_capacity(self.height);
            let mut index = 0;

            for _ in 0..padding1 {
                new_lines.push(vec![
                    self.lines[index].clone(),
                    vec![32; other.width],
                ].concat());

                index += 1;
            }

            for _ in 0..other.height {
                new_lines.push(vec![
                    self.lines[index].clone(),
                    other.lines[index - padding1].clone()
                ].concat());

                index += 1;
            }

            for _ in 0..padding2 {
                new_lines.push(vec![
                    self.lines[index].clone(),
                    vec![32; other.width],
                ].concat());

                index += 1;
            }

            Lines {
                lines: new_lines,
                width: self.width + other.width,
                height: self.height
            }
        }

    }

    /// top, bottom, left, right
    pub fn add_padding(&self, paddings: [usize; 4]) -> Lines {
        let new_width = self.width + paddings[2] + paddings[3];

        let new_lines = vec![
            vec![vec![32; new_width]; paddings[0]],
            self.lines.iter().map(
                |line|
                vec![
                    vec![32; paddings[2]],
                    line.to_vec(),
                    vec![32; paddings[3]],
                ].concat()
            ).collect::<Vec<Vec<u16>>>(),
            vec![vec![32; new_width]; paddings[1]],
        ].concat();

        Lines {
            lines: new_lines,
            width: new_width,
            height: self.height + paddings[0] + paddings[1]
        }
    }

    /// top, bottom, left, right
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

    pub fn from_string(s: &str, alignment: Alignment) -> Self {

        // it seems like s.split() when s is empty returns a non-empty vector
        if s.len() == 0 {
            return Lines::empty();
        }

        let mut max_width = 0;
        let raw_lines: Vec<Vec<u16>> = s.split("\n").map(
            |raw_line| {
                let result = raw_line.encode_utf16().collect::<Vec<u16>>();

                if result.len() > max_width {
                    max_width = result.len();
                }

                result
            }
        ).collect();
        let mut result = Vec::with_capacity(raw_lines.len());

        for raw_line in raw_lines.into_iter() {
            let (padding1, padding2) = match alignment {
                Alignment::Center => (
                    (max_width - raw_line.len()) / 2 + (max_width - raw_line.len()) % 2,
                    (max_width - raw_line.len()) / 2,
                ),
                Alignment::First => (
                    0,
                    max_width - raw_line.len()
                ),
                Alignment::Last => (
                    max_width - raw_line.len(),
                    0
                ),
                _ => todo!()
            };

            result.push(
                vec![
                    vec![32; padding1],
                    raw_line,
                    vec![32; padding2],
                ].concat()
            );
        }

        Lines {
            width: if result.len() > 0 { result[0].len() } else { 0 },
            height: result.len(),
            lines: result,
        }
    }

    pub fn to_string(&self) -> String {
        self.lines.iter().map(|line| String::from_utf16_lossy(line)).collect::<Vec<String>>().join("\n")
    }

    #[cfg(test)]
    pub fn is_valid(&self) -> bool {
        self.lines.len() == self.height && self.lines.iter().all(|line| line.len() == self.width)
    }
}

impl std::fmt::Display for Lines {

    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.to_string())
    }

}