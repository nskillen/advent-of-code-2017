use pooter::Generator;
// use std::iter::{Iterator};

// struct Generator {
//     val: u64,
//     factor: u64,
//     modulo: u64,
// }

// impl Generator {
//     fn new(seed: u64, factor: u64) -> Self {
//         Self {
//             val: seed,
//             factor: factor,
//             modulo: 2147483647
//         }
//     }
// }

// impl Iterator for Generator {
//     type Item = u64;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.val = self.val * self.factor % self.modulo;
//         Some(self.val)
//     }

//     fn size_hint(&self) -> (usize, Option<usize>) {
//         (usize::max_value(), None)
//     }
// }

pub fn part1() {
    let count1 = Generator::new(277, 16807).take(40_000_000)
        .zip(Generator::new(349, 48271).take(40_000_000))
        .fold(0, |acc,(v1,v2)| if v1 & 0xFFFF == v2 & 0xFFFF { acc + 1 } else { acc });

    println!("The final count for part 1 is {}", count1);
}

pub fn part2() {
    let count2 = Generator::new(277, 16807).filter(|&n| n % 4 == 0).take(5_000_000)
        .zip(Generator::new(349, 48271).filter(|&n| n % 8 == 0).take(5_000_000))
        .fold(0, |acc,(v1,v2)| if v1 & 0xFFFF == v2 & 0xFFFF { acc + 1 } else { acc });

    println!("The final count for part 2 is {}", count2);
}