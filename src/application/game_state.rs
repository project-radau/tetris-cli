use std::time::Duration;

pub struct GameState {
    score: usize,
    level: usize,
    lines_cleared: usize
}

impl GameState {
    pub fn new(start_level: usize) -> Self {
        Self {
            score: 0,
            level: start_level,
            lines_cleared: start_level * 10
        }
    }

    pub fn add_score(&mut self, cleared_rows: usize) {
        self.lines_cleared += cleared_rows;
        self.level = self.lines_cleared / 10;
        
        match cleared_rows {
            1 => self.score += 40 * (self.level + 1),
            2 => self.score += 100 * (self.level + 1),
            3 => self.score += 300 * (self.level + 1),
            4 => self.score += 1200 * (self.level + 1),
            _ => {}
        }
    }

    pub fn fall_delay(&self) -> Duration {
        const GRAVITY_FRAMES: [u64; 30] = [
            48, 43, 38, 33, 28, 23, 18, 13, 8, 6,
            5, 5, 5, 4, 4, 4, 3, 3, 3, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 1,
        ];

        let frames = GRAVITY_FRAMES[self.level.min(29)];

        Duration::from_millis(frames * 1000 / 60)
    }

    pub fn render(&self, x: u16, y: u16) {
        crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveTo(x, y)).unwrap();
        print!("Score: {}", self.score);
        crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveTo(x, y+1)).unwrap();
        println!("Level: {}", self.level);
        crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveTo(x, y+2)).unwrap();
        println!("Lines: {}", self.lines_cleared);
    }
}