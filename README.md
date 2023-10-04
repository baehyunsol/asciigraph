# Ascii Graph

Draw beautiful graphs in ascii art!

- [API Reference](https://docs.rs/asciigraph/latest/asciigraph/)

## Showcase

```rust
use asciigraph::*;
use hmath::Ratio;

fn main() {
    let mut g1 = Graph::default();

    g1.set_1d_data(&vec![0, 1, 1, 0, 2, 0, 1, 2, 0, 0, 0, 1, 0, 1000])
    .set_y_min(-1)
    .set_y_max(3)
    .set_plot_height(20)
    .set_block_width(3)
    .set_y_label_margin(1)
    .set_title("HEllo World!123123123")
    .set_paddings([1;4])
    .set_big_title(true)
    .set_x_axis_label("x_axis_label\nxz")
    .set_y_axis_label("y_axis_label\nyy");

    println!("{g1}");

    let mut g2 = Graph::default();
    g2.set_1d_data(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
    .set_plot_height(4)
    .set_y_min("0.0")
    .set_y_max(16.0)
    .set_block_width(3)
    .set_y_label_margin(1);

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

    let mut g6 = Graph::new(52, 26);
    g6.set_2d_data_high_resolution(
        &(0..(52usize * 104)).map(
            |i| (i / 52, i % 52)
        ).filter(
            |(x, y)| {
                let d = x.abs_diff(52) * x.abs_diff(52) + y.abs_diff(26) * y.abs_diff(26) * 4;

                1200 < d && d < 2000
            }
        ).collect::<Vec<(usize, usize)>>(),
        &vec![None; 52],
        &vec![None; 26],
    );

    println!("{g6}");

    let mut g7 = Graph::new(24, 24);
    g7.set_block_width(5);
    g7.set_1d_data(
        &vec![0, 0, 0, 5000, 0, 0, 0, 7000, 0]
    );

    let mut g8 = Graph::new(24, 24);
    g8.set_block_width(5);
    g8.set_skip_range(SkipValue::none());
    g8.set_1d_data(
        &vec![0, 0, 0, 5000, 0, 0, 0, 7000, 0]
    );

    println!("{}", merge_horiz(
        &g7.to_string(),
        &g8.to_string(),
        Alignment::First,
        2
    ));

    let mut g9 = Graph::new(24, 24);
    g9.set_block_width(5)
    .set_1d_data(
        &vec![
            0, 0, 0,
            60000,
            0, 0, 0,
            50000,
            0, 0,
            1200, 1500,
            0, 400
        ]
    )
    .set_y_min(0)
    .set_pretty_y(50);

    println!("{g9}");

    let mut g10 = Graph::new(72, 24);
    g10.set_1d_labeled_data(&(0..62832).map(
            |i| {
                let x = i as f64 / 10000.0;
                let y = x.sin();

                (x.to_string(), Ratio::try_from(y).unwrap())
            }
        ).collect())
        .set_y_max("1.2")
        .set_y_min("-1.2")
        .add_labeled_interval(0, 31415, "first pi".to_string())
        .add_labeled_interval(31416, 62831, "second pi".to_string())
        .add_labeled_interval(0, 62831, "two pi".to_string())
        .add_labeled_interval(0, 3000, "small interval".to_string())
        .add_labeled_interval(1200, 9000, "small interval2".to_string());

    println!("{g10}");
}
```

```

 ▌ ▐ ▛▀▀ ▜▌ ▜▌        ▌ ▐        ▜▌   ▐ ▐▌ ▟▌ ▞▀▚ ▞▀▚ ▟▌ ▞▀▚ ▞▀▚ ▟▌ ▞▀▚ ▞▀▚
 ▛▀▜ ▛▀▀ ▐▌ ▐▌ ▞▀▚    ▌▄▐ ▞▀▚ ▄▄ ▐▌ ▞▀▜ ▐▌ ▐▌   ▞  ▝▌ ▐▌   ▞  ▝▌ ▐▌   ▞  ▝▌
 ▌ ▐ ▙▄▄ ▐▙ ▐▙ ▚▄▞    ▛ ▜ ▚▄▞ ▌  ▐▙ ▚▄▟ ▗▖ ▟▙ ▗▙▄ ▚▄▞ ▟▙ ▗▙▄ ▚▄▞ ▟▙ ▗▙▄ ▚▄▞


         y_axis_label
         yy
            3│                                       ^^^
          2.8│                                       ███
          2.6│                                       ███
          2.4│                                       ███
          2.2│                                       ███
            2│            ███      ███               ███
          1.8│            ███      ███               ███
          1.6│            ███      ███               ███
          1.4│            ███      ███               ███
          1.2│            ███      ███               ███
            1│   ██████   ███   ██████         ███   ███
          0.8│   ██████   ███   ██████         ███   ███
          0.6│   ██████   ███   ██████         ███   ███
          0.4│   ██████   ███   ██████         ███   ███
          0.2│   ██████   ███   ██████         ███   ███
            0│██████████████████████████████████████████
         -0.2│██████████████████████████████████████████
         -0.4│██████████████████████████████████████████
         -0.6│██████████████████████████████████████████
         -0.8│██████████████████████████████████████████
             ╰──────────────────────────────────────────x_axis_label
              0     2     4     6     8     10    12    xz
                 1     3     5     7     9     11    13

16│                                       ▂▂▂▄▄▄▆▆▆███
12│                           ▂▂▂▄▄▄▆▆▆███████████████
 8│               ▂▂▂▄▄▄▆▆▆███████████████████████████
 4│   ▂▂▂▄▄▄▆▆▆███████████████████████████████████████
  ╰───────────────────────────────────────────────────
   0     2     4     6     8     10    12    14    16
      1     3     5     7     9     11    13    15
 8│
  │
 7│
  │
 6│
  │
 5│
  │
 4│
  │
 3│
  │
 2│
  │
 1│████████████
  │████████████
 0│████████████
  │████████████
-1│████████████
  │████████████
-2│████████████
  │████████████
-3│████████████
  │████████████
-4│████████████
  │████████████
-5│████████████
  │████████████
  ╰────────────
   0

1086│
    │
1055│
    │
1024│                                        █
    │                                        █
 993│                                        █
    │                                        █
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  34│                                        █
    │                                        █
  32│███████████████████████████████████████ ████████████████████████████████████████
    │███████████████████████████████████████ ████████████████████████████████████████
  30│███████████████████████████████████████ ████████████████████████████████████████
    │███████████████████████████████████████ ████████████████████████████████████████
  28│███████████████████████████████████████ ████████████████████████████████████████
    │███████████████████████████████████████ ████████████████████████████████████████
  26│███████████████████████████████████████ ████████████████████████████████████████
    │███████████████████████████████████████ ████████████████████████████████████████
  24│███████████████████████████████████████ ████████████████████████████████████████
    │███████████████████████████████████████ ████████████████████████████████████████
  22│███████████████████████████████████████ ████████████████████████████████████████
    │███████████████████████████████████████ ████████████████████████████████████████
  20│███████████████████████████████████████ ████████████████████████████████████████
    │███████████████████████████████████████ ████████████████████████████████████████
  18│███████████████████████████████████████ ████████████████████████████████████████
    │███████████████████████████████████████ ████████████████████████████████████████
  16│████████████████████████████████████████████████████████████████████████████████
    ╰────────────────────────────────────────────────────────────────────────────────
     0  51  153  307  410  563  666  820  922  1076  1230  1383  1537  1691  1845
      0  102  205  358  461  615  717  871  1024  1127  1281  1435  1588  1742  1896
    1006│
        │                                                                          ██████
    1005│                                                                          ██████
        │                                                                          ██████
    1004│                                                                          ██████
        │                                                                          ██████
    1004│                    ▄▄▄▄▄▄▄                                               ██████
        │                    ███████                                               ██████
    1003│                    ███████                                               ██████
        │              ▄▄▄▄▄▄███████▄▄▄▄▄▄▄                                        ██████
    1002│              ████████████████████                                        ██████
        │              ████████████████████                                        ██████
    1002│       ▆▆▆▆▆▆▆████████████████████▆▆▆▆▆▆                                  ██████
        │       █████████████████████████████████                                  ██████
    1001│       █████████████████████████████████                                  ██████
        │███████████████████████████████████████████████                           ██████
    1000│███████████████████████████████████████████████                           ██████
        │███████████████████████████████████████████████                    ▂▂▂▂▂▂▂██████
999.9802│███████████████████████████████████████████████                    █████████████
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
   3.125│███████████████████████████████████████████████             ▆▆▆▆▆▆▆█████████████
        │███████████████████████████████████████████████             ████████████████████
  2.5625│███████████████████████████████████████████████             ████████████████████
        │███████████████████████████████████████████████             ████████████████████
       2│███████████████████████████████████████████████       ██████████████████████████
        │███████████████████████████████████████████████       ██████████████████████████
  1.4375│███████████████████████████████████████████████       ██████████████████████████
        │███████████████████████████████████████████████▄▄▄▄▄▄▄██████████████████████████
        ╰────────────────────────────────────────────────────────────────────────────────
         0             2            4            6             8            10
                1            3             5            7            9             11
│
│
│                  ▗▄▄▄▟███████▄▄▄▄
│              ▗▄▟███████████████████▄▄
│           ▗▄██████████▀▀▀▀▀▀▜█████████▙▄
│         ▗▟██████▛▀▘             ▀▀███████▄
│        ▄██████▀                    ▝▜█████▙▖
│       ▟█████▀                        ▝▜█████▖
│      ▟████▛                            ▝█████▖
│     ▟████▛                              ▝█████▖
│    ▐████▛                                ▝█████
│    ▟████▘                                 ▜████▖
│    █████                                  ▐████▌
│    █████                                  ▐████▌
│    █████                                  ▐████▌
│    ▐████▌                                 █████
│    ▝█████▖                               ▟████▛
│     ▝█████▖                             ▟████▛
│      ▝█████▄                          ▗▟████▛
│       ▝██████▄                      ▗▟█████▛
│         ▜██████▄▖                 ▄▟██████▘
│          ▝▜███████▙▄▄▄      ▗▄▄▄████████▀
│             ▀▜███████████████████████▀▘
│                ▝▀▜███████████████▀▀
│                      ▝▀▀▀▀▀▀▀
│
╰────────────────────────────────────────────────────


7123│                                   ▂▂▂▂▂        7434│
    │                                   █████            │                                   ▆▆▆▆▆
6842│                                   █████        6778│                                   █████
    │                                   █████            │                                   █████
6561│                                   █████        6122│                                   █████
    │                                   █████            │                                   █████
6280│                                   █████        5466│                                   █████
    │                                   █████            │               ▆▆▆▆▆               █████
5999│                                   █████        4810│               █████               █████
    │                                   █████            │               █████               █████
5718│                                   █████        4154│               █████               █████
    │                                   █████            │               █████               █████
5437│                                   █████        3498│               █████               █████
    │                                   █████            │               █████               █████
5156│                                   █████        2842│               █████               █████
    │               █████               █████            │               █████               █████
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~   2186│               █████               █████
 314│               █████               █████            │               █████               █████
    │               █████               █████        1530│               █████               █████
 135│               █████               █████            │               █████               █████
    │▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█████▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█████▄▄▄▄▄  874.5│               █████               █████
 -44│█████████████████████████████████████████████       │               █████               █████
    │█████████████████████████████████████████████  218.5│▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█████▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█████▄▄▄▄▄
-223│█████████████████████████████████████████████       │█████████████████████████████████████████████
    ╰─────────────────────────────────────────────       ╰─────────────────────────────────────────────
     0         2         4         6         8            0         2         4         6         8
          1         3         5         7                      1         3         5         7
60600│               ▆▆▆▆▆
     │               █████
57400│               █████
     │               █████
54200│               █████
     │               █████
51000│               █████               ▄▄▄▄▄
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 1600│               █████               █████
     │               █████               █████               █████
 1400│               █████               █████               █████
     │               █████               █████               █████
 1200│               █████               █████          ██████████
     │               █████               █████          ██████████
 1000│               █████               █████          ██████████
     │               █████               █████          ██████████
  800│               █████               █████          ██████████
     │               █████               █████          ██████████
  600│               █████               █████          ██████████
     │               █████               █████          ██████████
  400│               █████               █████          ██████████     █████
     │               █████               █████          ██████████     █████
  200│               █████               █████          ██████████     █████
     │               █████               █████          ██████████     █████
     ╰──────────────────────────────────────────────────────────────────────
      0         2         4         6         8         10        12
           1         3         5         7         9         11        13
 1.2│
    │
   1│             ▄▄██████▄▄
    │           ▆▆██████████▆▆
 0.8│         ▆▆██████████████▆▆
    │       ▄▄██████████████████▄▄
 0.6│      ▂██████████████████████▂
    │     ██████████████████████████
 0.4│   ▄▄██████████████████████████▄▄
    │   ██████████████████████████████
 0.2│ ▆▆██████████████████████████████▆▆
    │ ██████████████████████████████████▂
   0│█████████████████████████████████████                                  █
    │█████████████████████████████████████▄▄                              ▄▄█
-0.2│███████████████████████████████████████                              ███
    │███████████████████████████████████████▆▆                          ▆▆███
-0.4│█████████████████████████████████████████▂                        ▂█████
    │███████████████████████████████████████████                      ███████
-0.6│███████████████████████████████████████████▆▆                  ▆▆███████
    │█████████████████████████████████████████████▄▄              ▄▄█████████
-0.8│███████████████████████████████████████████████▄▄          ▄▄███████████
    │█████████████████████████████████████████████████▆▆▂▂▂▂▂▂▆▆█████████████
  -1│████████████████████████████████████████████████████████████████████████
    │████████████████████████████████████████████████████████████████████████
    ╰────────────────────────────────────────────────────────────────────────
     0  0.3489  1.0471  1.7452  2.4433  3.1415  3.8396  4.5377  5.2359
      0.1744  0.8725  1.5707  2.2688  2.9669  3.6651  4.3632  5.0613  5.7595
     <─────────────first pi─────────────><────────────second pi─────────────>
     <────────────────────────────────two pi────────────────────────────────>
     <──>
      <─sma...─>
```