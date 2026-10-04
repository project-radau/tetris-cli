use crate::application::tetromino::Tetromino;
use crate::GameState;

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

    pub fn is_row_full(&self, y: usize) -> bool {
        for index in (y * self.width)..(y * self.width + self.width) {
            if self.cells[index] == false {
                return false;
            }
        }
        true
    }

    pub fn clear_full_rows(&mut self) -> usize {
        let mut cleared_rows = 0;

        let mut y = self.height();

        while y > 0 {
            y -= 1;
            if self.is_row_full(y) {
                for index in (self.width()..((y + 1) * self.width())).rev() {
                    self.cells[index] = self.cells[index - self.width()];
                }

                for index in 0..self.width() {
                    self.cells[index] = false;
                }

                cleared_rows += 1;
            }
        }

        cleared_rows
    }

    pub fn setTetromino(&mut self, tetromino: &Tetromino) -> Result<(), String> {
        let shape = tetromino.kind().shape(tetromino.rotation_state());

        let leading_empty_columns = shape.leading_empty_columns();
        let trailing_empty_columns = shape.trailing_empty_columns();
        let leading_empty_rows = shape.leading_empty_rows();
        let trailing_empty_rows = shape.trailing_empty_rows();

        //check for collision
        for height_index in leading_empty_rows..(shape.height() - trailing_empty_rows) {
            for width_index in leading_empty_columns..(shape.width() - trailing_empty_columns) {
                if shape.cells()[height_index * shape.width() + width_index] {
                    let board_x = tetromino.x() + width_index as isize - leading_empty_columns as isize;
                    let board_y = tetromino.y() + height_index as isize - leading_empty_rows as isize;
                    if board_x < 0
                        || board_y < 0
                        || board_x >= self.width as isize
                        || board_y >= self.height as isize
                    {
                        return Err("invalid tetromino position".to_string());
                    }
                    else {
                        if self.cells[(board_y * self.width() as isize + board_x) as usize] {
                            return Err("collision detected".to_string());
                        }
                    }
                }
            }
        }

        for height_index in leading_empty_rows..(shape.height() - trailing_empty_rows) {
            for width_index in leading_empty_columns..(shape.width() - trailing_empty_columns) {
                let shape_cell = shape.cells()[height_index * shape.width() + width_index];
                if shape_cell {
                    let board_x = tetromino.x() + width_index as isize - leading_empty_columns as isize;
                    let board_y = tetromino.y() + height_index as isize - leading_empty_rows as isize;
                    let board_index = board_y * self.width as isize + board_x;

                    self.cells[board_index as usize] = true;
                }
            }
        }

        Ok(())
    }

    pub fn removeTetromino(&mut self, tetromino: &Tetromino) -> Result<(), String> {
        let shape = tetromino.kind().shape(tetromino.rotation_state());

        let leading_empty_columns = shape.leading_empty_columns();
        let trailing_empty_columns = shape.trailing_empty_columns();
        let leading_empty_rows = shape.leading_empty_rows();
        let trailing_empty_rows = shape.trailing_empty_rows();

        //check for collision
        for height_index in leading_empty_rows..(shape.height() - trailing_empty_rows) {
            for width_index in leading_empty_columns..(shape.width() - trailing_empty_columns) {
                if shape.cells()[height_index * shape.width() + width_index] {
                    let board_x = tetromino.x() + width_index as isize - leading_empty_columns as isize;
                    let board_y = tetromino.y() + height_index as isize - leading_empty_rows as isize;
                    if board_x < 0
                        || board_y < 0
                        || board_x >= self.width as isize
                        || board_y >= self.height as isize
                    {
                        return Err("invalid tetromino position".to_string());
                    }
                    else {
                        if !self.cells[(board_y * self.width() as isize + board_x) as usize] {
                            return Err("no tetromino found".to_string());
                        }
                    }
                }
            }
        }

        for height_index in leading_empty_rows..(shape.height() - trailing_empty_rows) {
            for width_index in leading_empty_columns..(shape.width() - trailing_empty_columns) {
                let shape_cell = shape.cells()[height_index * shape.width() + width_index];
                if shape_cell {
                    let board_x = tetromino.x() + width_index as isize - leading_empty_columns as isize;
                    let board_y = tetromino.y() + height_index as isize - leading_empty_rows as isize;
                    let board_index = board_y * self.width as isize + board_x;

                    self.cells[board_index as usize] = false;
                }
            }
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

    pub fn rotate_tetromino(&mut self, tetromino: &mut Tetromino) -> Result<(), String> {
        self.removeTetromino(tetromino)?;
        tetromino.rotate();

        match self.setTetromino(tetromino) {
            Ok(()) => Ok(()),
            Err(error) => {
                tetromino.rotateBack();
                self.setTetromino(tetromino);
                Err(error)
            }
        }
    }

    pub fn hard_drop_tetromino(&mut self, tetromino: &mut Tetromino) {
        loop {
            match self.move_tetromino(tetromino, 0, 1) {
                Ok(()) => { }
                Err(_) => {
                    return;
                }
            }
        }
    }
}