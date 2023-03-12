mod graph;
mod utils;
mod merge;

/// It assumes a korean character (한글) consumes 2 spaces.
/// For real numbers, it uses fixed point numbers. (24 bits for fractional parts)

pub use graph::Graph;
pub use merge::{merge_horiz, merge_vert, Alignment};