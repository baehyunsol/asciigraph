//! Use strings if you want exact numbers. For example, `set_y_max(3.2)` uses f32 type which cannot represent `3.2` accurately.
//! But `set_y_max("3.2")` uses internal ratio type which can represent any rational number perfectly.

mod alignment;
mod color;
mod format;
mod graph;
mod interval;
mod lines;
mod skip_value;
mod table;
mod utils;

#[cfg(feature = "json")]
mod error;

#[cfg(feature = "json")]
mod json;

pub use alignment::Alignment;
pub use color::{Color, ColorMode};
pub use graph::{Graph, merge_horiz, merge_vert};
pub use skip_value::SkipValue;

#[cfg(feature = "json")]
pub use error::{Error, JsonType};
