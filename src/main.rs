#[derive(Clone)]
struct Cell<V: Clone> {
    pub value: V,
    pub terminal: bool,
}

impl<T: Clone> Cell<T> {
    pub fn normal(value: T) -> Self {
        Self {
            value,
            terminal: false,
        }
    }

    pub fn terminal(value: T) -> Self {
        Self {
            value,
            terminal: true,
        }
    }
}

#[derive(Clone)]
struct Grid<T: Clone> {
    pub cells: Vec<Vec<Option<Cell<T>>>>,
}

impl Grid<f64> {
    pub fn cons() -> Self {
        Self {
            cells: vec![
                vec![
                    Some(Cell::normal(0.0)),
                    Some(Cell::normal(0.0)),
                    Some(Cell::normal(0.0)),
                    Some(Cell::terminal(1.0)),
                ],
                vec![
                    Some(Cell::normal(0.0)),
                    None,
                    Some(Cell::normal(0.0)),
                    Some(Cell::terminal(-1.0)),
                ],
                vec![
                    Some(Cell::normal(0.0)),
                    Some(Cell::normal(0.0)),
                    Some(Cell::normal(0.0)),
                    Some(Cell::terminal(-1.0)),
                ]
            ]
        }
    }

    pub fn iterate(&mut self, evaluator: fn(current: &Cell<f64>, left: &Option<Cell<f64>>, right: &Option<Cell<f64>>, up: &Option<Cell<f64>>, down: &Option<Cell<f64>>) -> f64) {
        for ri in 0..self.cells.len() {
            let row =&self.cells[ri];
            for ci in 0..row.len() {
                if row[ci].is_some() {
                    row[ci].value = evaluator(cell, &row[ci - 1], &row[ci + 1], &self.cells[ri + 1][ci], &self.cells[ri - 1][ci]);
                }
            }
        }
    }
}

fn main() {}
