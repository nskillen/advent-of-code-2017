use std::collections::HashSet;
use pooter::KnotHasher;

pub fn part1() {
    let key_string = include_str!("../../inputs/d14.txt");
    let mut total = 0;
    for r in 0..128 {
        let knot = KnotHasher::new().hash(&format!("{}-{}", key_string.trim(), r)[..]);
        total += knot.iter().map(|n| n.count_ones()).fold(0,|acc,n| acc + n);
        // let display_str: String = knot.iter()
        //     .map(|n| {
        //         format!("{:b}",n).chars()
        //         .map(|c| match c { '1' => '#', '0' => '.', _ => panic!("unexpected char {}", c) })
        //         .collect::<String>()
        //     })
        //     .collect();
        // println!("{}", display_str);
    }

    println!("There are {} squares used", total);
}

pub fn part2() {
    let key_string = include_str!("../../inputs/d14.txt");
    let mut total_regions = 0;
    let mut grid: Vec<Vec<bool>> = (0..128).map(|r| {
        KnotHasher::new().hash(&format!("{}-{}", key_string.trim(), r)[..]).iter()
        .map(|n| {
            format!("{:08b}",n).chars()
            .map(|c| match c { '1' => true, '0' => false, _ => panic!("unexpected char {}", c) })
            .collect::<Vec<bool>>()
        })
        .fold(Vec::new(), |mut acc,vb| { acc.extend(vb); acc })
    })
    .collect::<Vec<Vec<bool>>>();

    // at this point we have a vec of vec of bool that maps to the binary values returned from the 128 knothashes
    // now start at t-l corner, and flood fill each block of true, setting to false, and bumping total_regions by 1
    // repeat until no new region found, and print results

    type Idx = (usize,usize);
    
    for y in 0..128 {
        for x in 0..128 {
            if grid[y][x] {
                total_regions += 1; // bump the region counter here, the flood below will clear it
                let mut seen: HashSet<Idx> = HashSet::new();
                let mut to_check: Vec<Idx> = vec![(y,x)];

                while !to_check.is_empty() {
                    let idx = to_check.pop().unwrap(); // get next square to check
                    seen.insert(idx); // mark as seen so we don't go back
                    
                    if !grid[idx.0][idx.1] { continue; } // this square was false. skip it.

                    //square was good, mark as false so we can't find it again
                    grid[idx.0][idx.1] = false;

                    // square was good, check the neighbours
                    if idx.1 > 0 {
                        let new_idx = (idx.0, idx.1 - 1);
                        if !seen.contains(&new_idx) { to_check.push(new_idx); }
                    }
                    if idx.1 < 127 {
                        let new_idx = (idx.0, idx.1 + 1);
                        if !seen.contains(&new_idx) { to_check.push(new_idx); }
                    }

                    if idx.0 > 0 {
                        let new_idx = (idx.0 - 1, idx.1);
                        if !seen.contains(&new_idx) { to_check.push(new_idx); }
                    }
                    if idx.0 < 127 {
                        let new_idx = (idx.0 + 1, idx.1);
                        if !seen.contains(&new_idx) { to_check.push(new_idx); }
                    }
                }
            }
        }
    }

    println!("There are {} regions", total_regions);
}