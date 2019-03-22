use super::*;

#[test]
#[should_panic]
fn set_x_above_bounds() {
    let mut grid = empty_five_by_five_grid();
    grid.set(5, 3, "Test");
}

#[test]
#[should_panic]
fn set_y_above_bounds() {
    let mut grid = empty_five_by_five_grid();
    grid.set(3, 5, "Test");
}

#[test]
#[should_panic]
fn set_both_above_bounds() {
    let mut grid = empty_five_by_five_grid();
    grid.set(5, 5, "Test");
}
