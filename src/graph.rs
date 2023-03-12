use crate::utils::{into_v16, from_v16, right_align, sns_int, fractional_number};

// All the strings returned by `Graph::draw()`, `merge_vert()` and `merge_horiz()` must be rectangles

/*
 * ascii art on (width + y_label_len + 3) * (height + 3)
 * without title, it's (width + y_label_len + 3) * (height + 2)
 *
 *  pppppppppppppppppppppppppn
 *  p         title         pn
 *  pccc                    pn
 *  paaaybbbbbbbbbbbbbbbbbbbpn
 *  paaaybbbbbbbbbbbbbbbbbbbpn
 *  paaaybbbbbbbbbbbbbbbbbbbpn
 *  paaaybbbbbbbbbbbbbbbbbbbpn
 *  paaayxxxxxxxxxxxxxxxxxxxpn
 *  p   ddddddddddddddddddddpn
 *  p                    eeepn
 *  pppppppppppppppppppppppppn
 * p: paddings
 * a, d: labels
 * x, y: borders
 * n: newline characters
 * c: y axis label
 * e: x axis label
 * b: plot
 * title: (width + y_label_len + 3) * 1, a: y_label_len * (height + 1), y: 2 * (height + 1), b: width * height
 * x: width * 1, d: (width + 1) * 1, n: 1 * (height + 2)
 */
#[derive(Clone)]
pub struct Graph {
    data: GraphData,
    title: Option<String>,
    x_label_interval: usize,
    y_label_interval: usize,
    y_label_max_len: usize,
    plot_width: usize,
    plot_height: usize,

    block_width: Option<usize>,

    x_axis_label: Option<String>,
    y_axis_label: Option<String>,

    padding_top: usize,
    padding_bottom: usize,
    padding_left: usize,
    padding_right: usize,

    // only for 1d graphs
    full_block_character: u16,
    half_block_character: u16,
    y_label_formatter: fn(i64) -> String,

    // if None, the range is set automatically
    y_min: Option<i64>,
    y_max: Option<i64>
}

#[derive(Clone, Debug)]
enum GraphData {
    OneDimensional(Vec<(String, i64)>),  // Vec<(XLabel, Data)>
    TwoDimensional {
        x_labels: Vec<Option<String>>,
        y_labels: Vec<Option<String>>,
        data: Vec<(usize, usize, u16)>
    },
    None
}

impl GraphData {

    fn unwrap_1d(&self) -> Vec<(String, i64)> {
        match self {
            GraphData::OneDimensional(d) => d.clone(),
            _ => panic!("called `GraphData::unwrap_1d()` on a `{self:?}` value")
        }
    }

    fn unwrap_2d(&self) -> (Vec<Option<String>>, Vec<Option<String>>, Vec<(usize, usize, u16)>) {
        match self {
            GraphData::TwoDimensional { data, x_labels, y_labels } => (
                x_labels.to_vec(), y_labels.to_vec(), data.to_vec()
            ),
            _ => panic!("called `GraphData::unwrap_2d()` on a `{self:?}` value")
        }
    }

}

impl Graph {

    pub fn new(plot_width: usize, plot_height: usize) -> Self {

        if plot_width < 20 {
            println!("Warning: `plot_width` too small");
        }

        if plot_height < 8 {
            println!("Warning: `plot_height` too small");
        }

        Graph {
            plot_width,
            plot_height,
            title: None,
            x_label_interval: 8,
            y_label_interval: 3,
            y_label_max_len: 6,

            block_width: None,

            x_axis_label: None,
            y_axis_label: None,

            padding_top: 0,
            padding_bottom: 0,
            padding_left: 0,
            padding_right: 0,

            full_block_character: '█' as u16,
            half_block_character: '▄' as u16,
            y_label_formatter: sns_int,
            y_max: None,
            y_min: None,
            data: GraphData::None
        }
    }

    pub fn set_title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn set_x_axis_label(&mut self, x_axis_label: String) -> &mut Self {
        self.x_axis_label = Some(x_axis_label);
        self
    }

    pub fn set_y_axis_label(&mut self, y_axis_label: String) -> &mut Self {
        self.y_axis_label = Some(y_axis_label);
        self
    }

    pub fn set_full_block_character(&mut self, full_block_character: u16) -> &mut Self {
        self.full_block_character = full_block_character;
        self
    }

    pub fn set_half_block_character(&mut self, half_block_character: u16) -> &mut Self {
        self.half_block_character = half_block_character;
        self
    }

