use hmath::Ratio;
use crate::alignment::Alignment;
use crate::lines::Lines;
use crate::format::format_ratio;
use crate::skip_value::SkipValue;
use std::collections::HashSet;

mod merge;
mod setters;

pub use merge::*;

#[derive(Clone)]
pub struct Graph {
    data: GraphData,

    title: Option<String>,
    big_title: bool,

    plot_width: usize,
    plot_height: usize,

    block_width: Option<usize>,

    y_label_interval: usize,

    x_axis_label: Option<String>,
    y_axis_label: Option<String>,

    y_min: Option<Ratio>,
    y_max: Option<Ratio>,

    skip_value: SkipValue,

    paddings: [usize; 4],
}

#[derive(Debug, PartialEq, Clone)]
enum GraphData {
    Data1D (Vec<(String, Ratio)>),
    Data2D {
        data: Vec<(usize, usize, u16)>,
        x_labels: Vec<Option<String>>,
        y_labels: Vec<Option<String>>,
    },
    None
}

impl GraphData {

    pub fn unwrap_1d(&self) -> &Vec<(String, Ratio)> {
        if let GraphData::Data1D(v) = self {
            v
        } else {
            panic!("Unable to unwrap 1d data from {self:?}")
        }
    }

    pub fn unwrap_2d(&self) -> ( &Vec<(usize, usize, u16)>, &Vec<Option<String>>, &Vec<Option<String>> ) {
        if let GraphData::Data2D { data, x_labels, y_labels } = self {
            (&data, &x_labels, &y_labels)
        } else {
            panic!("Unable to unwrap 2d data from {self:?}")
        }
    }

}

impl Graph {

    pub fn new(plot_width: usize, plot_height: usize) -> Self {
        Graph {
            plot_width,
            plot_height,
            ..Default::default()
        }
    }

    /// It panics if it's not well-configured. If you're not sure, try `.is_valid` before calling this method
    pub fn draw(&self) -> String {

        match &self.data {
            GraphData::Data1D(_) => self.draw_1d_graph(),
            GraphData::Data2D { .. } => self.draw_2d_graph(),
            GraphData::None => panic!("Cannot draw a graph without any data")
        }

    }

    /// 1. `self.data` must be set and for 1-D data, it must not be empty.
    /// 2. If `self.y_min` and `self.y_max` are set, `self.y_max` has to be greater than `self.y_min`.
    /// 3. If you're using a 2-dimensional data, `data`, `x_labels` and `y_labels` must have the same dimension.
    pub fn is_valid(&self) -> bool {
        (match (&self.y_min, &self.y_max) {  // why do I need to wrap it with parenthesis?
            (Some(n), Some(m)) if n.gt_rat(&m) => false,
            _ => true
        }) && match &self.data {
            GraphData::Data1D(v) => v.len() > 0,
            GraphData::Data2D { data, x_labels, y_labels } if x_labels.len() > 0 && y_labels.len() > 0 => {
                let mut x_max = 0;
                let mut y_max = 0;

                for (x, y, _) in data.iter() {

                    if *x > x_max {
                        x_max = *x;
                    }

                    if *y > y_max {
                        y_max = *y;
                    }

                }

                x_labels.len() >= x_max && y_labels.len() >= y_max && x_labels.len() == self.plot_width && y_labels.len() == self.plot_height
            }
            _ => false
        } && {
            // TODO
            true
        }
    }

