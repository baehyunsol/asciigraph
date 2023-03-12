mod graph;
mod utils;
mod merge;

/// It assumes a korean character (한글) consumes 2 spaces.
/// For real numbers, it uses fixed point numbers. (24 bits for fractional parts)

pub use graph::Graph;
pub use merge::{merge_horiz, merge_vert, Alignment};

#[test]
fn mmm() {
    let data: Vec<(String, i64)> = (0..256).map(|n| (n.to_string(), n)).collect();

    let ddd = Graph::new(36, 18).set_1d_data(data).set_y_max(128).draw();

    println!("{ddd}");
}