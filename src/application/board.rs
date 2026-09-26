pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![vec![false; width]; height]
        }
    }

    pub fn render(&self) {
        for rows in &self.cells {
            for cell in rows {
                if *cell {
                    print!("#");
                }
                else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }
}