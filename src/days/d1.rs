pub fn part1() {
    let contents = include_str!("../../inputs/d1.txt");
    for (lnum,line) in contents.split_whitespace().enumerate() {
        let nums: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let (mut sum, _) = nums.iter().fold((0,0), |(total,prev),&n| ((if n == prev { total + n } else { total }), n));

        if nums.first().unwrap() == nums.last().unwrap() { sum += nums.first().unwrap(); }

        println!("The sum of line {} is {}", lnum, sum);
    }
}

pub fn part2() {
    let contents = include_str!("../../inputs/d1.txt");
    for (lnum,line) in contents.split_whitespace().enumerate() {
        let nums: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut sum = 0;
        for idx in 0..nums.len() {
            if nums[idx] == nums[(idx + nums.len() / 2) % nums.len()] {
                sum += nums[idx];
            }
        }

        println!("The sum of line {} is {}", lnum, sum);
    }
}