    pub fn set_plot_width(&mut self, plot_width: usize) -> &mut Self {
        self.plot_width = plot_width;

        if plot_width < 20 {
            println!("Warning: `plot_width` too small");
        }

        self
    }

    pub fn set_plot_height(&mut self, plot_height: usize) -> &mut Self {
        self.plot_height = plot_height;

        if plot_height < 8 {
            println!("Warning: `plot_height` too small");
        }

        self
    }

    /// It works only with 1d data.
    /// It makes sense when `self.data.len()` is small enough.
    /// If both `self.plot_width` and `self.block_width` are set, `block_width` has a precedence.
    pub fn set_block_width(&mut self, block_width: usize) -> &mut Self {
        self.block_width = Some(block_width);

        match &self.data {
            GraphData::OneDimensional(v) => {
                self.plot_width = v.len() * block_width;
            }
            _ => {}
        }

        self
    }

    pub fn set_y_label_max_len(&mut self, y_label_max_len: usize) -> &mut Self {
        self.y_label_max_len = y_label_max_len;
        self
    }

    pub fn set_x_label_interval(&mut self, x_label_interval: usize) -> &mut Self {
        self.x_label_interval = x_label_interval;
        self
    }

    pub fn set_y_label_interval(&mut self, y_label_interval: usize) -> &mut Self {
        self.y_label_interval = y_label_interval;
        self
    }

    pub fn set_y_label_formatter(&mut self, y_label_formatter: fn(i64) -> String) -> &mut Self {
        self.y_label_formatter = y_label_formatter;
        self
    }

    pub fn set_1d_data(&mut self, data: Vec<(String, i64)>) -> &mut Self {

        match self.block_width {
            Some(n) => {
                self.plot_width = data.len() * n;
            }
            _ => {}
        }

        self.data = GraphData::OneDimensional(data);
        self
    }

    /// It uses fixed point numbers to represent real numbers. It uses 12 bits for the fractional parts.
    /// If you want another representation, you have to implement by yourself.
    pub fn set_1d_data_float(&mut self, data: Vec<(String, f64)>) -> &mut Self {
        self.data = GraphData::OneDimensional(data.into_iter().map(
            |(s, n)| (s, (n * 4096.0) as i64)
        ).collect());
        self.y_label_formatter = fractional_number;
        self
    }

    pub fn set_2d_data(&mut self, data: Vec<(usize, usize, u16)>, x_labels: Vec<Option<String>>, y_labels: Vec<Option<String>>) -> &mut Self {
        self.data = GraphData::TwoDimensional { data, x_labels, y_labels };
        self
    }

    pub fn set_y_min(&mut self, y_min: i64) -> &mut Self {
        self.y_min = Some(y_min);
        self
    }

    pub fn set_y_max(&mut self, y_max: i64) -> &mut Self {
        self.y_max = Some(y_max);
        self
    }

    pub fn set_y_min_float(&mut self, y_min: f64) -> &mut Self {
        self.y_min = Some((y_min * 4096.0) as i64);
        self.y_label_formatter = fractional_number;
        self
    }

    pub fn set_y_max_float(&mut self, y_max: f64) -> &mut Self {
        self.y_max = Some((y_max * 4096.0) as i64);
        self.y_label_formatter = fractional_number;
        self
    }

    pub fn set_padding_top(&mut self, padding_top: usize) -> &mut Self {
        self.padding_top = padding_top;
        self
    }

    pub fn set_padding_bottom(&mut self, padding_bottom: usize) -> &mut Self {
        self.padding_bottom = padding_bottom;
        self
    }

    pub fn set_padding_left(&mut self, padding_left: usize) -> &mut Self {
        self.padding_left = padding_left;
        self
    }

    pub fn set_padding_right(&mut self, padding_right: usize) -> &mut Self {
        self.padding_right = padding_right;
        self
    }

    /// top, bottom, left, right
    pub fn set_paddings(&mut self, paddings: [usize; 4]) -> &mut Self {
        self.padding_top = paddings[0];
        self.padding_bottom = paddings[1];
        self.padding_left = paddings[2];
        self.padding_right = paddings[3];

        self
    }

    pub fn set_y_range(&mut self, y_min: i64, y_max: i64) -> &mut Self {
        self.y_min = Some(y_min);
        self.y_max = Some(y_max);
        self
    }

    pub fn draw(&self) -> String {

        match self.data {
            GraphData::OneDimensional(_) => self.draw_1d(),
            GraphData::TwoDimensional { .. } => self.draw_2d(),
            GraphData::None => panic!("There's nothing to draw!")
        }

    }

