use crate::Tetromino;
use crate::application::tetromino::TetrominoBag;

pub struct TetrominoFactory {
    width: usize,
    bag: TetrominoBag
}

impl TetrominoFactory {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            bag: TetrominoBag::new()
        }
    }

    pub fn spawn(&mut self) -> Tetromino {
        let kind = self.bag.next();
        let shape = kind.shape(0);
        let spawn_position = (self.width / 2) - (shape.width() / 2);
        Tetromino::new(kind, spawn_position as isize, 0)
    }
    
}