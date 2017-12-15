use super::read_file;
use regex::Regex;
use std::collections::HashMap;

struct Registers {
    regs: HashMap<String,i32>,
    largest_seen: i32,
}

impl Registers {
    pub fn new() -> Registers {
        Registers { regs: HashMap::new(), largest_seen: 0 }
    }

    pub fn get(&self, name: String) -> i32 {
        match self.regs.get(&name) {
            Some(val) => val.clone(),
            None      => 0 // registers start with a value of 0, pretend all possible registers exist
        }
    }

    pub fn put(&mut self, name: String, val: i32) {
        self.regs.insert(name, val);
        if val > self.largest_seen { self.largest_seen = val; }
    }

    pub fn max_value(&self) -> i32 {
        self.regs.values().fold(0,|a,&v| if a > v { a } else { v })
    }
}

fn process(registers: &mut Registers, instruction: &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<target>\w+) (?P<op>inc|dec) (?P<cval>-?\d+) if (?P<cmp_tgt>\w+) (?P<cmp_op>[!<>=]=?) (?P<cmp_val>-?\d+)$").unwrap();
    }

    RE.captures(instruction).and_then(|caps| {
        let tgt     = caps.name("target").unwrap().as_str().to_string();
        let op      = caps.name("op").unwrap().as_str();
        let cval    = caps.name("cval").unwrap().as_str().parse::<i32>().unwrap();
        let cmp_tgt = caps.name("cmp_tgt").unwrap().as_str().to_string();
        let cmp_op  = caps.name("cmp_op").unwrap().as_str();
        let cmp_val = caps.name("cmp_val").unwrap().as_str().parse::<i32>().unwrap();

        let ct_val = registers.get(cmp_tgt);
        let cmp_res = match cmp_op {
            "==" => ct_val == cmp_val,
            "!=" => ct_val != cmp_val,
            "<=" => ct_val <= cmp_val,
            ">=" => ct_val >= cmp_val,
            "<"  => ct_val <  cmp_val,
            ">"  => ct_val >  cmp_val,
            op   => panic!("Unknown comparison operation: {}", op),
        };

        if cmp_res {
            let ival = registers.get(tgt.clone());
            match op {
                "inc" => registers.put(tgt, ival + cval),
                "dec" => registers.put(tgt, ival - cval),
                uop   => panic!("Unsupported operation: {}", uop),
            }
        }

        Some(instruction)
    })
    .or_else(|| panic!("Unable to parse instruction: {}", instruction));
}

pub fn part1() {
    let mut registers = Registers::new();

    match read_file("d8.txt") {
        Ok(contents) => {
            contents.split("\n").map(|l| l.trim())
            .for_each(|i| process(&mut registers, i));

            println!("The largest value in a register is {}", registers.max_value());
        },
        Err(e) => println!("Error: {}", e)
    }
}

pub fn part2() {
    let mut registers = Registers::new();
    match read_file("d8.txt") {
        Ok(contents) => {
            contents.split("\n").map(|l| l.trim())
            .for_each(|i| process(&mut registers, i));

            println!("The largest value seen was {}", registers.largest_seen);
        },
        Err(e) => println!("Error: {}", e)
    }
}