    fn draw_1d(&self) -> String {
        let mut data = self.data.unwrap_1d();
        let mut plot_width = self.plot_width;

        if data.len() > plot_width * 8 {

            if plot_width % 2 == 1 {
                println!("Warning: odd `plot_width` is not supported yet! it'll adjust the width...");
                plot_width += 1;
            }

            data = fit_data(&data, plot_width);
        }

        let data_max = *data.iter().map(|(_, n)| n).max().unwrap();
        let data_min = *data.iter().map(|(_, n)| n).min().unwrap();
        let line_width = plot_width + self.y_label_max_len + self.padding_left + self.padding_right + 3;

        let padding_top = draw_lines(line_width, self.padding_top);
        let padding_bottom = draw_lines(line_width, self.padding_bottom);

        let graph_margin = (data_max - data_min) / 8 + 1;
        let y_max = if let Some(n) = self.y_max { n } else if data_max < i64::MAX - graph_margin { data_max + graph_margin } else { data_max };
        let y_min = if let Some(n) = self.y_min { n } else if data_min > i64::MIN + graph_margin { data_min - graph_margin } else { data_min };

        let mut y_grid_size = (y_max - y_min) / self.plot_height as i64;

        // an error from integer division made a problem
        if y_max - (self.plot_height - 1) as i64 * y_grid_size > data_min {
            y_grid_size += 1;
        }

        let mut result = vec![' ' as u16; line_width * (self.plot_height + 2)];

        for y in 0..(self.plot_height + 2) {
            result[y * line_width + (line_width - 1)] = '\n' as u16;

            if y != self.plot_height + 1 {
                result[y * line_width + self.y_label_max_len + 1 + self.padding_left] = '|' as u16;

                if y % self.y_label_interval == 0 {
                    let ylabel = into_v16(&right_align((self.y_label_formatter)(y_max - y as i64 * y_grid_size), self.y_label_max_len));

                    for x in 0..ylabel.len() {
                        result[y * line_width + x + self.padding_left] = ylabel[x];
                    }

                }

            }

        }

        for x in 0..plot_width {
            let (curr_x_label, curr_val) = data[x * data.len() / plot_width].clone();
            let mut start_y = if y_max > curr_val { ((y_max - curr_val) * 2 / y_grid_size) as usize } else { 0 };
            let use_half_block_character = start_y % 2 == 1;
            start_y /= 2;

            result[self.plot_height * line_width + x + self.y_label_max_len + 2 + self.padding_left] = '-' as u16;

            for y in start_y..self.plot_height {
                result[y * line_width + x + self.y_label_max_len + 2 + self.padding_left] = self.full_block_character;
            }

            if use_half_block_character && start_y < self.plot_height {
                result[start_y * line_width + x + self.y_label_max_len + 2 + self.padding_left] = self.half_block_character;
            }

            if x % self.x_label_interval == 0 {
                let xlabel = into_v16(&curr_x_label);

                if x + xlabel.len() < plot_width + 1 {

                    for xx in 0..xlabel.len() {
                        result[(self.plot_height + 1) * line_width + x + self.y_label_max_len + 2 + xx + self.padding_left] = xlabel[xx];
                    }

                }

            }

        }

        result = vec![
            padding_top,
            self.draw_title_line(line_width, self.padding_left, self.padding_right),
            self.draw_y_axis_label(line_width, self.padding_left, self.y_label_max_len),
            result,
            self.draw_x_axis_label(line_width, self.padding_right),
            padding_bottom
        ].concat();

        from_v16(&result)
    }

