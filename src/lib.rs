mod format;
mod graph;
mod merge;
mod utils;

/// It assumes a korean character (한글) consumes 2 spaces.
/// For real numbers, it uses fixed point numbers. (14 bits for fractional parts)

pub use graph::{Graph, SkipValue};
pub use merge::{merge_horiz, merge_vert, Alignment};
pub use format::format_lines;
