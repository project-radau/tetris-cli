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
    x: isize,
    y: isize,
    rotation_state: u8
}

pub struct TetrominoShape {
    width: usize,
    height: usize,
    cells: Vec<bool>
}

pub struct TetrominoBag {
    items: Vec<TetrominoKind>
}

impl Tetromino {
    pub fn new(kind: TetrominoKind, x: isize, y: isize) -> Self {
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

    pub fn x(&self) -> isize {
        self.x
    }

    pub fn y(&self) -> isize {
        self.y
    }

    pub fn rotation_state(&self) -> u8 {
        self.rotation_state
    }

    pub fn move_by(&mut self, dx: isize, dy: isize) {
        self.x += dx;
        self.y += dy;
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

    fn rotate_shape_3x3(cells: Vec<bool>, pivot_x: usize, pivot_y: usize, amount_rotations: u8) -> Vec<bool> {
        let width = 3;
        let height = 3;
        let mut current_matrix = cells;

        let real_amount_rotations = amount_rotations % 4;

        for _ in 0..real_amount_rotations {
            let mut rotated_matrix = vec![false; width * height];

            for index_y in 0..height {
                for index_x in 0..width {
                    let base_index = index_y * width + index_x;
                    if current_matrix[base_index] {
                        let rotation_index_x = pivot_x as isize - (index_y as isize - pivot_y as isize);
                        let rotation_index_y = pivot_y as isize + (index_x as isize - pivot_x as isize);
                        let rotation_index = rotation_index_y * width as isize + rotation_index_x;
                        rotated_matrix[rotation_index as usize] = true;
                    }
                }
            }
            current_matrix = rotated_matrix;
        }
        
        current_matrix
    }

    // parameters pivot_x and pivot_y must be doubled
    fn rotate_shape_4x4(cells: Vec<bool>, pivot_x: isize, pivot_y: isize, amount_rotations: u8) -> Vec<bool> {
        let width = 4;
        let height = 4;
        let mut current_matrix = cells;

        let real_amount_rotations = amount_rotations % 4;

        for _ in 0..real_amount_rotations {
            let mut rotated_matrix = vec![false; width * height];

            for index_y in 0..height {
                for index_x in 0..width {
                    let base_index = index_y * width + index_x;
                    if current_matrix[base_index] {
                        let rotation_index_x = (pivot_x as isize - (2 * index_y as isize - pivot_y as isize)) / 2;
                        let rotation_index_y = (pivot_y as isize + (2 * index_x as isize - pivot_x as isize)) / 2;
                        let rotation_index = rotation_index_y * width as isize + rotation_index_x;
                        rotated_matrix[rotation_index as usize] = true;
                    }
                }
            }
            current_matrix = rotated_matrix;
        }
        
        current_matrix
    }

    pub fn leading_empty_columns(&self) -> usize {
        let mut ret: usize = 0;
        for index_x in 0..self.width() {
            for index_y in 0..self.height() {
                let cell_index = index_y * self.width + index_x;
                if self.cells()[cell_index] {
                    return ret;
                }
            }
            ret += 1;
        }
        ret
    }

    pub fn trailing_empty_columns(&self) -> usize {
        let mut ret: usize = 0;
        for index_x in (0..self.width()).rev() {
            for index_y in 0..self.height() {
                let cell_index = index_y * self.width + index_x;
                if self.cells()[cell_index] {
                    return ret;
                }
            }
            ret += 1;
        }
        ret
    }

    pub fn leading_empty_rows(&self) -> usize {
        let mut ret: usize = 0;
        for index_y in 0..self.height() {
            for index_x in 0..self.width() {
                let cell_index = index_y * self.width + index_x;
                if self.cells()[cell_index] {
                    return ret;
                }
            }
            ret += 1;
        }
        ret
    }

    pub fn trailing_empty_rows(&self) -> usize {
        let mut ret: usize = 0;
        for index_y in (0..self.height()).rev() {
            for index_x in 0..self.width() {
                let cell_index = index_y * self.width + index_x;
                if self.cells()[cell_index] {
                    return ret;
                }
            }
            ret += 1;
        }
        ret
    }
}

impl TetrominoKind {
    pub fn shape(&self, rotation_state: u8) -> TetrominoShape {
        
        if rotation_state > 3 {
            panic!("invalid rotation");
        }
        
        match self {
            TetrominoKind::I => {
                let base_shape = vec![
                    false, true, false, false,
                    false, true, false, false,
                    false, true, false, false,
                    false, true, false, false
                ];
                //doubled, actually it's 1.5
                let pivot_x = 3;
                let pivot_y = 3;
                TetrominoShape::new(
                    4,
                    4,
                    TetrominoShape::rotate_shape_4x4(base_shape, pivot_x, pivot_y, rotation_state)
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
                let base_shape = vec![
                    false, false, false,
                    true, true, true,
                    false, true, false
                ];
                let pivot_x = 1;
                let pivot_y = 1;
                TetrominoShape::new(
                    3,
                    3,
                    TetrominoShape::rotate_shape_3x3(base_shape, pivot_x, pivot_y, rotation_state)
                )
            }
            TetrominoKind::S => {
                let base_shape = vec![
                    false, false, false,
                    false, true, true,
                    true, true, false
                ];
                let pivot_x = 1;
                let pivot_y = 1;
                TetrominoShape::new(
                    3,
                    3,
                    TetrominoShape::rotate_shape_3x3(base_shape, pivot_x, pivot_y, rotation_state)
                )
            }
            TetrominoKind::Z => {
                let base_shape = vec![
                    false, false, false,
                    true, true, false,
                    false, true, true
                ];
                let pivot_x = 1;
                let pivot_y = 1;
                TetrominoShape::new(
                    3,
                    3,
                    TetrominoShape::rotate_shape_3x3(base_shape, pivot_x, pivot_y, rotation_state)
                )
            }
            TetrominoKind::J => {
                let base_shape = vec![
                    false, true, false,
                    false, true, false,
                    true, true, false
                ];
                let pivot_x = 1;
                let pivot_y = 1;
                TetrominoShape::new(
                    3,
                    3,
                    TetrominoShape::rotate_shape_3x3(base_shape, pivot_x, pivot_y, rotation_state)
                )
            }
            TetrominoKind::L => {
                let base_shape = vec![
                    false, true, false,
                    false, true, false,
                    false, true, true
                ];
                let pivot_x = 1;
                let pivot_y = 1;
                TetrominoShape::new(
                    3,
                    3,
                    TetrominoShape::rotate_shape_3x3(base_shape, pivot_x, pivot_y, rotation_state)
                )
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