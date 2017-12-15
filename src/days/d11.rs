use super::read_file;
use std::cmp::max;

struct Hex {
    q: i32,
    r: i32,
    s: i32,
}

impl Hex {
    fn new() -> Hex {
        Hex { q: 0, r: 0, s: 0 }
    }

    fn distance_to(&self, other: &Hex) -> i32 {
        ((self.q - other.q).abs() + (self.r - other.r).abs() + (self.s - other.s).abs()) / 2
    }

    fn move_to(&self, dir: &str) -> Hex {
        match dir {
            "n"  => Hex{ q: self.q,     r: self.r + 1, s: self.s - 1 },
            "ne" => Hex{ q: self.q + 1, r: self.r,     s: self.s - 1 },
            "se" => Hex{ q: self.q - 1, r: self.r + 1, s: self.s     },
            "s"  => Hex{ q: self.q,     r: self.r - 1, s: self.s + 1 },
            "sw" => Hex{ q: self.q - 1, r: self.r,     s: self.s + 1 },
            "nw" => Hex{ q: self.q + 1, r: self.r - 1, s: self.s     },
            _ => panic!("what direction is {}?", dir),
        }
    }
}

pub fn part1() {
    match read_file("d11.txt") {
        Ok(contents) => {
            let (max_dist,final_hex) = contents.split(",").fold((0,Hex::new()), |(max_dist,cur_hex), m| {
                let target_hex = cur_hex.move_to(m);
                (max(max_dist, Hex::new().distance_to(&target_hex)), target_hex)
            });
            println!("The distance to the little shit is {} hexes", Hex::new().distance_to(&final_hex));
            println!("At the worst, he was {} hexes away", max_dist);
        },
        Err(e) => println!("Error reading input file: {}", e),
    }
}

pub fn part2() {

}