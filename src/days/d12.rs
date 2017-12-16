use super::read_file;
use std::collections::{HashMap,HashSet,VecDeque};

fn parse(input: String) -> HashMap<u32,Vec<u32>> {
    let mut pipes: HashMap<u32,Vec<u32>> = HashMap::new();

    input.split("\n").map(|l| l.trim()).for_each(|l| {
        let chunks: Vec<&str> = l.split("<->").collect(); // should have 2 elements now, the source, and the targets
        let source = match chunks[0].trim().parse::<u32>() {
            Ok(n) => n,
            Err(e) => panic!("Failed to parse {}: {}", chunks[0], e)
        };
        let targets = chunks[1].split(",").map(|c| c.trim().parse::<u32>().unwrap()).collect();
        pipes.insert(source, targets);
    });

    pipes
}

fn group_participants(search_space: &HashMap<u32,Vec<u32>>, group_leader: u32) -> HashSet<u32> {
    let mut to_visit: VecDeque<u32> = VecDeque::new();
    let mut seen: HashSet<u32> = HashSet::new();

    to_visit.push_back(group_leader);
    while !to_visit.is_empty() {
        let cur = to_visit.pop_front().unwrap();
        seen.insert(cur);
        for n in search_space.get(&cur).unwrap() {
            if !seen.contains(&n) { to_visit.push_back(*n); }
        }
    }

    seen
}

fn count_in_group(search_space: &HashMap<u32,Vec<u32>>, group_leader: u32) -> usize {
    group_participants(search_space, group_leader).len()
}

pub fn part1() {
    match read_file("d12.txt") {
        Ok(contents) => {
            let groups = parse(contents);
            println!("There are {} programs in the group with 0", count_in_group(&groups, 0));
        },
        Err(e) => println!("error: {}", e),
    }
}

pub fn part2() {
    match read_file("d12.txt") {
        Ok(contents) => {
            let groups = parse(contents);
            let max_program_id = groups.keys().fold(0, |max_id,&id| if id > max_id { id } else { max_id });
            let mut seen_programs = HashSet::new();
            let mut group_count = 0;
            for program_id in 0..=max_program_id {
                if !seen_programs.contains(&program_id) {
                    seen_programs = group_participants(&groups, program_id).union(&seen_programs).map(|n| n.clone()).collect();
                    group_count += 1;
                }
            }
            println!("There are {} groups", group_count);
        },
        Err(e) => println!("error: {}", e),
    }
}