#[cfg(test)]
mod tests;

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