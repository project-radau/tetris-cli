mod application;

use std::thread;
use std::time::Duration;
use crate::application::board::Board;
use crate::application::tetromino::Tetromino;
use crate::application::tetromino::TetrominoKind;
use crate::application::tetromino_factory::TetrominoFactory;

fn main() {
    let mut board = Board::new(10, 6);

    let factory = TetrominoFactory::new(10, 20);
    let tetromino = factory.spawn();

    board.setTetromino(&tetromino);

    board.render();
}
