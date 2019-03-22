use super::*;

#[test]
fn updating() {
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
fn query() {
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