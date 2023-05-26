use crate::Graph;
use crate::graph::GraphData;
use crate::convert::IntoRatio;
use crate::skip_value::SkipValue;
use hmath::Ratio;

impl Graph {

    /// It plots characters on a 2-dimensional plane. The type of `data` is `Vec<(x, y, character)>`.
    /// The sizes of `x_labels` and `y_labels` must match `self.plot_width` and `self.plot_height`.
    /// If `self.plot_width` and `self.plot_height` are already set, it updates them.
    pub fn set_2d_data(&mut self, data: &Vec<(usize, usize, char)>, x_labels: &Vec<Option<String>>, y_labels: &Vec<Option<String>>) -> &mut Self {
        self.plot_width = x_labels.len();
        self.plot_height = y_labels.len();

        self.data = GraphData::Data2D {
            data: data.iter().map(|(x, y, c)| (
                *x, *y, *c as u16
            )).collect(),
            x_labels: x_labels.clone(),
            y_labels: y_labels.clone(),
        };

        self
    }

    /// `T` can be any number type, including f32 and f64. NaN is converted to 0, -Inf is converted to f32::MIN and Inf to f32::MAX (or f64).\
    /// The data is labeled using indices (from 0).
    pub fn set_1d_data<T: IntoRatio>(&mut self, data: &Vec<T>) -> &mut Self {
        let data: Vec<(String, Ratio)> = data.iter().enumerate().map(|(i, n)| (i.to_string(), n.into_ratio())).collect();

        self.data = GraphData::Data1D(data);
        self
    }

    /// `T` can be any number type, including f32 and f64. NaN is converted to 0, -Inf is converted to f32::MIN and Inf to f32::MAX (or f64).\
    pub fn set_1d_labeled_data<T: IntoRatio>(&mut self, data: &Vec<(String, T)>) -> &mut Self {
        let data: Vec<(String, Ratio)> = data.iter().map(|(label, n)| (label.to_string(), n.into_ratio())).collect();

        self.data = GraphData::Data1D(data);
        self
    }

    pub fn set_y_min<T: IntoRatio>(&mut self, y_min: T) -> &mut Self {
        self.y_min = Some(y_min.into_ratio());

        self
    }

    pub fn set_y_max<T: IntoRatio>(&mut self, y_max: T) -> &mut Self {
        self.y_max = Some(y_max.into_ratio());

        self
    }

    pub fn set_y_range<T: IntoRatio, U: IntoRatio>(&mut self, y_min: T, y_max: U) -> &mut Self {
        self.y_min = Some(y_min.into_ratio());
        self.y_max = Some(y_max.into_ratio());

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

    pub fn set_y_label_interval(&mut self, y_label_interval: usize) -> &mut Self {
        self.y_label_interval = y_label_interval;

        self
    }

    /// It sets `self.plot_width = self.data.len() * block_width`. If the `plot_width` is already set, it overrides it.
    /// It only works with 1-dimensional data.
    pub fn set_block_width(&mut self, block_width: usize) -> &mut Self {
        self.block_width = Some(block_width);

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

}

impl Default for Graph {

    fn default() -> Self {
        Graph {
            plot_width: 80,
            plot_height: 28,
            block_width: None,
            data: GraphData::None,
            y_label_interval: 2,
            paddings: [0; 4],
            y_max: None,
            y_min: None,
            title: None,
            skip_value: SkipValue::Automatic,
            x_axis_label: None,
            y_axis_label: None,
            big_title: false
        }
    }

}