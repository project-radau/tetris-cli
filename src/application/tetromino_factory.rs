use crate::TetrominoKind;
use crate::Tetromino;

pub struct TetrominoFactory {
    width: usize,
    height: usize
}

impl TetrominoFactory {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height
        }
    }

    fn get_random_tetromino_kind() -> TetrominoKind {
        match rand::random_range(0..7){
             0 => {
                TetrominoKind::I
             },
             1 => {
                TetrominoKind::O
             },
             2 => {
                TetrominoKind::T
             },
             3 => {
                TetrominoKind::S
             },
             4 => {
                TetrominoKind::Z
             },
             5 => {
                TetrominoKind::J
             },
             6 => {
                TetrominoKind::L
             },
             _ => panic!("error generating random tetromino")
        }
    }

    pub fn spawn(&self) -> Tetromino {
        let kind = TetrominoFactory::get_random_tetromino_kind();
        let shape = kind.shape();
        let spawn_position = (self.width / 2) - (shape.width() / 2);
        Tetromino::new(kind, spawn_position, 0)
    }
    
}