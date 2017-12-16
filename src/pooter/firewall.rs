use super::scanner::Scanner;

pub struct Firewall {
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
            Some(ref scanner) => if scanner.at_top() { (depth * scanner.get_range()) as u32 } else { 0 },
            None => 0,
        }
    }

    fn final_layer(&self) -> usize {
        self.layers.len()
    }
}