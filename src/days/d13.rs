use super::read_file;
use time;

#[derive(Clone)]
struct Scanner {
    position: usize,
    range: usize,
    velocity: i32
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { position: 1, range: 1, velocity: 1 }
    }

    fn with_range(range: usize) -> Scanner {
        Scanner { position: 1, range: range, velocity: 1 }
    }

    fn step(&mut self) {
        self.position = (self.position as i32 + self.velocity) as usize;
        if self.position == 1 { self.velocity = 1; }
        else if self.position == self.range { self.velocity = -1; }
    }

    fn at_top(&self) -> bool {
        self.position == 1
    }

    fn reset(&mut self) {
        self.position = 1;
        self.velocity = 1;
    }
}

struct Firewall {
    layers: Vec<Option<Scanner>>
}

impl Firewall {
    fn new() -> Firewall {
        Firewall { layers: Vec::new() }
    }

    fn from_input(input: String) -> Firewall {
        let mut fw = Firewall::new();
        input.split("\n").map(|l| l.trim()).for_each(|l| {
            let depth_and_range: Vec<usize> = l.split(":").map(|dnr| {
                match dnr.trim().parse::<usize>() {
                    Ok(n) => n,
                    Err(e) => panic!("Error parsing {}: {}", dnr, e),
                }
            }).collect();
            fw.add_layer(depth_and_range[0], Scanner::with_range(depth_and_range[1]));
        });

        fw
    }

    fn add_layer(&mut self, layer_depth: usize, scanner: Scanner) {
        if self.layers.len() < layer_depth + 1 {
            self.layers.resize(layer_depth + 1, None);
        }

        self.layers[layer_depth] = Some(scanner);
    }

    fn step(&mut self) {
        self.layers.iter_mut().for_each(|l| {
            match l {
                &mut Some(ref mut scanner) => scanner.step(),
                &mut None => ()
            }
        });
    }

    fn reset(&mut self) {
        self.layers.iter_mut().for_each(|l| {
            match l {
                &mut Some(ref mut scanner) => scanner.reset(),
                &mut None => ()
            }
        });
    }

    fn severity(&self, depth: usize) -> u32 {
        match self.layers[depth] {
            Some(ref scanner) => if scanner.at_top() { (depth * scanner.range) as u32 } else { 0 },
            None => 0,
        }
    }

    fn final_layer(&self) -> usize {
        self.layers.len()
    }
}

pub fn part1() {
    match read_file("d13.txt") {
        Ok(content) => {
            let mut firewall = Firewall::from_input(content);
            let mut current_layer = 0;
            let mut total_severity = 0;

            while current_layer < firewall.final_layer() {
                total_severity += firewall.severity(current_layer);
                firewall.step();
                current_layer += 1;
            }

            println!("The total severity of your trip is {}", total_severity);
        },
        Err(e) => println!("Error: {}", e),
    }
}

pub fn part2() {
    match read_file("d13.txt") {
        Ok(content) => {
            let mut delay = 0;
            let scanners: Vec<(u32,u32)> = content.split("\n").map(|l| l.trim()).map(|l| {
                let halves: Vec<u32> = l.split(": ").map(|n| n.parse::<u32>().unwrap()).collect();
                (halves[0], halves[1])
            }).collect(); // vec![(x,y),(q,z),(etc...)]

            let start_time = time::precise_time_ns();
            let mut caught = false;
            loop {
                for &(l,r) in scanners.iter() {
                    let ps = delay + l;
                    if ps % (2*(r-1)) == 0 { caught = true; break; }
                }
                if !caught { break; }

                delay += 1;
                caught = false;
            }
            let end_time = time::precise_time_ns();

            println!("The shortest delay that results in not getting caught is {}ps", delay);
            println!("This took {}ns to calculate", end_time - start_time);
        },
        Err(e) => println!("Error: {}", e),
    }
}