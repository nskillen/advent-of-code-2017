use pooter::MemoryBanks;

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