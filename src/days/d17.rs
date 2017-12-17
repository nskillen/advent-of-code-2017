const PUZZLE_INPUT: usize = 376;

pub fn part1() {
    let mut buffer = vec![0];
    let mut cur_idx = 0;
    for n in 1..=2017 {
        let insert_idx = (cur_idx + PUZZLE_INPUT) % buffer.len() + 1;
        if insert_idx == buffer.len() {
            buffer.push(n);
        } else {
            buffer.insert(insert_idx, n);
        }
        cur_idx = insert_idx;
    }
    println!("The value after 2017 is {}", buffer[cur_idx + 1 % buffer.len()]);
}

pub fn part2() {
    let (_, val_after_zero) = (1..=50_000_000).fold((0,0), |acc,iter| {
        let next_index = (acc.0 + PUZZLE_INPUT) % iter;
        (next_index + 1, if next_index == 0 { iter } else { acc.1 })
    });

    println!("The value after 0 is {}", val_after_zero);
}