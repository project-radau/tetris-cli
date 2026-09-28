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

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
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

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        match self.index(x, y) {
            Some(index) => Some(self.cells[index]),
            None => None
        }
    }

    pub fn setTetromino(&mut self, tetromino: &Tetromino) -> Result<(), String> {
        let shape = tetromino.kind().shape();

        if  tetromino.x() + shape.width() > self.width ||
            tetromino.y() + shape.height() > self.height {
            return Err("invalid tetromino position".to_string());
        }

        //check for collision
        for height_index in 0..shape.height() {
            for width_index in 0..shape.width() {
                if shape.cells()[height_index * shape.width() + width_index] {
                    match self.get(tetromino.x() + width_index, tetromino.y() + height_index) {
                        Some(cell) => {
                            if cell {
                                return Err("collision detected".to_string());
                            }
                        },
                        None => { return Err("invalid tetromino position".to_string()); }
                    };
                }
            }
        }

        let mut start = tetromino.y() * self.width + tetromino.x();

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

    pub fn removeTetromino(&mut self, tetromino: &Tetromino) -> Result<(), String> {
        let shape = tetromino.kind().shape();

        if  tetromino.x() + shape.width() > self.width ||
            tetromino.y() + shape.height() > self.height {
            return Err("invalid tetromino position".to_string());
        }

        //check for collision
        for height_index in 0..shape.height() {
            for width_index in 0..shape.width() {
                if shape.cells()[height_index * shape.width() + width_index] {
                    match self.get(tetromino.x() + width_index, tetromino.y() + height_index) {
                        Some(cell) => {
                            if !cell {
                                return Err("no tetromino found".to_string());
                            }
                        },
                        None => { return Err("invalid tetromino position".to_string()); }
                    };
                }
            }
        }

        let mut start = tetromino.y() * self.width + tetromino.x();

        for height_index in 0..shape.height() {
            for width_index in 0..shape.width() {
                let shape_cell = shape.cells()[height_index * shape.width() + width_index];
                if shape_cell {
                    self.cells[start + width_index] = false;
                }
            }
            start = start + self.width;
        }

        Ok(())
    }

    pub fn move_tetromino(
        &mut self,
        tetromino: &mut Tetromino,
        dx: isize,
        dy: isize
    ) -> Result<(), String> {
        self.removeTetromino(&tetromino)?;
        tetromino.move_by(dx, dy);

        match self.setTetromino(&tetromino) {
            Ok(()) => Ok(()),
            Err(error) => {
                tetromino.move_by(-dx,-dy);
                self.setTetromino(&tetromino);
                Err(error)
            }
        }  
    }
}