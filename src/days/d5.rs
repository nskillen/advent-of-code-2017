pub fn part1() {
    let contents = include_str!("../../inputs/d5.txt");
    let mut instructions: Vec<i32> = contents
        .split("\n")
        .map(|instr| instr.trim().parse::<i32>().unwrap())
        .collect();

    let mut offset: i32 = 0;
    let mut count = 0;

    while offset >= 0 && offset < instructions.len() as i32 {
        let old_offset = offset;
        count += 1;
        offset += instructions[offset as usize];
        instructions[old_offset as usize] += 1;
    }

    println!("It took {} steps to exit the jump maze", count);
}

pub fn part2() {
    let contents = include_str!("../../inputs/d5.txt");
    let mut instructions: Vec<i32> = contents
        .split("\n")
        .map(|instr| instr.trim().parse::<i32>().unwrap())
        .collect();
    
    let mut offset: i32 = 0;
    let mut count = 0;

    while offset >= 0 && offset < instructions.len() as i32 {
        let old_offset = offset as usize;
        let i = instructions[offset as usize];
        count += 1;
        offset += i;
        instructions[old_offset] += match i {
            n if n >= 3 => -1,
            _           =>  1
        }
    }

    println!("It took {} steps to exit the jump maze", count);
}