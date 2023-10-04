use crate::lines::Lines;
use crate::alignment::Alignment;

#[derive(Clone, Debug)]
pub struct Interval {
    start: i32,  // allows neg intervals
    end: i32,

    // actual position of `start` and `end` when plotted
    plot_start: usize,
    plot_end: usize,

    label: String,
}

impl Interval {
    // TODO: make the index correspond with data's coordinate, not the actual rendered plane's coordinate
    pub fn new(start: i32, end: i32, label: String) -> Self {
        Interval {
            start, end, label,

            // must call `.adjust_coordinate` later
            plot_start: 0,
            plot_end: 0,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.end >= self.start
    }

    pub fn adjust_coordinate(&mut self, graph_width: usize, data_size: usize) {
        let start = self.start.max(0).min(data_size as i32 * 2) as usize;
        let end = self.end.max(0).min(data_size as i32 * 2) as usize;

        self.plot_start = start * graph_width / data_size;
        self.plot_end = end * graph_width / data_size;
    }

    pub fn render_full(&self) -> Vec<u16> {
        let label: Vec<u16> = self.label.encode_utf16().map(
            |c| c.max(' ' as u16)  // replace newline characters
        ).collect();
        let len = self.plot_end - self.plot_start + 1;  // inclusive end

        if len >= label.len() + 4 {
            let rem = len - label.len() - 2;
            let left = rem / 2;
            let right = rem / 2 + rem % 2;
            vec![
                vec!['<' as u16],
                vec!['─' as u16; left],
                label,
                vec!['─' as u16; right],
                vec!['>' as u16],
            ].concat()
        }

        else if label.len() > 8 && len > 7 {
            vec![
                vec!['<' as u16, '─' as u16],
                label[0..(len - 7)].to_vec(),
                vec!['.' as u16; 3],
                vec!['─' as u16, '>' as u16],
            ].concat()
        }

        else if len > 1 {
            vec![
                vec!['<' as u16],
                vec!['─' as u16; len - 2],
                vec!['>' as u16],
            ].concat()
        }

        // Too small to draw
        else {
            vec![]
        }
    }
}

pub fn draw_labeled_intervals(intervals: &Vec<Interval>, graph_width: usize) -> Lines {
    let mut masks = vec![vec![false; graph_width]];
    let mut rows = vec![vec![]];

    'outer: for interval in intervals.iter() {
        if interval.start < 0 || interval.plot_end >= graph_width {
            continue;
        }

        for (index, mask) in masks.iter_mut().enumerate() {
            if can_push(mask, interval) {
                push(mask, interval);
                rows[index].push(interval);
                continue 'outer;
            }
        }

        let mut new_mask = vec![false; graph_width];
        let new_row = vec![interval];
        push(&mut new_mask, interval);
        masks.push(new_mask);
        rows.push(new_row);
    }

    let mut result = Lines::new(graph_width, rows.len());

    for (index, row) in rows.iter().enumerate() {
        for interval in row.iter() {
            if interval.start < 0 {
                todo!()
            }

            else if interval.plot_end >= graph_width {
                todo!()
            }

            else {
                let i = interval.render_full();
                let l = Lines::from_string(&String::from_utf16_lossy(&i), Alignment::First);
                result = result.blit(&l, interval.plot_start as usize, index, None);
            }
        }
    }

    result
}

fn can_push(mask: &Vec<bool>, interval: &Interval) -> bool {
    let start = interval.plot_start;
    let end = interval.plot_end;

    mask[start..(end + 1)].iter().all(|c| !c)
}

fn push(mask: &mut Vec<bool>, interval: &Interval) {
    let start = interval.plot_start;
    let end = interval.plot_end;

    for i in start..(end + 1) {
        mask[i] = true;
    }
}
