use std::cmp::{max};

pub fn part1() {
    // done by hand
}

const EXTENT: usize = 1001;
const CENTER: usize = 500;

#[derive(Copy)]
struct MemPoint { x: usize, y: usize }

impl MemPoint {
    // pub fn new() -> MemPoint {
    //     MemPoint {
    //         x: CENTER,
    //         y: CENTER
    //     }
    // }

    pub fn x_off(&self) -> i32 { self.x as i32 - CENTER as i32 }
    pub fn y_off(&self) -> i32 { self.y as i32 - CENTER as i32 }
    // pub fn dist(&self)  -> usize { (self.x_off().abs() + self.y_off().abs()) as usize }
    pub fn add(&mut self, (x,y): (i32,i32)) {
        self.x = (self.x as i32 + x) as usize;
        self.y = (self.y as i32 + y) as usize;
    }
}

impl Clone for MemPoint {
    fn clone(&self) -> MemPoint { *self }
}

struct Grid {
    grid: Vec<Vec<u32>>,
    loc: MemPoint,
    vel: (i32,i32)
}

impl Grid {
    pub fn new() -> Grid {
        let mut grid = vec![vec![0; EXTENT]; EXTENT];
        grid[CENTER][CENTER] = 1;

        Grid {
            grid: grid,
            loc: MemPoint { x: CENTER + 1, y: CENTER },
            vel: (0,-1)
        }
    }

    fn radius(&self) -> i32 {
        max(self.loc.x_off().abs(),self.loc.y_off().abs())
    }

    pub fn next(&mut self) {
        self.loc.add(self.vel);
        let r = self.radius() as i32;

        let (x,y) = (self.loc.x_off(), self.loc.y_off());

        //println!("({},{}),{}",x,y,r);

        if      x ==  r && y == -r { /*println!("turning base");*/ self.vel = (-1,  0) }
        else if x == -r && y == -r { /*println!("turning downwind");*/ self.vel = ( 0,  1) }
        else if x == -r && y ==  r { /*println!("turning crosswind");*/ self.vel = ( 1,  0) }
        else if x == r && y == r-1 { /*println!("turning final");*/ self.vel = ( 0, -1) }
    }

    pub fn get(&self) -> u32 { self.grid[self.loc.x as usize][self.loc.y as usize] }
    pub fn set(&mut self, val: u32) { self.grid[self.loc.x as usize][self.loc.y as usize] = val; }
    // pub fn move_to(&mut self, p: MemPoint) { self.loc = p; }
    // pub fn at(&self) -> MemPoint { self.loc }

    pub fn neighbour_sum(&self) -> u32{
        let g = &self.grid;
        let l = &self.loc;

        //println!("Getting neighbour sum at ({},{})", l.x, l.y);
        
        g[l.x-1][l.y-1] + g[l.x  ][l.y-1] + g[l.x+1][l.y-1] +
        g[l.x-1][l.y  ] + 0               + g[l.x+1][l.y  ] +
        g[l.x-1][l.y+1] + g[l.x  ][l.y+1] + g[l.x+1][l.y+1]
    }

    // pub fn dist(&self) -> usize { self.loc.dist() }

    pub fn print(&self) {
        let tx = self.loc.x_off().abs() as usize;
        let ty = self.loc.y_off().abs() as usize;
        for y in (CENTER-ty)..=(CENTER+ty) {
            for x in (CENTER-tx)..=(CENTER+tx) {
                print!("{:06} ", self.grid[x][y]);
            }
            println!("");
        }
    }
}

pub fn part2() {
    let mut grid = Grid::new();

    loop {
        let n_sum = grid.neighbour_sum();
        grid.set(n_sum);
        if grid.get() > 277678 { break; }
        grid.next();
    }
    grid.print();
    println!("The value is {}", grid.get());
}