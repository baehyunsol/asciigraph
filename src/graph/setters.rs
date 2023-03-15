use super::{Graph, GraphData, SkipValue};
use crate::utils::{sns_int, fractional_number};

impl Graph {

    /// default: false
    pub fn quiet(&mut self, quiet: bool) -> &mut Self {
        self.quiet = quiet;
        self
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

    /// only for 1d graphs
    /// default: '█'
    pub fn set_full_block_character(&mut self, full_block_character: u16) -> &mut Self {
        self.full_block_character = full_block_character;
        self
    }

    /// only for 1d graphs
    /// default: '▄'
    pub fn set_half_block_character(&mut self, half_block_character: u16) -> &mut Self {
        self.half_block_character = half_block_character;
        self
    }

    /// only for 1d graphs
    /// default: '^'
    pub fn set_overflow_character(&mut self, overflow_character: u16) -> &mut Self {
        self.overflow_character = overflow_character;
        self
    }

    pub fn set_plot_width(&mut self, plot_width: usize) -> &mut Self {
        self.plot_width = plot_width;

        self
    }

    pub fn set_plot_height(&mut self, plot_height: usize) -> &mut Self {
        self.plot_height = plot_height;

        self
    }

    /// It works only with 1d data.
    /// It makes sense when `self.data.len()` is small enough.
    /// If both `self.plot_width` and `self.block_width` are set, `plot_width` has no effect.
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

    pub fn set_1d_data(&mut self, data: Vec<i64>) -> &mut Self {

        match self.block_width {
            Some(n) => {
                self.plot_width = data.len() * n;
            }
            _ => {}
        }

        self.data = GraphData::OneDimensional(
            data.into_iter().enumerate().map(
                |(ind, val)| (ind.to_string(), val)
            ).collect()
        );
        self.y_label_formatter = sns_int;
        self
    }

    pub fn set_1d_labeled_data(&mut self, data: Vec<(String, i64)>) -> &mut Self {

        match self.block_width {
            Some(n) => {
                self.plot_width = data.len() * n;
            }
            _ => {}
        }

        self.data = GraphData::OneDimensional(data);
        self.y_label_formatter = sns_int;
        self
    }

    /// It uses fixed point numbers to represent real numbers. It uses 12 bits for the fractional parts.
    /// If you want another representation, you have to implement by yourself.
    pub fn set_1d_data_float(&mut self, data: Vec<f64>) -> &mut Self {

        match self.block_width {
            Some(n) => {
                self.plot_width = data.len() * n;
            }
            _ => {}
        }

        self.data = GraphData::OneDimensional(
            data.into_iter().enumerate().map(
                |(ind, val)| (ind.to_string(), (val * 16384.0) as i64)
            ).collect()
        );
        self.y_label_formatter = fractional_number;
        self
    }

    pub fn set_1d_labeled_data_float(&mut self, data: Vec<(String, f64)>) -> &mut Self {

        match self.block_width {
            Some(n) => {
                self.plot_width = data.len() * n;
            }
            _ => {}
        }

        self.data = GraphData::OneDimensional(data.into_iter().map(
            |(s, n)| (s, (n * 16384.0) as i64)
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
        self.y_label_formatter = sns_int;
        self
    }

    pub fn set_y_max(&mut self, y_max: i64) -> &mut Self {
        self.y_max = Some(y_max);
        self.y_label_formatter = sns_int;
        self
    }

    pub fn set_y_min_float(&mut self, y_min: f64) -> &mut Self {
        self.y_min = Some((y_min * 16384.0) as i64);
        self.y_label_formatter = fractional_number;
        self
    }

    pub fn set_y_max_float(&mut self, y_max: f64) -> &mut Self {
        self.y_max = Some((y_max * 16384.0) as i64);
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

    /// it does not plot data between this range.
    /// it's applied only when the height of the plot is greater than 18
    pub fn set_skip_values(&mut self, skip_value: SkipValue) -> &mut Self {
        self.skip_value = skip_value;
        self
    }

    pub fn set_skip_values_float(&mut self, from: f64, to: f64) -> &mut Self {
        self.skip_value = SkipValue::Range((from * (16384.0)) as i64, (to * (16384.0)) as i64);
        self
    }

}