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
