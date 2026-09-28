mod application;

use std::thread;
use std::time::Duration;
use std::io::Write;
use crate::application::board::Board;
use crate::application::tetromino::Tetromino;
use crate::application::tetromino::TetrominoKind;
use crate::application::tetromino_factory::TetrominoFactory;

fn main() {
    let mut board = Board::new(10, 6);

    let factory = TetrominoFactory::new(board.width(), board.height());
    let mut tetromino = factory.spawn();

    board.setTetromino(&tetromino).unwrap();

    loop {
        //clear terminal
        print!("\x1B[2J\x1B[H");
        std::io::stdout().flush().unwrap();
        board.render();

        match board.move_tetromino(&mut tetromino, 0, 1) {
            Err(_) => {
                tetromino = factory.spawn();
                match board.setTetromino(&tetromino) {
                    Ok(()) => { }
                    Err(_) => {
                        println!("game over!");
                        return;
                    }
                }
            }
            Ok(()) => {}
        }
        thread::sleep(Duration::from_millis(200));
    }
}
