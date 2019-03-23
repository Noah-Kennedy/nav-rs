pub struct Coordinate2D<T> where T: num_traits::Num {
    x: T,
    y: T
}

impl <T> Coordinate2D<T> where T: num_traits::Num {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn get_x(&self) -> T {
        &self.x
    }

    pub fn get_y(&self) -> T {
        &self.y
    }
}