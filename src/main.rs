#[macro_use]
extern crate ndarray;

use ndarray::{Array, Dim};
use std::cmp::Ordering::Equal;

pub struct Grid {
    pub values: Array<f32, Dim<[usize; 2]>>,
    pub terminal: Array<bool, Dim<[usize; 2]>>,
    pub mutable: Array<bool, Dim<[usize; 2]>>,
    pub traversable: Array<bool, Dim<[usize; 2]>>,
}

impl Grid {
    pub fn constrain(&self, agent_width: usize, agent_height: usize) -> Array<f32, Dim<[usize; 2]>> {
        let mut constrained = self.values.clone();

        for ri in 0..constrained.rows() {
            for ci in 0..constrained.cols() {
                if self.mutable[[ci, ri]] {
                    constrained[[ci, ri]] = *self.values
                        .slice(s![ci-agent_width..ci+agent_width, ri-agent_height..ri+agent_height])
                        .iter()
                        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
                        .unwrap();
                }
            }
        }

        constrained
    }
}

fn main() {}
