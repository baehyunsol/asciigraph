use crate::alignment::Alignment;
use crate::color::{Color, ColorMode};
use crate::format::format_ratio;
use crate::interval::{Interval, draw_labeled_intervals};
use crate::lines::Lines;
use crate::skip_value::SkipValue;
use hmath::Ratio;
use std::collections::HashSet;

mod merge;
mod setters;

pub use merge::*;

#[derive(Clone)]
pub struct Graph {
    data: GraphData,

    title: Option<String>,
    big_title: bool,
    title_color: Option<Color>,

    plot_width: usize,
    plot_height: usize,

    block_width: Option<usize>,

    x_label_margin: usize,
    y_label_margin: usize,

    x_axis_label: Option<String>,
    y_axis_label: Option<String>,

    labeled_intervals: Vec<Interval>,

    y_min: Option<Ratio>,
    y_max: Option<Ratio>,

    pretty_y: Option<Ratio>,

    skip_value: SkipValue,

    paddings: [usize; 4],

    color_mode: ColorMode,
    primary_color: Option<Color>,
}

#[derive(Debug, PartialEq, Clone)]
enum GraphData {
    Data1D (Vec<(String, Ratio)>),
    Data2D {
        data: Vec<(usize, usize, u16)>,
        x_labels: Vec<Option<String>>,
        y_labels: Vec<Option<String>>,
    },
    None,
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

