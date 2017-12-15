use super::read_file;
use std::collections::HashSet;

struct MemoryBanks {
    banks: Vec<u32>
}

impl MemoryBanks {
    pub fn from_vec(v: Vec<u32>) -> MemoryBanks {
        MemoryBanks { banks: v }
    }
    pub fn redistribute(&mut self) -> u32 {
        let mut seen = HashSet::new();
        
        while !seen.contains(&self.banks) {
            seen.insert(self.banks.clone());
            let mut cur = self.banks.iter()
                .enumerate()
                .fold((0,0), |(midx,mval),(idx,val)| if *val > mval { (idx,*val) } else { (midx,mval) }) // 2-tuples of the form (index,value)
                .0;
            let mut to_alloc = self.banks[cur];
            self.banks[cur] = 0;
            cur += 1;
            if cur >= self.banks.len() { cur = 0; }
            while to_alloc > 0 {
                self.banks[cur] += 1;
                cur += 1;
                if cur >= self.banks.len() { cur = 0; }
                to_alloc -= 1;
            }
        }

        seen.len() as u32
    }
}

fn process_input(input: &str) -> Vec<u32> {
    input.split_whitespace()
    .map(|bank| bank.trim().parse::<u32>().unwrap())
    .collect()
}

pub fn part1() {
    let contents = include_str!("../../inputs/d6.txt");
    let mut banks: Vec<u32> = process_input(contents);
    let mut mbanks = MemoryBanks::from_vec(banks);
    let cycle_count = mbanks.redistribute();
    println!("There were {} cycles before a repeat was seen", cycle_count);
}

pub fn part2() {
    let contents = include_str!("../../inputs/d6.txt");
    let mut banks: Vec<u32> = process_input(contents);
    let mut mbanks = MemoryBanks::from_vec(banks);
    mbanks.redistribute();
    let cycle_count = mbanks.redistribute();
    println!("There are {} cycles in the loop", cycle_count);
}