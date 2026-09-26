mod application;

use crate::application::board::Board;

fn main() {
    let board = Board::new(10, 10);
    board.render();
}
