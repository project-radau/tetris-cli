mod application;

use crate::application::board::Board;

fn main() {
    let mut board = Board::new(10, 6);

    board.set(2, 1, true).unwrap();
    board.set(3, 1, true).unwrap();
    board.set(4, 1, true).unwrap();

    board.render();
}
