use num::integer::lcm;
use regex::Regex;

#[derive(Clone,Debug)]
enum Instruction {
    Spin(usize),
    Exchange(usize,usize),
    Partner(String,String),
}

fn parse_instruction(instruction_str: &str) -> Result<Instruction,String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^s(?P<places>\d+)|x(?P<A>\d+)/(?P<B>\d+)|p(?P<a>[a-p])/(?P<b>[a-p])$").unwrap();
    }
    match RE.captures(instruction_str) {
        Some(caps) => {
            match instruction_str.chars().nth(0).unwrap() {
                's'   => Ok(Instruction::Spin     (caps.name("places").unwrap().as_str().parse::<usize>().unwrap())),
                'x'   => Ok(Instruction::Exchange (caps.name("A").unwrap().as_str().parse::<usize>().unwrap(), caps.name("B").unwrap().as_str().parse::<usize>().unwrap())),
                'p'   => Ok(Instruction::Partner  (caps.name("a").unwrap().as_str().to_string(), caps.name("b").unwrap().as_str().to_string())),
                other => Err(format!("Unknown instruction name: {}", other)),
            }
        },
        None => Err(format!("Unable to parse instruction: {}", instruction_str)),
    }
}

fn run(stage: Vec<char>, instructions: &Vec<Instruction>) -> Vec<char> {
    let mut new_stage = stage;
    instructions.iter().for_each(|i| {
        match i {
            &Instruction::Spin(x) => {
                let front = new_stage.len() - x;
                let     p1: Vec<char> = new_stage.iter().take(front).map(|c| c.clone()).collect();
                let mut p2: Vec<char> = new_stage.iter().skip(front).map(|c| c.clone()).collect();
                p2.extend(p1);
                new_stage = p2;
            },
            &Instruction::Exchange(a,b) => {
                let c = new_stage[a];
                new_stage[a] = new_stage[b];
                new_stage[b] = c;
            },
            &Instruction::Partner(ref a, ref b) => {
                let pA = new_stage.iter().position(|&e| e == a.chars().next().unwrap()).unwrap();
                let pB = new_stage.iter().position(|&e| e == b.chars().next().unwrap()).unwrap();
                let c = new_stage[pA];
                new_stage[pA] = new_stage[pB];
                new_stage[pB] = c;
            }
            _ => panic!("{:?} is not implemented!", i)
        }
    });
    new_stage
}

pub fn part1() {
    let instructions: Vec<Instruction> = include_str!("../../inputs/d16.txt").split(",").map(parse_instruction).map(|i| i.unwrap()).collect();
    let initial_places = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p'];
    let final_places = run(initial_places, &instructions);
    println!("The final arrangement is: {}", final_places.into_iter().collect::<String>());
}

pub fn part2() {
    let instructions: Vec<Instruction> = include_str!("../../inputs/d16.txt").split(",").map(parse_instruction).map(|i| i.unwrap()).collect();
    let non_partner_instructions = instructions.iter().filter(|&i| match i { &Instruction::Partner(_,_) => false, _ => true }).map(|i| i.clone()).collect();
    let partner_instructions     = instructions.iter().filter(|&i| match i { &Instruction::Partner(_,_) => true, _ => false }).map(|i| i.clone()).collect();

    let mut iters = 0;
    let mut non_partner_cycle_len = 0;
    let mut partner_cycle_len = 0;
    let string = "abcdefghijklmnop".to_string();
    let mut np_perm = string.clone();
    let mut p_perm = string.clone();
    loop {
        iters += 1;
        if non_partner_cycle_len == 0 {
            np_perm = run(np_perm.chars().collect(), &non_partner_instructions).iter().collect::<String>();
            if np_perm == string { non_partner_cycle_len = iters; }
        }
        if partner_cycle_len == 0 {
            p_perm  = run(p_perm.chars().collect(), &partner_instructions).iter().collect::<String>();
            if p_perm == string { partner_cycle_len = iters; }
        }
        if non_partner_cycle_len != 0 && partner_cycle_len != 0 { break; }
    }

    let cycle_len = lcm(partner_cycle_len, non_partner_cycle_len);
    
    let mut final_perm = string;
    for _ in 0..(1_000_000_000 % cycle_len) {
        final_perm = run(final_perm.chars().collect(), &instructions).iter().collect::<String>();
    }

    println!("The non-partner cycle length is {} iterations", non_partner_cycle_len);
    println!("The partner cycle length is {} iterations", partner_cycle_len);
    println!("The combined cycle length is {} iterations", cycle_len);
    println!("Need to calculate {} cycles (1e9 % {})", 1_000_000_000 % cycle_len, cycle_len);
    println!("The final arrangement, after 1 BILLION iterations, is: {}", final_perm);
}