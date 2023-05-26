use asciigraph::*;

fn main() {
    let mut g = Graph::default();

    g.set_1d_data(&vec![0, 1, 1, 0, 2, 0, 1, 2, 0, 0, 0, 1, 0, 1000])
    .set_y_min(-1)
    .set_y_max(3)
    .set_plot_height(20)
    .set_block_width(3)
    .set_y_label_interval(1)
    .set_title("HEllo World!123123123")
    .set_paddings([1;4])
    .set_big_title(true)
    .set_x_axis_label("x_axis_label\nxz")
    .set_y_axis_label("y_axis_label\nyy");

    println!("{g}");

    let mut g2 = Graph::default();
    g2.set_1d_data(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
    .set_plot_height(4)
    .set_y_min("0.0")
    .set_y_max(16.0)
    .set_block_width(3)
    .set_y_label_interval(1);

    println!("{g2}");

    let mut g3 = Graph::default();
    g3.set_1d_data::<u32>(&vec![1])
    .set_block_width(12);

    println!("{g3}");

    let mut g4 = Graph::default();
    g4.set_1d_data(
        &vec![
            vec![32; 1024],
            vec![16, 1024],
            vec![32; 1024],
        ].concat()
    );

    println!("{g4}");

    let mut g5 = Graph::default();
    g5.set_1d_data(&vec![1001, 1002, 1003, 1004, 1003, 1002, 1001, 1, 2, 3, 1000, 1006]);

    println!("{g5}");
}
