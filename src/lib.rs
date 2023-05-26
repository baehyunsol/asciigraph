//! Use strings if you want exact numbers. For example, `set_y_max(3.2)` uses f32 type which cannot represent `3.2` accurately.
//! But `set_y_max("3.2")` uses internal ratio type which can represent any rational number perfectly.

mod alignment;
mod convert;
mod format;
mod graph;
mod lines;
mod skip_value;
mod utils;

pub use graph::Graph;

pub use convert::IntoRatio;
pub use alignment::Alignment;
pub use skip_value::SkipValue;

pub use graph::{merge_horiz, merge_vert};