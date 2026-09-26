enum TetrominoKind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L
}

pub struct Tetromino {
    kind: TetrominoKind,
    x: usize,
    y: usize
}

struct TetrominoShape {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Tetromino {
    pub fn new(kind: TetrominoKind, x: usize, y: usize) -> Self {
        Self {
            kind,
            x,
            y
        }
    }
}

impl TetrominoShape {
    pub fn new(width: usize, height: usize, cells: Vec<bool>) -> Self {
        Self {
            width,
            height,
            cells
        }
    }
}

impl TetrominoKind {
    fn shape(&self) -> TetrominoShape {
        match self {
            TetrominoKind::I => {
                TetrominoShape::new(
                    1, 
                    4,
                    vec![
                        true,
                        true,
                        true,
                        true
                    ]
                )
            },
            TetrominoKind::O => {
                TetrominoShape::new(
                    2, 
                    2,
                    vec![
                        true, true,
                        true, true
                    ]
                )
            }
            TetrominoKind::T => {
                TetrominoShape::new(
                    3,
                    2,
                    vec![
                        true, true, true,
                        false, true, false
                    ]
                )
            }
            TetrominoKind::S => {
                TetrominoShape::new(
                    3,
                    2,
                    vec![
                        false, true, true,
                        true, true, false
                    ]
                )
            }
            TetrominoKind::Z => {
                TetrominoShape::new(
                    3,
                    2,
                    vec![
                        true, true, false,
                        false, true, true
                    ]
                )
            }
            TetrominoKind::J => {
                TetrominoShape::new(
                    2,
                    3,
                    vec![
                        false, true,
                        false, true,
                        true, true
                    ]
                )
            }
            TetrominoKind::L => {
                TetrominoShape::new(
                    2,
                    3,
                    vec![
                        true, false,
                        true, false,
                        true, true
                    ]
                )
            }
        }
    }
}