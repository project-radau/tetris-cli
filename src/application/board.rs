use crate::application::tetromino::Tetromino;

pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<bool>
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![false; width * height]
        }
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(y * self.width + x)
    }

    pub fn render(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y * self.width + x] {
                    print!("#")
                }
                else {
                    print!(".")
                }
            }
            print!("\n");
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) -> Result<(), String> {
        match self.index(x, y) {
            Some(index) => {
                self.cells[index] = value;
                Ok(())
            }
            None => Err(String::from("cell not found."))
        }
    }

    pub fn setTetromino(&mut self, tetromino: Tetromino) -> Result<(), String> {
        let mut start = tetromino.y() * self.width + tetromino.x();

        let shape = tetromino.kind().shape();

        for height_index in 0..shape.height() {
            for width_index in 0..shape.width() {
                let shape_cell = shape.cells()[height_index * shape.width() + width_index];
                if shape_cell {
                    self.cells[start + width_index] = true;
                }
            }
            start = start + self.width;
        }

        Ok(())
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        match self.index(x, y) {
            Some(index) => Some(self.cells[index]),
            None => None
        }
    }
}