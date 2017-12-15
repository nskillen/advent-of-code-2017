use std::cmp;
use std::i32;

pub fn part1() {
    let contents = include_str!("../../inputs/d2.txt");
    let sum = contents.split("\n").map(|line| {
        line
        .split_whitespace()
        .map(|el| el.parse::<i32>().unwrap())
        .fold((i32::MAX, i32::MIN), |(min,max),val| ((cmp::min(min, val)),(cmp::max(max,val))))
    })
    .fold(0, |acc,(min,max)| acc + max - min);
    println!("The checksum is {}.", sum);
}

pub fn part2() {
    let contents = include_str!("../../inputs/d2.txt");
    let sum = contents.split("\n").map(|line| {
        let nums: Vec<i32> = line
        .split_whitespace()
        .map(|el| el.parse::<i32>().unwrap())
        .collect();

        for i in 0..nums.len() {
            for j in (i+1)..nums.len() {
                if nums[i] % nums[j] == 0 { return nums[i] / nums[j] }
                else if nums[j] % nums[i] == 0 { return nums[j] / nums[i] }
            }
        }
        0
    })
    .fold(0, |acc,n| acc + n);
    println!("The checksum is {}.", sum);
}