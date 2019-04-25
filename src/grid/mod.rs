use ndarray::{Array, Dim, Array2, SliceInfo, arr0};
use std::cmp::Ordering::Equal;
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct Grid {
    pub values: Array2<f32>,

    /// One indicates terminal
    pub terminal: Array2<u8>,

    /// One indicates mutable
    pub mutable: Array2<u8>,

    /// One indicates traversable
    pub traversable: Array2<u8>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let shape = (rows, cols);

        Self {
            values: Array2::zeros(shape),
            terminal: Array2::zeros(shape),
            mutable: Array2::ones(shape),
            traversable: Array2::ones(shape),
        }
    }

    pub fn add_obstruction(&mut self, row: usize, col: usize, value: f32) {
        let position = [row, col];
        self.values[position] = value;
        self.mutable[position] = 0;
        self.traversable[position] = 0;
        self.terminal[position] = 0;
    }

    pub fn add_terminal(&mut self, row: usize, col: usize, value: f32) {
        let position = [row, col];
        self.values[position] = value;
        self.mutable[position] = 0;
        self.traversable[position] = 1;
        self.terminal[position] = 1;
    }

    pub fn constrain(&self, agent_width: usize, agent_height: usize) -> Array2<f32> {
        let mut constrained = self.values.clone();

        for ri in 0..constrained.rows() {
            for ci in 0..constrained.cols() {

                if self.mutable[[ri, ci]] == 1 {

                    let min_height = ri - agent_height / 2;
                    let max_height = ri + agent_height / 2;
                    let min_width = ci - agent_width / 2;
                    let max_width = ci + agent_width / 2;

                    let height_range = min_height..max_height;
                    let width_range = min_width..max_width;
                    let slice_info = s![height_range, width_range];

                    constrained[[ri, ci]] = *self.values
                        .slice(slice_info)
                        .iter()
                        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
                        .unwrap();
                }
            }
        }

        constrained
    }

    pub fn draw(&self) {
        for ri in 0..self.values.rows() {
            for ci in 0..self.values.cols() {
                let pos = [ri, ci];

                let value = if self.terminal[pos] > 0 {
                    self.values[pos].to_string().as_str().red()
                } else {
                    self.values[pos].to_string().as_str().blue()
                };

                let open_trav = "[";
                let close_trav = "]";

                let (open_trav, close_trav) = if self.traversable[pos] > 0 {
                    (open_trav.red(), close_trav.red())
                } else {
                    (open_trav.blue(), close_trav.blue())
                };

                let open_mut = "[";
                let close_mut = "]";

                let (open_mut, close_mut) = if self.mutable[pos] > 0 {
                    (open_mut.red(), close_mut.red())
                } else {
                    (open_mut.blue(), close_mut.blue())
                };

                print!(" {}{:^5}{} ", open_trav, value, close_trav)
            }
            println!();
        }
    }
}