    fn draw_1d_graph(&self) -> String {
        let mut data = self.data.unwrap_1d().clone();

        let plot_width = match &self.block_width {
            Some(w) => w * data.len(),
            _ => self.plot_width
        };

        if data.len() > plot_width * 2 {
            data = pick_meaningful_values(&data, plot_width);
        }

        let (data_min, data_max, max_diff) = get_min_max_diff(&data, self.plot_height);
        let (mut y_min, mut y_max) = unwrap_y_min_max(&self.y_min, &self.y_max, &data_min, &data_max);

        let mut ratio_of_subgraphs = (3, 3);

        let mut skip_range = match &self.skip_value {
            _ if self.plot_height <= 18 => None,
            SkipValue::None => None,
            SkipValue::Automatic => {

                if !max_diff.is_zero() && y_max.sub_rat(&y_min).div_rat(&max_diff).lt_i32(3) {
                    let (y_min_, from, to, y_max_, ratio_of_subgraphs_) = get_where_to_skip(data.clone());
                    ratio_of_subgraphs = ratio_of_subgraphs_;
                    y_min = y_min_;
                    y_max = y_max_;

                    Some((from, to))
                }

                else {
                    None
                }

            },
            SkipValue::Manual { from, to } => {
                let mut values_below_skip_range = HashSet::new();
                let mut values_above_skip_range = HashSet::new();

                for (_, n) in data.iter() {

                    if n.lt_rat(from) {
                        values_below_skip_range.insert(n.clone());
                    }

                    else if n.gt_rat(to) {
                        values_above_skip_range.insert(n.clone());
                    }

                }

                if values_below_skip_range.len() * 2 > values_above_skip_range.len() * 3 {
                    ratio_of_subgraphs = (4, 2);
                }

                else if values_above_skip_range.len() * 2 > values_below_skip_range.len() * 3 {
                    ratio_of_subgraphs = (2, 4);
                }

                Some((from.clone(), to.clone()))
            }
        };

        if let Some((from, to)) = &skip_range {

            if from.lt_rat(&y_min) || to.gt_rat(&y_max) {
                skip_range = None;
            }

        }

        let mut plot = match &skip_range {
            None => {
                let mut plot = plot_1d(&data, plot_width, self.plot_height, &y_min, &y_max, false);
                plot = plot.add_border([false, true, true, false]);

                let y_labels = draw_y_labels_1d_plot(&y_min, &y_max, self.plot_height, self.y_label_interval);

                y_labels.merge_horizontally(&plot, Alignment::First)
            }
            Some((from, to)) => {
                let (mut height1, mut height2) = (
                    self.plot_height * ratio_of_subgraphs.0 / 6,
                    self.plot_height * ratio_of_subgraphs.1 / 6,
                );
                height2 += self.plot_height - height1 - height2;

                if height1 > height2 {
                    height1 -= 1;
                }

                else {
                    height2 -= 1;
                }

                let mut plot1 = plot_1d(&data, plot_width, height1, &y_min, &from, true);
                plot1 = plot1.add_border([false, true, true, false]);

                let mut plot2 = plot_1d(&data, plot_width, height2, &to, &y_max, false);
                plot2 = plot2.add_border([false, false, true, false]);

                let mut y_labels1 = draw_y_labels_1d_plot(&y_min, &from, height1, self.y_label_interval);
                let mut y_labels2 = draw_y_labels_1d_plot(&to, &y_max, height2, self.y_label_interval);

                if y_labels1.get_width() < y_labels2.get_width() {
                    y_labels1 = y_labels1.add_padding([0, 0, y_labels2.get_width() - y_labels1.get_width(), 0]);
                }

                else if y_labels2.get_width() < y_labels1.get_width() {
                    y_labels2 = y_labels2.add_padding([0, 0, y_labels1.get_width() - y_labels2.get_width(), 0]);
                }

                plot1 = y_labels1.merge_horizontally(&plot1, Alignment::First);
                plot2 = y_labels2.merge_horizontally(&plot2, Alignment::First);

                let horizontal_line = Lines::from_string(&"~".repeat(plot1.get_width()), Alignment::First);

                plot1 = horizontal_line.merge_vertically(&plot1, Alignment::First);

                plot2.merge_vertically(&plot1, Alignment::First)
            }
        };

        let x_labels = draw_x_labels(&data, plot_width);
        plot = plot.merge_vertically(&x_labels, Alignment::Last);

        if let Some(xal) = &self.x_axis_label {
            let mut xal = Lines::from_string(xal, Alignment::First);
            xal = xal.add_padding([self.plot_height, 0, 0, 0]);
            plot = plot.merge_horizontally(&xal, Alignment::First);
        }

        if let Some(yal) = &self.y_axis_label {
            let yal = Lines::from_string(yal, Alignment::First);
            plot = yal.merge_vertically(&plot, Alignment::First);
        }

        if let Some(t) = &self.title {
            let title = draw_title(t, self.big_title);
            plot = title.merge_vertically(&plot, Alignment::Center);
        }

        plot = plot.add_padding(self.paddings);

        plot.to_string()
    }

    fn draw_2d_graph(&self) -> String {
        let (
            data, x_labels, y_labels
        ) = self.data.unwrap_2d();
        let mut plot = plot_2d(&data, self.plot_width, self.plot_height);
        plot = plot.add_border([false, true, true, false]);

        let x_labels = draw_x_labels(
            &x_labels.iter().map(
                |s| (
                    match s { Some(s) => s.to_string(), _ => String::new() },
                    ()
                )
            ).collect(),
            self.plot_width
        );
        plot = plot.merge_vertically(&x_labels, Alignment::Last);

        let y_labels = draw_y_labels_2d_plot(y_labels);
        plot = y_labels.merge_horizontally(&plot, Alignment::First);

        if let Some(xal) = &self.x_axis_label {
            let mut xal = Lines::from_string(xal, Alignment::First);
            xal = xal.add_padding([self.plot_height, 0, 0, 0]);
            plot = plot.merge_horizontally(&xal, Alignment::First);
        }

        if let Some(yal) = &self.y_axis_label {
            let yal = Lines::from_string(yal, Alignment::First);
            plot = yal.merge_vertically(&plot, Alignment::First);
        }

        if let Some(t) = &self.title {
            let title = draw_title(t, self.big_title);
            plot = title.merge_vertically(&plot, Alignment::Center);
        }

        plot = plot.add_padding(self.paddings);

        plot.to_string()
    }

}

