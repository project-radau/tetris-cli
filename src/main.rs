mod application;

use crate::application::board::Board;
use crate::application::tetromino::Tetromino;
use crate::application::tetromino::TetrominoKind;

fn main() {
    let mut board = Board::new(10, 6);

    board.setTetromino(Tetromino::new(TetrominoKind::T, 3, 3)).unwrap();

    board.render();
}
