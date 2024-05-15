// from https://baehyunsol.github.io/MDxt-Reference.html#colors
#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Black,
    Dark,
    Gray,
    Lightgray,
    White,
    Red,
    Green,
    Blue,
    Brown,
    Slateblue,
    Seagreen,
    Aqua,
    Emerald,
    Violet,
    Turquoise,
    Pink,
    Grassgreen,
    Gold,
}

impl Color {
    pub fn get_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Black => (0, 0, 0),
            Color::Dark => (64, 64, 64),
            Color::Gray => (128, 128, 128),
            Color::Lightgray => (192, 192, 192),
            Color::White => (255, 255, 255),
            Color::Red => (192, 32, 32),
            Color::Green => (32, 192, 32),
            Color::Blue => (32, 32, 192),
            Color::Brown => (192, 128, 32),
            Color::Slateblue => (64, 64, 192),
            Color::Seagreen => (32, 192, 192),
            Color::Aqua => (64, 192, 255),
            Color::Emerald => (64, 192, 64),
            Color::Violet => (192, 64, 255),
            Color::Turquoise => (64, 255, 192),
            Color::Pink => (255, 64, 192),
            Color::Grassgreen => (192, 255, 64),
            Color::Gold => (255, 192, 64),
        }
    }

    pub fn append_start_marker(&self, buffer: &mut Vec<char>, color_mode: &ColorMode) {
        match color_mode {
            ColorMode::Html { prefix } => {
                for ch in format!("<span class=\"{prefix}{}\">", format!("{self:?}").to_lowercase()).chars() {
                    buffer.push(ch);
                }
            },
            ColorMode::Terminal => {
                let (r, g, b) = self.get_rgb();

                for ch in format!("\x1b[38;2;{r};{g};{b}m").chars() {
                    buffer.push(ch);
                }
            },
            ColorMode::None => {},
        }
    }

    pub fn append_end_marker(&self, buffer: &mut Vec<char>, color_mode: &ColorMode) {
        match color_mode {
            ColorMode::Html { .. } => {
                for ch in "</span>".chars() {
                    buffer.push(ch);
                }
            },
            ColorMode::Terminal => {
                for ch in "\x1b[39m".chars() {
                    buffer.push(ch);
                }
            },
            ColorMode::None => {},
        }
    }
}

#[derive(Clone)]
pub enum ColorMode {
    None,

    /// `<span class="{prefix}red">`
    Html { prefix: String },

    // in python,
    // print('\033[38;2;255;82;197mHello\033[39m')
    // `[38` to change foreground color,
    // `[48` for background
    // `[39` to reset foreground and `[49` for background
    // https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
    /// https://en.wikipedia.org/wiki/ANSI_escape_code
    Terminal,
}

impl ColorMode {
    pub fn apply_colors(
        &self,
        string: String,
        colors: Vec<Option<Color>>,
    ) -> String {
        if let ColorMode::None = self {
            string
        }

        else {
            let chars = string.chars().collect::<Vec<_>>();
            let mut buffer = Vec::with_capacity(chars.len());
            let mut curr_color = None;

            assert_eq!(chars.len(), colors.len());

            for i in 0..chars.len() {
                if curr_color != colors[i] {
                    match &colors[i] {
                        Some(color) => {
                            if let Some(curr_color) = &curr_color {
                                curr_color.append_end_marker(&mut buffer, self);
                            }

                            color.append_start_marker(&mut buffer, self);
                        },
                        None => if let Some(curr_color) = &curr_color {
                            curr_color.append_end_marker(&mut buffer, self);
                        } else {
                            unreachable!()
                        },
                    }

                    curr_color = colors[i].clone();
                }

                buffer.push(chars[i]);
            }

            if let Some(color) = curr_color {
                color.append_end_marker(&mut buffer, self);
            }

            buffer.into_iter().collect()
        }
    }
}
