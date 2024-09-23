use crate::{
    Color,
    ColorMode,
    DefaultFormatter,
    Error,
    Graph,
    NumberFormatter,
    SkipValue,
};
use crate::error::{JsonType, get_type};
use hmath::Ratio;
use json::JsonValue;
use std::str::FromStr;

pub struct YLabelFormatter {
    base: Box<dyn NumberFormatter>,
    prefix: String,
    suffix: String,
}

impl NumberFormatter for YLabelFormatter {
    fn f(&self, n: &Ratio) -> String {
        format!(
            "{}{}{}",
            self.prefix,
            self.base.f(n),
            self.suffix,
        )
    }
}

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
    /// - color_title: String
    ///   - <<https://docs.rs/asciigraph/latest/asciigraph/enum.Color.html>>
    /// - primary_color: String
    ///   - <<https://docs.rs/asciigraph/latest/asciigraph/enum.Color.html>>
    /// - color_mode: String
    ///   - <<https://docs.rs/asciigraph/latest/asciigraph/enum.ColorMode.html>>
    /// - skip_range: Optional[[Number, Number]]
    ///   - if it's not set, it's default to `SkipValue::Automatic`
    ///   - if you want it to be `SkipValue::None`, set this value to null
    ///   - otherwise, it's set to `SkipValue::Manual { from: v[0], to: v[1] }`
    /// - y_label_prefix: String
    /// - y_label_suffix: String
    /// - labeled_intervals: Array[[Integer, Integer, String]]
    /// - horizontal_break: [Integer, Integer]
    ///
    /// For `Number`s in the above type annotations,
    ///
    /// 1. If it's an integer or a float in json, everything's fine.
    /// 2. If it's a string in json, it tries to parse it.
    /// 3. Otherwise, it's a type error.
    ///
    /// If it's an array, it interprets the array as `1d_data`.
    pub fn from_json(json_str: &str) -> Result<Self, Error> {
        let parsed = json::parse(json_str)?;
        let mut result = Graph::default();
        let mut formatter = YLabelFormatter {
            prefix: String::new(),
            suffix: String::new(),
            base: Box::new(DefaultFormatter),
        };

        result.set_skip_range(SkipValue::Automatic);

        if parsed.is_object() {
            for (key, value) in parsed.entries() {
                match key {
                    "1d_data" => {
                        if value.is_array() {
                            let mut v = Vec::with_capacity(value.members().count());

                            for n in value.members() {
                                v.push(json_to_ratio(n)?);
                            }

                            result.set_1d_data(&v);
                        }

                        else {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::Array(Box::new(JsonType::Number)),
                                got: get_type(value),
                            });
                        }
                    },
                    "1d_labeled_data" => {
                        if value.is_array() {
                            let mut labels_and_numbers = vec![];

                            for member in value.members() {
                                match member {
                                    JsonValue::Array(label_and_number) => {
                                        if label_and_number.len() == 2 {
                                            let label = if let Some(s) = label_and_number[0].as_str() {
                                                s.to_string()
                                            } else {
                                                return Err(Error::JsonTypeError {
                                                    key: Some(key.to_string()),
                                                    expected: JsonType::String,
                                                    got: get_type(&label_and_number[0]),
                                                });
                                            };
                                            let number = json_to_ratio(&label_and_number[1])?;

                                            labels_and_numbers.push((label, number));
                                        }

                                        else {
                                            return Err(Error::JsonArrayLengthError {
                                                key: Some(key.to_string()),
                                                expected: 2,
                                                got: label_and_number.len(),
                                            });
                                        }
                                    },
                                    _ => {
                                        return Err(Error::JsonTypeError {
                                            key: Some(key.to_string()),
                                            expected: JsonType::Array(Box::new(JsonType::Any)),
                                            got: get_type(member),
                                        });
                                    },
                                }
                            }

                            result.set_1d_labeled_data(&labels_and_numbers);
                        }

                        else {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::Array(Box::new(JsonType::Array(Box::new(JsonType::Any)))),
                                got: get_type(value),
                            });
                        }
                    },
                    "y_min" => {
                        result.set_y_min(json_to_ratio(value)?);
                    },
                    "y_max" => {
                        result.set_y_max(json_to_ratio(value)?);
                    },
                    "y_range" => {
                        let values = value.members().collect::<Vec<_>>();

                        if values.len() == 2 {
                            result.set_y_range(
                                json_to_ratio(&values[0])?,
                                json_to_ratio(&values[1])?,
                            );
                        }

                        else {
                            return Err(Error::JsonArrayLengthError {
                                key: Some(key.to_string()),
                                expected: 2,
                                got: values.len(),
                            });
                        }
                    },
                    "pretty_y" => {
                        result.set_pretty_y(json_to_ratio(value)?);
                    },
                    // 1, u64 as usize would fail in some old machines, but u32 as usize would never fail
                    // 2, why would someone draw a graph with more than 4 billion bars?
                    "plot_width" => match value.as_u32() {
                        Some(n) => {
                            result.set_plot_width(n as usize);
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::Integer,
                                got: get_type(value),
                            });
                        },
                    },
                    "plot_height" => match value.as_u32() {
                        Some(n) => {
                            result.set_plot_height(n as usize);
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::Integer,
                                got: get_type(value),
                            });
                        },
                    },
                    "x_label_margin" => match value.as_u32() {
                        Some(n) => {
                            result.set_x_label_margin(n as usize);
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::Integer,
                                got: get_type(value),
                            });
                        },
                    },
                    "y_label_margin" => match value.as_u32() {
                        Some(n) => {
                            result.set_y_label_margin(n as usize);
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::Integer,
                                got: get_type(value),
                            });
                        },
                    },
                    "block_width" => match value.as_u32() {
                        Some(n) => {
                            result.set_block_width(n as usize);
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::Integer,
                                got: get_type(value),
                            });
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
                                    return Err(Error::JsonTypeError {
                                        key: Some(key.to_string()),
                                        expected: JsonType::Integer,
                                        got: get_type(n),
                                    });
                                },
                            }
                        }

                        if paddings.len() != 4 {
                            return Err(Error::JsonArrayLengthError {
                                key: Some(key.to_string()),
                                expected: 4,
                                got: paddings.len(),
                            });
                        }

                        result.set_paddings([
                            paddings[0],
                            paddings[1],
                            paddings[2],
                            paddings[3],
                        ]);
                    } else {
                        return Err(Error::JsonTypeError {
                            key: Some(key.to_string()),
                            expected: JsonType::Array(Box::new(JsonType::Integer)),
                            got: get_type(value),
                        });
                    },
                    "title" => match value.as_str() {
                        Some(t) => {
                            result.set_title(t);
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::String,
                                got: get_type(value),
                            });
                        },
                    },
                    "x_axis_label" => match value.as_str() {
                        Some(t) => {
                            result.set_x_axis_label(t);
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::String,
                                got: get_type(value),
                            });
                        },
                    },
                    "y_axis_label" => match value.as_str() {
                        Some(t) => {
                            result.set_y_axis_label(t);
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::String,
                                got: get_type(value),
                            });
                        },
                    },
                    "big_title" => match value.as_bool() {
                        Some(b) => {
                            result.set_big_title(b);
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::Boolean,
                                got: get_type(value),
                            });
                        },
                    },
                    "title_color" => match value.as_str() {
                        Some(color) => {
                            result.set_title_color(
                                Some(Color::from_str(color).map_err(
                                    |e| Error::InvalidColorName(e)
                                )?)
                            );
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::String,
                                got: get_type(value),
                            });
                        },
                    },
                    "primary_color" => match value.as_str() {
                        Some(color) => {
                            result.set_primary_color(
                                Some(Color::from_str(color).map_err(
                                    |e| Error::InvalidColorName(e)
                                )?)
                            );
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::String,
                                got: get_type(value),
                            });
                        },
                    },
                    "color_mode" => match value.as_str() {
                        Some(color_mode) => {
                            result.set_color_mode(
                                ColorMode::from_str(color_mode).map_err(
                                    |e| Error::InvalidColorMode(e)
                                )?
                            );
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::String,
                                got: get_type(value),
                            });
                        },
                    },
                    "skip_range" => match value {
                        JsonValue::Null => {
                            result.set_skip_range(SkipValue::None);
                        },
                        JsonValue::Array(numbers) => {
                            if numbers.len() == 2 {
                                let from = json_to_ratio(&numbers[0])?;
                                let to = json_to_ratio(&numbers[1])?;

                                result.set_skip_range(SkipValue::Manual {
                                    from,
                                    to,
                                });
                            }

                            else {
                                return Err(Error::JsonArrayLengthError {
                                    key: Some(key.to_string()),
                                    expected: 2,
                                    got: numbers.len(),
                                });
                            }
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::Array(Box::new(JsonType::Number)),
                                got: get_type(value),
                            });
                        },
                    },
                    "y_label_prefix" => match value.as_str() {
                        Some(p) => {
                            formatter.prefix = p.to_string();
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::String,
                                got: get_type(value),
                            });
                        },
                    },
                    "y_label_suffix" => match value.as_str() {
                        Some(p) => {
                            formatter.suffix = p.to_string();
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::String,
                                got: get_type(value),
                            });
                        },
                    },
                    "labeled_intervals" => match value {
                        JsonValue::Array(intervals) => {
                            for interval in intervals.iter() {
                                match interval {
                                    JsonValue::Array(interval) => if interval.len() == 3 {
                                        let start = match interval[0].as_i32() {
                                            Some(n) => n,
                                            _ => {
                                                return Err(Error::JsonTypeError {
                                                    key: Some(key.to_string()),
                                                    expected: JsonType::Integer,
                                                    got: get_type(&interval[0]),
                                                });
                                            },
                                        };
                                        let end = match interval[1].as_i32() {
                                            Some(n) => n,
                                            _ => {
                                                return Err(Error::JsonTypeError {
                                                    key: Some(key.to_string()),
                                                    expected: JsonType::Integer,
                                                    got: get_type(&interval[1]),
                                                });
                                            },
                                        };
                                        let label = match interval[2].as_str() {
                                            Some(s) => s,
                                            _ => {
                                                return Err(Error::JsonTypeError {
                                                    key: Some(key.to_string()),
                                                    expected: JsonType::String,
                                                    got: get_type(&interval[2]),
                                                });
                                            },
                                        };

                                        result.add_labeled_interval(start, end, label);
                                    } else {
                                        return Err(Error::JsonArrayLengthError {
                                            key: Some(key.to_string()),
                                            expected: 3,
                                            got: interval.len(),
                                        });
                                    },
                                    _ => {
                                        return Err(Error::JsonTypeError {
                                            key: Some(key.to_string()),
                                            expected: JsonType::Array(Box::new(JsonType::Any)),
                                            got: get_type(value),
                                        });
                                    },
                                }
                            }
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::Array(Box::new(JsonType::Array(Box::new(JsonType::Any)))),
                                got: get_type(value),
                            });
                        },
                    },
                    "horizontal_break" => match value {
                        JsonValue::Array(numbers) => if numbers.len() == 2 {
                            let start = match numbers[0].as_usize() {
                                Some(n) => n,
                                _ => {
                                    return Err(Error::JsonTypeError {
                                        key: Some(key.to_string()),
                                        expected: JsonType::Integer,
                                        got: get_type(&numbers[0]),
                                    });
                                },
                            };
                            let end = match numbers[1].as_usize() {
                                Some(n) => n,
                                _ => {
                                    return Err(Error::JsonTypeError {
                                        key: Some(key.to_string()),
                                        expected: JsonType::Integer,
                                        got: get_type(&numbers[1]),
                                    });
                                },
                            };

                            result.set_horizontal_break(start, end);
                        } else {
                            return Err(Error::JsonArrayLengthError {
                                key: Some(key.to_string()),
                                expected: 2,
                                got: numbers.len(),
                            });
                        },
                        _ => {
                            return Err(Error::JsonTypeError {
                                key: Some(key.to_string()),
                                expected: JsonType::Array(Box::new(JsonType::Integer)),
                                got: get_type(value),
                            });
                        },
                    },
                    _ => {
                        return Err(Error::UnknownKey(key.to_string()));
                    },
                }
            }

            result.set_y_label_formatter(Box::new(formatter));
            Ok(result)
        }

        else if parsed.is_array() {
            let mut v = Vec::with_capacity(parsed.members().count());

            for n in parsed.members() {
                v.push(json_to_ratio(n)?);
            }

            result.set_1d_data(&v);
            Ok(result)
        }

        else {
            Err(Error::JsonTypeError {
                key: None,
                expected: JsonType::Object,
                got: get_type(&parsed),
            })
        }
    }
}

fn json_to_ratio(n: &JsonValue) -> Result<Ratio, Error> {
    if let Some(n) = n.as_number() {
        // Ratio::from_string is lossless
        let (positive, mantissa, exponent) = n.as_parts();

        Ok(Ratio::from_string(&format!(
            "{}{mantissa}e{exponent}",
            if positive { "" } else { "-" },
        ))?)
    }

    else if let Some(n) = n.as_str() {
        Ok(Ratio::from_string(n)?)
    }

    else {
        Err(Error::JsonTypeError {
            key: None,
            expected: JsonType::Number,
            got: get_type(n),
        })
    }
}
