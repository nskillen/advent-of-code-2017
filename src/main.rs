#![feature(inclusive_range_syntax)]

#[macro_use] extern crate lazy_static;
extern crate num;
extern crate regex;
extern crate time;

use std::env;

mod days;
mod pooter;

use days::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("Usage: {} <d#>", args[0]),
        n => {
            for i in 1..n {
                match args[i].as_str() {
                    "d1"    => d1::part1(),
                    "d1_2"  => d1::part2(),
                    "d2"    => d2::part1(),
                    "d2_2"  => d2::part2(),
                    "d3"    => d3::part1(),
                    "d3_2"  => d3::part2(),
                    "d4"    => d4::part1(),
                    "d4_2"  => d4::part2(),
                    "d5"    => d5::part1(),
                    "d5_2"  => d5::part2(),
                    "d6"    => d6::part1(),
                    "d6_2"  => d6::part2(),
                    "d7"    => d7::part1(),
                    "d7_2"  => d7::part2(),
                    "d8"    => d8::part1(),
                    "d8_2"  => d8::part2(),
                    "d9"    => d9::part1(),
                    "d9_2"  => d9::part2(),
                    "d10"   => d10::part1(),
                    "d10_2" => d10::part2(),
                    "d11"   => d11::part1(),
                    "d11_2" => d11::part2(),
                    "d12"   => d12::part1(),
                    "d12_2" => d12::part2(),
                    "d13"   => d13::part1(),
                    "d13_2" => d13::part2(),
                    "d14"   => d14::part1(),
                    "d14_2" => d14::part2(),
                    "d15"   => d15::part1(),
                    "d15_2" => d15::part2(),
                    "d16"   => d16::part1(),
                    "d16_2" => d16::part2(),
                    d       => println!("Error: unknown day '{}'", d)
                }
            }
        }
    }
}
