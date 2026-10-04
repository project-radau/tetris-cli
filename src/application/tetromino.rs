use rand::seq::SliceRandom;

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
    y: usize,
    rotation_state: u8
}

pub struct TetrominoShape {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

pub struct TetrominoBag {
    items: Vec<TetrominoKind>
}

impl Tetromino {
    pub fn new(kind: TetrominoKind, x: usize, y: usize) -> Self {
        Self {
            kind,
            x,
            y,
            rotation_state: 0 as u8
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

    pub fn rotation_state(&self) -> u8 {
        self.rotation_state
    }

    pub fn move_by(&mut self, dx: isize, dy: isize) {
        let new_x = self.x as isize + dx;
        let new_y = self.y as isize + dy;

        if new_x >= 0 && new_y >= 0 {
            self.x = new_x as usize;
            self.y = new_y as usize;
        }
    }

    pub fn rotate(&mut self) {
        self.rotation_state = (self.rotation_state + 1) % 4;
    }

    pub fn rotateBack(&mut self) {
        self.rotation_state = (self.rotation_state + 3) % 4;
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
    pub fn shape(&self, rotation_state: u8) -> TetrominoShape {
        
        if rotation_state > 3 {
            panic!("invalid rotation");
        }
        
        match self {
            TetrominoKind::I => {
                match rotation_state {
                    0 => {
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
                    1 => {
                        TetrominoShape::new(
                            4, 
                            1,
                            vec![
                                true, true, true, true
                            ]
                        )
                    },
                    2 => {
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
                    3 => {
                        TetrominoShape::new(
                            4, 
                            1,
                            vec![
                                true, true, true, true
                            ]
                        )
                    },
                    _ => panic!("invalid rotation")
                }
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
                match rotation_state {
                    0 => {
                        TetrominoShape::new(
                            3,
                            2,
                            vec![
                                true, true, true,
                                false, true, false
                            ]
                        )
                    },
                    1 => {
                        TetrominoShape::new(
                            2,
                            3,
                            vec![
                                false, true, 
                                true, true, 
                                false, true
                            ]
                        )
                    },
                    2 => {
                        TetrominoShape::new(
                            3,
                            2,
                            vec![
                                false, true, false,
                                true, true, true
                            ]
                        )
                    },
                    3 => {
                        TetrominoShape::new(
                            2,
                            3,
                            vec![
                                true, false, 
                                true, true, 
                                true, false
                            ]
                        )
                    },
                    _ => panic!("invalid rotation")
                }
            }
            TetrominoKind::S => {
                match rotation_state {
                    0 => {
                        TetrominoShape::new(
                            3,
                            2,
                            vec![
                                false, true, true,
                                true, true, false
                            ]
                        )
                    },
                    1 => {
                        TetrominoShape::new(
                            2,
                            3,
                            vec![
                                true, false,
                                true, true,
                                false, true
                            ]
                        )
                    },
                    2 => {
                        TetrominoShape::new(
                            3,
                            2,
                            vec![
                                false, true, true,
                                true, true, false
                            ]
                        )
                    },
                    3 => {
                        TetrominoShape::new(
                            2,
                            3,
                            vec![
                                true, false,
                                true, true,
                                false, true
                            ]
                        )
                    },
                    _ => panic!("invalid rotation")
                }
            }
            TetrominoKind::Z => {
                match rotation_state {
                    0 => {
                        TetrominoShape::new(
                            3,
                            2,
                            vec![
                                true, true, false,
                                false, true, true
                            ]
                        )
                    },
                    1 => {
                        TetrominoShape::new(
                            2,
                            3,
                            vec![
                                false, true,
                                true, true,
                                true, false
                            ]
                        )
                    },
                    2 => {
                        TetrominoShape::new(
                            3,
                            2,
                            vec![
                                true, true, false,
                                false, true, true
                            ]
                        )
                    },
                    3 => {
                        TetrominoShape::new(
                            2,
                            3,
                            vec![
                                false, true,
                                true, true,
                                true, false
                            ]
                        )
                    },
                    _ => panic!("invalid rotation")
                }
            }
            TetrominoKind::J => {
                match rotation_state {
                    0 => {
                        TetrominoShape::new(
                            2,
                            3,
                            vec![
                                false, true,
                                false, true,
                                true, true
                            ]
                        )
                    },
                    1 => {
                        TetrominoShape::new(
                            3,
                            2,
                            vec![
                                true, false, false,
                                true, true, true
                            ]
                        )
                    },
                    2 => {
                        TetrominoShape::new(
                            2,
                            3,
                            vec![
                                true, true,
                                true, false,
                                true, false
                            ]
                        )
                    },
                    3 => {
                        TetrominoShape::new(
                            3,
                            2,
                            vec![
                                true, true, true,
                                false, false, true
                            ]
                        )
                    },
                    _ => panic!("invalid rotation")
                }
            }
            TetrominoKind::L => {
                match rotation_state {
                    0 => {
                        TetrominoShape::new(
                            2,
                            3,
                            vec![
                                true, false,
                                true, false,
                                true, true
                            ]
                        )
                    },
                    1 => {
                        TetrominoShape::new(
                            3,
                            2,
                            vec![
                                true, true, true,
                                true, false, false
                            ]
                        )
                    },
                    2 => {
                        TetrominoShape::new(
                            2,
                            3,
                            vec![
                                true, true,
                                false, true,
                                false, true
                            ]
                        )
                    },
                    3 => {
                        TetrominoShape::new(
                            3,
                            2,
                            vec![
                                false, false, true,
                                true, true, true
                            ]
                        )
                    },
                    _ => panic!("invalid rotation")
                }
            }
        }
    }
}

impl TetrominoBag {
    fn get_random_tetromino_kind_bag_items() -> Vec<TetrominoKind> {
        let mut rng = rand::rng();
        let mut items = vec![TetrominoKind::I, TetrominoKind::O, TetrominoKind::T, TetrominoKind::S, TetrominoKind::Z, TetrominoKind::J, TetrominoKind::L];
        items.shuffle(&mut rng);
        items
    }

    pub fn new() -> Self {
        Self {
            items: TetrominoBag::get_random_tetromino_kind_bag_items()
        }
    }

    pub fn next(&mut self) -> TetrominoKind {
        match self.items.pop() {
            Some(item) => {
                item
            }
            None => {
                self.items = TetrominoBag::get_random_tetromino_kind_bag_items();
                let item = self.items.pop().unwrap();
                item
            }
        }
    }
}