fn pick_meaningful_values(data: &Vec<(String, Ratio)>, width: usize) -> Vec<(String, Ratio)> {
    // a graph with odd-sized width is not supported because of this line
    let half_width = width / 2;

    let mut last_ind = 0;
    let mut result = Vec::with_capacity(width);

    for i in 0..half_width {
        let curr_ind = (i + 1) * data.len() / half_width;
        let mut min_ind = 0;
        let mut min_val = &data[last_ind].1;
        let mut max_ind = 0;
        let mut max_val = &data[last_ind].1;

        for (ind, (_, val)) in data[last_ind..curr_ind].iter().enumerate() {

            if val.gt_rat(&max_val) {
                max_ind = ind;
                max_val = val;
            }

            else if val.lt_rat(&min_val) {
                min_ind = ind;
                min_val = val;
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

fn get_where_to_skip(mut data: Vec<(String, Ratio)>) -> (Ratio, Ratio, Ratio, Ratio, (usize, usize)) {
    let mut curr_max_diff = Ratio::zero();
    let mut curr_max_diff_ind = 0;
    data.sort_unstable_by_key(|(_, n)| n.clone());

    for i in 0..(data.len() - 1) {
        let curr_diff = data[i + 1].1.sub_rat(&data[i].1);

        if curr_diff.gt_rat(&curr_max_diff) {
            curr_max_diff = curr_diff;
            curr_max_diff_ind = i;
        }

    }

    let mut padding1 = data[curr_max_diff_ind].1.sub_rat(&data[0].1).div_i32(16);
    let mut padding2 = data[data.len() - 1].1.sub_rat(&data[curr_max_diff_ind + 1].1).div_i32(16);

    if padding1.is_zero() {
        padding1 = data[curr_max_diff_ind + 1].1.sub_rat(&data[curr_max_diff_ind].1).div_i32(16);
    }

    if padding2.is_zero() {
        padding2 = data[curr_max_diff_ind + 1].1.sub_rat(&data[curr_max_diff_ind].1).div_i32(16);
    }

    let mut ratio_of_subgraphs = (3, 3);
    let values_below_skip_range = data[0..(curr_max_diff_ind + 1)].iter().map(|(_, n)| n).collect::<HashSet<&Ratio>>();
    let values_above_skip_range = data[(curr_max_diff_ind + 1)..].iter().map(|(_, n)| n).collect::<HashSet<&Ratio>>();

    if values_below_skip_range.len() * 2 > values_above_skip_range.len() * 3 {
        ratio_of_subgraphs = (4, 2);
    }

    else if values_above_skip_range.len() * 2 > values_below_skip_range.len() * 3 {
        ratio_of_subgraphs = (2, 4);
    }

    (
        data[0].1.sub_rat(&padding1),
        data[curr_max_diff_ind].1.add_rat(&padding1),
        data[curr_max_diff_ind + 1].1.sub_rat(&padding2),
        data[data.len() - 1].1.add_rat(&padding2),
        ratio_of_subgraphs
    )
}

fn unwrap_y_min_max(self_y_min: &Option<Ratio>, self_y_max: &Option<Ratio>, data_min: &Ratio, data_max: &Ratio) -> (Ratio, Ratio) {
    match (&self_y_min, &self_y_max) {
        (Some(n), Some(m)) => (n.clone(), m.clone()),
        (Some(n), None) => if n.lt_rat(&data_max) {
            (n.clone(), data_max.clone())
        } else {
            (n.clone(), n.add_i32(1))
        },
        (None, Some(n)) => if n.gt_rat(&data_min) {
            (data_min.clone(), n.clone())
        } else {
            (n.sub_i32(1), n.clone())
        },
        (None, None) => (data_min.clone(), data_max.clone())
    }
}

fn get_min_max_diff(v: &Vec<(String, Ratio)>, height: usize) -> (Ratio, Ratio, Ratio) {  // (y_min, y_max, max_diff)

    if v.len() == 0 {
        return (Ratio::zero(), Ratio::one(), Ratio::zero());
    }

    let mut data = v.iter().map(|(_, n)| n.clone()).collect::<Vec<Ratio>>();
    data.sort_unstable();

    let curr_min = &data[0];
    let curr_max = &data[data.len() - 1];
    let mut max_diff = Ratio::zero();

    for i in 0..(data.len() - 1) {
        let diff = data[i + 1].sub_rat(&data[i]);

        if diff.gt_rat(&max_diff) {
            max_diff = diff;
        }

    }

    let mut diff = curr_max.sub_rat(curr_min).div_i32(16);

    if diff.is_zero() {
        diff = Ratio::from_i32(height as i32).div_i32(4);
    }

    let min = curr_min.sub_rat(&diff);
    let max = curr_max.add_rat(&diff);

    (min, max, max_diff)
}

fn draw_title(title: &str, big_title: bool) -> Lines {

    if big_title {
        Lines::from_string(&asciibox::render_string(title, asciibox::RenderOption::default()), Alignment::First)
    }

    else {
        Lines::from_string(title, Alignment::Center)
    }

}

// no axis
fn draw_y_labels_2d_plot(y_labels: &Vec<Option<String>>) -> Lines {
    Lines::from_string(&y_labels.iter().map(
        |s| match s {
            Some(s) => s.replace("\n", " "),
            _ => String::new()
        }
    ).collect::<Vec<String>>().join("\n"), Alignment::Last)
}

// no axis
fn draw_y_labels_1d_plot(y_min: &Ratio, y_max: &Ratio, height: usize, interval: usize) -> Lines {
    let mut labels = Vec::with_capacity(height);
    let y_diff = y_max.sub_rat(y_min);
    let mut curr_max_width = 0;

    for y in 0..height {

        if interval > 1 && y % interval != 0 {
            labels.push(String::new());
            continue;
        }

        let curr_y = y_max.sub_rat(&y_diff.mul_i32(y as i32).div_i32(height as i32));
        let curr_label = format_ratio(&curr_y);

        if curr_label.len() > curr_max_width {
            curr_max_width = curr_label.len();
        }

        labels.push(curr_label);
    }

    Lines::from_string(&labels.join("\n"), Alignment::Last)
}

// no axis
fn draw_x_labels<T>(data: &Vec<(String, T)>, width: usize) -> Lines {
    let mut result = Lines::new(width, 2);

    let mut first_line_filled = 0;
    let mut second_line_filled = 0;
    let mut on_first_line = false;
    let mut last_ind = usize::MAX;

    for x in 0..width {
        let data_ind = x * data.len() / width;

        if on_first_line && x < first_line_filled || !on_first_line && x < second_line_filled || data_ind == last_ind {
            continue;
        }

        let curr_label = &data[data_ind].0;
        let y_ind = on_first_line as usize;

        if curr_label.len() + x >= width {
            on_first_line = !on_first_line;
            continue;
        }

        for (lab_ind, c) in curr_label.chars().enumerate() {
            let c = if c == '\n' { 32 } else { c as u16 };

            result.set(x + lab_ind, y_ind, c);
        }

        last_ind = data_ind;

        if on_first_line {
            first_line_filled = x + curr_label.len() + 2;
        }

        else {
            second_line_filled = x + curr_label.len() + 2;
        }

        on_first_line = !on_first_line;
    }

    result
}

// no axis, no labels, only plots
fn plot_2d(data: &Vec<(usize, usize, u16)>, width: usize, height: usize) -> Lines {
    let mut result = Lines::new(width, height);

    for (x, y, c) in data.iter() {

        if *c == '\n' as u16 {
            result.set(*x, *y, 32);
        }

        else {
            result.set(*x, *y, *c);
        }

    }

    result
}

// no axis, no labels, only plots
fn plot_1d(data: &Vec<(String, Ratio)>, width: usize, height: usize, y_min: &Ratio, y_max: &Ratio, no_overflow_char: bool) -> Lines {
    let mut result = Lines::new(width, height);
    let y_diff = y_max.sub_rat(y_min);

    for x in 0..width {
        let data_ind = x * data.len() / width;
        let data_val = &data[data_ind].1;
        // truncate(((y_max - data_val) / y_diff * height * 2).max(0))
        let mut overflow = false;
        let mut y_start = match y_max.sub_rat(data_val).div_rat(&y_diff).mul_i32(height as i32).mul_i32(4).truncate_bi().to_i32() {
            Ok(n) if n < 0 => {
                overflow = true;
                0
            },
            Ok(n) => n as usize,
            Err(_) => usize::MAX,
        };

        let block_type = y_start % 4;
        y_start /= 4;

        if y_start + 1 > height {
            continue;
        }

        for y in y_start..height {
            result.set(x, y, '█' as u16);
        }

        if overflow && !no_overflow_char {
            result.set(x, 0, '^' as u16);
        }

        else {
            result.set(x, y_start, [
                '█' as u16,
                '▆' as u16,
                '▄' as u16,
                '▂' as u16
            ][block_type])
        }

    }

    result
}

use std::fmt;

impl fmt::Display for Graph {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.draw())
    }

}