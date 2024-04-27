use crate::Graph;
use hmath::Ratio;

impl Graph {
    /// The json must be an object or an array.
    ///
    /// If it's an object, these keys are allowed:
    ///
    /// - 1d_data: Array[Number]
    /// - 1d_labeled_data: Array[[String, Number]]
    /// - y_min: Number
    /// - y_max: Number
    /// - y_range: [Number, Number]
    /// - pretty_y: Number
    /// - plot_width: Integer
    /// - plot_height: Integer
    /// - x_label_margin: Integer
    /// - y_label_margin: Integer
    /// - block_width: Integer
    /// - paddings: [Integer, Integer, Integer, Integer]
    /// - title: String
    /// - x_axis_label: String
    /// - y_axis_label: String
    /// - big_title: Bool
    ///
    /// It checks types except when it expects `Number`. For `Number`s, it tries to be as generous as possible.
    /// It even tries to parse strings into numbers. When it cannot parse a number, it just interprets that as 0.
    ///
    /// If it's an array, it interprets the array as `1d_data`.
    pub fn from_json(json_str: &str) -> Result<Self, ()> {  // TODO: error type
        let parsed = if let Ok(parsed) = json::parse(json_str) { parsed } else { return Err(()); };
        let mut result = Graph::default();

        if parsed.is_object() {
            for (key, value) in parsed.entries() {
                match key {
                    "1d_data" => {
                        if value.is_array() {
                            result.set_1d_data(&value.members().map(
                                |n| json_to_ratio(n)
                            ).collect::<Vec<_>>());
                        }

                        else {
                            return Err(());
                        }
                    },
                    "y_min" => {
                        result.set_y_min(json_to_ratio(value));
                    },
                    "y_max" => {
                        result.set_y_max(json_to_ratio(value));
                    },
                    "y_range" => {
                        let values = value.members().collect::<Vec<_>>();

                        if values.len() == 2 {
                            result.set_y_range(json_to_ratio(&values[0]), json_to_ratio(&values[1]));
                        }

                        else {
                            return Err(());
                        }
                    },
                    "pretty_y" => {
                        result.set_pretty_y(json_to_ratio(value));
                    },
                    // 1, u64 as usize would fail in some old machines, but u32 as usize would never fail
                    // 2, why would someone draw a graph with more than 4 billion bars?
                    "plot_width" => match value.as_u32() {
                        Some(n) => {
                            result.set_plot_width(n as usize);
                        },
                        _ => {
                            return Err(());
                        },
                    },
                    "plot_height" => match value.as_u32() {
                        Some(n) => {
                            result.set_plot_height(n as usize);
                        },
                        _ => {
                            return Err(());
                        },
                    },
                    "x_label_margin" => match value.as_u32() {
                        Some(n) => {
                            result.set_x_label_margin(n as usize);
                        },
                        _ => {
                            return Err(());
                        },
                    },
                    "y_label_margin" => match value.as_u32() {
                        Some(n) => {
                            result.set_y_label_margin(n as usize);
                        },
                        _ => {
                            return Err(());
                        },
                    },
                    "block_width" => match value.as_u32() {
                        Some(n) => {
                            result.set_block_width(n as usize);
                        },
                        _ => {
                            return Err(());
                        },
                    },
                    "paddings" => if value.is_array() {
                        let mut paddings = vec![];

                        for n in value.members() {
                            match n.as_u32() {
                                Some(n) => {
                                    paddings.push(n as usize);
                                },
                                _ => {
                                    return Err(());
                                },
                            }
                        }

                        if paddings.len() != 4 {
                            return Err(());
                        }

                        result.set_paddings([
                            paddings[0],
                            paddings[1],
                            paddings[2],
                            paddings[3],
                        ]);
                    } else {
                        return Err(());
                    },
                    "title" => match value.as_str() {
                        Some(t) => {
                            result.set_title(t);
                        },
                        _ => {
                            return Err(());
                        },
                    },
                    "x_axis_label" => match value.as_str() {
                        Some(t) => {
                            result.set_x_axis_label(t);
                        },
                        _ => {
                            return Err(());
                        },
                    },
                    "y_axis_label" => match value.as_str() {
                        Some(t) => {
                            result.set_y_axis_label(t);
                        },
                        _ => {
                            return Err(());
                        },
                    },
                    "big_title" => match value.as_bool() {
                        Some(b) => {
                            result.set_big_title(b);
                        },
                        _ => {
                            return Err(());
                        },
                    },
                    _ => {
                        return Err(());
                    },
                }
            }

            Ok(result)
        }

        else if parsed.is_array() {
            result.set_1d_data(&parsed.members().map(
                |n| json_to_ratio(n)
            ).collect::<Vec<_>>());

            Ok(result)
        }

        else {
            Err(())
        }
    }
}

/// It returns 0 if `n` is not parse-able.
fn json_to_ratio(n: &json::JsonValue) -> Ratio {
    if let Some(n) = n.as_number() {
        // Ratio::from_string is lossless
        let (positive, mantissa, exponent) = n.as_parts();

        Ratio::from_string(&format!(
            "{}{mantissa}e{exponent}",
            if positive { "" } else { "-" },
        )).unwrap_or_else(|_| Ratio::zero())
    }

    else if let Some(n) = n.as_str() {
        Ratio::from_string(n).unwrap_or_else(|_| Ratio::zero())
    }

    else {
        Ratio::zero()
    }
}