    fn draw_2d(&self) -> String {
        let (x_labels, y_labels, data) = self.data.unwrap_2d();

        assert_eq!(self.plot_width, x_labels.len());
        assert_eq!(self.plot_height, y_labels.len());

        let line_width = self.plot_width + self.y_label_max_len + 3 + self.padding_left + self.padding_right;

        let padding_top = draw_lines(line_width, self.padding_top);
        let padding_bottom = draw_lines(line_width, self.padding_bottom);

        let mut result = vec![' ' as u16; line_width * (self.plot_height + 2)];

        for y in 0..(self.plot_height + 2) {
            result[y * line_width + (line_width - 1)] = '\n' as u16;

            if y != self.plot_height + 1 {
                result[y * line_width + self.y_label_max_len + 1 + self.padding_left] = '|' as u16;

                if y != self.plot_height && y_labels[y].is_some() {
                    let ylabel = into_v16(&right_align(y_labels[y].as_ref().unwrap().to_string(), self.y_label_max_len));

                    for x in 0..ylabel.len() {
                        result[y * line_width + x + self.padding_left] = ylabel[x];
                    }

                }

            }

        }

        let mut last_x = 0;

        for x in 0..self.plot_width {
            result[self.plot_height * line_width + x + self.y_label_max_len + 2 + self.padding_left] = '-' as u16;

            if x_labels[x].is_some() && (x - last_x >= self.x_label_interval || x == 0) {
                let xlabel = into_v16(x_labels[x].as_ref().unwrap());

                if x + xlabel.len() < self.plot_width + 1 {
                    last_x = x;

                    for xx in 0..xlabel.len() {
                        result[(self.plot_height + 1) * line_width + x + self.y_label_max_len + 2 + xx + self.padding_left] = xlabel[xx];
                    }

                }

            }

        }

        for (x, y, c) in data.into_iter() {

            if x >= self.plot_width || y >= self.plot_height {
                panic!("{} {} {} {}", x, y, self.plot_width, self.plot_height);
            }

            result[y * line_width + self.y_label_max_len + 2 + x + self.padding_left] = c;
        }

        result = vec![
            padding_top,
            self.draw_title_line(line_width, self.padding_left, self.padding_right),
            self.draw_y_axis_label(line_width, self.padding_left, self.y_label_max_len),
            result,
            self.draw_x_axis_label(line_width, self.padding_right),
            padding_bottom
        ].concat();

        from_v16(&result)
    }

    fn draw_x_axis_label(&self, line_width: usize, padding_right: usize) -> Vec<u16> {
        match &self.x_axis_label {
            Some(l) => {
                let mut line = vec![' ' as u16; line_width];
                line[line_width - 1] = '\n' as u16;

                let mut title = into_v16(&l);
                let title_begin_x = if title.len() + padding_right > line_width {
                    title = title[0..(line_width - padding_right - 1)].to_vec();
                    0
                } else {
                    line_width - padding_right - title.len() - 1
                };
        
                for (i, c) in title.into_iter().enumerate() {
                    line[title_begin_x + i] = c;
                }

                line
            }
            None => vec![]
        }
    }

    fn draw_y_axis_label(&self, line_width: usize, padding_left: usize, y_label_max_len: usize) -> Vec<u16> {
        match &self.y_axis_label {
            Some(l) => {
                let mut line = vec![' ' as u16; line_width];
                line[line_width - 1] = '\n' as u16;

                let mut title = into_v16(&l);

                if title.len() + padding_left + y_label_max_len >= line_width {
                    title = title[0..(line_width - padding_left - y_label_max_len)].to_vec();
                }

                for (i, c) in title.into_iter().enumerate() {
                    line[i + padding_left + y_label_max_len] = c;
                }

                line
            }
            None => vec![]
        }
    }

    fn draw_title_line(&self, line_width: usize, padding_left: usize, padding_right: usize) -> Vec<u16> {
        match &self.title {
            Some(t) => {
                let mut line = vec![' ' as u16; line_width];
                line[line_width - 1] = '\n' as u16;

                let title = into_v16(&t);
                let title_begin_x = if title.len() > line_width {
                    0
                } else {
                    (line_width - padding_left - padding_right - title.len()) / 2
                };
        
                for (i, c) in title.into_iter().enumerate() {
                    line[title_begin_x + i + padding_left] = c;
                }

                line
            }
            _ => vec![]
        }
    }

}

fn draw_lines(line_width: usize, height: usize) -> Vec<u16> {

    if height == 0 {
        return vec![];
    }

    vec![vec![vec![' ' as u16; line_width - 1], vec!['\n' as u16]].concat(); height].concat()
}

fn fit_data(data: &Vec<(String, i64)>, width: usize) -> Vec<(String, i64)> {
    // a graph with odd-sized width is not supported because of this line
    let half_width = width / 2;

    let mut last_ind = 0;
    let mut result = Vec::with_capacity(width);

    for i in 0..half_width {
        let curr_ind = (i + 1) * data.len() / half_width;
        let mut min_ind = 0;
        let mut min_val = data[last_ind].1;
        let mut max_ind = 0;
        let mut max_val = data[last_ind].1;

        for (ind, (_, val)) in data[last_ind..curr_ind].iter().enumerate() {

            if val > &max_val {
                max_ind = ind;
                max_val = *val;
            }

            else if val < &min_val {
                min_ind = ind;
                min_val = *val;
            }

        }

        if min_ind < max_ind {
            result.push(data[last_ind + min_ind].clone());
            result.push(data[last_ind + max_ind].clone());
        }

        else {
            result.push(data[last_ind + max_ind].clone());
            result.push(data[last_ind + min_ind].clone());
        }

        last_ind = curr_ind;
    }

    result
}