use bit_vec::BitVec;
use serialize::hex::{FromHex};

pub fn part1() {
    //let contents = include_str!("../../inputs/d14.txt");
    let hasher = KnotHasher::new();
    let mut squares = 0;
    for r in 0..128 {
        let output = hasher.hash(format!("jxqlasbh-{}",r)[..]);
        let bytes = output.from_hex();
        let bitvec = BitVec::from_bytes(bytes);
        squares += bitvec.iter().filter(|x| *x).count();
    }
}

pub fn part2() {
    let contents = include_str!("../../inputs/d14.txt");
}

fn main() {
    part1();
    part2();
}