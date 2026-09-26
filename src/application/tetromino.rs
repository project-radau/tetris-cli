#[derive(Copy, Clone)]
pub enum TetrominoKind {
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

pub struct TetrominoShape {
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

    pub fn kind(&self) -> TetrominoKind {
        self.kind
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
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

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn cells(&self) -> &Vec<bool> {
        &self.cells
    }
}

impl TetrominoKind {
    pub fn shape(&self) -> TetrominoShape {
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