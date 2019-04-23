use nalgebra::base::{Matrix, VecStorage, MatrixMN};
use nalgebra::base::dimension::{Dynamic, DimName};
use nalgebra::{Dim, DefaultAllocator};
use nalgebra::base::allocator::Allocator;

pub struct Grid<R, C> where R: Dim + DimName, C: Dim + DimName,
                            DefaultAllocator: Allocator<f32, R, C> {
    pub terminals: Matrix<bool, R, C, DefaultAllocator>,
    pub values: MatrixMN<f32, R, C>,
    pub mutables: Matrix<bool, R, C, DefaultAllocator>,
    pub traversables: Matrix<bool, R, C, DefaultAllocator>,
}

impl<R, C> Grid<R, C> where R: Dim + DimName, C: Dim + DimName,
                            DefaultAllocator: Allocator<f32, R, C> {
    pub fn constrain(&self) -> MatrixMN<f32, R, C> {
        unimplemented!()
    }
}

fn main() {}
