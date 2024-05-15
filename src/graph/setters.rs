use crate::Graph;
use crate::color::{Color, ColorMode};
use crate::graph::GraphData;
use crate::interval::Interval;
use crate::skip_value::SkipValue;
use hmath::Ratio;

impl Graph {

    /// It plots characters on a 2-dimensional plane. The type of `data` is `Vec<(x, y, character)>`.
    /// The sizes of `x_labels` and `y_labels` must match `self.plot_width` and `self.plot_height`.
    /// If `self.plot_width` and `self.plot_height` are already set, it updates them.
    pub fn set_2d_data(&mut self, data: &[(usize, usize, char)], x_labels: &[Option<String>], y_labels: &[Option<String>]) -> &mut Self {
        self.plot_width = x_labels.len();
        self.plot_height = y_labels.len();

        self.data = GraphData::Data2D {
            data: data.iter().map(|(x, y, c)| (
                *x, *y, *c as u16
            )).collect(),
            x_labels: x_labels.to_vec(),
            y_labels: y_labels.to_vec(),
        };

        self.adjust_all_labeled_intervals();

        self
    }

    /// It's like `set_2d_data`, but has twice higher resolution. You cannot set characters, you can only plot dots.
    /// That means the width and the height of `data` has to be twice of that of `x_labels` and `y_labels`.
    pub fn set_2d_data_high_resolution(&mut self, data: &[(usize, usize)], x_labels: &[Option<String>], y_labels: &[Option<String>]) -> &mut Self {
        self.plot_width = x_labels.len();
        self.plot_height = y_labels.len();
        let mut grid = vec![vec![false; self.plot_width * 2]; self.plot_height * 2];

        for (x, y) in data.iter() {
            grid[*y][*x] = true;
        }

        // the new capacity might be bigger than `data.len() / 2`. it's just a rough optimization
        let mut data = Vec::with_capacity(data.len() / 2);

        for x in 0..self.plot_width {
            for y in 0..self.plot_height {
                match (
                    grid[y * 2][x * 2], grid[y * 2][x * 2 + 1],
                    grid[y * 2 + 1][x * 2], grid[y * 2 + 1][x * 2 + 1],
                ) {
                    (
                        true, true,
                        true, true,
                    ) => {
                        data.push((x, y, '█' as u16));
                    },
                    (
                        true, true,
                        true, false,
                    ) => {
                        data.push((x, y, '▛' as u16));
                    },
                    (
                        true, true,
                        false, true,
                    ) => {
                        data.push((x, y, '▜' as u16));
                    },
                    (
                        true, true,
                        false, false,
                    ) => {
                        data.push((x, y, '▀' as u16));
                    },
                    (
                        true, false,
                        true, true,
                    ) => {
                        data.push((x, y, '▙' as u16));
                    },
                    (
                        true, false,
                        true, false,
                    ) => {
                        data.push((x, y, '▌' as u16));
                    },
                    (
                        true, false,
                        false, true,
                    ) => {
                        data.push((x, y, '▚' as u16));
                    },
                    (
                        true, false,
                        false, false,
                    ) => {
                        data.push((x, y, '▘' as u16));
                    },
                    (
                        false, true,
                        true, true,
                    ) => {
                        data.push((x, y, '▟' as u16));
                    },
                    (
                        false, true,
                        true, false,
                    ) => {
                        data.push((x, y, '▞' as u16));
                    },
                    (
                        false, true,
                        false, true,
                    ) => {
                        data.push((x, y, '▐' as u16));
                    },
                    (
                        false, true,
                        false, false,
                    ) => {
                        data.push((x, y, '▝' as u16));
                    },
                    (
                        false, false,
                        true, true,
                    ) => {
                        data.push((x, y, '▄' as u16));
                    },
                    (
                        false, false,
                        true, false,
                    ) => {
                        data.push((x, y, '▖' as u16));
                    },
                    (
                        false, false,
                        false, true,
                    ) => {
                        data.push((x, y, '▗' as u16));
                    },
                    (
                        false, false,
                        false, false,
                    ) => { },
                }
            }
        }

        self.data = GraphData::Data2D {
            data,
            x_labels: x_labels.to_vec(),
            y_labels: y_labels.to_vec(),
        };

        self.adjust_all_labeled_intervals();

        self
    }

