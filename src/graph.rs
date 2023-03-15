mod setters;

use crate::utils::{into_v16, from_v16, right_align, sns_int, into_lines};

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
    quiet: bool,
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

    /// only for 1d graphs
    full_block_character: u16,
    /// only for 1d graphs
    half_block_character: u16,
    /// only for 1d graphs
    overflow_character: u16,
    /// only for 1d graphs
    y_label_formatter: fn(i64) -> String,

    // it prevents y_label from hidden by `~` characters
    y_label_interval_offset: usize,

    skip_value: SkipValue,

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

#[derive(Clone)]
pub enum SkipValue {
    None,
    Auto,
    Range(i64, i64)
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

impl Default for Graph {

    fn default() -> Graph {
        Graph::new(48, 24)
    }

}

impl Graph {

    pub fn new(plot_width: usize, plot_height: usize) -> Self {

        Graph {
            quiet: false,
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

            y_label_interval_offset: 0,

            skip_value: SkipValue::Auto,
            full_block_character: '█' as u16,
            half_block_character: '▄' as u16,
            overflow_character: '^' as u16,
            y_label_formatter: sns_int,
            y_max: None,
            y_min: None,
            data: GraphData::None
        }
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
        let mut plot_height = self.plot_height;

        if plot_width < 3 {
            if !self.quiet { println!("Warning: `plot_width` is too small! it'll adjust the width..."); }
            plot_width = 3;
        }

        if plot_height < 3 {
            if !self.quiet { println!("Warning: `plot_height` is too small! it'll adjust the height..."); }
            plot_height = 3;
        }

        if data.len() > plot_width * 8 {

            if plot_width % 2 == 1 {
                if !self.quiet { println!("Warning: odd `plot_width` is not supported yet! it'll adjust the width..."); }
                plot_width += 1;
            }

            data = fit_data(&data, plot_width);
        }

        let mut data_sorted = data.clone();
        data_sorted.sort_by_key(|(_, val)| *val);

        let data_max = if data.len() > 0 { data_sorted[data.len() - 1].1 } else { 0 };
        let data_min = if data.len() > 0 { data_sorted[0].1 } else { 0 };

        let graph_margin = (data_max - data_min) / 8 + 1;
        let mut y_max = if let Some(n) = self.y_max { n } else if data_max < i64::MAX - graph_margin { data_max + graph_margin } else { data_max };
        let mut y_min = if let Some(n) = self.y_min { n } else if data_min > i64::MIN + graph_margin { data_min - graph_margin } else { data_min };

        if data_max == data_min && self.y_max.is_none() && self.y_min.is_none() {

            if data_max > i64::MAX - 800 {
                y_max = i64::MAX;
                y_min = i64::MAX - 800;
            }

            else if data_min < i64::MIN + 800 {
                y_max = i64::MIN + 800;
                y_min = i64::MIN;
            }

            else {
                y_max = data_max + 800;
                y_min = data_min - 800;
            }

        }

        if let Some((skip_from, skip_to)) = match self.skip_value {
            SkipValue::Auto if data.len() > 1 && plot_height > 18 => {
                let mut max_diff = 0;
                let mut suspicious = (0, 0);

                for i in 0..(data.len() - 1) {
                    let diff = data_sorted[i + 1].1 - data_sorted[i].1;

                    if diff > max_diff {
                        max_diff = diff;
                        suspicious = (data_sorted[i].1, data_sorted[i + 1].1);
                    }

                }

                if max_diff > (data_max - data_min) / 4 && max_diff > 4 {
                    Some((suspicious.0 + 1, suspicious.1 - 1))
                }

                else {
                    None
                }

            }
            SkipValue::Range(skip_from, skip_to) if data.len() > 1 && plot_height > 18 => if skip_from < data_min || skip_to > data_max || skip_from >= skip_to {
                if !self.quiet {
                    println!(
                        "Warning: SkipValue::Range({skip_from}, {skip_to}) is invalid!\n`skip_from >= data_min && skip_to <= data_max && skip_from < skip_to` must hold but skip_from: {skip_from}, skip_to: {skip_to}, data_min: {data_min}, data_max: {data_max}"
                    );
                }
                None
            } else {
                Some((skip_from, skip_to))
            },
            _ => None
        } {
            let upper_y_max = if let Some(n) = self.y_max { n } else if data_max > i64::MAX - (data_max - skip_to) / 8 {
                i64::MAX
            } else {
                data_max + (data_max - skip_to) / 8 + 1
            };
            let upper_y_min = skip_to - (data_max - skip_to) / 8;

            let lower_y_max = skip_from + (skip_from - data_min) / 8;
            let lower_y_min = if let Some(n) = self.y_min { n } else if data_min < i64::MIN + (skip_from - data_min) / 8 {
                i64::MIN
            } else {
                data_min - (skip_from - data_min) / 8 - 1
            };

            let mut upper_graph_height = plot_height / 3;
            let mut lower_graph_height = plot_height - upper_graph_height - 1;

            // upper_graph has more values
            if skip_to < data_sorted[data.len() / 2].1 {
                let tmp = upper_graph_height;
                upper_graph_height = lower_graph_height;
                lower_graph_height = tmp;
            }

            let upper_graph = Graph {
                x_axis_label: None,
                padding_bottom: 0,
                skip_value: SkipValue::None,
                plot_height: upper_graph_height,
                y_max: Some(upper_y_max),
                y_min: Some(upper_y_min),
                data: GraphData::OneDimensional(data.clone()),
                ..self.clone()
            }.draw();
            let lower_graph = Graph {
                padding_top: 0,
                title: None,
                y_axis_label: None,
                skip_value: SkipValue::None,
                plot_height: lower_graph_height,
                y_label_interval_offset: 1,
                y_max: Some(lower_y_max),
                y_min: Some(lower_y_min),
                data: GraphData::OneDimensional(data),
                ..self.clone()
            }.draw();

            let mut upper_graph = into_lines(&upper_graph);
            upper_graph = upper_graph[0..(upper_graph.len() - 3)].to_vec();  // remove x axis
            let mut lower_graph = into_lines(&lower_graph);
            lower_graph = lower_graph[1..].to_vec();  // remove overflow characters
            let line_width = upper_graph[0].len();

            let result = vec![
                upper_graph,
                vec![
                    vec!['~' as u16; line_width],
                    vec!['~' as u16; line_width],
                ],
                lower_graph
            ].concat().join(&['\n' as u16][..]);

            return from_v16(&result);
        }

        let line_width = plot_width + self.y_label_max_len + self.padding_left + self.padding_right + 3;

        let padding_top = draw_empty_lines(line_width, self.padding_top);
        let padding_bottom = draw_empty_lines(line_width, self.padding_bottom);

        let mut y_grid_size = (y_max - y_min) / plot_height as i64;

        // an error from integer division made a problem
        if y_max - (plot_height - 1) as i64 * y_grid_size > data_min {
            y_grid_size += 1;

            // but it must not cause an overflow
            if y_max < i64::MIN + (plot_height as i64 + 1) * y_grid_size {
                y_grid_size -= 1;
            }

        }

        let mut result = vec![' ' as u16; line_width * (plot_height + 2)];

        for y in 0..(plot_height + 2) {
            result[y * line_width + (line_width - 1)] = '\n' as u16;

            if y < plot_height {
                result[y * line_width + self.y_label_max_len + 1 + self.padding_left] = '|' as u16;

                if y % self.y_label_interval == self.y_label_interval_offset {
                    let ylabel = into_v16(&right_align((self.y_label_formatter)(y_max - y as i64 * y_grid_size), self.y_label_max_len));

                    for x in 0..ylabel.len() {
                        result[y * line_width + x + self.padding_left] = ylabel[x];
                    }

                }

            }

        }

        result[plot_height * line_width + self.y_label_max_len + 1 + self.padding_left] = '└' as u16;

        for x in 0..plot_width {

            if data.len() > 0 {
                let (curr_x_label, curr_val) = data[x * data.len() / plot_width].clone();
                let mut overflow = false;
                let mut start_y = if y_max > curr_val {
                    ((y_max - curr_val) * 2 / y_grid_size) as usize
                } else {
                    overflow = true;
                    0
                };
                let use_half_block_character = start_y % 2 == 1;
                start_y /= 2;

                for y in start_y..plot_height {
                    result[y * line_width + x + self.y_label_max_len + 2 + self.padding_left] = self.full_block_character;
                }

                if use_half_block_character && start_y < plot_height {
                    result[start_y * line_width + x + self.y_label_max_len + 2 + self.padding_left] = self.half_block_character;
                }

                else if overflow {
                    result[x + self.y_label_max_len + 2 + self.padding_left] = self.overflow_character;
                }

                if x % self.x_label_interval == 0 {
                    let xlabel = into_v16(&curr_x_label);

                    if x + xlabel.len() < plot_width + 1 {

                        for xx in 0..xlabel.len() {
                            result[(plot_height + 1) * line_width + x + self.y_label_max_len + 2 + xx + self.padding_left] = xlabel[xx];
                        }

                    }

                }

            }

            result[plot_height * line_width + x + self.y_label_max_len + 2 + self.padding_left] = '-' as u16;
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

        let padding_top = draw_empty_lines(line_width, self.padding_top);
        let padding_bottom = draw_empty_lines(line_width, self.padding_bottom);

        let mut result = vec![' ' as u16; line_width * (self.plot_height + 2)];

        for y in 0..(self.plot_height + 2) {
            result[y * line_width + (line_width - 1)] = '\n' as u16;

            if y < self.plot_height {
                result[y * line_width + self.y_label_max_len + 1 + self.padding_left] = '|' as u16;

                if y != self.plot_height && y_labels[y].is_some() {
                    let ylabel = into_v16(&right_align(y_labels[y].as_ref().unwrap().to_string(), self.y_label_max_len));

                    for x in 0..ylabel.len() {
                        result[y * line_width + x + self.padding_left] = ylabel[x];
                    }

                }

            }

        }

        result[self.plot_height * line_width + self.y_label_max_len + 1 + self.padding_left] = '└' as u16;

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

fn draw_empty_lines(line_width: usize, height: usize) -> Vec<u16> {

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