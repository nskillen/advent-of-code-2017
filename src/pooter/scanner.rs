#[derive(Clone)]
pub struct Scanner {
    position: usize,
    range: usize,
    velocity: i32
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner { position: 1, range: 1, velocity: 1 }
    }

    pub fn with_range(range: usize) -> Scanner {
        Scanner { position: 1, range: range, velocity: 1 }
    }

    pub fn step(&mut self) {
        self.position = (self.position as i32 + self.velocity) as usize;
        if self.position == 1 { self.velocity = 1; }
        else if self.position == self.range { self.velocity = -1; }
    }

    pub fn at_top(&self) -> bool {
        self.position == 1
    }

    pub fn reset(&mut self) {
        self.position = 1;
        self.velocity = 1;
    }

    pub fn get_range(&self) -> usize { self.range }
}
