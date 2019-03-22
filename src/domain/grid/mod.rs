pub struct DynamicSizedGrid<E> {
    array: Vec<Vec<E>>,
    x_max: usize,
    y_max: usize,
}

impl<E> DynamicSizedGrid<E> where E: Default {
    pub fn new(x_max: usize, y_max: usize) -> Self {
        let mut array = Vec::with_capacity(x_max);
        for _ in 0..x_max {
            let mut row = Vec::with_capacity(y_max);

            for _ in 0..y_max {
                row.push(E::default())
            }

            array.push(row);
        }

        Self {
            array,
            x_max,
            y_max,
        }
    }
}

impl<E> DynamicSizedGrid<E> {
    pub fn get(&self, x: usize, y: usize) -> &E {
        &self.array[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, new_value: E) {
        self.array[x][y] = new_value;
    }

    pub fn get_x_max(&self) -> usize {
        self.x_max
    }

    pub fn get_y_max(&self) -> usize {
        self.y_max
    }

    fn get_array(&mut self) -> &mut Vec<Vec<E>> {
        &mut self.array
    }
}

#[cfg(test)]
mod tests {
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
    fn test_setting_in_bounds() {
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
    fn test_getting_in_bounds() {
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
}