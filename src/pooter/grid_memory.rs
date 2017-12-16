use std::cmp::max;
use std::ops::{Add,AddAssign,Sub,SubAssign};

#[derive(Clone,Copy,Debug)]
struct MemPoint(i64, i64);
#[derive(Clone,Copy,Debug)]
struct PointerVelocity(i64,i64);

// impl MemPoint {
//     pub fn x_off(&self) -> i64 { self.x - CENTER }
//     pub fn y_off(&self) -> i64 { self.y - CENTER }
// }

impl Add for MemPoint {
    type Output = MemPoint;

    fn add(self, rhs: Self) -> Self {
        MemPoint(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<'a> Add<&'a PointerVelocity> for MemPoint {
    type Output = MemPoint;
    
    fn add(self, rhs: &'a PointerVelocity) -> Self {
        let mut s = self.clone();
        s += rhs;
        s
    }
}

impl AddAssign for MemPoint {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<'a> AddAssign<&'a PointerVelocity> for MemPoint {
    fn add_assign(&mut self, rhs: &'a PointerVelocity) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for MemPoint {
    type Output = MemPoint;
    
    fn sub(self, rhs: Self) -> Self {
        MemPoint(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for MemPoint {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

pub struct MemoryGrid {
    current_size: i64,
    grid: Vec<Vec<i64>>,
    loc: MemPoint,
    vel: PointerVelocity
}

impl MemoryGrid {
    pub fn new() -> Self {
        let current_size: i64 = 1001;
        let center = (current_size - 1) / 2;

        let mut grid = vec![vec![0; current_size as usize]; current_size as usize];
        grid[center as usize][center as usize] = 1;

        Self {
            current_size: current_size,
            grid: grid,
            loc: MemPoint(center + 1, center),
            vel: PointerVelocity(0,-1)
        }
    }

    pub fn get(&self) -> i64        { self.grid[self.loc.0 as usize][self.loc.1 as usize] }
    pub fn set(&mut self, val: i64) { self.grid[self.loc.0 as usize][self.loc.1 as usize] = val; }

    pub fn next(&mut self) {
        self.loc += &self.vel; // move to next cell

        // since the grid is declared to be infinite, grow as needed. Of course,
        // if you call next() too much, you'll eventually fill all your real computer's
        // memory, but that's not my problem.
        if self.radius() + 1 >= self.current_size { self.grow(); }

        // change heading at each "corner". The last case is when moving out a ring.
        let r = self.radius();
        let (x,y) = (self.x_off(), self.y_off());

        if      x ==  r && y == -r { self.vel = PointerVelocity(-1,  0) }
        else if x == -r && y == -r { self.vel = PointerVelocity( 0,  1) }
        else if x == -r && y ==  r { self.vel = PointerVelocity( 1,  0) }
        else if x == r && y == r-1 { self.vel = PointerVelocity( 0, -1) }
    }

    pub fn neighbour_sum(&self) -> i64 {
        // NOTE: this function will explode if called when self.loc is on an edge.
        // with the grow() function, that may never happen though.
        let g = &self.grid;
        let l = &self.loc;

        let cx = l.0 as usize;
        let cy = l.1 as usize;

        g[cx - 1][cy - 1] + g[cx    ][cy - 1] + g[cx + 1][cy - 1] + 
        g[cx - 1][cy    ] + 0                 + g[cx + 1][cy    ] + 
        g[cx - 1][cy + 1] + g[cx    ][cy + 1] + g[cx + 1][cy + 1]
    }

    pub fn print(&self) {
        let tx = self.x_off().abs();
        let ty = self.y_off().abs();
        for y in (self.center().1-ty)..=(self.center().1+ty) {
            for x in (self.center().0-tx)..=(self.center().0+tx) {
                print!("{:06} ", self.grid[x as usize][y as usize]);
            }
            println!("");
        }
    }

    fn center(&self) -> MemPoint { MemPoint((self.current_size - 1) / 2, (self.current_size - 1) / 2) }
    fn radius(&self) -> i64    { max(self.x_off().abs(), self.y_off().abs()) }
    fn x_off(&self)  -> i64      { self.loc.0 - self.center().0 }
    fn y_off(&self)  -> i64      { self.loc.1 - self.center().1 }

    fn grow(&mut self) {
        let old_center = self.center();
        let r = self.radius(); // needed for copying old values
        let (oxo,oyo) = (self.x_off(), self.y_off());

        let new_size = (self.current_size - 1) * 2 + 1; // eg go from 1001 to 2001 to 4001, etc...
        let new_center = (new_size - 1) / 2;

        let mut new_grid = vec![vec![0; new_size as usize]; new_size as usize];
        for x in -r..=r {
            let nx = (new_center + x) as usize;
            let ox = (old_center.0 + x) as usize;
            for y in -r..=r {
                let ny = (new_center + y) as usize;
                let oy = (old_center.1 + y) as usize;
                new_grid[nx][ny] = self.grid[ox][oy];
            }
        }

        self.current_size = new_size;
        self.grid = new_grid;
        self.loc = MemPoint((new_center + oxo), (new_center + oyo));
    }
}