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
    const ARR_DELAY: Duration = Duration::from_millis(100);
    const ROTATION_DEBOUNCE_FRAMES: u64 = 6;
    const LOCK_DELAY_FRAMES: u64 = 6;

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
    let mut current_tetromino = factory.spawn();
    let mut next_tetromino  = factory.spawn();

    board.setTetromino(&current_tetromino).unwrap();

    render(&board, &game_state, &next_tetromino);

    let mut last_fall = Instant::now();
    let mut last_das = Instant::now();
    let mut last_arr = Instant::now();
    let mut last_rotation = Instant::now();
    let mut last_lock_delay = Instant::now();
    let mut das_finished = false;
    let mut needs_render: bool = false;

    let mut left_held = false;
    let mut right_held = false;

    let mut down_held = false;

    let mut lock_delay_active = false;
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
                                    das_finished = false;
                                    match board.move_tetromino(&mut current_tetromino, -1, 0) {
                                        Ok(()) => {
                                            needs_render = true;
                                        },
                                        Err(_) => {}
                                    }
                                    last_das = Instant::now();
                                },
                                KeyCode::Right => {
                                    right_held = true;
                                    match board.move_tetromino(&mut current_tetromino, 1, 0) {
                                        Ok(()) => {
                                            needs_render = true;
                                        },
                                        Err(_) => {}
                                    }
                                    last_das = Instant::now();
                                },
                                KeyCode::Down => {
                                    down_held = true;
                                },
                                KeyCode::Char(' ') => {
                                    if last_rotation.elapsed() >= Duration::from_millis(ROTATION_DEBOUNCE_FRAMES * 1000 / 60) {
                                        match board.rotate_tetromino(&mut current_tetromino) {
                                            Ok(()) => {
                                                needs_render = true;
                                            },
                                            Err(_) => {}
                                        }
                                        last_rotation = Instant::now();
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
                                KeyCode::Down => {
                                    down_held = false;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if left_held {
            if last_das.elapsed() >= DAS_DELAY {
                match board.move_tetromino(&mut current_tetromino, -1, 0) {
                    Ok(()) => {
                        needs_render = true;
                    },
                    Err(_) => {}
                }
                das_finished = true;
                last_arr = Instant::now();
            }

            if das_finished && last_arr.elapsed() >= ARR_DELAY {
                match board.move_tetromino(&mut current_tetromino, -1, 0) {
                    Ok(()) => {
                        needs_render = true;
                    },
                    Err(_) => {}
                }
                last_arr = Instant::now();
            }
        }

        if right_held {
            if last_das.elapsed() >= DAS_DELAY {
                match board.move_tetromino(&mut current_tetromino, 1, 0) {
                    Ok(()) => {
                        needs_render = true;
                    },
                    Err(_) => {}
                }
                das_finished = true;
                last_arr = Instant::now();
            }

            if das_finished && last_arr.elapsed() >= ARR_DELAY {
                match board.move_tetromino(&mut current_tetromino, 1, 0) {
                    Ok(()) => {
                        needs_render = true;
                    },
                    Err(_) => {}
                }
                last_arr = Instant::now();
            }
        }

        let fall_delay = if down_held {
            Duration::from_millis(16)
        } else {
            game_state.fall_delay()
        };

        if last_fall.elapsed() >= fall_delay {
            match board.move_tetromino(&mut current_tetromino, 0, 1) {
                Err(_) => {

                    if lock_delay_active && last_lock_delay.elapsed() >= Duration::from_millis(LOCK_DELAY_FRAMES * 1000 / 60) {
                        lock_delay_active = false;
                        
                        let cleared_rows = board.clear_full_rows();

                        if cleared_rows > 0 {
                            game_state.add_score(cleared_rows);
                        }

                        current_tetromino = next_tetromino;
                        next_tetromino = factory.spawn();
                        match board.setTetromino(&current_tetromino) {
                            Ok(()) => { 
                                lock_delay_active = false;
                                needs_render = true;
                            }
                            Err(_) => {
                                println!("game over!");
                                return;
                            }
                        }
                    }
                    else {
                        if !lock_delay_active {
                            lock_delay_active = true;
                            last_lock_delay = Instant::now();
                        }
                    }
                }
                Ok(()) => { needs_render = true; }
            }
            last_fall = Instant::now();
        }
        if needs_render {
            render(&board, &game_state, &next_tetromino);
            needs_render = false;
        }
    }
}

fn render(board: &Board, game_state: &GameState, next_tetromino: &Tetromino) {
    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::Hide
    ).unwrap();

    render_board(board);
    render_stats(board, game_state);
    render_next_piece(board, next_tetromino);

    crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveTo(0 as u16, (board.height() + 2) as u16)).unwrap();

    std::io::stdout().flush().unwrap();
}

fn render_board(board: &Board) {
    //clear terminal
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush().unwrap();
    board.render();
}

fn render_stats(board: &Board, game_state: &GameState) {
    let x = board.width() + 3;
    let y = 0;

    game_state.render(x as u16, y as u16);
}

fn render_next_piece(board: &Board, tetromino: &Tetromino) {
    let x = board.width() + 3;
    let y = 5;

    crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveTo(x as u16, y as u16)).unwrap();
    let shape = tetromino.kind().shape(tetromino.rotation_state());

    for index_y in 0..shape.height() {
        crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveTo(x as u16, (y + index_y) as u16)).unwrap();
        for index_x in 0..shape.width() {
            if shape.cells()[index_y * shape.width() + index_x] {
                print!("#");
            }
            else {
                print!(" ");
            }
        }
    }
}
