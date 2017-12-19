use std::collections::{HashMap,VecDeque};
use regex::Regex;

pub fn part1() {
    lazy_static! {
        static ref re: Regex = Regex::new(r"^(?P<cmd>snd|set|add|mul|mod|rcv|jgz) (?P<x>\w)(?: (?P<y>[\w-]+))?$").unwrap();
    }
    let contents = include_str!("../../inputs/d18.txt");
    let mut regs: HashMap<String,i64> = HashMap::new();
    let mut last_freq = 0;
    
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut idx = 0;
    while idx < lines.len() {
        let l = lines[idx];
        let caps = re.captures(l.trim()).unwrap();
        let cmd = caps.name("cmd").unwrap().as_str();
        let x = caps.name("x").unwrap().as_str().to_string();
        let xv = match regs.get(&x) {
            Some(&n) => n,
            None => 0,
        };
        let y = caps.name("y").map(|y| {
            match y.as_str().to_string().parse::<i64>() {
                Ok(n) => n,
                Err(_) => regs[&y.as_str().to_string()],
            }
        });

        match cmd {
            "snd" => {last_freq = xv;},
            "set" => {regs.insert(x, y.unwrap());},
            "add" => {regs.insert(x, xv + y.unwrap());},
            "mul" => {regs.insert(x, xv * y.unwrap());},
            "mod" => {regs.insert(x, xv % y.unwrap());},
            "rcv" => {println!("Recovered frequency: {}", last_freq);},
            "jgz" => if xv > 0 { let _y = y.unwrap(); if _y > 0 { idx += _y as usize } else { idx -= (_y * -1) as usize }; continue; },
            c => panic!("Unknown command: {}", c),
        }

        idx += 1;
    }
}

#[derive(Debug)]
struct Program {
    regs: HashMap<String,i64>,
    queue: VecDeque<i64>,
    instruction_count: usize,
    send_count: usize,
    recv_count: usize,
    to_send: Option<i64>,
    jump_val: i64,
    id: i64,
}

impl Program {
    fn new(program_id: i64) -> Self {
        let mut p = Program {
            regs: HashMap::new(),
            queue: VecDeque::new(),
            instruction_count: 0,
            send_count: 0,
            recv_count: 0,
            to_send: None,
            jump_val: 0,
            id: program_id,
        };

        p.set_reg(String::from("p"), program_id);

        p
    }

    fn get_jump_val(&mut self) -> i64 {
        let jv = self.jump_val;
        self.jump_val = 0;
        jv
    }

    fn set_reg(&mut self, reg: String, val: i64) { self.regs.insert(reg.clone(), val); }
    fn get_reg(&self, reg: &String) -> i64 { match self.regs.get(reg) { Some(&v) => v, None => 0 } }

    fn recv(&mut self, val: i64) { self.queue.push_back(val); }

    fn send(&mut self) -> Option<i64> {
        let val_to_send = self.to_send;
        self.to_send = None;
        val_to_send
    }

    fn exec(&mut self, instr: String) -> bool {
        lazy_static! {
            static ref re: Regex = Regex::new(r"^(?P<cmd>snd|set|add|mul|mod|rcv|jgz) (?P<x>\w)(?: (?P<y>[\w-]+))?$").unwrap();
        }

        let caps = re.captures(instr.trim()).unwrap();
        let cmd = caps.name("cmd").unwrap().as_str();
        let x = caps.name("x").unwrap().as_str().to_string();
        let xv = match x.parse::<i64>() {
            Ok(v) => v,
            Err(_) => self.get_reg(&x),
        };
        let y = caps.name("y").map(|y| {
            match y.as_str().to_string().parse::<i64>() {
                Ok(n) => n,
                Err(_) => self.get_reg(&y.as_str().to_string()),
            }
        });

        match cmd {
            "snd" => {self.to_send = Some(xv); self.send_count += 1;},
            "set" => {self.set_reg(x, y.unwrap());},
            "add" => {self.set_reg(x, xv + y.unwrap());},
            "mul" => {self.set_reg(x, xv * y.unwrap());},
            "mod" => {self.set_reg(x, xv % y.unwrap());},
            "rcv" => {
                if self.queue.len() > 0 {
                    self.recv_count += 1;
                    let val = self.queue.pop_front().unwrap();
                    self.set_reg(x, val);
                } else {
                    return false
                }
            },
            "jgz" => if xv > 0 { self.jump_val = y.unwrap() },
            c => panic!("Unknown command: {}", c),
        }

        self.instruction_count += 1;
        true
    }
}

pub fn part2() {
    
    let contents = include_str!("../../inputs/d18.txt");
    let instructions: Vec<String> = contents.lines().map(|l| l.trim().to_string()).collect();
    
    let mut p1 = Program::new(0);
    let mut p2 = Program::new(1);
    let mut p1_idx = 0;
    let mut p2_idx = 0;

    loop {
        let i1 = instructions[p1_idx].clone();
        let i2 = instructions[p2_idx].clone();

        match p1.send() {
            Some(v) => p2.recv(v),
            None => (),
        }

        match p2.send() {
            Some(v) => p1.recv(v),
            None => (),
        }

        let r1 = p1.exec(i1);
        let j1 = p1.get_jump_val();
        match (r1,j1) {
            (false, _)         => (),
            (true, 0)          => { p1_idx += 1; },
            (true, n) if n < 0 => { p1_idx -= (n * -1) as usize; },
            (true, n) if n > 0 => { p1_idx += n as usize; },
            (true, n) => panic!("how is n {}?", n)
        }

        let r2 = p2.exec(i2);
        let j2 = p2.get_jump_val();
        match (r2,j2) {
            (false, _)         => (),
            (true, 0)          => { p2_idx += 1; },
            (true, n) if n < 0 => { p2_idx -= (n * -1) as usize; },
            (true, n) if n > 0 => { p2_idx += n as usize; },
            (true, n) => panic!("how is n {}?", n)
        }

        if !r1 && !r2 { break; }
    }

    println!("p1 sent {} values before deadlocking", p1.send_count);
    println!("{:?}\n{:?}", p1, p2);
}