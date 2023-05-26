# Ascii Graph

Draw beautiful graphs in ascii art!

- [API Reference](https://docs.rs/asciigraph/latest/asciigraph/)

## Showcase

```rust
let mut g1 = Graph::default();

g1.set_1d_data(&vec![0, 1, 1, 0, 2, 0, 1, 2, 0, 0, 0, 1, 0, 1000])
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

println!("{g1}");

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
```

```
 ▌ ▐ ▛▀▀ ▜▌ ▜▌        ▌ ▐        ▜▌   ▐ ▐▌ ▟▌ ▞▀▚ ▞▀▚ ▟▌ ▞▀▚ ▞▀▚ ▟▌ ▞▀▚ ▞▀▚
 ▛▀▜ ▛▀▀ ▐▌ ▐▌ ▞▀▚    ▌▄▐ ▞▀▚ ▄▄ ▐▌ ▞▀▜ ▐▌ ▐▌   ▞  ▝▌ ▐▌   ▞  ▝▌ ▐▌   ▞  ▝▌
 ▌ ▐ ▙▄▄ ▐▙ ▐▙ ▚▄▞    ▛ ▜ ▚▄▞ ▌  ▐▙ ▚▄▟ ▗▖ ▟▙ ▗▙▄ ▚▄▞ ▟▙ ▗▙▄ ▚▄▞ ▟▙ ▗▙▄ ▚▄▞


       x_axis_label
       xz
           1062│
           1044│
           1026│
           1008│                                       ▄▄▄
       991.0892│                                       ███
       973.2678│                                       ███
       955.4464│                                       ███
       ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          2.125│            ▄▄▄      ▄▄▄               ███
       1.951923│            ███      ███               ███
       1.778846│            ███      ███               ███
       1.605769│            ███      ███               ███
       1.432692│            ███      ███               ███
       1.259615│            ███      ███               ███
       1.086538│   ▄▄▄▄▄▄   ███   ▄▄▄███         ▄▄▄   ███
       0.913461│   ██████   ███   ██████         ███   ███
       0.740384│   ██████   ███   ██████         ███   ███
       0.567307│   ██████   ███   ██████         ███   ███
        0.39423│   ██████   ███   ██████         ███   ███
       0.221153│   ██████   ███   ██████         ███   ███
       0.048076│▆▆▆██████▆▆▆███▆▆▆██████▆▆▆▆▆▆▆▆▆███▆▆▆███y_axis_label
               ╰──────────────────────────────────────────yy
                0     2     4     6     8     10    12
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
 1061│
     │
 1036│
     │                                        █
 1011│                                        █
     │                                        █
986.8│                                        █
     │                                        █
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
   33│                                        █
     │███████████████████████████████████████ ████████████████████████████████████████
   31│███████████████████████████████████████ ████████████████████████████████████████
     │███████████████████████████████████████ ████████████████████████████████████████
   29│███████████████████████████████████████ ████████████████████████████████████████
     │███████████████████████████████████████ ████████████████████████████████████████
   27│███████████████████████████████████████ ████████████████████████████████████████
     │███████████████████████████████████████ ████████████████████████████████████████
   25│███████████████████████████████████████ ████████████████████████████████████████
     │███████████████████████████████████████ ████████████████████████████████████████
   23│███████████████████████████████████████ ████████████████████████████████████████
     │███████████████████████████████████████ ████████████████████████████████████████
   21│███████████████████████████████████████ ████████████████████████████████████████
     │███████████████████████████████████████ ████████████████████████████████████████
   19│███████████████████████████████████████ ████████████████████████████████████████
     │███████████████████████████████████████ ████████████████████████████████████████
   17│███████████████████████████████████████ ████████████████████████████████████████
     │████████████████████████████████████████████████████████████████████████████████
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
   3.125│███████████████████████████████████████████████             ▄▄▄▄▄▄▄█████████████
        │███████████████████████████████████████████████             ████████████████████
   2.625│███████████████████████████████████████████████             ████████████████████
        │███████████████████████████████████████████████             ████████████████████
   2.125│███████████████████████████████████████████████       ▄▄▄▄▄▄████████████████████
        │███████████████████████████████████████████████       ██████████████████████████
   1.625│███████████████████████████████████████████████       ██████████████████████████
        │███████████████████████████████████████████████       ██████████████████████████
   1.125│███████████████████████████████████████████████▄▄▄▄▄▄▄██████████████████████████
        ╰────────────────────────────────────────────────────────────────────────────────
         0             2            4            6             8            10
                1            3             5            7            9             11
```