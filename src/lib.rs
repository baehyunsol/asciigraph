mod graph;
mod utils;
mod merge;

/// It assumes a korean character (한글) consumes 2 spaces.
/// For real numbers, it uses fixed point numbers. (24 bits for fractional parts)

pub use graph::Graph;
pub use merge::{merge_horiz, merge_vert, Alignment};

#[test]
fn mmm() {
    let sample: Vec<i64> = vec![i64::MIN; 4095];

    let draw = Graph::new(32, 32).set_1d_data(sample.into_iter().enumerate().map(|(ind, val)| (ind.to_string(), val)).collect()).draw();

    println!("{draw}");
}