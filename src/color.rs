use std::str::FromStr;

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
            ColorMode::TerminalFg
            | ColorMode::TerminalBg => {
                let (r, g, b) = self.get_rgb();
                let head = if let ColorMode::TerminalFg = color_mode {
                    "38"
                } else {
                    "48"
                };

                for ch in format!("\x1b[{head};2;{r};{g};{b}m").chars() {
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
            ColorMode::TerminalFg => {
                for ch in "\x1b[39m".chars() {
                    buffer.push(ch);
                }
            },
            ColorMode::TerminalBg => {
                for ch in "\x1b[49m".chars() {
                    buffer.push(ch);
                }
            },
            ColorMode::None => {},
        }
    }
}

impl FromStr for Color {
    type Err = String;

    /// returns Err(s) if it fails
    fn from_str(s: &str) -> Result<Color, String> {
        match s.replace(" ", "").replace("_", "").replace("-", "").to_ascii_lowercase() {
            s if s == "black" => Ok(Color::Black),
            s if s == "dark" => Ok(Color::Dark),
            s if s == "gray" => Ok(Color::Gray),
            s if s == "lightgray" => Ok(Color::Lightgray),
            s if s == "white" => Ok(Color::White),
            s if s == "red" => Ok(Color::Red),
            s if s == "green" => Ok(Color::Green),
            s if s == "blue" => Ok(Color::Blue),
            s if s == "brown" => Ok(Color::Brown),
            s if s == "slateblue" => Ok(Color::Slateblue),
            s if s == "seagreen" => Ok(Color::Seagreen),
            s if s == "aqua" => Ok(Color::Aqua),
            s if s == "emerald" => Ok(Color::Emerald),
            s if s == "violet" => Ok(Color::Violet),
            s if s == "turquoise" => Ok(Color::Turquoise),
            s if s == "pink" => Ok(Color::Pink),
            s if s == "grassgreen" => Ok(Color::Grassgreen),
            s if s == "gold" => Ok(Color::Gold),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Clone)]
pub enum ColorMode {
    None,

    /// `<span class="{prefix}red">`
    Html { prefix: String },

    // https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
    /// https://en.wikipedia.org/wiki/ANSI_escape_code
    TerminalFg,
    TerminalBg,
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

impl FromStr for ColorMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s.replace(" ", "").replace("_", "").replace("-", "").to_ascii_lowercase() {
            s if s == "none" => Ok(ColorMode::None),
            s if s == "html" => Ok(ColorMode::Html { prefix: String::new() }),  // TODO: make it configurable
            s if s == "terminalfg" => Ok(ColorMode::TerminalFg),
            s if s == "terminalbg" => Ok(ColorMode::TerminalBg),
            _ => Err(s.to_string()),
        }
    }
}
