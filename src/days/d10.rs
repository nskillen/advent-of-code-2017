use std::env;

struct KnotHasher {
    lengths: Vec<u8>,
    list: Vec<u8>,
    current_position: usize,
    skip_size: usize,
}

impl KnotHasher {
    fn new() -> KnotHasher {
        KnotHasher {
            lengths: Vec::new(),
            list: (0..=0xFF).collect(),
            current_position: 0,
            skip_size: 0,
        }
    }

    pub fn part1(&mut self) -> u32 {
        self.lengths = vec![157,222,1,2,177,254,0,228,159,140,249,187,255,51,76,30];
        self.round();
        self.list[0] as u32 * self.list[1] as u32
    }

    pub fn hash(&mut self, input: &str) -> String {
        self.lengths = input.bytes().collect(); // Vec<u8>
        self.lengths.append(&mut vec![17, 31, 73, 47, 23]);

        for _ in 0..64 { self.round(); } // 64 rounds to form sparse hash

        let mut outputs = [0; 16];
        for i in 0..16 { outputs[i] = self.list[(i * 16)..((i+1) * 16)].iter().fold(0, |acc, n| acc ^ n); }

        outputs.iter().map(|n| format!("{:02x}", n)).collect::<Vec<String>>().join("")
    }

    fn round(&mut self) {
        let list_len = self.list.len();
        for idx in 0..self.lengths.len() {
            let length: u8 = self.lengths[idx];
            let mut front = self.current_position;
            let mut back = self.current_position + length as usize - 1;
            while front < back {
                self.list.swap((front % list_len),(back % list_len));
                front += 1;
                back -= 1;
            }

            self.current_position += length as usize + self.skip_size;
            self.current_position %= list_len;
            self.skip_size += 1;
        }
    }
}

pub fn part1() {
    println!("The product of the first two numbers is: {}", KnotHasher::new().part1());
}

pub fn part2() {
    let args: Vec<_> = env::args().collect();
    println!("The hex string produced is: {}", KnotHasher::new().hash(&args[2].trim()));
}