    /// `T` can be any number type, including f32 and f64. NaN is converted to 0, -Inf is converted to f32::MIN and Inf to f32::MAX (or f64).\
    /// The data is labeled using indices (from 0).
    pub fn set_1d_data<T: TryInto<Ratio> + Clone>(&mut self, data: &[T]) -> &mut Self {

        // in order for `String`s to be `T`, it has to clone n inside the `map` method.
        let data: Vec<(String, Ratio)> = data.iter().enumerate().map(|(i, n)| (i.to_string(), n.clone().try_into().unwrap_or(Ratio::zero()))).collect();

        self.data = GraphData::Data1D(data);
        self.adjust_all_labeled_intervals();

        self
    }

    /// `T` can be any number type, including f32 and f64. NaN is converted to 0, -Inf is converted to f32::MIN and Inf to f32::MAX (or f64).\
    pub fn set_1d_labeled_data<T: TryInto<Ratio> + Clone>(&mut self, data: &[(String, T)]) -> &mut Self {

        // in order for `String`s to be `T`, it has to clone n inside the `map` method.
        let data: Vec<(String, Ratio)> = data.iter().map(|(label, n)| (label.to_string(), n.clone().try_into().unwrap_or(Ratio::zero()))).collect();

        self.data = GraphData::Data1D(data);
        self.adjust_all_labeled_intervals();

        self
    }

    pub fn set_y_min<T: TryInto<Ratio>>(&mut self, y_min: T) -> &mut Self {
        self.y_min = Some(y_min.try_into().unwrap_or(Ratio::zero()));

        self
    }

    pub fn set_y_max<T: TryInto<Ratio>>(&mut self, y_max: T) -> &mut Self {
        self.y_max = Some(y_max.try_into().unwrap_or(Ratio::zero()));

        self
    }

    pub fn set_y_range<T: TryInto<Ratio>, U: TryInto<Ratio>>(&mut self, y_min: T, y_max: U) -> &mut Self {
        self.y_min = Some(y_min.try_into().unwrap_or(Ratio::zero()));
        self.y_max = Some(y_max.try_into().unwrap_or(Ratio::zero()));

        self
    }

    /// If the engine automatically sets the range of y axis, the value would be ugly.
    /// For example, let's say (y_min, y_max) = (-0.1, 499.9). In this case, if you set `set_pretty_y(5)`,
    /// it makes all the y_labels multiple of 5.
    pub fn set_pretty_y<T: TryInto<Ratio>>(&mut self, y: T) -> &mut Self {
        self.pretty_y = Some(y.try_into().unwrap_or(Ratio::zero()));

        self
    }

    pub fn set_plot_width(&mut self, plot_width: usize) -> &mut Self {
        self.plot_width = plot_width;
        self.adjust_all_labeled_intervals();

        self
    }

    pub fn set_plot_height(&mut self, plot_height: usize) -> &mut Self {
        self.plot_height = plot_height;

        self
    }

    pub fn set_x_label_margin(&mut self, x_label_margin: usize) -> &mut Self {
        self.x_label_margin = x_label_margin;

        self
    }

    pub fn set_y_label_margin(&mut self, y_label_margin: usize) -> &mut Self {
        self.y_label_margin = y_label_margin;

        self
    }

    /// It sets `self.plot_width = self.data.len() * block_width`. If the `plot_width` is already set, it overrides it.
    /// It only works with 1-dimensional data.
    pub fn set_block_width(&mut self, block_width: usize) -> &mut Self {
        self.block_width = Some(block_width);
        self.adjust_all_labeled_intervals();

        self
    }

