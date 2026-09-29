mod application;

use std::thread;
use std::time::Duration;
use std::io::Write;
use std::time::Instant;
use crate::application::board::Board;
use crate::application::tetromino::Tetromino;
use crate::application::tetromino::TetrominoKind;
use crate::application::tetromino_factory::TetrominoFactory;
use crossterm::event;
use crossterm::event::KeyCode;

fn main() {
    let mut board = Board::new(10, 20);

    let factory = TetrominoFactory::new(board.width(), board.height());
    let mut tetromino = factory.spawn();

    board.setTetromino(&tetromino).unwrap();

    render(&board);

    let mut last_fall = Instant::now();
    let mut last_input = Instant::now();
    let mut needs_render: bool = false;

    loop {
        if event::poll(Duration::from_millis(1)).unwrap() {
            let event = event::read().unwrap();

            match event {
                event::Event::Key(key_event) => {
                    if last_input.elapsed() >= Duration::from_millis(100) {
                        match key_event.code {
                            KeyCode::Left => {
                                match board.move_tetromino(&mut tetromino, -1, 0) {
                                    Ok(()) => {
                                        last_input = Instant::now();
                                        needs_render = true;
                                    },
                                    Err(_) => {}
                                }
                            },
                            KeyCode::Right => {
                                match board.move_tetromino(&mut tetromino, 1, 0) {
                                    Ok(()) => {
                                        last_input = Instant::now();
                                        needs_render = true;
                                    },
                                    Err(_) => {}
                                }
                            },
                            KeyCode::Down => {
                                match board.move_tetromino(&mut tetromino, 0, 1) {
                                    Ok(()) => {
                                        last_input = Instant::now();
                                        needs_render = true;
                                    },
                                    Err(_) => {}
                                }
                            },
                            KeyCode::Char(' ') => {
                                match board.rotate_tetromino(&mut tetromino) {
                                    Ok(()) => {
                                        last_input = Instant::now();
                                        needs_render = true;
                                    },
                                    Err(_) => {}
                                }
                            }
                            _ => {}
                        }
                        
                    }
                }
                _ => {}
            }
        }

        if last_fall.elapsed() >= Duration::from_millis(200) {
            match board.move_tetromino(&mut tetromino, 0, 1) {
                Err(_) => {
                    tetromino = factory.spawn();
                    match board.setTetromino(&tetromino) {
                        Ok(()) => { needs_render = true; }
                        Err(_) => {
                            println!("game over!");
                            return;
                        }
                    }
                }
                Ok(()) => { needs_render = true; }
            }
            last_fall = Instant::now();
        }
        if needs_render {
            render(&board);
            needs_render = false;
        }
    }
}

fn render(board: &Board) {
    //clear terminal
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush().unwrap();
    board.render();
}
