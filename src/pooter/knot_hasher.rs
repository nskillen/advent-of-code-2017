use std::string::ToString;

pub struct KnotHash([u8; 16]);
impl KnotHash {
    pub fn iter(&self) -> KnotHashIterator {
        KnotHashIterator{ hash: self, cur_idx: 0 }
    }
}
impl ToString for KnotHash {
    fn to_string(&self) -> String {
        self.0.iter().map(|n| format!("{:02x}", n)).collect::<Vec<String>>().join("")
    }
}

pub struct KnotHashIterator<'k> {
    hash: &'k KnotHash,
    cur_idx: usize
}

impl<'k> Iterator for KnotHashIterator<'k> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_idx >= self.hash.0.len() { None }
        else {
            let idx = self.cur_idx;
            self.cur_idx += 1;
            Some(self.hash.0[idx])
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (self.hash.0.len(), Some(self.hash.0.len())) }
}

pub struct KnotHasher {
    lengths: Vec<u8>,
    list: Vec<u8>,
    current_position: usize,
    skip_size: usize,
}

impl KnotHasher {
    pub fn new() -> KnotHasher {
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

    pub fn hash(&mut self, input: &str) -> KnotHash {
        self.lengths = input.bytes().collect(); // Vec<u8>
        self.lengths.append(&mut vec![17, 31, 73, 47, 23]);

        for _ in 0..64 { self.round(); } // 64 rounds to form sparse hash

        let mut outputs = KnotHash([0; 16]);
        for i in 0..16 { outputs.0[i] = self.list[(i * 16)..((i+1) * 16)].iter().fold(0, |acc, n| acc ^ n); }

        outputs
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