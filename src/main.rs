mod application;

use std::thread;
use std::time::Duration;
use crate::application::board::Board;
use crate::application::tetromino::Tetromino;
use crate::application::tetromino::TetrominoKind;

fn main() {
    let mut board = Board::new(10, 6);

    loop {

        

        thread::sleep(Duration::from_millis(500));
    }
}
