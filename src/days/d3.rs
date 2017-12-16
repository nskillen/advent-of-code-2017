use pooter::MemoryGrid;

pub fn part1() {
    // done by hand
}

pub fn part2() {
    let mut grid = MemoryGrid::new();

    loop {
        let n_sum = grid.neighbour_sum();
        grid.set(n_sum);
        if grid.get() > 277678 { break; }
        grid.next();
    }
    grid.print();
    println!("The value is {}", grid.get());
}