    pub fn len(&self) -> usize {
        match self {
            GraphData::Data1D(data) => data.len(),
            GraphData::Data2D { data, .. } => data.len(),
            GraphData::None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            GraphData::Data1D(data) => data.is_empty(),
            GraphData::Data2D { data, .. } => data.is_empty(),
            GraphData::None => true,
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

    /// It panics if it's not well-configured. If you're not sure, call `.is_valid` before calling this method
    pub fn draw(&self) -> String {
        match &self.data {
            GraphData::Data1D(_) => self.draw_1d_graph(),
            GraphData::Data2D { .. } => self.draw_2d_graph(),
            GraphData::None => panic!("Cannot draw a graph without any data"),
        }
    }

    pub(crate) fn get_actual_plot_width(&self) -> usize {
        match &self.data {
            GraphData::Data1D(_) => match self.block_width {
                Some(w) => w * self.data.len(),
                _ => self.plot_width,
            },
            _ => self.plot_width,
        }
    }

    /// 1. `self.data` must be set and for 1-D data, it must not be empty.
    /// 2. If `self.y_min` and `self.y_max` are set, `self.y_max` has to be greater than `self.y_min`.
    /// 3. If you're using a 2-dimensional data, `data`, `x_labels` and `y_labels` must have the same dimension.
    /// 4. If there're labeled_intervals, their interval must be valid.
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
            },
            _ => false,
        } && {
            self.labeled_intervals.iter().all(|i| i.is_valid())
        } && {
            // TODO
            true
        }
    }

    fn draw_1d_graph(&self) -> String {
        let mut data = self.data.unwrap_1d().clone();

        let plot_width = self.get_actual_plot_width();

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

                    // respect self.y_min and self.y_max if they're explicitly set
                    if self.y_min.is_none() {
                        y_min = y_min_;
                    }

                    if self.y_max.is_none() {
                        y_max = y_max_;
                    }

                    // if the explicitly set y_min and y_max are not compatible with the skipped range, it doesn't skip
                    if y_min.lt_rat(&from) && to.lt_rat(&y_max) {
                        Some((from, to))
                    }

                    else {
                        None
                    }

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
            },
        };

        if let Some((from, to)) = &skip_range {
            if from.lt_rat(&y_min) || to.gt_rat(&y_max) {
                skip_range = None;
            }
        }

        let mut plot = match &skip_range {
            None => {
                let (y_min, y_max) = prettify_y_labels(
                    &y_min,
                    &y_max,
                    self.plot_height,
                    self.pretty_y.as_ref().map(|n| (self.y_min.is_none(), self.y_max.is_none(), n.clone()))
                );

                let mut plot = plot_1d(
                    &data,
                    plot_width,
                    self.plot_height,
                    &y_min,
                    &y_max,
                    false,  // no_overflow_char
                    self.primary_color.clone(),
                );
                plot = plot.add_border([false, true, true, false]);

                let y_labels = draw_y_labels_1d_plot(&y_min, &y_max, self.plot_height, self.y_label_margin);

                y_labels.merge_horizontally(&plot, Alignment::First)
            },
            Some((from, to)) => {
                let (mut height1, mut height2) = (
                    self.plot_height * ratio_of_subgraphs.0 / 6,
                    self.plot_height * ratio_of_subgraphs.1 / 6,
                );

                // it has to be height1 + height2 + 1 == self.plot_height
                // 1 is for the delimiter line
                if height1 > height2 {
                    height1 += self.plot_height - height1 - height2;
                    height2 -= 1;
                }

                else {
                    height2 += self.plot_height - height1 - height2;
                    height1 -= 1;
                }

                // if y_min, y_max, or skip_value is explicitly set by the user, it never touches them
                // otherwise it tries to adjust them for prettier y_labels
                let (plot1_y_min, plot1_y_max) = prettify_y_labels(
                    &y_min,
                    &from,
                    height1,
                    self.pretty_y.as_ref().map(|n| (self.y_min.is_none(), self.skip_value.is_automatic(), n.clone()))
                );

                let mut plot1 = plot_1d(
                    &data,
                    plot_width,
                    height1,
                    &plot1_y_min,
                    &plot1_y_max,
                    true,  // no_overflow_char
                    self.primary_color.clone(),
                );
                plot1 = plot1.add_border([false, true, true, false]);

                let (plot2_y_min, plot2_y_max) = prettify_y_labels(
                    &to,
                    &y_max,
                    height2,
                    self.pretty_y.as_ref().map(|n| (self.skip_value.is_automatic(), self.y_max.is_none(), n.clone()))
                );

                let mut plot2 = plot_1d(
                    &data,
                    plot_width,
                    height2,
                    &plot2_y_min,
                    &plot2_y_max,
                    false,  // no_overflow_char
                    self.primary_color.clone(),
                );
                plot2 = plot2.add_border([false, false, true, false]);

                let mut y_labels1 = draw_y_labels_1d_plot(&plot1_y_min, &plot1_y_max, height1, self.y_label_margin);
                let mut y_labels2 = draw_y_labels_1d_plot(&plot2_y_min, &plot2_y_max, height2, self.y_label_margin);

                if y_labels1.get_width() < y_labels2.get_width() {
                    y_labels1 = y_labels1.add_padding([0, 0, y_labels2.get_width() - y_labels1.get_width(), 0]);
                }

                else if y_labels2.get_width() < y_labels1.get_width() {
                    y_labels2 = y_labels2.add_padding([0, 0, y_labels1.get_width() - y_labels2.get_width(), 0]);
                }

                plot1 = y_labels1.merge_horizontally(&plot1, Alignment::First);
                plot2 = y_labels2.merge_horizontally(&plot2, Alignment::First);

                let mut horizontal_line = Lines::from_string(&"~".repeat(plot1.get_width()), Alignment::First, &ColorMode::None);
                horizontal_line.set_color_all(self.primary_color.clone());

                plot1 = horizontal_line.merge_vertically(&plot1, Alignment::First);

                plot2.merge_vertically(&plot1, Alignment::First)
            },
        };

        let x_labels = draw_x_labels(&data, plot_width, self.x_label_margin);
        plot = plot.merge_vertically(&x_labels, Alignment::Last);

        if !self.labeled_intervals.is_empty() {
            let arrows = draw_labeled_intervals(&self.labeled_intervals, plot_width);
            plot = plot.merge_vertically(&arrows, Alignment::Last);
        }

        if let Some(xal) = &self.x_axis_label {
            let mut xal = Lines::from_string(xal, Alignment::First, &ColorMode::None);
            xal = xal.add_padding([self.plot_height, 0, 0, 0]);
            plot = plot.merge_horizontally(&xal, Alignment::First);
        }

        if let Some(yal) = &self.y_axis_label {
            let yal = Lines::from_string(yal, Alignment::First, &ColorMode::None);
            plot = yal.merge_vertically(&plot, Alignment::First);
        }

        if let Some(t) = &self.title {
            let title = draw_title(t, self.big_title, self.title_color.clone());
            plot = title.merge_vertically(&plot, Alignment::Center);
        }

        plot = plot.add_padding(self.paddings);

        plot.to_string(&self.color_mode)
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
            self.plot_width,
            self.x_label_margin
        );
        plot = plot.merge_vertically(&x_labels, Alignment::Last);

        let y_labels = draw_y_labels_2d_plot(y_labels);
        plot = y_labels.merge_horizontally(&plot, Alignment::First);

        if let Some(xal) = &self.x_axis_label {
            let mut xal = Lines::from_string(xal, Alignment::First, &ColorMode::None);
            xal = xal.add_padding([self.plot_height, 0, 0, 0]);
            plot = plot.merge_horizontally(&xal, Alignment::First);
        }

        if let Some(yal) = &self.y_axis_label {
            let yal = Lines::from_string(yal, Alignment::First, &ColorMode::None);
            plot = yal.merge_vertically(&plot, Alignment::First);
        }

        if let Some(t) = &self.title {
            let title = draw_title(t, self.big_title, self.title_color.clone());
            plot = title.merge_vertically(&plot, Alignment::Center);
        }

        plot = plot.add_padding(self.paddings);

        plot.to_string(&self.color_mode)
    }

    fn adjust_all_labeled_intervals(&mut self) {
        let plot_width = self.get_actual_plot_width();
        let data_len = self.data.len();

        if !self.data.is_empty() {
            self.labeled_intervals.iter_mut().for_each(
                |i| i.adjust_coordinate(plot_width, data_len)
            );
        }
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
        ratio_of_subgraphs,
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
        (None, None) => (data_min.clone(), data_max.clone()),
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

fn draw_title(title: &str, big_title: bool, title_color: Option<Color>) -> Lines {
    let mut result = if big_title {
        Lines::from_string(&asciibox::render_string(title, asciibox::RenderOption::default()), Alignment::First, &ColorMode::None)
    }

    else {
        Lines::from_string(title, Alignment::Center, &ColorMode::None)
    };

    result.set_color_all(title_color);

    result
}

// no axis
fn draw_y_labels_2d_plot(y_labels: &Vec<Option<String>>) -> Lines {
    Lines::from_string(
        &y_labels.iter().map(
            |s| match s {
                Some(s) => s.replace("\n", " "),
                _ => String::new(),
            }
        ).collect::<Vec<String>>().join("\n"),
        Alignment::Last,
        &ColorMode::None,
    )
}

// no axis
fn draw_y_labels_1d_plot(y_min: &Ratio, y_max: &Ratio, height: usize, margin: usize) -> Lines {
    let mut labels = Vec::with_capacity(height);
    let y_diff = y_max.sub_rat(y_min);
    let y_label_step = y_diff.div_i32(height as i32);
    let mut curr_max_width = 0;

    for y in 0..height {
        if margin > 1 && y % margin != 0 {
            labels.push(String::new());
            continue;
        }

        let curr_y = y_max.sub_rat(&y_label_step.mul_i32(y as i32));
        let curr_label = format_ratio(&curr_y);

        if curr_label.len() > curr_max_width {
            curr_max_width = curr_label.len();
        }

        labels.push(curr_label);
    }

    Lines::from_string(&labels.join("\n"), Alignment::Last, &ColorMode::None)
}

// no axis
fn draw_x_labels<T>(data: &Vec<(String, T)>, width: usize, margin: usize) -> Lines {
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
            let c = if c == '\n' { ' ' as u16 } else { c as u16 };

            result.set(x + lab_ind, y_ind, c);
        }

        last_ind = data_ind;

        if on_first_line {
            first_line_filled = x + curr_label.len() + margin;
        }

        else {
            second_line_filled = x + curr_label.len() + margin;
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
fn plot_1d(data: &Vec<(String, Ratio)>, width: usize, height: usize, y_min: &Ratio, y_max: &Ratio, no_overflow_char: bool, overflow_char_color: Option<Color>) -> Lines {
    let mut result = Lines::new(width, height);
    let y_diff = y_max.sub_rat(&y_min);

    for x in 0..width {
        let data_ind = x * data.len() / width;
        let data_val = &data[data_ind].1;
        let mut overflow = false;

        // truncate(((y_max - data_val) / y_diff * height * 2).max(0))
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
            result.set_color(x, 0, overflow_char_color.clone());
        }

        else {
            result.set(x, y_start, [
                '█' as u16,
                '▆' as u16,
                '▄' as u16,
                '▂' as u16,
            ][block_type])
        }

    }

    result
}

// if y_min and y_max are (0, 499.8), the output would be very ugly
// it adjusts numbers in such cases
//
// it works when both y_min and y_max are movable
// -> in order to make all the labels pretty, both end(start) point and interval have to be modified
fn prettify_y_labels(old_y_min: &Ratio, old_y_max: &Ratio, height: usize, pretty_y_label_info: Option<(bool, bool, Ratio)>) -> (Ratio, Ratio) {
    if let Some((y_min_movable, y_max_movable, interval)) = pretty_y_label_info {
        if !y_max_movable || (!y_min_movable && !old_y_min.div_rat(&interval).is_integer()) {
            (old_y_min.clone(), old_y_max.clone())
        }

        else {
            let curr_interval = old_y_max.sub_rat(old_y_min).div_i32(height as i32);

            // curr_interval / interval =
            // 15/16 ~ 17/16   -> 1, 30/16 ~ 34/16   -> 2, 45/16 ~ 51/16  -> 3,
            // 60/16 ~ 68/16   -> 4, 75/16 ~ 85/16   -> 5, 90/16 ~ 102/16 -> 6,
            // 105/16 ~ 119/16 -> 7, 120/16 ~ 136/16 -> 8 ... okay from here
            let should_be_multiple_of_16 = curr_interval.div_rat(&interval).mul_i32(16).round_bi();

            if let Ok(n) = should_be_multiple_of_16.to_i32() {
                if n < 15 || (17 < n && n < 30)
                    || (34 < n && n < 45)
                    || (51 < n && n < 60)
                    || (68 < n && n < 75)
                    || (85 < n && n < 90)
                    || (102 < n && n < 105)
                {
                    return (old_y_min.clone(), old_y_max.clone());
                }

            }

            // round y_min to the closest multiple of `interval`
            // round (y_max - y_min) to the closest multiple of `interval` then add it to new `y_min`
            let new_y_min = old_y_min.div_rat(&interval).round().mul_rat(&interval);
            let y_diff = old_y_max.sub_rat(&new_y_min);
            let new_y_diff = y_diff.div_rat(&interval).div_i32(height as i32).round().mul_rat(&interval).mul_i32(height as i32);
            let new_y_max = new_y_min.add_rat(&new_y_diff);

            (new_y_min, new_y_max)
        }

    }

    else {
        (old_y_min.clone(), old_y_max.clone())
    }
}

use std::fmt;

impl fmt::Display for Graph {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.draw())
    }
}
