pub struct PathNode {
    x_coord: isize,
    y_coord: isize,
    next: Option<Box<PathNode>>
}

impl PathNode {
    pub fn route(x_coord: usize, y_coord: usize, next: PathNode) -> Self {
        Self {
            x_coord,
            y_coord,
            next: Some(Box::new(next))
        }
    }

    pub fn destination(x_coord: isize, y_coord: isize) -> Self {
        Self {
            x_coord,
            y_coord,
            next: None,
        }
    }

    pub fn follow(self) -> Option<Box<PathNode>> {
        self.next
    }

    pub fn get_x(&self) -> isize {
        self.x_coord
    }
}