    pub fn set_padding_top(&mut self, padding_top: usize) -> &mut Self {
        self.paddings[0] = padding_top;

        self
    }

    pub fn set_padding_bottom(&mut self, padding_bottom: usize) -> &mut Self {
        self.paddings[1] = padding_bottom;

        self
    }

    pub fn set_padding_left(&mut self, padding_left: usize) -> &mut Self {
        self.paddings[2] = padding_left;

        self
    }

    pub fn set_padding_right(&mut self, padding_right: usize) -> &mut Self {
        self.paddings[3] = padding_right;

        self
    }

    /// top, bottom, left, right
    pub fn set_paddings(&mut self, paddings: [usize; 4]) -> &mut Self {
        self.paddings = paddings;

        self
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());

        self
    }

    pub fn set_title_color(&mut self, title_color: Option<Color>) -> &mut Self {
        self.title_color = title_color;

        self
    }

    pub fn set_x_axis_label(&mut self, x_axis_label: &str) -> &mut Self {
        self.x_axis_label = Some(x_axis_label.to_string());

        self
    }

    pub fn set_y_axis_label(&mut self, y_axis_label: &str) -> &mut Self {
        self.y_axis_label = Some(y_axis_label.to_string());

        self
    }

    pub fn set_big_title(&mut self, big_title: bool) -> &mut Self {
        self.big_title = big_title;

        self
    }

    /// It does not plot data between this range. It's applied only when the plot height is greater than 18.
    pub fn set_skip_range(&mut self, skip_value: SkipValue) -> &mut Self {
        self.skip_value = skip_value;

        self
    }

    /// I need a better name for this.
    ///
    /// If the skip_range is included in this range, the skip_range is not set. It works only when its skip_value is `SkipValue::Automatic`.
    /// You can set open ends with `None` values.
    pub fn set_skip_skip_range<T: TryInto<Ratio> + Clone, U: TryInto<Ratio> + Clone>(&mut self, from: Option<T>, to: Option<U>) -> &mut Self {
        self.skip_skip_range = Some((
            from.map(|n| n.clone().try_into().unwrap_or(Ratio::zero())),
            to.map(|n| n.clone().try_into().unwrap_or(Ratio::zero())),
        ));

        self
    }

    /// See `README.md` to see how it works. `start` and `end` are both inclusive.
    /// `start` and `end` corresponds to the index of `self.data`. That means if the interval is (0, 32),
    /// it's `self.data[0]` ~ `self.data[32]`. The actual number of the characters used depends on the size of the graph.
    pub fn add_labeled_interval(&mut self, start: i32, end: i32, label: String) -> &mut Self {
        let mut interval = Interval::new(start, end, label);

        if !self.data.is_empty() {
            interval.adjust_coordinate(self.get_actual_plot_width(), self.data.len());
        }

        self.labeled_intervals.push(interval);

        self
    }

    pub fn set_primary_color(&mut self, color: Option<Color>) -> &mut Self {
        self.primary_color = color;

        self
    }

    pub fn set_color_mode(&mut self, color_mode: ColorMode) -> &mut Self {
        self.color_mode = color_mode;

        self
    }
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            plot_width: 80,
            plot_height: 28,
            block_width: None,
            data: GraphData::None,
            x_label_margin: 2,
            y_label_margin: 2,
            paddings: [0; 4],
            y_max: None,
            y_min: None,
            pretty_y: Some(Ratio::try_from(0.5).unwrap()),
            title: None,
            title_color: None,
            skip_value: SkipValue::Automatic,
            skip_skip_range: None,
            x_axis_label: None,
            y_axis_label: None,
            labeled_intervals: vec![],
            big_title: false,
            color_mode: ColorMode::None,
            primary_color: None,
        }
    }
}
