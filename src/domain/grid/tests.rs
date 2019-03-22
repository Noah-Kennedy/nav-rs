use super::*;

fn empty_five_by_five_grid<E>() -> DynamicSizedGrid<E> where E: Default {
    DynamicSizedGrid::new(5, 5)
}

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
fn test_capacity() {
    let mut empty_five_by_five: DynamicSizedGrid<u64> = empty_five_by_five_grid();
    check_capacity(&mut empty_five_by_five, 5, 5);
}

#[test]
fn test_valid_setting_in_bounds() {
    let mut int_grid = empty_five_by_five_grid();
    for x in 0..5 {
        for y in 0..5 {
            let expected = format!("{} {}", x, y);
            int_grid.set(x, y, expected.clone());
            let actual = int_grid.get_array()[x][y].clone();
            assert_eq!(expected, actual);
        }
    }
}

#[test]
fn test_valid_getting_in_bounds() {
    let mut int_grid = empty_five_by_five_grid();
    for x in 0..5 {
        for y in 0..5 {
            let expected = format!("{} {}", x, y);
            int_grid.get_array()[x][y] = expected.clone();
            let actual = int_grid.get(x, y);
            assert_eq!(&expected, actual);
        }
    }
}
