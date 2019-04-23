use nalgebra::base::{Matrix, VecStorage};
use nalgebra::base::dimension::Dynamic;
use nalgebra::Dim;

pub struct Grid<R, C> where R: Dim, C: Dim {
    pub terminals: Matrix<bool, R, C, VecStorage<bool, R, C>>,
    pub values: Matrix<f32, R, C, VecStorage<f32, R, C>>,
    pub mutables: Matrix<bool, R, C, VecStorage<bool, R, C>>,
    pub traversables: Matrix<bool, R, C, VecStorage<bool, R, C>>,
}

impl <R, C> Grid<R, C> where R: Dim, C: Dim {
    pub fn constrain(&self) -> Matrix<f32, R, C, VecStorage<f32, R, C>> {
        let mut constrained = self.values.clone();
        constrained
    }
}

fn main() {

}
