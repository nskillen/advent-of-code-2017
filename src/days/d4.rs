pub fn part1() {
    let contents = include_str!("../../inputs/d4.txt");
    let valid = contents
    .split("\n")
    .map(|line| {
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut sorted_words = words.clone();
        sorted_words.sort();
        sorted_words.dedup();
        if words.len() == sorted_words.len() { 1 } else { 0 }
    })
    .fold(0, |acc,was_valid| acc + was_valid);
    println!("There are {} valid passphrases", valid);
}

pub fn part2() {
    let contents = include_str!("../../inputs/d4.txt");
    let valid = contents
    .split("\n")
    .map(|line| {
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut sorted_words: Vec<String> = words
            .clone()
            .iter()
            .map(|word| {
                let mut wchars: Vec<char> = word.chars().collect();
                wchars.sort();
                wchars.into_iter().collect()
            }).collect();
        sorted_words.sort();
        sorted_words.dedup();
        if words.len() == sorted_words.len() { 1 } else { 0 }
    })
    .fold(0, |acc,was_valid| acc + was_valid);
    println!("There are {} valid passphrases", valid);
}