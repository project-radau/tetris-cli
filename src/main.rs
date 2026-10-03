mod application;

use std::time::Duration;
use std::io;
use std::io::Write;
use std::time::Instant;
use crate::application::board::Board;
use crate::application::game_state;
use crate::application::tetromino::Tetromino;
use crate::application::tetromino::TetrominoKind;
use crate::application::tetromino_factory::TetrominoFactory;
use crate::application::game_state::GameState;
use crossterm::event;
use crossterm::event::KeyCode;

fn main() {
    const DAS_DELAY: Duration = Duration::from_millis(266);

    let mut start_level: usize = 0;
    let mut run = true;

    while run {
        let mut input = String::new();

        println!("Startlevel (0-29)?");
        std::io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<usize>() {
            Ok(value) => {
                if value > 29 {
                    println!("input not valid")
                }
                else {
                    start_level = value;
                    run = false;
                }
            },
            Err(_) => { println!("input not valid") }
        }
    }

    let mut game_state = GameState::new(start_level);
    let mut board = Board::new(10, 20);

    let factory = TetrominoFactory::new(board.width(), board.height());
    let mut tetromino = factory.spawn();

    board.setTetromino(&tetromino).unwrap();

    render(&board, &game_state);

    let mut last_fall = Instant::now();
    let mut last_das = Instant::now();
    let mut last_arr = Instant::now();
    let mut needs_render: bool = false;

    let mut left_held = false;
    let mut right_held = false;

    loop {
        
        if event::poll(Duration::from_millis(1)).unwrap() {
            let event = event::read().unwrap();
            match event {
                event::Event::Key(key_event) => {
                    match key_event.kind {
                        event::KeyEventKind::Press => {
                            match key_event.code {
                                KeyCode::Left => {
                                    left_held = true;
                                    match board.move_tetromino(&mut tetromino, -1, 0) {
                                        Ok(()) => {
                                            needs_render = true;
                                        },
                                        Err(_) => {}
                                    }
                                    last_das = Instant::now();
                                },
                                KeyCode::Right => {
                                    right_held = true;
                                    match board.move_tetromino(&mut tetromino, 1, 0) {
                                        Ok(()) => {
                                            needs_render = true;
                                        },
                                        Err(_) => {}
                                    }
                                    last_das = Instant::now();
                                },
                                KeyCode::Down => {
                                    match board.move_tetromino(&mut tetromino, 0, 1) {
                                        Ok(()) => {
                                            needs_render = true;
                                        },
                                        Err(_) => {}
                                    }
                                },
                                KeyCode::Char(' ') => {
                                    match board.rotate_tetromino(&mut tetromino) {
                                        Ok(()) => {
                                            needs_render = true;
                                        },
                                        Err(_) => {}
                                    }
                                }
                                _ => {}
                            }
                        },
                        event::KeyEventKind::Release => {
                            match key_event.code {
                                KeyCode::Left => {
                                    left_held = false;
                                },
                                KeyCode::Right => {
                                    right_held = false;
                                },
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if left_held && last_das.elapsed() >= DAS_DELAY {
            match board.move_tetromino(&mut tetromino, -1, 0) {
                Ok(()) => {
                    needs_render = true;
                },
                Err(_) => {}
            }

            last_das = Instant::now();
        }

        if right_held && last_das.elapsed() >= DAS_DELAY {
            match board.move_tetromino(&mut tetromino, 1, 0) {
                Ok(()) => {
                    needs_render = true;
                },
                Err(_) => {}
            }

            last_das = Instant::now();
        }

        if last_fall.elapsed() >= game_state.fall_delay() {
            match board.move_tetromino(&mut tetromino, 0, 1) {
                Err(_) => {
                    let cleared_rows = board.clear_full_rows();

                    if cleared_rows > 0 {
                        game_state.add_score(cleared_rows);
                        needs_render = true;
                    }

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
            render(&board, &game_state);
            needs_render = false;
        }
    }
}

fn render(board: &Board, game_state: &GameState) {
    //clear terminal
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush().unwrap();
    board.render(game_state);
}
