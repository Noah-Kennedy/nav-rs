use super::*;

fn check_capacity<E>(grid: &mut DynamicSizedGrid<E>, y_cap: usize, x_cap: usize) {
    assert_eq!(x_cap, grid.get_x_max(), "'get_x_max' did not match the expected");
    assert_eq!(y_cap, grid.get_y_max(), "'get_y_max' did not match the expected");

    assert_eq!(x_cap, grid.get_array().capacity());
    assert_eq!(y_cap, grid.get_array().capacity());

    assert_eq!(x_cap, grid.get_array().len());

    for y_row in grid.get_array() {
        assert_eq!(y_cap, y_row.len());
    }
}

#[test]
fn capacity() {
    let mut empty_five_by_five: DynamicSizedGrid<u64> = empty_five_by_five_grid();
    check_capacity(&mut empty_five_by_five, 5, 5);
}
