#![feature(inclusive_range_syntax)]

extern crate bit_vec;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate time;

use std::env;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("Usage: {} <d#>", args[0]),
        _ => match args[1].as_str() {
            "d1"    => days::d1::part1(),
            "d1_2"  => days::d1::part2(),
            "d2"    => days::d2::part1(),
            "d2_2"  => days::d2::part2(),
            "d3"    => days::d3::part1(),
            "d3_2"  => days::d3::part2(),
            "d4"    => days::d4::part1(),
            "d4_2"  => days::d4::part2(),
            "d5"    => days::d5::part1(),
            "d5_2"  => days::d5::part2(),
            "d6"    => days::d6::part1(),
            "d6_2"  => days::d6::part2(),
            "d7"    => days::d7::part1(),
            "d7_2"  => days::d7::part2(),
            "d8"    => days::d8::part1(),
            "d8_2"  => days::d8::part2(),
            "d9"    => days::d9::part1(),
            "d9_2"  => days::d9::part2(),
            "d10"   => days::d10::part1(),
            "d10_2" => days::d10::part2(),
            "d11"   => days::d11::part1(),
            "d11_2" => days::d11::part2(),
            "d12"   => days::d12::part1(),
            "d12_2" => days::d12::part2(),
            "d13"   => days::d13::part1(),
            "d13_2" => days::d13::part2(),
            "d14"   => days::d14::part1(),
            "d14_2" => days::d14::part2(),
            d       => println!("Error: unknown day '{}'", d)
        }
    }
}
