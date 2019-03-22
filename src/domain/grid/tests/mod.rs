use super::*;

#[cfg(test)]
mod grid_bounds;

#[cfg(test)]
mod normal_flow;

#[cfg(test)]
mod capacity;

fn empty_five_by_five_grid<E>() -> DynamicSizedGrid<E> where E: Default {
    DynamicSizedGrid::new(5, 5)
}